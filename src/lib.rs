// Copyright 2017 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! cuckoo-miner is a Rust wrapper around John Tromp's Cuckoo Miner 
//! C implementations, intended primarily for use in the Grin MimbleWimble
//! blockhain development project.
//!

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate error;
extern crate miner;
extern crate manager;

pub use error::CuckooMinerError;

pub use miner::{CuckooMinerConfig,
                CuckooMiner,
                CuckooMinerSolution};

pub use manager::{CuckooPluginManager,
                  CuckooPluginCapabilities};


