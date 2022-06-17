//! Thin wrappers over the coqui-stt API.
use std::ptr::NonNull;

#[derive(Copy, Clone, Debug)]
pub struct StreamingState(NonNull<coqui_stt_sys::StreamingState>);
unsafe impl Send for StreamingState {}
unsafe impl Sync for StreamingState {}
impl StreamingState {
    pub fn new(ptr: *mut coqui_stt_sys::StreamingState) -> Self {
        if ptr.is_null() {
            unreachable!("attempted to construct StreamingState with a null pointer");
        }
        Self(NonNull::new(ptr).expect("asserted ptr is non-null"))
    }

    pub fn as_ptr(&self) -> *mut coqui_stt_sys::StreamingState {
        self.0.as_ptr()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ModelState(NonNull<coqui_stt_sys::ModelState>);
unsafe impl Send for ModelState {}
unsafe impl Sync for ModelState {}
impl ModelState {
    pub fn new(ptr: *mut coqui_stt_sys::ModelState) -> Self {
        if ptr.is_null() {
            unreachable!("attempted to construct ModelState with a null pointer");
        }
        Self(NonNull::new(ptr).expect("asserted ptr is non-null"))
    }

    pub fn as_ptr(&self) -> *mut coqui_stt_sys::ModelState {
        self.0.as_ptr()
    }
}
