//! RDB serialization and deserialization for graph persistence.
//!
//! This module provides functions to save and load a [`Graph`] to/from Redis
//! RDB format. The serialization format is versioned for backward compatibility.
//!
//! ## Encoding Format (v14)
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │ Header                                   │
//! │  ├── encoding_version (u64)              │
//! │  ├── graph_name (string)                 │
//! │  ├── node_count (u64)                    │
//! │  ├── edge_count (u64)                    │
//! │  ├── deleted_node_count (u64)            │
//! │  ├── deleted_edge_count (u64)            │
//! │  ├── label_count (u64)                   │
//! │  └── rel_type_count (u64)                │
//! ├─────────────────────────────────────────┤
//! │ Schema                                   │
//! │  ├── node_attr_count + names             │
//! │  ├── rel_attr_count + names              │
//! │  ├── label names                         │
//! │  └── rel type names                      │
//! ├─────────────────────────────────────────┤
//! │ Nodes: [id, prop_count, [attr_id, val]] │
//! ├─────────────────────────────────────────┤
//! │ Edges: [id, prop_count, [attr_id, val]] │
//! ├─────────────────────────────────────────┤
//! │ Deleted entities (roaring bitmaps)       │
//! ├─────────────────────────────────────────┤
//! │ Matrices (GxB serialized blobs)          │
//! └─────────────────────────────────────────┘
//! ```

use std::io::Cursor;
use std::sync::Arc;

use roaring::RoaringTreemap;

use crate::graph::graph::Graph;
use crate::graph::matrix::Matrix;
use crate::graph::tensor::Tensor;
use crate::graph::versioned_matrix::VersionedMatrix;
use crate::runtime::orderset::OrderSet;
use crate::runtime::value::Value;

use super::GraphBLAS::{
    GrB_BOOL, GrB_Info, GrB_Matrix, GxB_Matrix_deserialize, GxB_Matrix_serialize,
};

/// Current encoding version for the Rust serializer.
const ENCODING_VERSION: u64 = 14;

/// Trait abstracting Redis module IO for save operations.
pub trait RdbSaveIO {
    fn save_unsigned(
        &mut self,
        val: u64,
    );
    fn save_signed(
        &mut self,
        val: i64,
    );
    fn save_double(
        &mut self,
        val: f64,
    );
    fn save_float(
        &mut self,
        val: f32,
    );
    fn save_string(
        &mut self,
        val: &str,
    );
    fn save_slice(
        &mut self,
        val: &[u8],
    );
}

/// Trait abstracting Redis module IO for load operations.
pub trait RdbLoadIO {
    fn load_unsigned(&mut self) -> u64;
    fn load_signed(&mut self) -> i64;
    fn load_double(&mut self) -> f64;
    fn load_float(&mut self) -> f32;
    fn load_string(&mut self) -> String;
    fn load_slice(&mut self) -> Vec<u8>;
}

// ---------------------------------------------------------------------------
// Save
// ---------------------------------------------------------------------------

/// Saves the graph to the RDB stream.
pub fn rdb_save(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
    graph_name: &str,
) {
    save_header(io, graph, graph_name);
    save_schema(io, graph);
    save_nodes(io, graph);
    save_edges(io, graph);
    save_deleted_entities(io, graph);
    save_matrices(io, graph);
}

fn save_header(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
    graph_name: &str,
) {
    io.save_unsigned(ENCODING_VERSION);
    io.save_string(graph_name);
    io.save_unsigned(graph.node_count());
    io.save_unsigned(graph.relationship_count());
    io.save_unsigned(graph.deleted_nodes().len());
    io.save_unsigned(graph.deleted_relationships().len());
    io.save_unsigned(graph.labels_count() as u64);
    io.save_unsigned(graph.relationship_types_count() as u64);
}

