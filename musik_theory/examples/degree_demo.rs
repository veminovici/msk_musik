/// Example demonstrating scale degrees with alterations
use musik_theory::{ChromaticNote, Degree, DegreeCollection, Note, Scale, ScaleType};

fn main() {
    println!("🎵 Scale Degrees with Alterations Demo\n");

    let c_major = Scale::new(Note::C, ScaleType::Major);
    println!("Working with {} scale", c_major.name());

    // Basic degree concepts
    println!("\n📊 Degree Types:");
    let degrees = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::flat(3),
        Degree::sharp(4),
        Degree::natural(5),
        Degree::flat(7),
        Degree::natural(7),
    ];

    for degree in &degrees {
        if let Some(note) = c_major.note_for_degree(degree) {
            println!(
                "   {} ({}) = {} ({})",
                degree.symbol(),
                degree.name(),
                note.name(),
                degree.to_semitone_offset().unwrap_or(0)
            );
        }
    }

    // Chord construction using degrees
    println!("\n🎸 Chord Construction with Degrees:");

    // Major triad: 1, 3, 5
    let major_triad = vec![Degree::natural(1), Degree::natural(3), Degree::natural(5)];
    let major_notes = c_major.valid_notes_for_degrees(&major_triad);
    println!(
        "   Major Triad [1, 3, 5]: {:?}",
        major_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Minor triad: 1, ♭3, 5
    let minor_triad = vec![Degree::natural(1), Degree::flat(3), Degree::natural(5)];
    let minor_notes = c_major.valid_notes_for_degrees(&minor_triad);
    println!(
        "   Minor Triad [1, ♭3, 5]: {:?}",
        minor_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Diminished triad: 1, ♭3, ♭5
    let dim_triad = vec![Degree::natural(1), Degree::flat(3), Degree::flat(5)];
    let dim_notes = c_major.valid_notes_for_degrees(&dim_triad);
    println!(
        "   Diminished [1, ♭3, ♭5]: {:?}",
        dim_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Augmented triad: 1, 3, ♯5
    let aug_triad = vec![Degree::natural(1), Degree::natural(3), Degree::sharp(5)];
    let aug_notes = c_major.valid_notes_for_degrees(&aug_triad);
    println!(
        "   Augmented [1, 3, ♯5]: {:?}",
        aug_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Seventh chords
    println!("\n🎷 Seventh Chords:");

    // Major 7th: 1, 3, 5, 7
    let maj7 = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::natural(5),
        Degree::natural(7),
    ];
    let maj7_notes = c_major.valid_notes_for_degrees(&maj7);
    println!(
        "   Major 7th [1, 3, 5, 7]: {:?}",
        maj7_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Dominant 7th: 1, 3, 5, ♭7
    let dom7 = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::natural(5),
        Degree::flat(7),
    ];
    let dom7_notes = c_major.valid_notes_for_degrees(&dom7);
    println!(
        "   Dominant 7th [1, 3, 5, ♭7]: {:?}",
        dom7_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Minor 7th: 1, ♭3, 5, ♭7
    let min7 = vec![
        Degree::natural(1),
        Degree::flat(3),
        Degree::natural(5),
        Degree::flat(7),
    ];
    let min7_notes = c_major.valid_notes_for_degrees(&min7);
    println!(
        "   Minor 7th [1, ♭3, 5, ♭7]: {:?}",
        min7_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Blues and jazz chords
    println!("\n🎺 Blues & Jazz Harmony:");

    // Blues scale: 1, ♭3, 4, ♭5, 5, ♭7
    let blues_degrees = vec![
        Degree::natural(1),
        Degree::flat(3),
        Degree::natural(4),
        Degree::flat(5),
        Degree::natural(5),
        Degree::flat(7),
    ];
    let blues_notes = c_major.valid_notes_for_degrees(&blues_degrees);
    println!(
        "   Blues Scale [1, ♭3, 4, ♭5, 5, ♭7]: {:?}",
        blues_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Altered dominant: 1, 3, ♭5, ♭7
    let altered_dom = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::flat(5),
        Degree::flat(7),
    ];
    let altered_notes = c_major.valid_notes_for_degrees(&altered_dom);
    println!(
        "   Altered Dominant [1, 3, ♭5, ♭7]: {:?}",
        altered_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Different keys
    println!("\n🗝️  Same Formulas in Different Keys:");

    let keys = vec![
        (Note::G, "G Major"),
        (Note::D, "D Major"),
        (Note::A, "A Major"),
        (Note::E, "E Major"),
    ];

    for (root, key_name) in keys {
        let scale = Scale::new(root, ScaleType::Major);
        let major_chord = scale.valid_notes_for_degrees(&major_triad);
        let minor_chord = scale.valid_notes_for_degrees(&minor_triad);

        println!("   {}:", key_name);
        println!(
            "     Major [1, 3, 5]: {:?}",
            major_chord.iter().map(|n| n.name()).collect::<Vec<_>>()
        );
        println!(
            "     Minor [1, ♭3, 5]: {:?}",
            minor_chord.iter().map(|n| n.name()).collect::<Vec<_>>()
        );
    }

    // Complex altered chords
    println!("\n🎹 Complex Altered Chords:");

    // Lydian chord: 1, 3, ♯4, 5, 7
    let lydian_chord = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::sharp(4),
        Degree::natural(5),
        Degree::natural(7),
    ];
    let lydian_notes = c_major.valid_notes_for_degrees(&lydian_chord);
    println!(
        "   Lydian [1, 3, ♯4, 5, 7]: {:?}",
        lydian_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Half-diminished: 1, ♭3, ♭5, ♭7
    let half_dim = vec![
        Degree::natural(1),
        Degree::flat(3),
        Degree::flat(5),
        Degree::flat(7),
    ];
    let half_dim_notes = c_major.valid_notes_for_degrees(&half_dim);
    println!(
        "   Half-Diminished [1, ♭3, ♭5, ♭7]: {:?}",
        half_dim_notes.iter().map(|n| n.name()).collect::<Vec<_>>()
    );

    // Demonstration of degree validation
    println!("\n✅ Degree Validation:");
    let test_degrees = vec![
        Degree::natural(1),
        Degree::flat(2),
        Degree::natural(8), // This should be invalid
    ];

    for degree in test_degrees {
        if c_major.is_valid_degree(&degree) {
            let note = c_major.note_for_degree(&degree).unwrap();
            println!("   {} is valid: {}", degree.symbol(), note.name());
        } else {
            println!("   {} is invalid", degree.symbol());
        }
    }

    println!("\n🎉 Scale degrees demonstration complete!");
    println!("\nKey takeaways:");
    println!("• Degrees can be natural (1,2,3...), flat (♭3,♭7...), or sharp (♯4,♯5...)");
    println!("• Same degree formulas work in any key");
    println!("• Complex harmonies are built by combining altered degrees");
    println!("• The system enables flexible chord and scale construction");
}
