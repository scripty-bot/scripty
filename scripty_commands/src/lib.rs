#![feature(let_chains)]
#[macro_use]
extern crate scripty_i18n;
#[macro_use]
extern crate tracing;

pub mod cmds;

use scripty_bot_utils::{Context, Error};
