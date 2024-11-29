#![feature(linked_list_cursors)]
use wasm_bindgen::prelude::*;
use zkwasm_rest_abi::*;
pub mod elf;
pub mod config;
pub mod error;
pub mod events;
pub mod object;
pub mod player;
pub mod state;
mod prop;

use crate::config::Config;
use crate::state::{State, Transaction};
zkwasm_rest_abi::create_zkwasm_apis!(Transaction, State, Config);
