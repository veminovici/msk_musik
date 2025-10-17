# musik_std

A comprehensive Rust library for music theory applications, providing type-safe and performant building blocks for working with musical concepts.

## Features

- **Notes**: MIDI note representation (0-255) with octave and pitch class extraction
- **Pitch Classes**: Chromatic pitch classes (C, C#, D, etc.) with ergonomic constants
- **Semitones**: Musical interval calculations and transformations
- **Scales**: Major, minor, pentatonic, blues, and custom scale formulas
- **Chords**: Triads, seventh chords, extensions, and complex jazz harmonies
- **Type Safety**: Compile-time guarantees and const functions for zero-cost abstractions
- **Rich API**: Comprehensive methods for musical calculations and transformations

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
musik_std = "0.1.0"
```

### Basic Usage

```rust
use musik_std::prelude::*;

// Create notes
let middle_c = Note::new(60); // MIDI middle C
let a440 = Note::new(69);     // A above middle C

// Work with pitch classes
let c_pitch = C;  // Pitch class constant
assert_eq!(middle_c.pitch_class(), c_pitch);

// Semitone operations
let perfect_fifth = Semitone::new(7);
let g = middle_c + perfect_fifth;
println!("{} + {} semitones = {}", middle_c, perfect_fifth.value(), g);

// Scale construction
let c_major = ScaleFormula::major();
let scale_notes = c_major.notes_from_root(middle_c);
println!("C Major: {:?}", scale_notes);
```

## Core Types

### Note

Represents a MIDI note (0-255) with rich functionality:

```rust
use musik_std::*;

let note = Note::new(60); // Middle C
assert_eq!(note.octave().value(), 4);
assert_eq!(note.pitch_class(), C);
assert_eq!(format!("{}", note), "C4");

// Arithmetic operations
let octave_higher = note << 1u8; // Shift up one octave
let fifth_higher = note + Semitone::new(7);
```

### PitchClass

Represents the 12 chromatic pitch classes with convenient constants:

```rust
use musik_std::prelude::*;

// Use pitch class constants directly
let notes = [C, C_SHARP, D, D_SHARP, E, F, F_SHARP, G, G_SHARP, A, A_SHARP, B];

// Or create from values
let c_sharp = PitchClass::new(1);
assert_eq!(c_sharp, C_SHARP);
assert_eq!(c_sharp.name(), "C#");
```

### Semitone

Represents musical intervals as semitones:

```rust
use musik_std::*;

let semitone = Semitone::new(7); // Perfect fifth
let pitch_class = semitone.pitch_class(); // G (7 % 12 = 7)
let octave = semitone.octave(); // 0 (7 / 12 = 0)

// Arithmetic operations
let note = Note::new(60);
let higher = note + semitone;
let lower = note - semitone;
```

### ScaleFormula

Defines scale patterns as semitone intervals:

```rust
use musik_std::*;

// Built-in scales
let major = ScaleFormula::major();
let minor = ScaleFormula::minor();
let pentatonic = ScaleFormula::pentatonic_major();

// Custom scales
let blues = ScaleFormula::from_semitones(&[0, 3, 5, 6, 7, 10]);

// Generate notes from root
let root = Note::new(60); // C
let scale_notes = major.notes_from_root(root);
```

### ChordFormula

Defines chord structures using formula degrees:

```rust
use musik_std::*;

// Major triad (1, 3, 5)
let major_triad = ChordFormula::new()
    .with_degree(FormulaDegree::natural(1))
    .with_degree(FormulaDegree::natural(3))
    .with_degree(FormulaDegree::natural(5));

// Dominant seventh (1, 3, 5, ♭7)
let dom7 = ChordFormula::new()
    .with_degree(FormulaDegree::natural(1))
    .with_degree(FormulaDegree::natural(3))
    .with_degree(FormulaDegree::natural(5))
    .with_degree(FormulaDegree::flat(7));
```

## Examples

The `examples/` directory contains practical usage examples:

```bash
# Run examples
cargo run --example pitch_class_demo
cargo run --example semitone_pitch_class_demo
cargo run --example chord_progressions
```

### Working with Octaves

```rust
use musik_std::*;

let c4 = Note::new(60);
let c5 = c4 << 1u8; // Shift up one octave
let c3 = c4 >> 1u8; // Shift down one octave

assert_eq!(c5.semitone(), 72);
assert_eq!(c3.semitone(), 48);
assert_eq!(c4.octave().value(), 4);
```

### Scale and Chord Analysis

```rust
use musik_std::*;

let c_major = ScaleFormula::major();
let g_major = ScaleFormula::major();

// Check if a semitone is in the scale
assert!(c_major.contains_semitone(Semitone::new(0))); // C
assert!(c_major.contains_semitone(Semitone::new(4))); // E
assert!(!c_major.contains_semitone(Semitone::new(1))); // C#

// Chord formula operations
let triad = ChordFormula::new()
    .with_degree(FormulaDegree::natural(1))
    .with_degree(FormulaDegree::natural(3))
    .with_degree(FormulaDegree::natural(5));

assert!(triad.has_degree(FormulaDegree::natural(1)));
assert!(!triad.has_degree(FormulaDegree::natural(2)));
```

## Performance

- **Zero-cost abstractions**: Most operations compile to simple arithmetic
- **Const functions**: Many operations can be evaluated at compile time
- **No allocations**: All core types are stack-allocated
- **Efficient representations**: Compact memory layout for all types

```rust
// These are const fn and evaluate at compile time
const MIDDLE_C: Note = Note::new(60);
const PERFECT_FIFTH: Semitone = Semitone::new(7);
const C_PITCH_CLASS: PitchClass = PitchClass::new(0);
```

## Testing

Run the comprehensive test suite:

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Documentation tests
cargo test --doc
```

The library includes 124+ unit tests and 98+ documentation tests covering:

- All core type operations
- Musical interval calculations
- Scale and chord construction
- Edge cases and boundary conditions
- API consistency across types

## Documentation

Generate and view full API documentation:

```bash
cargo doc --open
```

The documentation includes:

- Comprehensive API reference
- Usage examples for all public functions
- Mathematical relationships between types
- Performance characteristics
- Common patterns and idioms

## Advanced Usage

### Custom Scales

```rust
use musik_std::*;

// Hungarian minor scale
let hungarian_minor = ScaleFormula::from_semitones(&[0, 2, 3, 6, 7, 8, 11]);

// Check scale properties
assert_eq!(hungarian_minor.note_count(), 7);
assert!(hungarian_minor.contains_semitone(Semitone::new(6))); // Augmented 4th
```

### Complex Chords

```rust
use musik_std::*;

// Cmaj13#11 chord
let complex_chord = ChordFormula::new()
    .with_degree(FormulaDegree::natural(1))   // Root
    .with_degree(FormulaDegree::natural(3))   // Major third
    .with_degree(FormulaDegree::natural(5))   // Perfect fifth
    .with_degree(FormulaDegree::natural(7))   // Major seventh
    .with_degree(FormulaDegree::natural(9))   // Ninth
    .with_degree(FormulaDegree::sharp(11))    // Sharp eleventh
    .with_degree(FormulaDegree::natural(13)); // Thirteenth
```

### Pitch Class Arithmetic

```rust
use musik_std::prelude::*;

// Pitch classes wrap around automatically
let c = C;
let c_sharp = PitchClass::new(c.value() + 1);
assert_eq!(c_sharp, C_SHARP);

// Extract pitch class from any semitone
let semitone = Semitone::new(25); // Two octaves + C#
assert_eq!(semitone.pitch_class(), C_SHARP);
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No clippy warnings (`cargo clippy`)
4. Documentation is updated for new features
5. Examples are provided for significant new functionality

See the [workspace README](../README.md) for development workflow details.
