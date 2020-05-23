//! This crate provide control commands to work with gitlab
//!
//! ## Download
//!
//!

#[cfg(feature = "gitlab12")]
extern crate gitlab12 as gitlab;

pub mod error;
pub mod gitlabctl;
pub mod label;