fn save_schema(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
) {
    // Node attribute names
    let node_attrs = graph.node_attr_names();
    io.save_unsigned(node_attrs.len() as u64);
    for name in &node_attrs {
        io.save_string(name);
    }

    // Relationship attribute names
    let rel_attrs = graph.relationship_attr_names();
    io.save_unsigned(rel_attrs.len() as u64);
    for name in &rel_attrs {
        io.save_string(name);
    }

    // Label names
    for name in &graph.get_labels() {
        io.save_string(name);
    }

    // Relationship type names
    for name in &graph.get_types() {
        io.save_string(name);
    }
}

fn save_value(
    io: &mut impl RdbSaveIO,
    value: &Value,
) {
    match value {
        Value::Null => {
            io.save_unsigned(0);
        }
        Value::Bool(b) => {
            io.save_unsigned(1);
            io.save_signed(i64::from(*b));
        }
        Value::Int(i) => {
            io.save_unsigned(2);
            io.save_signed(*i);
        }
        Value::Float(f) => {
            io.save_unsigned(3);
            io.save_double(*f);
        }
        Value::String(s) => {
            io.save_unsigned(4);
            io.save_string(s);
        }
        Value::List(list) => {
            io.save_unsigned(5);
            io.save_unsigned(list.len() as u64);
            for v in list.iter() {
                save_value(io, v);
            }
        }
        Value::Point(p) => {
            io.save_unsigned(6);
            io.save_double(f64::from(p.latitude));
            io.save_double(f64::from(p.longitude));
        }
        Value::VecF32(vec) => {
            io.save_unsigned(7);
            let bytes: Vec<u8> = vec.iter().flat_map(|f| f.to_le_bytes()).collect();
            io.save_slice(&bytes);
        }
        Value::Date(ts) => {
            io.save_unsigned(8);
            io.save_signed(*ts);
        }
        Value::Time(ns) => {
            io.save_unsigned(9);
            io.save_signed(*ns);
        }
        Value::Datetime(ts) => {
            io.save_unsigned(10);
            io.save_signed(*ts);
        }
        Value::Duration(ms) => {
            io.save_unsigned(11);
            io.save_signed(*ms);
        }
        Value::Arc(inner) => {
            save_value(io, inner);
        }
        _ => {
            // Node/Relationship/Path/Map not persisted as entity properties
            io.save_unsigned(0);
        }
    }
}

fn save_nodes(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
) {
    let empty = OrderSet::default();
    for node_id in graph.get_nodes(&empty, 0) {
        let id: u64 = node_id.into();
        io.save_unsigned(id);
        let attrs = graph.get_node_all_attrs(node_id);
        io.save_unsigned(attrs.len() as u64);
        for (name, value) in attrs.iter() {
            let attr_id = graph
                .get_node_attribute_id(name)
                .expect("missing node attribute id during save");
            io.save_unsigned(attr_id as u64);
            save_value(io, value);
        }
    }
}

fn save_edges(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
) {
    for (edge_id, _type_id) in graph.all_relationship_ids() {
        io.save_unsigned(edge_id);
        let attrs = graph.get_relationship_all_attrs_by_raw_id(edge_id);
        io.save_unsigned(attrs.len() as u64);
        for (name, value) in attrs.iter() {
            let attr_id = graph
                .get_relationship_attribute_id_by_name(name)
                .expect("missing relationship attribute id during save");
            io.save_unsigned(attr_id as u64);
            save_value(io, value);
        }
    }
}

fn save_deleted_entities(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
) {
    // Deleted nodes
    let mut buf = Vec::new();
    graph
        .deleted_nodes()
        .serialize_into(&mut buf)
        .expect("failed to serialize deleted nodes bitmap");
    io.save_slice(&buf);

    // Deleted relationships
    buf.clear();
    graph
        .deleted_relationships()
        .serialize_into(&mut buf)
        .expect("failed to serialize deleted relationships bitmap");
    io.save_slice(&buf);
}

