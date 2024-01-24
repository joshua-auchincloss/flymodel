#![allow(non_snake_case)]

#[cynic::schema("flymodel")]
pub mod schema {}

pub mod enums;

pub mod fragments;
pub mod gql;
pub mod scalars;
pub mod wasm;

#[cfg(test)]
pub mod tests {}
