#[macro_use]
extern crate serde;

mod cfg;
mod load;

// we place all checks here that way they're tossed as early in the build as possible

#[cfg(target_os = "windows")]
compile_error!("Scripty *will not* run on Windows whatsoever. Don't even try.");

#[cfg(not(target_os = "linux"))]
compile_error!(
	"Scripty is only designed for Linux.\
 It may not run at all on other platforms.\
 If you'd like to try anyways, comment out this section of code."
);

pub use cfg::*;
pub use load::*;
