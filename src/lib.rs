//! Board Support Crate for the [1Bitsy]
//!
//! [1Bitsy]: https://1bitsy.org
//!
//! # Usage
//!
//! Follow `cortex-m-quickstart` [instructions][i] but remove the `memory.x`
//! linker script and the `build.rs` build script file as part of the
//! configuration of the quickstart crate.
//!
//! [i]: https://docs.rs/cortex-m-quickstart/0.1.8/cortex_m_quickstart/

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(const_fn)]
#![feature(never_type)]
#![no_std]

pub extern crate stm32f41x;

pub mod led;
