use musik_std::ScaleFormula;

fn main() {
    println!("🎼 Extended Harmony Demo - Two Octave Scale Formulas");
    println!("==================================================");

    // Create standard major scale (one octave)
    let major = ScaleFormula::major();
    println!("\n📊 Standard Major Scale (One Octave):");
    println!(
        "  Notes: {} | Semitones: {:?}",
        major.note_count(),
        major.semitones()
    );
    println!("  Bits: {:024b}", major.bits());

    // Create extended major scale (two octaves)
    let major_extended = ScaleFormula::major_extended();
    println!("\n🎹 Extended Major Scale (Two Octaves):");
    println!(
        "  Notes: {} | Semitones: {:?}",
        major_extended.note_count(),
        major_extended.semitones()
    );
    println!("  Bits: {:024b}", major_extended.bits());

    // Demonstrate jazz chord extensions
    println!("\n🎷 Jazz Chord Extensions:");
    println!(
        "  9th (major 2nd + octave): {}",
        if major_extended.contains_semitone(14) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  11th (perfect 4th + octave): {}",
        if major_extended.contains_semitone(17) {
            "✓"
        } else {
            "✗"
        }
    );
    println!(
        "  13th (major 6th + octave): {}",
        if major_extended.contains_semitone(21) {
            "✓"
        } else {
            "✗"
        }
    );

    // Create custom extended chord formula
    let dom13_formula = ScaleFormula::from_semitones(&[0, 4, 7, 10, 14, 17, 21]);
    println!("\n🎺 Dominant 13th Chord Formula:");
    println!("  Root(0) + Maj3(4) + P5(7) + ♭7(10) + 9(14) + 11(17) + 13(21)");
    println!(
        "  Notes: {} | Semitones: {:?}",
        dom13_formula.note_count(),
        dom13_formula.semitones()
    );

    // Compare chromatic scales
    let chromatic_standard = ScaleFormula::chromatic();
    let chromatic_extended = ScaleFormula::chromatic_extended();

    println!("\n🌈 Chromatic Scale Comparison:");
    println!(
        "  Standard (12 semitones): {} notes",
        chromatic_standard.note_count()
    );
    println!(
        "  Extended (24 semitones): {} notes",
        chromatic_extended.note_count()
    );

    // Demonstrate octave displacement
    println!("\n🔄 Octave Displacement Examples:");
    for semitone in [2, 4, 7, 9] {
        let second_octave = semitone + 12;
        println!(
            "  Semitone {} → Octave higher: {} ({})",
            semitone,
            second_octave,
            if major_extended.contains_semitone(second_octave) {
                "in extended scale"
            } else {
                "not in scale"
            }
        );
    }

    println!("\n✅ Two-octave scale formulas enable rich harmonic analysis!");
}
