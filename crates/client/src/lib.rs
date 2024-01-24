cfg_if::cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub mod client;
pub(crate) mod wasm;

pub use client::{Error, FlymodelClient};
