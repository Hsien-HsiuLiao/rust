// Basic Types Examples with Music
// This file demonstrates Rust's basic types using music-related examples

fn main() {
    // String type
    let song_title: String = String::from("Bohemian Rhapsody");
    let artist: &str = "Queen"; // String slice
    
    // Vec (Vector) type
    let mut playlist: Vec<String> = Vec::new();
    playlist.push("Stairway to Heaven".to_string());
    playlist.push("Hotel California".to_string());
    playlist.push("Sweet Child O' Mine".to_string());
    
    // Alternative way to create a Vec
    let genres = vec!["Rock", "Jazz", "Classical", "Hip-Hop"];
    
    // Option type - representing something that might not exist
    let maybe_album = Some("A Night at the Opera");
    let no_album: Option<&str> = None;
    
    match maybe_album {
        Some(album) => println!("Album: {}", album),
        None => println!("No album found"),
    }
    
    // Result type - representing success or failure
    let search_result = search_song("Bohemian Rhapsody");
    match search_result {
        Ok(song_info) => println!("Found: {}", song_info),
        Err(error) => println!("Error: {}", error),
    }
    
    // Using ? operator for cleaner error handling
    if let Ok(song) = search_song("Hotel California") {
        println!("Successfully found: {}", song);
    }
    
    // Arrays
    let top_songs: [&str; 3] = ["Bohemian Rhapsody", "Stairway to Heaven", "Hotel California"];
    
    // Tuples
    let song_data: (&str, &str, u32) = ("Bohemian Rhapsody", "Queen", 1975);
    let (title, artist, year) = song_data;
    println!("{} by {} ({})", title, artist, year);
    
    // Numeric types
    let duration_minutes: u8 = 6;
    let duration_seconds: u8 = 0;
    let bpm: u16 = 144;
    let rating: f32 = 9.8;
    
    println!("Duration: {}:{}", duration_minutes, duration_seconds);
    println!("BPM: {}", bpm);
    println!("Rating: {}/10", rating);
    
    // Boolean
    let is_classic = true;
    let is_popular = true;
    
    if is_classic && is_popular {
        println!("This is a classic and popular song!");
    }
    
    // Char type
    let first_letter: char = 'B';
    println!("First letter: {}", first_letter);
}

// Function that returns a Result
fn search_song(query: &str) -> Result<String, String> {
    if query == "Bohemian Rhapsody" {
        Ok("Found: Bohemian Rhapsody by Queen (1975)".to_string())
    } else {
        Err("Song not found in database".to_string())
    }
}

// Function that returns an Option
fn get_song_duration(title: &str) -> Option<u32> {
    match title {
        "Bohemian Rhapsody" => Some(360), // 6 minutes in seconds
        "Stairway to Heaven" => Some(482), // 8 minutes 2 seconds
        "Hotel California" => Some(391),   // 6 minutes 31 seconds
        _ => None,
    }
} 