//! Background task management.
//!
//! How to use:
//! 1) Create a background task struct in its own mod inside of mod tasks.
//! 2) Implement `core::BackgroundTask` for the struct.
//! 3) Call the macro `init_task!()` in `init_background_tasks`. The argument is the **full** path to the struct.

mod core;
mod tasks;

pub use self::core::init_background_tasks;