fn serialize_matrix(
    io: &mut impl RdbSaveIO,
    matrix: &Matrix,
) {
    matrix.wait();
    let raw = matrix.raw();
    unsafe {
        let mut blob: *mut std::os::raw::c_void = std::ptr::null_mut();
        let mut blob_size: u64 = 0;
        let info = GxB_Matrix_serialize(
            &raw mut blob,
            &raw mut blob_size,
            **raw,
            std::ptr::null_mut(),
        );
        assert_eq!(
            info,
            GrB_Info::GrB_SUCCESS,
            "GxB_Matrix_serialize failed: {info:?}"
        );
        let slice = std::slice::from_raw_parts(blob as *const u8, blob_size as usize);
        io.save_slice(slice);
        super::matrix::graphblas_free(blob);
    }
}

fn save_versioned_matrix(
    io: &mut impl RdbSaveIO,
    vm: &VersionedMatrix,
) {
    // Save all 3 components to avoid GraphBLAS mutation ops (fork-safe).
    serialize_matrix(io, vm.committed_matrix());
    serialize_matrix(io, vm.delta_plus());
    serialize_matrix(io, vm.delta_minus());
}

fn save_matrices(
    io: &mut impl RdbSaveIO,
    graph: &Graph,
) {
    // Adjacency matrix
    save_versioned_matrix(io, graph.adjacency_matrix());

    // All nodes matrix
    save_versioned_matrix(io, graph.all_nodes_matrix());

    // Node labels matrix
    save_versioned_matrix(io, graph.node_labels_matrix());

    // Relationship type matrix
    save_versioned_matrix(io, graph.relationship_type_matrix());

    // Label matrices
    io.save_unsigned(graph.label_matrices().len() as u64);
    for lm in graph.label_matrices() {
        save_versioned_matrix(io, lm);
    }

    // Relationship tensors
    io.save_unsigned(graph.relationship_tensors().len() as u64);
    for tensor in graph.relationship_tensors() {
        // Forward matrix (m)
        save_versioned_matrix(io, tensor.matrix());
        // Transpose matrix (mt)
        save_versioned_matrix(io, tensor.transpose_matrix());
        // Edge ID matrix (me)
        save_versioned_matrix(io, tensor.edge_matrix());
    }
}

// ---------------------------------------------------------------------------
// Load
// ---------------------------------------------------------------------------

/// Loads a graph from the RDB stream.
pub fn rdb_load(
    io: &mut impl RdbLoadIO,
    cache_size: usize,
) -> (String, Graph) {
    let version = io.load_unsigned();
    assert_eq!(version, ENCODING_VERSION, "Unsupported encoding version");
    load_v14(io, cache_size)
}

