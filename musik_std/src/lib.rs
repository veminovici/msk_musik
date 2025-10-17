//! # musik_std
//!
//! A standard music library providing common utilities and algorithms for music applications.
//!
//! This library is designed to complement the core music theory functionality with
//! practical utilities, algorithms, and common patterns used in music software development.
//!
//! ## Features
//!
//! This library is currently empty and ready for development. Future features may include:
//!
//! - **Music Algorithms**: Pattern recognition, harmony analysis, rhythm processing
//! - **Utilities**: File I/O helpers, format conversion, data structures
//! - **Common Patterns**: Builder patterns, iterators, and convenience functions
//! - **Integration Helpers**: Bridge functions between different music representations
//!
//! ## Usage
//!
//! ```rust
//! use musik_std;
//!
//! // Library functionality will be added here
//! ```

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Module declarations
mod semitone;

// Re-exports
pub use semitone::Semitone;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        // Test that version is a valid semver format (e.g., "0.1.0")
        assert!(VERSION.len() >= 5); // At least "0.0.0"
        assert!(VERSION.contains('.')); // Should contain dots

        // Test that it parses as valid semver parts
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert!(
            parts.len() >= 3,
            "Version should have at least major.minor.patch"
        );

        // Each part should be numeric
        for part in parts.iter().take(3) {
            assert!(
                part.parse::<u32>().is_ok(),
                "Version part '{}' should be numeric",
                part
            );
        }
    }
}
