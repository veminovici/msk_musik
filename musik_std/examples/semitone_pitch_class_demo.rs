use musik_std::{Semitone, F_SHARP, G};

fn main() {
    println!("=== Semitone Pitch Class Demonstration ===\n");

    // Show how different semitones in different octaves have the same pitch class
    println!("1. Pitch classes across octaves:");
    
    let c_octaves = vec![
        Semitone::new(0),   // C-1 
        Semitone::new(12),  // C0
        Semitone::new(24),  // C1
        Semitone::new(60),  // C4 (middle C)
        Semitone::new(108), // C8
    ];

    for semitone in &c_octaves {
        let pitch_class = semitone.pitch_class();
        println!("   Semitone {} -> Pitch Class {} ({})", 
                 u8::from(*semitone), 
                 pitch_class.value(),
                 pitch_class.name());
    }

    // Show all 12 pitch classes
    println!("\n2. All 12 pitch classes from C4 octave:");
    for i in 0..12 {
        let semitone = Semitone::new(60 + i); // Starting from middle C
        let pitch_class = semitone.pitch_class();
        println!("   Semitone {} -> {} (pitch class {})", 
                 60 + i,
                 pitch_class.name(),
                 pitch_class.value());
    }

    // Demonstrate with constants
    println!("\n3. Using pitch class constants for comparison:");
    let f_sharp_4 = Semitone::new(66); // F#4
    let f_sharp_6 = Semitone::new(90); // F#6
    
    println!("   F#4 (semitone {}) has pitch class: {}", 
             u8::from(f_sharp_4),
             f_sharp_4.pitch_class().name());
    println!("   F#6 (semitone {}) has pitch class: {}", 
             u8::from(f_sharp_6),
             f_sharp_6.pitch_class().name());
    
    // Check if they match the constant
    assert_eq!(f_sharp_4.pitch_class(), F_SHARP);
    assert_eq!(f_sharp_6.pitch_class(), F_SHARP);
    println!("   ✓ Both match the F_SHARP constant");

    // Show edge case with high semitone values
    println!("\n4. Edge case - high semitone value:");
    let high_semitone = Semitone::new(255);
    let pitch_class = high_semitone.pitch_class();
    println!("   Semitone 255 -> {} (pitch class {})", 
             pitch_class.name(),
             pitch_class.value());
    println!("   (255 % 12 = {})", 255 % 12);

    // Practical example: finding enharmonic equivalents
    println!("\n5. Finding notes with the same pitch class:");
    let target_pitch_class = G;
    
    println!("   All G notes across different octaves:");
    for octave in 0..=8 {
        let semitone_value = octave * 12 + u8::from(target_pitch_class);
        let semitone = Semitone::new(semitone_value);
        
        if semitone.pitch_class() == target_pitch_class {
            println!("      Semitone {} -> {} (octave {})", 
                     semitone_value,
                     semitone.pitch_class().name(),
                     semitone.octave().value());
        }
    }
    
    println!("\n=== Demo Complete ===");
}