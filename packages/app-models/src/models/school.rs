use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Builder)]
pub struct School {
    pub id: String,
    pub name: String,
}