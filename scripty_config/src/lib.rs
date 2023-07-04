#[macro_use]
extern crate serde;

mod cfg;
mod load;

// we place all checks here that way they're tossed as early in the build as possible

#[cfg(target_os = "windows")]
compile_error!(
	"\
Scripty *will not* run on Windows whatsoever. Don't even try.\n\
Note: one of our downstream dependencies also has a chance to segfault on Windows.\
"
);

#[cfg(not(target_os = "linux"))]
compile_error!(
	"Scripty is only designed for Linux.\
 It may not run at all on other platforms.\
 If you'd like to try anyway, comment this section of code."
);

pub use cfg::*;
pub use load::*;
