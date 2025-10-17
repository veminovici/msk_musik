# MUSIK PROJECT

[![CI](https://github.com/veminovici/msk_musik/workflows/CI/badge.svg)](https://github.com/veminovici/msk_musik/actions)
[![Security](https://github.com/veminovici/msk_musik/workflows/Security%20and%20Dependencies/badge.svg)](https://github.com/veminovici/msk_musik/actions)
[![Documentation](https://github.com/veminovici/msk_musik/workflows/Documentation/badge.svg)](https://github.com/veminovici/msk_musik/actions)

A Rust workspace containing the `musik_std` library for comprehensive music theory processing and applications.

## Overview

This workspace provides a robust foundation for music theory applications in Rust. The core library `musik_std` offers essential building blocks including notes, scales, chords, pitch classes, semitones, and musical intervals with a focus on type safety and performance.

## Workspace Structure

```
msk_musik/
├── Cargo.toml          # Workspace configuration
├── musik_std/          # Core music theory library
│   ├── Cargo.toml      # Library configuration
│   ├── examples/       # Usage examples
│   └── src/
│       ├── lib.rs      # Library root and exports
│       ├── prelude.rs  # Common imports
│       ├── note.rs     # MIDI notes and operations
│       ├── semitone.rs # Semitone representations
│       ├── octave.rs   # Octave handling
│       ├── pitch_class.rs # Pitch class system
│       ├── degree_alteration.rs # Sharp/flat alterations
│       ├── formula_degree.rs    # Formula degrees (1st, 3rd, etc.)
│       ├── chord_formula.rs     # Chord construction
│       └── scale_formula.rs     # Scale patterns
├── scripts/            # Development and CI scripts
└── README.md           # This file
```

## Features

### musik_std Library

- **Core Music Theory**: Notes (MIDI 0-255), pitch classes (C, C#, D, etc.), semitones, and octaves
- **Scales and Modes**: Major, minor, pentatonic, blues, and custom scale formulas
- **Chord Theory**: Triads, seventh chords, extensions, alterations, and complex jazz harmonies
- **Type Safety**: Compile-time guarantees and const functions for performance
- **Rich API**: Comprehensive methods for musical calculations and transformations

## Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Building the Workspace

```bash
# Build all workspace members
cargo build

# Build in release mode
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests for specific package
cargo test -p musik_std
```

### Code Quality

```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run lints
cargo clippy
```

### Development Workflow

This project includes Git pre-commit hooks that ensure all commits pass CI checks locally:

```bash
# Set up Git hooks (run once)
make setup-hooks

# Quick development checks
make check          # Format + clippy
make test          # Run all tests
make build         # Build workspace

# Simulate full CI locally
make ci-check      # Run complete CI workflow

# View all available commands
make help
```

**Pre-commit validation includes:**

- Code formatting (`cargo fmt`)
- Clippy lints with warnings as errors
- Full workspace build
- All unit tests and documentation tests

Commits will be **automatically blocked** if any check fails, ensuring only CI-passing code reaches the repository.

## Usage Example

```rust
use musik_std::prelude::*;

fn main() {
    // Work with musical notes
    let middle_c = Note::new(60); // MIDI middle C
    println!("Note: {} (octave: {})", middle_c, middle_c.octave());
    
    // Pitch class operations
    let pitch_class = middle_c.pitch_class();
    println!("Pitch class: {} ({})", pitch_class.name(), pitch_class.value());
    
    // Scale construction
    let c_major = ScaleFormula::major();
    let scale_notes = c_major.notes_from_root(middle_c);
    println!("C Major scale: {:?}", scale_notes);
    
    // Chord construction
    let major_triad = ChordFormula::new()
        .with_degree(FormulaDegree::natural(1))
        .with_degree(FormulaDegree::natural(3))
        .with_degree(FormulaDegree::natural(5));
    println!("Major triad formula: {}", major_triad);
    
    // Semitone operations
    let semitone = Semitone::new(7); // Perfect fifth
    let fifth_above = middle_c + semitone;
    println!("Fifth above C4: {}", fifth_above);
}
```

## Development

### Adding New Features

1. Create new modules in `musik_std/src/`
2. Export them in `lib.rs`
3. Add comprehensive tests and documentation examples
4. Update the prelude if needed for common usage

### Testing

Each module includes comprehensive unit tests. Run tests frequently during development:

```bash
cargo test
```

### Documentation

Generate and view documentation:

```bash
cargo doc --open
```

## License

This project is licensed under the MIT OR Apache-2.0 license.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Roadmap

- [x] Core music theory primitives (notes, semitones, octaves)
- [x] Pitch class system with ergonomic constants
- [x] Scale and chord formula systems
- [x] Comprehensive test coverage (120+ tests)
- [ ] Interval calculations and inversions  
- [ ] Circle of fifths utilities
- [ ] Key signature support
- [ ] Extended jazz chord notations
- [ ] MIDI file parsing and generation
- [ ] Audio synthesis integration
