//! Serialization traits and type tags for RDB persistence.
//!
//! Provides `Writer`/`Reader` traits, `Encode`/`Decode` traits, and
//! type-tag modules used by the encoder/decoder in the `serializers`
//! module which handles the actual Redis Module IO.

use roaring::RoaringTreemap;

/// Typed failure modes for decoding GraphBLAS vectors/matrices from an
/// untrusted payload (`GRAPH.RESTORE` / RDB load). Every variant is a clean
/// rejection of malformed input — never a panic — because the module's
/// process-exiting panic hook turns any panic into a full-server crash.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    /// The declared value-buffer byte length does not match the buffer that
    /// was actually read (copying would over-read attacker-controlled bytes).
    #[error("Vector decode: declared byte length {declared} does not match buffer length {actual}")]
    ByteLengthMismatch { declared: u64, actual: usize },
    /// The declared entry count cannot fit in the declared byte length.
    #[error("Vector decode: entry count {n_entries} exceeds byte length {n_bytes}")]
    EntryCountExceedsBytes { n_entries: u64, n_bytes: u64 },
    /// The serialized type name is not a single NUL-terminated C string.
    #[error("Vector decode: type name is not NUL-terminated")]
    TypeNameNotNulTerminated,
    /// The declared byte length cannot be represented as an allocation layout.
    #[error("Vector decode: invalid buffer layout: {0}")]
    InvalidLayout(String),
    /// Allocating the backing buffer for the decoded values failed.
    #[error("Vector decode: buffer allocation failed")]
    AllocationFailed,
    /// The serialized container/blob is smaller than the fixed-size header.
    #[error("container buffer too small: {actual} bytes < {required} bytes required")]
    ContainerTooSmall { actual: usize, required: usize },
    /// A GraphBLAS call rejected the reconstructed data.
    #[error("{call} failed: {info:?}")]
    GraphBlasFailure {
        call: &'static str,
        info: super::GrB_Info,
    },
}

impl From<DecodeError> for String {
    fn from(err: DecodeError) -> Self {
        err.to_string()
    }
}

/// Abstraction over a serialization sink.
///
/// The root crate implements this for `BufferedWriter` (v19 buffered IO).
/// The graph crate uses it via `Encode` impls without knowing about Redis.
pub trait Writer {
    fn write_unsigned(
        &mut self,
        val: u64,
    );
    fn write_signed(
        &mut self,
        val: i64,
    );
    fn write_double(
        &mut self,
        val: f64,
    );
    fn write_buffer(
        &mut self,
        data: &[u8],
    );
}

/// Types that can serialize themselves into a [`Writer`].
pub trait Encode<const VERSION: u64> {
    fn encode(
        &self,
        w: &mut dyn Writer,
    );

    /// Encode a range of entities starting at `offset`, encoding `count` items.
    fn encode_with_range(
        &self,
        w: &mut dyn Writer,
        count: u64,
        offset: u64,
    ) {
        let _ = (w, count, offset);
        unimplemented!()
    }
}

/// Abstraction over a deserialization source.
///
/// The root crate implements this for `BufferedReader` (v19 buffered IO).
/// The graph crate uses it via `Decode` impls without knowing about Redis.
pub trait Reader {
    fn read_unsigned(&mut self) -> Result<u64, String>;
    fn read_signed(&mut self) -> Result<i64, String>;
    fn read_double(&mut self) -> Result<f64, String>;
    fn read_buffer(&mut self) -> Result<Vec<u8>, String>;
}

/// Types that can deserialize themselves from a [`Reader`].
pub trait Decode<const VERSION: u64>: Sized {
    fn decode(r: &mut dyn Reader) -> Result<Self, String>;

    /// Decode `count` entities from the reader into `self`.
    fn decode_with_count(
        &mut self,
        r: &mut dyn Reader,
        count: u64,
    ) -> Result<(), String> {
        let _ = (r, count);
        unimplemented!()
    }
}

/// Index field type bitmask matching C FalkorDB index_field.h.
pub mod index_field_type {
    pub const INDEX_FLD_FULLTEXT: u64 = 0x01;
    pub const INDEX_FLD_NUMERIC: u64 = 0x02;
    pub const INDEX_FLD_GEO: u64 = 0x04;
    pub const INDEX_FLD_STR: u64 = 0x08;
    pub const INDEX_FLD_VECTOR: u64 = 0x10;
}

/// SIValue type tags for binary serialization (matching C FalkorDB format).
pub mod si_type {
    pub const T_ARRAY: u64 = 1 << 3;
    pub const T_DATETIME: u64 = 1 << 5;
    pub const T_DATE: u64 = 1 << 7;
    pub const T_TIME: u64 = 1 << 8;
    pub const T_DURATION: u64 = 1 << 10;
    pub const T_STRING: u64 = 1 << 11;
    pub const T_BOOL: u64 = 1 << 12;
    pub const T_INT64: u64 = 1 << 13;
    pub const T_DOUBLE: u64 = 1 << 14;
    pub const T_NULL: u64 = 1 << 15;
    pub const T_POINT: u64 = 1 << 17;
    pub const T_VECTOR_F32: u64 = 1 << 18;
    pub const T_INTERN: u64 = 1 << 19;
}

/// Identifies which payload section a key entry represents in the RDB format.
///
/// Each virtual key stores a directory of `(EncodeState, count)` pairs describing
/// which payload sections it contains and how many entities per section.
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeState {
    Init = 0,
    Nodes = 1,
    DeletedNodes = 2,
    Edges = 3,
    DeletedEdges = 4,
    GraphSchema = 5,
    LabelsMatrices = 6,
    RelationMatrices = 7,
    AdjMatrix = 8,
    LblsMatrix = 9,
    Final = 10,
}

