//! # GPIO
//!
//! The GPIO module is used to configure GPIO pins through the
//! [PORT](crate::target_device::PORT) interface.
//!
//! ## Versions
//!
//! There are currently two versions of the GPIO module. The inital GPIO API
//! was based on a macro-heavy implementation. The discussion in issue
//! [#214](https://github.com/atsamd-rs/atsamd/issues/214) spurred the creation
//! of a new module with less macro-use and a refactored API.
//!
//! The new module is provided in [v2]. The old module was removed, but a
//! compatibility shim is provided in [v1] to support existing code.
//!
//! ## Migration
//!
//! The [v2] module will eventually replace [v1]. New users are encouraged to
//! use [v2] instead of [v1]. Existing code should expect to migrate to [v2]
//! before this crate reaches `1.0`.

pub mod v1;
pub use v1::*;

pub mod v2;
