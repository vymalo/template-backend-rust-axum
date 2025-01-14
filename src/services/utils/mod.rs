use anyhow::Result;
use gen_server::types::ByteArray;
use serde::Serialize;
use serde_cbor::ser::to_vec_packed as encode;

pub fn pack_cbor<T: Serialize>(input: &T) -> Result<ByteArray> {
    let res = encode(input)?;
    Ok(ByteArray(res))
}