fn load_v14(
    io: &mut impl RdbLoadIO,
    cache_size: usize,
) -> (String, Graph) {
    let graph_name = io.load_string();
    let node_count = io.load_unsigned();
    let edge_count = io.load_unsigned();
    let deleted_node_count = io.load_unsigned();
    let deleted_edge_count = io.load_unsigned();
    let label_count = io.load_unsigned();
    let rel_type_count = io.load_unsigned();

    // Schema
    let node_attr_count = io.load_unsigned();
    let mut node_attr_names = OrderSet::default();
    for _ in 0..node_attr_count {
        node_attr_names.insert(Arc::new(io.load_string()));
    }

    let rel_attr_count = io.load_unsigned();
    let mut rel_attr_names = OrderSet::default();
    for _ in 0..rel_attr_count {
        rel_attr_names.insert(Arc::new(io.load_string()));
    }

    let mut label_names = Vec::with_capacity(label_count as usize);
    for _ in 0..label_count {
        label_names.push(Arc::new(io.load_string()));
    }

    let mut rel_type_names = Vec::with_capacity(rel_type_count as usize);
    for _ in 0..rel_type_count {
        rel_type_names.push(Arc::new(io.load_string()));
    }

    // Create graph with sufficient capacity
    let total_nodes = node_count + deleted_node_count;
    let total_edges = edge_count + deleted_edge_count;
    let node_cap = total_nodes
        .checked_next_power_of_two()
        .unwrap_or(total_nodes)
        .max(16384);
    let edge_cap = total_edges
        .checked_next_power_of_two()
        .unwrap_or(total_edges)
        .max(16384);

    let mut graph = Graph::new(node_cap, edge_cap, cache_size, 0, &graph_name);

    // Restore schema
    graph.set_node_attr_names(node_attr_names);
    graph.set_relationship_attr_names(rel_attr_names);
    graph.set_label_names(label_names);
    graph.set_relationship_type_names(rel_type_names);

    // Load nodes
    for _ in 0..node_count {
        let node_id = io.load_unsigned();
        let prop_count = io.load_unsigned();
        let mut attrs = crate::runtime::ordermap::OrderMap::default();
        for _ in 0..prop_count {
            let attr_id = io.load_unsigned() as usize;
            let value = load_value(io);
            if let Some(name) = graph.node_attr_name_by_id(attr_id) {
                attrs.insert(name, value);
            }
        }
        graph.restore_node(node_id, attrs);
    }

    // Load edges
    for _ in 0..edge_count {
        let edge_id = io.load_unsigned();
        let prop_count = io.load_unsigned();
        let mut attrs = crate::runtime::ordermap::OrderMap::default();
        for _ in 0..prop_count {
            let attr_id = io.load_unsigned() as usize;
            let value = load_value(io);
            if let Some(name) = graph.relationship_attr_name_by_id(attr_id) {
                attrs.insert(name, value);
            }
        }
        graph.restore_edge(edge_id, attrs);
    }

    // Load deleted entities
    let deleted_nodes_bytes = io.load_slice();
    let deleted_nodes = if deleted_nodes_bytes.is_empty() {
        RoaringTreemap::new()
    } else {
        RoaringTreemap::deserialize_from(&mut Cursor::new(&deleted_nodes_bytes))
            .expect("Failed to deserialize deleted nodes bitmap")
    };

    let deleted_rels_bytes = io.load_slice();
    let deleted_rels = if deleted_rels_bytes.is_empty() {
        RoaringTreemap::new()
    } else {
        RoaringTreemap::deserialize_from(&mut Cursor::new(&deleted_rels_bytes))
            .expect("Failed to deserialize deleted relationships bitmap")
    };

    graph.set_deleted_nodes(deleted_nodes);
    graph.set_deleted_relationships(deleted_rels);
    graph.set_node_count(node_count);
    graph.set_relationship_count_val(edge_count);

    // Load matrices
    let adj_matrix = deserialize_versioned_matrix(io);
    let all_nodes_matrix = deserialize_versioned_matrix(io);
    let node_labels_matrix = deserialize_versioned_matrix(io);
    let rel_type_matrix = deserialize_versioned_matrix(io);

    let label_matrix_count = io.load_unsigned();
    let mut label_matrices = Vec::with_capacity(label_matrix_count as usize);
    for _ in 0..label_matrix_count {
        label_matrices.push(deserialize_versioned_matrix(io));
    }

    let tensor_count = io.load_unsigned();
    let mut tensors = Vec::with_capacity(tensor_count as usize);
    for _ in 0..tensor_count {
        let m = deserialize_versioned_matrix(io);
        let mt = deserialize_versioned_matrix(io);
        let me = deserialize_versioned_matrix(io);
        tensors.push(Tensor::from_parts(m, mt, me));
    }

    graph.set_adjacency_matrix(adj_matrix);
    graph.set_all_nodes_matrix(all_nodes_matrix);
    graph.set_node_labels_matrix(node_labels_matrix);
    graph.set_relationship_type_matrix(rel_type_matrix);
    graph.set_label_matrices(label_matrices);
    graph.set_relationship_tensors(tensors);

    graph.commit_attrs();

    (graph_name, graph)
}

