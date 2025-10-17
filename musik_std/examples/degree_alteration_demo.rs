use musik_std::prelude::*;

fn main() {
    println!("🎵 DegreeAlteration Demo - Sharp and Flat Modifications");
    println!("======================================================\n");

    // Create basic alterations
    let sharp = DegreeAlteration::Sharp;
    let flat = DegreeAlteration::Flat;

    println!("📝 Basic Alterations:");
    println!("  Sharp: {} | Flat: {}", sharp, flat);
    println!(
        "  Sharp symbol: {} | Flat symbol: {}",
        sharp.symbol(),
        flat.symbol()
    );
    println!(
        "  Sharp offset: {} | Flat offset: {}",
        sharp.semitone_offset(),
        flat.semitone_offset()
    );

    // Test properties
    println!("\n🔍 Properties:");
    println!(
        "  Sharp is sharp: {} | Sharp is flat: {}",
        sharp.is_sharp(),
        sharp.is_flat()
    );
    println!(
        "  Flat is sharp: {} | Flat is flat: {}",
        flat.is_sharp(),
        flat.is_flat()
    );

    // Demonstrate opposites
    println!("\n↔️ Opposites:");
    println!(
        "  Sharp opposite: {} | Flat opposite: {}",
        sharp.opposite(),
        flat.opposite()
    );

    // Musical calculations
    println!("\n🎼 Musical Calculations:");

    // Major third to minor third
    let major_third = 4; // E in C major (4 semitones from C)
    let minor_third = major_third + flat.semitone_offset();
    println!(
        "  Major 3rd ({} semitones) + flat = Minor 3rd ({} semitones)",
        major_third, minor_third
    );

    // Perfect fourth to augmented fourth
    let perfect_fourth = 5; // F in C major (5 semitones from C)
    let augmented_fourth = perfect_fourth + sharp.semitone_offset();
    println!(
        "  Perfect 4th ({} semitones) + sharp = Augmented 4th ({} semitones)",
        perfect_fourth, augmented_fourth
    );

    // Major seventh to dominant seventh
    let major_seventh = 11; // B in C major (11 semitones from C)
    let dominant_seventh = major_seventh + flat.semitone_offset();
    println!(
        "  Major 7th ({} semitones) + flat = Dominant 7th ({} semitones)",
        major_seventh, dominant_seventh
    );

    // Jazz chord alterations
    println!("\n🎷 Jazz Chord Alterations:");
    let natural_ninth = 14; // D in next octave (14 semitones from C)
    let sharp_ninth = natural_ninth + sharp.semitone_offset();
    let flat_ninth = natural_ninth + flat.semitone_offset();
    println!("  Natural 9th: {} semitones", natural_ninth);
    println!(
        "  Sharp 9th ({} + {}): {} semitones",
        natural_ninth,
        sharp.symbol(),
        sharp_ninth
    );
    println!(
        "  Flat 9th ({} + {}): {} semitones",
        natural_ninth,
        flat.symbol(),
        flat_ninth
    );

    let natural_eleventh = 17; // F in next octave (17 semitones from C)
    let sharp_eleventh = natural_eleventh + sharp.semitone_offset();
    println!("  Natural 11th: {} semitones", natural_eleventh);
    println!(
        "  Sharp 11th ({} + {}): {} semitones",
        natural_eleventh,
        sharp.symbol(),
        sharp_eleventh
    );

    // Demonstrate with scale degrees
    println!("\n🎯 Scale Degree Applications:");
    let scale_degrees = [
        ("Root", 0),
        ("Major 2nd", 2),
        ("Major 3rd", 4),
        ("Perfect 4th", 5),
        ("Perfect 5th", 7),
        ("Major 6th", 9),
        ("Major 7th", 11),
    ];

    for (name, semitones) in &scale_degrees {
        let sharp_version = semitones + sharp.semitone_offset();
        let flat_version = semitones + flat.semitone_offset();
        println!(
            "  {}: {} | {}{}:{} | {}{}:{}",
            name,
            semitones,
            sharp.symbol(),
            name,
            sharp_version,
            flat.symbol(),
            name,
            flat_version
        );
    }

    // Default and equality
    println!("\n⚙️ Default and Equality:");
    let default_alteration = DegreeAlteration::default();
    println!("  Default alteration: {}", default_alteration);
    println!("  Default is Sharp: {}", default_alteration == sharp);
    println!("  Sharp == Flat: {}", sharp == flat);

    // Ordering
    println!("\n📊 Ordering:");
    println!("  Flat < Sharp: {}", flat < sharp);
    println!("  Sharp > Flat: {}", sharp > flat);

    println!("\n✅ DegreeAlteration provides flexible alteration representation!");
}
