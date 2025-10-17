use musik_std::{ChordFormula, DegreeAlteration};

fn main() {
    println!("🎵 API Simplification Success! 🎵");

    // Before: ChordFormula::empty().with_degree(1, None).with_degree(3, Some(DegreeAlteration::Flat))
    // After: ChordFormula::empty().with_degree(1, DegreeAlteration::None).with_degree(3, DegreeAlteration::Flat)

    let minor_chord = ChordFormula::empty()
        .with_degree(1, DegreeAlteration::None) // Root (natural)
        .with_degree(3, DegreeAlteration::Flat) // Minor 3rd (flat)
        .with_degree(5, DegreeAlteration::None); // Perfect 5th (natural)

    // Before: chord.has_degree(1, None) and chord.has_degree(3, Some(DegreeAlteration::Flat))
    // After: chord.has_degree(1, DegreeAlteration::None) and chord.has_degree(3, DegreeAlteration::Flat)

    println!("Minor chord created with simplified API:");
    println!(
        "  Has root (natural): {}",
        minor_chord.has_degree(1, DegreeAlteration::None)
    );
    println!(
        "  Has minor 3rd (flat): {}",
        minor_chord.has_degree(3, DegreeAlteration::Flat)
    );
    println!(
        "  Has perfect 5th (natural): {}",
        minor_chord.has_degree(5, DegreeAlteration::None)
    );
    println!(
        "  Does NOT have major 3rd: {}",
        !minor_chord.has_degree(3, DegreeAlteration::None)
    );

    // Test built-in chord matches our custom chord
    let built_in_minor = ChordFormula::minor_triad();
    println!(
        "  Matches built-in minor triad: {}",
        minor_chord == built_in_minor
    );

    // Display the chord
    println!("  Chord formula: {}", minor_chord);

    println!("\n✅ API Simplified Successfully!");
    println!("   - No more Option<DegreeAlteration> wrapper");
    println!("   - DegreeAlteration::None represents natural degrees");
    println!("   - Cleaner, more direct API");
}
