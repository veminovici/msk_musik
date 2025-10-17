/// Example demonstrating the musical scale functionality
use musik_theory::{ChromaticNote, MusicalScale, Note, Scale, ScaleType};

fn main() {
    println!("🎵 Musical Scale Demonstration\n");

    // Create various scales
    let scales = vec![
        Scale::new(Note::C, ScaleType::Major),
        Scale::new(Note::A, ScaleType::NaturalMinor),
        Scale::new(Note::G, ScaleType::Dorian),
        Scale::new(Note::E, ScaleType::PentatonicMajor),
        Scale::new(Note::A, ScaleType::BluesScale),
        Scale::new(Note::D, ScaleType::HarmonicMinor),
    ];

    for scale in &scales {
        println!("📊 {}", scale.name());
        println!("   Root: {}", scale.root().name());
        println!(
            "   Notes: {:?}",
            scale.notes().iter().map(|n| n.name()).collect::<Vec<_>>()
        );
        println!("   Length: {} notes", scale.len());
        println!("   Intervals: {:?}", scale.intervals());

        // Show scale degrees for a few notes
        let test_notes = vec![
            scale.root(),
            scale.note_at_degree(3).unwrap_or(scale.root()),
            scale.note_at_degree(5).unwrap_or(scale.root()),
        ];

        for note in test_notes {
            if let Some(degree) = scale.degree_of(&note) {
                println!("   {} is degree {}", note.name(), degree);
            }
        }
        println!();
    }

    // Demonstrate scale relationships
    println!("🔄 Scale Relationships\n");

    let c_major = Scale::new(Note::C, ScaleType::Major);
    println!(
        "C Major scale: {:?}",
        c_major.notes().iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Create modes
    if let Some(d_dorian) = c_major.mode(2) {
        println!(
            "D Dorian (2nd mode): {:?}",
            d_dorian
                .notes()
                .iter()
                .map(|n| n.name())
                .collect::<Vec<_>>()
        );
    }

    if let Some(e_phrygian) = c_major.mode(3) {
        println!(
            "E Phrygian (3rd mode): {:?}",
            e_phrygian
                .notes()
                .iter()
                .map(|n| n.name())
                .collect::<Vec<_>>()
        );
    }

    // Scale analysis
    println!("\n🔍 Scale Analysis\n");

    let g_major = Scale::new(Note::G, ScaleType::Major);
    println!("Analyzing {} scale:", g_major.name());

    let test_notes_for_analysis = vec![
        Note::G,
        Note::A,
        Note::B,
        Note::C,
        Note::D,
        Note::E,
        Note::FSharp,
        Note::F,
    ];

    for note in test_notes_for_analysis {
        if g_major.contains(&note) {
            let degree = g_major.degree_of(&note).unwrap();
            println!(
                "✓ {} is in {} (degree {})",
                note.name(),
                g_major.name(),
                degree
            );
        } else {
            println!("✗ {} is not in {}", note.name(), g_major.name());
        }
    }

    // Pentatonic comparison
    println!("\n🎸 Pentatonic Scales Comparison\n");

    let c_penta_major = Scale::new(Note::C, ScaleType::PentatonicMajor);
    let a_penta_minor = Scale::new(Note::A, ScaleType::PentatonicMinor);

    println!(
        "{}: {:?}",
        c_penta_major.name(),
        c_penta_major
            .notes()
            .iter()
            .map(|n| n.name())
            .collect::<Vec<_>>()
    );

    println!(
        "{}: {:?}",
        a_penta_minor.name(),
        a_penta_minor
            .notes()
            .iter()
            .map(|n| n.name())
            .collect::<Vec<_>>()
    );

    // Find common notes
    let common_notes: Vec<_> = c_penta_major
        .notes()
        .into_iter()
        .filter(|note| a_penta_minor.contains(note))
        .map(|n| n.name())
        .collect();

    println!("Common notes: {:?}", common_notes);
}
