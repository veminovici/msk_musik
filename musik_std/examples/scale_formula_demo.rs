//! Example demonstrating ScaleFormula usage for scale pattern analysis.
//!
//! This example shows how to use ScaleFormula to represent musical scales
//! using bit flags, analyze scale properties, and perform scale operations.

use musik_std::ScaleFormula;

fn main() {
    println!("🎼 ScaleFormula Demo - Bit Flag Scale Representation");
    println!("====================================================\n");

    // Common scale patterns
    println!("📚 Common Scale Patterns:");
    let scales = [
        ("Major", ScaleFormula::major()),
        ("Minor", ScaleFormula::minor()),
        ("Pentatonic Major", ScaleFormula::pentatonic_major()),
        ("Pentatonic Minor", ScaleFormula::pentatonic_minor()),
        ("Blues", ScaleFormula::blues()),
        ("Chromatic", ScaleFormula::chromatic()),
    ];

    for (name, scale) in &scales {
        println!(
            "  {:18} | {} notes | Pattern: {} | Bits: {:012b}",
            name,
            scale.note_count(),
            scale,
            scale.bits()
        );
    }

    println!("\n🔍 Scale Analysis:");
    let major = ScaleFormula::major();
    let minor = ScaleFormula::minor();

    println!("Major scale semitones: {:?}", major.semitones());
    println!("Minor scale semitones: {:?}", minor.semitones());

    // Check specific intervals
    println!("\n📊 Interval Analysis:");
    println!(
        "Major has major 3rd (semitone 4): {}",
        major.contains_semitone(4)
    );
    println!(
        "Minor has minor 3rd (semitone 3): {}",
        minor.contains_semitone(3)
    );
    println!(
        "Major has tritone (semitone 6): {}",
        major.contains_semitone(6)
    );
    println!(
        "Blues has tritone (semitone 6): {}",
        ScaleFormula::blues().contains_semitone(6)
    );

    // Scale operations
    println!("\n🔧 Scale Operations:");

    // Union: combine two scales
    let major_minor_union = major | minor;
    println!(
        "Major ∪ Minor notes: {} ({})",
        major_minor_union.note_count(),
        major_minor_union
    );

    // Intersection: common notes
    let major_minor_intersection = major & minor;
    println!(
        "Major ∩ Minor notes: {} ({})",
        major_minor_intersection.note_count(),
        major_minor_intersection
    );

    // Complement: notes NOT in scale
    let major_complement = !major;
    println!(
        "Major complement notes: {} ({})",
        major_complement.note_count(),
        major_complement
    );

    // Custom scale creation
    println!("\n🎨 Custom Scale Creation:");

    // Create a custom scale from semitone list
    let custom_scale = ScaleFormula::from_semitones(&[0, 1, 4, 6, 7, 10]);
    println!(
        "Custom scale: {} ({} notes)",
        custom_scale,
        custom_scale.note_count()
    );

    // Create scale with specific bit pattern
    let exotic = ScaleFormula::new(0b101001010101); // Custom pattern
    println!("Exotic scale: {} ({} notes)", exotic, exotic.note_count());

    // Mode analysis
    println!("\n🎯 Mode Comparison:");
    let dorian = ScaleFormula::from_semitones(&[0, 2, 3, 5, 7, 9, 10]); // D Dorian relative to C
    let mixolydian = ScaleFormula::from_semitones(&[0, 2, 4, 5, 7, 9, 10]); // G Mixolydian relative to C

    println!("Dorian mode: {}", dorian);
    println!("Mixolydian mode: {}", mixolydian);

    // Check what they have in common with major
    let dorian_vs_major = dorian & major;
    let mixolydian_vs_major = mixolydian & major;

    println!(
        "Dorian ∩ Major: {} ({} common notes)",
        dorian_vs_major,
        dorian_vs_major.note_count()
    );
    println!(
        "Mixolydian ∩ Major: {} ({} common notes)",
        mixolydian_vs_major,
        mixolydian_vs_major.note_count()
    );

    // Demonstrate notes_from_root method
    println!("\n🎹 Scale Notes from Root:");
    use musik_std::Note;

    // C Major scale notes
    let c_major_notes = major.notes_from_root(Note::new(0)); // C root
    let c_major_semitones: Vec<u8> = c_major_notes.iter().map(|note| u8::from(*note)).collect();
    println!("C Major notes (semitones): {:?}", c_major_semitones);

    // F# Major scale notes (enharmonic with Gb)
    let fs_major_notes = major.notes_from_root(Note::new(6)); // F# root
    let fs_major_semitones: Vec<u8> = fs_major_notes.iter().map(|note| u8::from(*note)).collect();
    println!("F# Major notes (semitones): {:?}", fs_major_semitones);

    // G Minor pentatonic scale
    let g_minor_pent_notes = ScaleFormula::pentatonic_minor().notes_from_root(Note::new(7)); // G root
    let g_minor_pent_semitones: Vec<u8> = g_minor_pent_notes
        .iter()
        .map(|note| u8::from(*note))
        .collect();
    println!(
        "G Minor Pentatonic notes (semitones): {:?}",
        g_minor_pent_semitones
    );

    // Extended harmony example
    let c_major_extended_notes = ScaleFormula::major_extended().notes_from_root(Note::new(0));
    println!(
        "C Major Extended ({} notes): includes 9ths, 11ths, 13ths",
        c_major_extended_notes.len()
    );

    println!("\n✅ ScaleFormula provides efficient bit-flag scale representation!");
}
