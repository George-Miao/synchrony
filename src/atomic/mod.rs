//!  Atomic scalars

/// Multithreaded atomic scalars
pub mod sync {
    pub use std::sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
        AtomicU32, AtomicU64, AtomicUsize,
    };
}

/// Singlethreaded atomic scalars based on [`std::cell::Cell`]
pub mod unsync;