fn load_value(io: &mut impl RdbLoadIO) -> Value {
    let tag = io.load_unsigned();
    match tag {
        0 => Value::Null,
        1 => Value::Bool(io.load_signed() != 0),
        2 => Value::Int(io.load_signed()),
        3 => Value::Float(io.load_double()),
        4 => Value::String(Arc::new(io.load_string())),
        5 => {
            let len = io.load_unsigned() as usize;
            let mut list = thin_vec::ThinVec::with_capacity(len);
            for _ in 0..len {
                list.push(load_value(io));
            }
            Value::List(list)
        }
        6 => {
            let lat = io.load_double() as f32;
            let lon = io.load_double() as f32;
            Value::Point(crate::runtime::value::Point::new(lat, lon))
        }
        7 => {
            let bytes = io.load_slice();
            assert!(
                bytes.len() % 4 == 0,
                "corrupt VecF32 data: length {} not a multiple of 4",
                bytes.len()
            );
            let vec: thin_vec::ThinVec<f32> = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                .collect();
            Value::VecF32(vec)
        }
        8 => Value::Date(io.load_signed()),
        9 => Value::Time(io.load_signed()),
        10 => Value::Datetime(io.load_signed()),
        11 => Value::Duration(io.load_signed()),
        _ => Value::Null,
    }
}

fn deserialize_matrix(io: &mut impl RdbLoadIO) -> Matrix {
    let blob = io.load_slice();
    unsafe {
        let mut m_ptr: GrB_Matrix = std::ptr::null_mut();
        let info = GxB_Matrix_deserialize(
            &raw mut m_ptr,
            GrB_BOOL,
            blob.as_ptr() as *const std::os::raw::c_void,
            blob.len() as u64,
            std::ptr::null_mut(),
        );
        assert_eq!(
            info,
            GrB_Info::GrB_SUCCESS,
            "GxB_Matrix_deserialize failed: {info:?}"
        );
        Matrix::from_raw(m_ptr)
    }
}

