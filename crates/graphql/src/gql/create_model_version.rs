use crate::{jsvalue, schema};
use flymodel_macros::hybrid_feature_class;
use serde::{Deserialize, Serialize};

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryVariables, Debug, Clone, Deserialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify), tsify(from_wasm_abi))]
pub struct CreateModelVersionVariables {
    pub model_id: i32,
    pub version_tag: String,
}

crate::new_for! {
    CreateModelVersionVariables,
    version_tag: &str,
    model_id: i32,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(graphql_type = "Mutation", variables = "CreateModelVersionVariables")]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct CreateModelVersion {
    #[arguments(model: $model_id, name: $version_tag)]
    pub create_model_version: ModelVersion,
}

#[hybrid_feature_class(python = true)]
#[derive(cynic::QueryFragment, Clone, Debug, Serialize)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]

pub struct ModelVersion {
    pub id: i32,
    pub version: String,
    pub model_id: i32,
}

jsvalue! {
    ModelVersion,
    CreateModelVersion
}
