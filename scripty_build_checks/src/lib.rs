#![no_std]

#[cfg(target_os = "windows")]
compile_error!(
	"\
Scripty *will not* run on Windows whatsoever. Don't even try.\nNote: one of our downstream \
	 dependencies also has a chance to segfault on Windows."
);

#[cfg(all(not(target_os = "linux"), not(ignore_os)))]
compile_error!(
	"Scripty is only designed for Linux. It may not run at all on other platforms. If you'd like \
	 to try anyway, enable the `--cfg ignore_os` flag in RUSTFLAGS."
);
