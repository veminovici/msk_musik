/// Example demonstrating chord building with scales and degrees
use musik_theory::{ChordBuilder, ChromaticNote, Degree, Note, Scale, ScaleType};

fn main() {
    println!("🎵 Chord Builder Demonstration\n");

    // Create different scales to work with
    let c_major = Scale::new(Note::C, ScaleType::Major);
    let f_major = Scale::new(Note::F, ScaleType::Major);
    let a_major = Scale::new(Note::A, ScaleType::Major);

    // Basic triads
    println!("🎸 Basic Triads:");

    let c_maj = c_major.major_triad().unwrap();
    println!("   {}: {:?}", c_maj, c_maj.note_names());

    let c_min = c_major.minor_triad().unwrap();
    println!("   {}: {:?}", c_min, c_min.note_names());

    let c_dim = c_major.diminished_triad().unwrap();
    println!("   {}: {:?}", c_dim, c_dim.note_names());

    let c_aug = c_major.augmented_triad().unwrap();
    println!("   {}: {:?}", c_aug, c_aug.note_names());

    // Seventh chords
    println!("\n🎷 Seventh Chords:");

    let f_maj7 = f_major.major_seventh().unwrap();
    println!("   {}: {:?}", f_maj7, f_maj7.note_names());

    let f_min7 = f_major.minor_seventh().unwrap();
    println!("   {}: {:?}", f_min7, f_min7.note_names());

    let f_dom7 = f_major.dominant_seventh().unwrap();
    println!("   {}: {:?}", f_dom7, f_dom7.note_names());

    // Custom chord construction
    println!("\n🎹 Custom Chord Construction:");

    // Sus2 chord: 1, 2, 5
    let sus2_degrees = vec![Degree::natural(1), Degree::natural(2), Degree::natural(5)];
    let a_sus2 = a_major
        .build_named_chord(&sus2_degrees, "sus2".to_string())
        .unwrap();
    println!("   {}: {:?}", a_sus2, a_sus2.note_names());

    // Sus4 chord: 1, 4, 5
    let sus4_degrees = vec![Degree::natural(1), Degree::natural(4), Degree::natural(5)];
    let a_sus4 = a_major
        .build_named_chord(&sus4_degrees, "sus4".to_string())
        .unwrap();
    println!("   {}: {:?}", a_sus4, a_sus4.note_names());

    // Add9 chord: 1, 3, 5, 9 (where 9 = 2)
    let add9_degrees = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::natural(5),
        Degree::natural(2), // 9th = 2nd in next octave
    ];
    let c_add9 = c_major
        .build_named_chord(&add9_degrees, "add9".to_string())
        .unwrap();
    println!("   {}: {:?}", c_add9, c_add9.note_names());

    // Jazz harmonies
    println!("\n🎺 Jazz Harmonies:");

    // Major 7 #11: 1, 3, 5, 7, #4
    let maj7_sharp11_degrees = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::natural(5),
        Degree::natural(7),
        Degree::sharp(4),
    ];
    let c_maj7_sharp11 = c_major
        .build_named_chord(&maj7_sharp11_degrees, "Maj7♯11".to_string())
        .unwrap();
    println!("   {}: {:?}", c_maj7_sharp11, c_maj7_sharp11.note_names());

    // Half-diminished (m7♭5): 1, ♭3, ♭5, ♭7
    let half_dim_degrees = vec![
        Degree::natural(1),
        Degree::flat(3),
        Degree::flat(5),
        Degree::flat(7),
    ];
    let f_half_dim = f_major
        .build_named_chord(&half_dim_degrees, "m7♭5".to_string())
        .unwrap();
    println!("   {}: {:?}", f_half_dim, f_half_dim.note_names());

    // Altered dominant: 1, 3, ♭5, ♭7, ♯9 (♯9 = ♭3)
    let altered_dom_degrees = vec![
        Degree::natural(1),
        Degree::natural(3),
        Degree::flat(5),
        Degree::flat(7),
        Degree::flat(3), // ♯9 enharmonic with ♭3
    ];
    let a_alt = a_major
        .build_named_chord(&altered_dom_degrees, "7alt".to_string())
        .unwrap();
    println!("   {}: {:?}", a_alt, a_alt.note_names());

    // Same chord in different keys
    println!("\n🗝️  Same Chord Formula in Different Keys:");
    let keys = vec![
        ("C", Note::C),
        ("F", Note::F),
        ("G", Note::G),
        ("D", Note::D),
        ("A", Note::A),
        ("E", Note::E),
    ];

    println!("   Major Triads [1, 3, 5]:");
    for (key_name, root_note) in &keys {
        let scale = Scale::new(*root_note, ScaleType::Major);
        let major_chord = scale.major_triad().unwrap();
        println!("     {} Major: {:?}", key_name, major_chord.note_names());
    }

    println!("   Minor 7th Chords [1, ♭3, 5, ♭7]:");
    for (key_name, root_note) in &keys {
        let scale = Scale::new(*root_note, ScaleType::Major);
        let min7_chord = scale.minor_seventh().unwrap();
        println!("     {} Minor 7th: {:?}", key_name, min7_chord.note_names());
    }

    // Chord properties demonstration
    println!("\n📊 Chord Properties:");
    let demo_chord = c_major.major_seventh().unwrap();
    println!("   Chord: {}", demo_chord);
    println!("   Root: {}", demo_chord.root.name());
    println!("   Number of notes: {}", demo_chord.len());
    println!(
        "   Degrees: {:?}",
        demo_chord
            .degrees
            .iter()
            .map(|d| d.symbol())
            .collect::<Vec<_>>()
    );
    println!("   Note names: {:?}", demo_chord.note_names());
    println!("   Contains E: {}", demo_chord.contains(&Note::E));
    println!("   Contains F: {}", demo_chord.contains(&Note::F));

    // Anonymous chord (no name provided)
    println!("\n🎭 Anonymous Chord:");
    let mystery_degrees = vec![Degree::natural(1), Degree::sharp(2), Degree::sharp(4)];
    let mystery_chord = f_major.build_chord(&mystery_degrees).unwrap();
    println!("   {}: {:?}", mystery_chord, mystery_chord.note_names());
    println!("   Display name: {}", mystery_chord.display_name());

    println!("\n🎉 Chord builder demonstration complete!");
    println!("\nKey takeaways:");
    println!("• ChordBuilder trait enables flexible chord construction from any scale");
    println!("• Predefined methods for common chords (major, minor, 7ths, etc.)");
    println!("• Custom chords built from degree collections work in any key");
    println!("• Chord objects contain notes, degrees, and optional names");
    println!("• Same harmonic formulas work across all musical keys");
}
