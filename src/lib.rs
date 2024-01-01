// Copyright (c) 2023-2024 The MobileCoin Foundation

#![doc = include_str!("../README.md")]
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]

mod cargo_build;
mod env;
mod utils;
mod vars;

pub use crate::{
    cargo_build::CargoBuilder,
    env::{
        Endianness, EndiannessError, Environment, EnvironmentError, TargetFamily, TargetFamilyError,
    },
    utils::rerun_if_path_changed,
};
