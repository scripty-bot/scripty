#[macro_use]
extern crate tracing;

use poise::{FrameworkBuilder, FrameworkOptions};

mod error;

type Error = error::Error;
type Data = ();

async fn entrypoint() {
    let builder = poise::FrameworkBuilder::default();
}
