//! Shared pointer based on [`Arc`] or [`Rc`].
//!
//! [`Arc`]: std::sync::Arc
//! [`Rc`]: std::rc::Rc

/// Multithreaded [`Shared`] based on [`std::sync::Arc`]
///
/// [`Shared`]: sync::Shared
pub mod sync {
    pub use std::sync::Arc as Shared;
}

/// Singlethreaded [`Shared`] based on [`std::rc::Rc`]
///
/// [`Shared`]: unsync::Shared
pub mod unsync {
    pub use std::rc::Rc as Shared;
}
