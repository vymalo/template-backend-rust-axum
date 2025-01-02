use serde::{Deserialize, Serialize};
use strum::{EnumIs, EnumIter, EnumString, EnumTryAs};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, EnumString, EnumIter, EnumIs, EnumTryAs)]
#[strum(serialize_all = "snake_case")]
pub enum MethodName {
    GetSchool,
    GetAllSchool,
}