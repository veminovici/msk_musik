# MUSIK PROJECT

[![CI](https://github.com/veminovici/msk_musik/workflows/CI/badge.svg)](https://github.com/veminovici/msk_musik/actions)
[![Security](https://github.com/veminovici/msk_musik/workflows/Security%20and%20Dependencies/badge.svg)](https://github.com/veminovici/msk_musik/actions)
[![Documentation](https://github.com/veminovici/msk_musik/workflows/Documentation/badge.svg)](https://github.com/veminovici/msk_musik/actions)

A Rust workspace containing the `musik_theory` library for music processing and audio applications.

## Overview

This workspace provides a comprehensive set of tools for music theory, audio processing, and MIDI handling in Rust. The core library `musik_theory` offers fundamental building blocks for music-related applications.

## Workspace Structure

```
msk_musik/
├── Cargo.toml          # Workspace configuration
├── musik_theory/       # Core music library
│   ├── Cargo.toml      # Library configuration
│   └── src/
│       ├── lib.rs      # Library root
│       ├── theory.rs   # Music theory primitives
│       ├── audio.rs    # Audio processing utilities
│       └── midi.rs     # MIDI support
└── README.md           # This file
```

## Features

### musik_theory Library

- **Music Theory**: Notes, intervals, scales, and chord progressions
- **Audio Processing**: Sample buffers, waveform generation, and audio utilities
- **MIDI Support**: MIDI message handling, note/frequency conversion, and protocol utilities

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
cargo test -p musik_theory
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
use musik_theory::{Note, midi, audio};

fn main() {
    // Work with musical notes
    let note = Note::A;
    println!("Note: {} (semitone: {})", note, note.semitone());

    // Convert to MIDI
    if let Some(midi_note) = midi::note_to_midi(note, 4) {
        let frequency = midi::midi_to_frequency(midi_note);
        println!("A4 = MIDI {} = {:.2} Hz", midi_note, frequency);
    }

    // Generate audio
    let samples = audio::generate_sine_wave(440.0, 1.0, 44100, 0.5);
    println!("Generated {} samples", samples.len());
}
```

## Development

### Adding New Features

1. Create new modules in `musik_theory/src/`
2. Export them in `lib.rs`
3. Add comprehensive tests
4. Update documentation

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

- [ ] Extended chord progressions and scales
- [ ] Audio effects processing
- [ ] Real-time audio I/O
- [ ] Advanced MIDI sequencing
- [ ] Music notation support
- [ ] Audio file format support