impl EncodeState {
    #[must_use]
    pub const fn from_u64(v: u64) -> Option<Self> {
        match v {
            0 => Some(Self::Init),
            1 => Some(Self::Nodes),
            2 => Some(Self::DeletedNodes),
            3 => Some(Self::Edges),
            4 => Some(Self::DeletedEdges),
            5 => Some(Self::GraphSchema),
            6 => Some(Self::LabelsMatrices),
            7 => Some(Self::RelationMatrices),
            8 => Some(Self::AdjMatrix),
            9 => Some(Self::LblsMatrix),
            10 => Some(Self::Final),
            _ => None,
        }
    }
}

/// A single payload entry with state, count, and offset into the entity stream.
#[derive(Debug, Clone, Copy)]
pub struct PayloadEntry {
    pub state: EncodeState,
    pub count: u64,
    pub offset: u64,
}

impl Encode<19> for RoaringTreemap {
    fn encode(
        &self,
        w: &mut dyn Writer,
    ) {
        self.encode_with_range(w, self.len(), 0);
    }

    fn encode_with_range(
        &self,
        w: &mut dyn Writer,
        count: u64,
        offset: u64,
    ) {
        let mut buf = Vec::with_capacity(count as usize * 8);
        for id in self.iter().skip(offset as usize).take(count as usize) {
            buf.extend_from_slice(&id.to_le_bytes());
        }
        w.write_buffer(&buf);
    }
}

impl Decode<19> for RoaringTreemap {
    fn decode(r: &mut dyn Reader) -> Result<Self, String> {
        let bytes = r.read_buffer()?;
        if bytes.len() % 8 != 0 {
            return Err(format!(
                "misaligned deleted entities buffer: {} bytes is not a multiple of 8",
                bytes.len()
            ));
        }
        let count = bytes.len() / 8;
        let mut bitmap = Self::new();
        for i in 0..count {
            let id = u64::from_le_bytes(
                bytes[i * 8..(i + 1) * 8]
                    .try_into()
                    .map_err(|_| "invalid id bytes")?,
            );
            bitmap.insert(id);
        }
        Ok(bitmap)
    }

    fn decode_with_count(
        &mut self,
        r: &mut dyn Reader,
        count: u64,
    ) -> Result<(), String> {
        let bytes = r.read_buffer()?;
        let expected_len = (count as usize)
            .checked_mul(8)
            .ok_or("deleted entities count overflows buffer length")?;
        if bytes.len() != expected_len {
            return Err(format!(
                "deleted entities buffer length mismatch: got {} bytes, expected {} bytes",
                bytes.len(),
                expected_len
            ));
        }
        for i in 0..count as usize {
            let id = u64::from_le_bytes(
                bytes[i * 8..(i + 1) * 8]
                    .try_into()
                    .map_err(|_| "invalid id bytes")?,
            );
            self.insert(id);
        }
        Ok(())
    }
}

/// Shared in-memory [`Reader`]/[`Writer`] mocks for unit tests: they record /
/// replay scripted values in call order so `Encode` output can be fed back
/// into `Decode` (or hand-crafted malformed payloads can drive the
/// validation paths) without any Redis IO.
#[cfg(test)]
pub(crate) mod test_io {
    use std::collections::VecDeque;

    use super::{Reader, Writer};

    #[derive(Default)]
    pub struct MockReader {
        pub buffers: VecDeque<Vec<u8>>,
        pub unsigned: VecDeque<u64>,
        pub signed: VecDeque<i64>,
    }

    impl Reader for MockReader {
        fn read_unsigned(&mut self) -> Result<u64, String> {
            self.unsigned
                .pop_front()
                .ok_or_else(|| "mock: no more unsigned values".to_string())
        }
        fn read_signed(&mut self) -> Result<i64, String> {
            self.signed
                .pop_front()
                .ok_or_else(|| "mock: no more signed values".to_string())
        }
        fn read_double(&mut self) -> Result<f64, String> {
            Err("mock: read_double unused".to_string())
        }
        fn read_buffer(&mut self) -> Result<Vec<u8>, String> {
            self.buffers
                .pop_front()
                .ok_or_else(|| "mock: no more buffers".to_string())
        }
    }

    #[derive(Default)]
    pub struct MockWriter {
        pub buffers: VecDeque<Vec<u8>>,
        pub unsigned: VecDeque<u64>,
        pub signed: VecDeque<i64>,
    }

    impl MockWriter {
        /// Consume the writer, replaying everything it recorded as a reader.
        pub fn into_reader(self) -> MockReader {
            MockReader {
                buffers: self.buffers,
                unsigned: self.unsigned,
                signed: self.signed,
            }
        }
    }

    impl Writer for MockWriter {
        fn write_unsigned(
            &mut self,
            val: u64,
        ) {
            self.unsigned.push_back(val);
        }
        fn write_signed(
            &mut self,
            val: i64,
        ) {
            self.signed.push_back(val);
        }
        fn write_double(
            &mut self,
            val: f64,
        ) {
            let _ = val;
            unreachable!("write_double unused by GraphBLAS encode");
        }
        fn write_buffer(
            &mut self,
            data: &[u8],
        ) {
            self.buffers.push_back(data.to_vec());
        }
    }
}