fn deserialize_versioned_matrix(io: &mut impl RdbLoadIO) -> VersionedMatrix {
    let m = deserialize_matrix(io);
    let dp = deserialize_matrix(io);
    let dm = deserialize_matrix(io);
    VersionedMatrix::from_parts(m, dp, dm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::matrix::{New, Set, Size};
    use crate::runtime::ordermap::OrderMap;
    use std::sync::Once;
    use thin_vec::thin_vec;

    static INIT: Once = Once::new();

    fn init_graphblas() {
        INIT.call_once(|| unsafe {
            super::super::GraphBLAS::GrB_init(
                super::super::GraphBLAS::GrB_Mode::GrB_NONBLOCKING as _,
            );
        });
    }

    /// Buffer-based mock IO for testing serialization round-trips.
    struct MockIO {
        buf: Vec<u8>,
        pos: usize,
    }

    impl MockIO {
        fn new() -> Self {
            Self {
                buf: Vec::new(),
                pos: 0,
            }
        }

        fn write_bytes(
            &mut self,
            data: &[u8],
        ) {
            self.buf.extend_from_slice(data);
        }

        fn read_bytes(
            &mut self,
            n: usize,
        ) -> Vec<u8> {
            let end = self.pos + n;
            assert!(end <= self.buf.len(), "read past end of buffer");
            let data = self.buf[self.pos..end].to_vec();
            self.pos = end;
            data
        }
    }

    impl RdbSaveIO for MockIO {
        fn save_unsigned(
            &mut self,
            val: u64,
        ) {
            self.write_bytes(&val.to_le_bytes());
        }
        fn save_signed(
            &mut self,
            val: i64,
        ) {
            self.write_bytes(&val.to_le_bytes());
        }
        fn save_double(
            &mut self,
            val: f64,
        ) {
            self.write_bytes(&val.to_le_bytes());
        }
        fn save_float(
            &mut self,
            val: f32,
        ) {
            self.write_bytes(&val.to_le_bytes());
        }
        fn save_string(
            &mut self,
            val: &str,
        ) {
            let bytes = val.as_bytes();
            self.write_bytes(&(bytes.len() as u64).to_le_bytes());
            self.write_bytes(bytes);
        }
        fn save_slice(
            &mut self,
            val: &[u8],
        ) {
            self.write_bytes(&(val.len() as u64).to_le_bytes());
            self.write_bytes(val);
        }
    }

    impl RdbLoadIO for MockIO {
        fn load_unsigned(&mut self) -> u64 {
            let bytes = self.read_bytes(8);
            u64::from_le_bytes(bytes.try_into().unwrap())
        }
        fn load_signed(&mut self) -> i64 {
            let bytes = self.read_bytes(8);
            i64::from_le_bytes(bytes.try_into().unwrap())
        }
        fn load_double(&mut self) -> f64 {
            let bytes = self.read_bytes(8);
            f64::from_le_bytes(bytes.try_into().unwrap())
        }
        fn load_float(&mut self) -> f32 {
            let bytes = self.read_bytes(4);
            f32::from_le_bytes(bytes.try_into().unwrap())
        }
        fn load_string(&mut self) -> String {
            let len = self.load_unsigned() as usize;
            let bytes = self.read_bytes(len);
            String::from_utf8(bytes).unwrap()
        }
        fn load_slice(&mut self) -> Vec<u8> {
            let len = self.load_unsigned() as usize;
            self.read_bytes(len)
        }
    }

    /// Build a graph with nodes, edges, attributes, labels, and deleted entities,
    /// then serialize and deserialize it, and verify the round-trip preserves state.
    #[test]
    fn test_serializer_round_trip() {
        init_graphblas();

        let graph_name = "test_graph";
        let mut graph = Graph::new(16384, 16384, 25, 0, graph_name);

        // Set up schema
        let mut node_attr_names = OrderSet::default();
        node_attr_names.insert(Arc::new("name".to_string()));
        node_attr_names.insert(Arc::new("age".to_string()));
        graph.set_node_attr_names(node_attr_names);

        let mut rel_attr_names = OrderSet::default();
        rel_attr_names.insert(Arc::new("since".to_string()));
        graph.set_relationship_attr_names(rel_attr_names);

        graph.set_label_names(vec![Arc::new("Person".to_string())]);
        graph.set_relationship_type_names(vec![Arc::new("KNOWS".to_string())]);

        // Insert node attributes
        let mut attrs0 = OrderMap::default();
        attrs0.insert(Arc::new("name".to_string()), Value::String(Arc::new("Alice".to_string())));
        attrs0.insert(Arc::new("age".to_string()), Value::Int(30));
        graph.restore_node(0, attrs0);

        let mut attrs1 = OrderMap::default();
        attrs1.insert(Arc::new("name".to_string()), Value::String(Arc::new("Bob".to_string())));
        attrs1.insert(Arc::new("age".to_string()), Value::Int(25));
        graph.restore_node(1, attrs1);

        // Insert edge attributes
        let mut edge_attrs = OrderMap::default();
        edge_attrs.insert(Arc::new("since".to_string()), Value::Int(2020));
        graph.restore_edge(0, edge_attrs);

        // Set node/edge counts
        graph.set_node_count(2);
        graph.set_relationship_count_val(1);

        // Set deleted entities
        let mut deleted_nodes = RoaringTreemap::new();
        deleted_nodes.insert(5);
        graph.set_deleted_nodes(deleted_nodes);

        let mut deleted_rels = RoaringTreemap::new();
        deleted_rels.insert(3);
        graph.set_deleted_relationships(deleted_rels);

        // Populate matrices with entries so nodes/edges are iterable
        // all_nodes_matrix: mark nodes 0 and 1 as existing (diagonal entries)
        let mut all_nodes = VersionedMatrix::new(16384, 16384);
        all_nodes.set(0, 0, true);
        all_nodes.set(1, 1, true);
        graph.set_all_nodes_matrix(all_nodes);

        // adjacency matrix: edge from 0 -> 1
        let mut adj = VersionedMatrix::new(16384, 16384);
        adj.set(0, 1, true);
        graph.set_adjacency_matrix(adj);

        // node_labels_matrix
        let node_labels = VersionedMatrix::new(16384, 16384);
        graph.set_node_labels_matrix(node_labels);

        // relationship_type_matrix: edge 0 at column 0 (type index 0)
        let mut rel_type = VersionedMatrix::new(16384, 16384);
        rel_type.set(0, 0, true);
        graph.set_relationship_type_matrix(rel_type);

        // label matrices: one label matrix with nodes 0, 1
        let mut label_mat = VersionedMatrix::new(16384, 16384);
        label_mat.set(0, 0, true);
        label_mat.set(1, 0, true);
        graph.set_label_matrices(vec![label_mat]);

        // relationship tensors: one tensor for "KNOWS"
        let mut t_m = VersionedMatrix::new(16384, 16384);
        t_m.set(0, 1, true);
        let mut t_mt = VersionedMatrix::new(16384, 16384);
        t_mt.set(1, 0, true);
        let t_me = VersionedMatrix::new(
            crate::graph::tensor::GrB_INDEX_MAX,
            crate::graph::tensor::GrB_INDEX_MAX,
        );
        graph.set_relationship_tensors(vec![Tensor::from_parts(t_m, t_mt, t_me)]);

        graph.commit_attrs();

        // Serialize
        let mut save_io = MockIO::new();
        rdb_save(&mut save_io, &graph, graph_name);

        // Deserialize
        save_io.pos = 0;
        let (loaded_name, loaded_graph) = rdb_load(&mut save_io, 25);

        // Verify header
        assert_eq!(loaded_name, graph_name);
        assert_eq!(loaded_graph.node_count(), 2);
        assert_eq!(loaded_graph.relationship_count(), 1);
        assert_eq!(loaded_graph.labels_count(), 1);
        assert_eq!(loaded_graph.relationship_types_count(), 1);

        // Verify schema
        let node_attrs = loaded_graph.node_attr_names();
        assert_eq!(node_attrs.len(), 2);
        assert_eq!(*node_attrs[0], "name");
        assert_eq!(*node_attrs[1], "age");

        let rel_attrs = loaded_graph.relationship_attr_names();
        assert_eq!(rel_attrs.len(), 1);
        assert_eq!(*rel_attrs[0], "since");

        let labels = loaded_graph.get_labels();
        assert_eq!(labels.len(), 1);
        assert_eq!(*labels[0], "Person");

        let types = loaded_graph.get_types();
        assert_eq!(types.len(), 1);
        assert_eq!(*types[0], "KNOWS");

        // Verify deleted entities
        assert_eq!(loaded_graph.deleted_nodes().len(), 1);
        assert!(loaded_graph.deleted_nodes().contains(5));
        assert_eq!(loaded_graph.deleted_relationships().len(), 1);
        assert!(loaded_graph.deleted_relationships().contains(3));

        // Verify node attributes
        let empty = OrderSet::default();
        let node_ids: Vec<_> = loaded_graph.get_nodes(&empty, 0).collect();
        assert_eq!(node_ids.len(), 2);

        let alice_attrs = loaded_graph.get_node_all_attrs(node_ids[0]);
        assert_eq!(alice_attrs.len(), 2);

        let bob_attrs = loaded_graph.get_node_all_attrs(node_ids[1]);
        assert_eq!(bob_attrs.len(), 2);

        // Verify edge attributes
        let edge_ids = loaded_graph.all_relationship_ids();
        assert_eq!(edge_ids.len(), 1);

        let edge_attrs = loaded_graph.get_relationship_all_attrs_by_raw_id(edge_ids[0].0);
        assert_eq!(edge_attrs.len(), 1);

        // Verify matrices have correct entry counts
        // Entries set via VersionedMatrix::set go to delta_plus (dp), not committed (m)
        assert_eq!(loaded_graph.all_nodes_matrix().delta_plus().nvals(), 2);
        assert_eq!(loaded_graph.adjacency_matrix().delta_plus().nvals(), 1);
        assert_eq!(loaded_graph.label_matrices().len(), 1);
        assert_eq!(loaded_graph.relationship_tensors().len(), 1);
    }

    /// Test value serialization round-trip for all supported types.
    #[test]
    fn test_value_round_trip() {
        init_graphblas();

        let test_values = vec![
            Value::Null,
            Value::Bool(true),
            Value::Bool(false),
            Value::Int(42),
            Value::Int(-100),
            Value::Float(3.14),
            Value::String(Arc::new("hello world".to_string())),
            Value::Date(1234567890),
            Value::Time(9876543210),
            Value::Datetime(1111111111),
            Value::Duration(5000),
            Value::Point(crate::runtime::value::Point::new(40.7128, -74.0060)),
            Value::VecF32(thin_vec::thin_vec![1.0f32, 2.5, 3.75]),
            Value::List(thin_vec::thin_vec![Value::Int(1), Value::String(Arc::new("two".to_string())), Value::Null]),
        ];

        for original in &test_values {
            let mut io = MockIO::new();
            save_value(&mut io, original);
            io.pos = 0;
            let loaded = load_value(&mut io);
            assert_eq!(io.pos, io.buf.len(), "not all bytes consumed for {original:?}");
            match (original, &loaded) {
                (Value::Null, Value::Null) => {}
                (Value::Bool(a), Value::Bool(b)) => assert_eq!(a, b),
                (Value::Int(a), Value::Int(b)) => assert_eq!(a, b),
                (Value::Float(a), Value::Float(b)) => assert!((a - b).abs() < f64::EPSILON),
                (Value::String(a), Value::String(b)) => assert_eq!(a, b),
                (Value::Date(a), Value::Date(b)) => assert_eq!(a, b),
                (Value::Time(a), Value::Time(b)) => assert_eq!(a, b),
                (Value::Datetime(a), Value::Datetime(b)) => assert_eq!(a, b),
                (Value::Duration(a), Value::Duration(b)) => assert_eq!(a, b),
                (Value::Point(a), Value::Point(b)) => {
                    assert!((a.latitude - b.latitude).abs() < f32::EPSILON);
                    assert!((a.longitude - b.longitude).abs() < f32::EPSILON);
                }
                (Value::VecF32(a), Value::VecF32(b)) => {
                    assert_eq!(a.len(), b.len());
                    for (x, y) in a.iter().zip(b.iter()) {
                        assert!((x - y).abs() < f32::EPSILON);
                    }
                }
                (Value::List(a), Value::List(b)) => assert_eq!(a.len(), b.len()),
                _ => panic!("type mismatch: {original:?} vs {loaded:?}"),
            }
        }
    }

    /// Test serialization of an empty graph.
    #[test]
    fn test_empty_graph_round_trip() {
        init_graphblas();

        let graph_name = "empty_graph";
        let graph = Graph::new(16384, 16384, 25, 0, graph_name);

        let mut save_io = MockIO::new();
        rdb_save(&mut save_io, &graph, graph_name);

        save_io.pos = 0;
        let (loaded_name, loaded_graph) = rdb_load(&mut save_io, 25);

        assert_eq!(loaded_name, graph_name);
        assert_eq!(loaded_graph.node_count(), 0);
        assert_eq!(loaded_graph.relationship_count(), 0);
        assert_eq!(loaded_graph.labels_count(), 0);
        assert_eq!(loaded_graph.relationship_types_count(), 0);
        assert_eq!(loaded_graph.deleted_nodes().len(), 0);
        assert_eq!(loaded_graph.deleted_relationships().len(), 0);
    }

    /// Test that matrix serialization/deserialization preserves entries.
    #[test]
    fn test_matrix_serialize_round_trip() {
        init_graphblas();

        let mut matrix = Matrix::new(100, 100);
        matrix.set(0, 1, true);
        matrix.set(5, 10, true);
        matrix.set(99, 99, true);

        let mut io = MockIO::new();
        serialize_matrix(&mut io, &matrix);

        io.pos = 0;
        let loaded = deserialize_matrix(&mut io);

        assert_eq!(loaded.nvals(), 3);
        assert_eq!(loaded.nrows(), 100);
        assert_eq!(loaded.ncols(), 100);
    }
}
