use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_cbor::from_slice as from_cbor;
use serde_cbor::to_vec as to_cbor;
use serde_json::from_slice as from_json;
use serde_json::to_string as to_json;

pub fn serialize_payload<T: Serialize>(json: bool, payload: &T) -> Result<Bytes> {
    let content = if json {
        let str_content = to_json(&payload)?;
        str_content.as_bytes().to_vec()
    } else {
        to_cbor(&payload)?
    };

    Ok(Bytes::from(content))
}

pub fn deserialize_payload<T>(json: bool, payload: Bytes) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let vec = payload.to_vec();
    let content = if json {
        from_json(&vec)?
    } else {
        from_cbor(&vec)?
    };

    Ok(content)
}