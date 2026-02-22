//! Unit tests for Tensor bit manipulation safety checks

#[cfg(test)]
mod tests {
    use crate::graph::{
        matrix::New,
        tensor::{GrB_INDEX_MAX, Tensor},
    };

    #[test]
    fn test_tensor_set_get_roundtrip_small_values() {
        let mut tensor = Tensor::new(100, 100);

        // Test with small node IDs (well within 32-bit range)
        tensor.set(0, 0, 1);
        tensor.set(1, 2, 100);
        tensor.set(10, 20, 500);

        let mut edges = tensor.get(0, 0).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (0, 1));

        edges = tensor.get(1, 2).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (2, 100));

        edges = tensor.get(10, 20).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0], (20, 500));
    }

    #[test]
    fn test_tensor_boundary_values() {
        // Use GrB_INDEX_MAX as the tensor size - this is the GraphBLAS maximum index
        let mut tensor = Tensor::new(GrB_INDEX_MAX, GrB_INDEX_MAX);

        // Test with boundary values (max 32-bit value)
        let max_32bit = u64::from(u32::MAX);
        tensor.set(0, max_32bit, 1);
        tensor.set(max_32bit, 0, 2);
        tensor.set(max_32bit, max_32bit, 3);

        let edges = tensor.get(0, max_32bit).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].2, 1);

        let edges = tensor.get(max_32bit, 0).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].2, 2);

        let edges = tensor.get(max_32bit, max_32bit).collect::<Vec<_>>();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].2, 3);
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_set_overflow_src() {
        let mut tensor = Tensor::new(u64::MAX, 100);
        // This should panic because src > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        tensor.set(overflow_value, 0, 1);
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_set_overflow_dest() {
        let mut tensor = Tensor::new(100, u64::MAX);
        // This should panic because dest > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        tensor.set(0, overflow_value, 1);
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_get_overflow_src() {
        let tensor = Tensor::new(u64::MAX, 100);
        // This should panic because src > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        let _ = tensor.get(overflow_value, 0).collect::<Vec<_>>();
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_get_overflow_dest() {
        let tensor = Tensor::new(100, u64::MAX);
        // This should panic because dest > u32::MAX
        let overflow_value = u64::from(u32::MAX) + 1;
        let _ = tensor.get(0, overflow_value).collect::<Vec<_>>();
    }

    #[test]
    fn test_tensor_multiple_edges_same_pair() {
        let mut tensor = Tensor::new(100, 100);

        // Add multiple edges between same node pair
        tensor.set(5, 10, 100);
        tensor.set(5, 10, 200);
        tensor.set(5, 10, 300);

        let edges = tensor.get(5, 10).collect::<Vec<_>>();
        assert_eq!(edges.len(), 3);

        // Verify all edge IDs are present
        let edge_ids: Vec<u64> = edges.iter().map(|(_, id)| *id).collect();
        assert!(edge_ids.contains(&100));
        assert!(edge_ids.contains(&200));
        assert!(edge_ids.contains(&300));
    }

    #[test]
    #[should_panic(expected = "Node IDs must fit in 32 bits")]
    fn test_tensor_remove_all_overflow() {
        let mut tensor = Tensor::new(u64::MAX, u64::MAX);
        let overflow_value = u64::from(u32::MAX) + 1;

        // This should panic because node IDs are > u32::MAX
        tensor.remove_all(vec![(1, overflow_value, 0)]);
    }
}
