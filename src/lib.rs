//! This crate provide control commands to work with gitlab
//!
//! ## Install
//!
//! ### Download pre-compiled binaries:
//! any published version for Windows Linux and Mac can be downloaded from [release github
//! page](https://github.com/xoac/gitlabctl/releases)
//!
//! ### From source
//! ```
//! cargo install --git https://github.com/xoac/gitlabctl
//! ```
//!
//! ## Usage
//!
//! To use this program you need [Personal Access
//! Token](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html)
//!
//! type `gitlabctl help` to start work with.
//!

#[cfg(feature = "gitlab12")]
extern crate gitlab12 as gitlab;

pub mod error;
pub mod gitlabctl;
pub mod label;
