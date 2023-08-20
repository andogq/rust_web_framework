mod component;
mod controller;

pub use component::*;
pub use controller::*;

/// A method that will update the internal state of a [`Component`].
pub type UpdateComponentFn = dyn Fn(&[usize]);
