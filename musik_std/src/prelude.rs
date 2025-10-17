//! Prelude module for musik_std
//!
//! This module provides a convenient way to import the most commonly used
//! items from the musik_std library. By importing the prelude, users get
//! access to the essential types and traits needed for most music applications.
//!
//! # Usage
//!
//! ```rust
//! use musik_std::prelude::*;
//!
//! // Now you have access to all common types and traits
//! let semitone = Semitone::from(7u8);
//! ```

// Re-export commonly used types and traits
pub use crate::note::Note;
pub use crate::semitone::Semitone;

// Future additions will include:
// pub use crate::interval::Interval;
// pub use crate::chord::Chord;
// pub use crate::scale::Scale;
// pub use crate::traits::*;
