use crate::error::CoreResult;
use bincode::{Decode, Encode};

pub fn encode<T: Encode>(data: &T) -> CoreResult<Vec<u8>> {
    let encoded = bincode::encode_to_vec(data, bincode::config::standard())?;
    let compressed = zstd::encode_all(encoded.as_slice(), 22)?;
    Ok(compressed)
}

pub fn decode<T: Decode<()>>(data: &[u8]) -> CoreResult<T> {
    let decompressed = zstd::decode_all(data)?;
    let (decoded, _) =
        bincode::decode_from_slice(decompressed.as_slice(), bincode::config::standard())?;
    Ok(decoded)
}
