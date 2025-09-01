//! # Stabilizer State CH-Form Simulator
//! 
//! A Rust library for simulating quantum stabilizer states using the CH-form representation,
//! based on the work in arXiv:1808.00128.

pub mod stabilizer_ch_form;
pub mod api;

pub use stabilizer_ch_form::StabilizerCHForm;
pub mod prelude {
    pub use crate::api::gates::*;
}