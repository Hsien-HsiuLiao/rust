// Functions & Methods Examples with Music
// This file demonstrates Rust's functions and methods using music-related examples

fn main() {
    // Basic function calls
    let song_info = get_song_info("Bohemian Rhapsody", "Queen", 1975);
    println!("{}", song_info);
    
    // Function with multiple parameters
    let duration = calculate_duration(6, 0);
    println!("Duration: {} seconds", duration);
    
    // Function that returns early
    let rating = get_song_rating("Bohemian Rhapsody");
    println!("Rating: {}/10", rating);
    
    // Function with default parameters (using Option)
    let song1 = create_song_with_year("Stairway to Heaven", "Led Zeppelin", Some(1971));
    let song2 = create_song_with_year("Hotel California", "Eagles", None);
    println!("{}", song1);
    println!("{}", song2);
    
    // Methods on structs
    let mut playlist = Playlist::new("Classic Rock");
    playlist.add_song("Bohemian Rhapsody".to_string());
    playlist.add_song("Stairway to Heaven".to_string());
    playlist.add_song("Hotel California".to_string());
    
    println!("Playlist: {}", playlist.name);
    println!("Song count: {}", playlist.song_count());
    println!("Is empty: {}", playlist.is_empty());
    
    // Method chaining
    let mut album = Album::new("A Night at the Opera")
        .add_track("Death on Two Legs")
        .add_track("Lazing on a Sunday Afternoon")
        .add_track("I'm in Love with My Car");
    
    println!("Album: {}", album.title);
    println!("Tracks: {}", album.track_count());
    
    // Associated functions (like static methods)
    let default_playlist = Playlist::default();
    println!("Default playlist: {}", default_playlist.name);
    
    // Function that takes a closure
    let songs = vec![
        "Bohemian Rhapsody".to_string(),
        "Stairway to Heaven".to_string(),
        "Hotel California".to_string(),
    ];
    
    let filtered_songs = filter_songs(songs, |song| song.len() > 15);
    println!("Long song titles: {:?}", filtered_songs);
    
    // Function with generic type
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = multiply_elements(numbers, 2);
    println!("Doubled: {:?}", doubled);
}

// Basic function
fn get_song_info(title: &str, artist: &str, year: u32) -> String {
    format!("{} by {} ({})", title, artist, year)
}

// Function with early return
fn get_song_rating(title: &str) -> u8 {
    if title == "Bohemian Rhapsody" {
        return 10; // Early return
    }
    
    if title == "Stairway to Heaven" {
        return 9;
    }
    
    7 // Default rating
}

// Function with Option parameter
fn create_song_with_year(title: &str, artist: &str, year: Option<u32>) -> String {
    match year {
        Some(y) => format!("{} by {} ({})", title, artist, y),
        None => format!("{} by {} (Unknown year)", title, artist),
    }
}

// Function that takes a closure
fn filter_songs<F>(songs: Vec<String>, predicate: F) -> Vec<String>
where
    F: Fn(&String) -> bool,
{
    songs.into_iter().filter(predicate).collect()
}

// Generic function
fn multiply_elements<T>(elements: Vec<T>, multiplier: T) -> Vec<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    elements.into_iter().map(|x| x * multiplier).collect()
}

// Struct with methods
struct Playlist {
    name: String,
    songs: Vec<String>,
}

impl Playlist {
    // Constructor (associated function)
    fn new(name: &str) -> Self {
        Playlist {
            name: name.to_string(),
            songs: Vec::new(),
        }
    }
    
    // Method that takes &mut self
    fn add_song(&mut self, song: String) {
        self.songs.push(song);
    }
    
    // Method that takes &self
    fn song_count(&self) -> usize {
        self.songs.len()
    }
    
    fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }
    
    // Associated function (like static method)
    fn default() -> Self {
        Playlist {
            name: "Default Playlist".to_string(),
            songs: Vec::new(),
        }
    }
}

// Struct with method chaining
struct Album {
    title: String,
    tracks: Vec<String>,
}

impl Album {
    fn new(title: &str) -> Self {
        Album {
            title: title.to_string(),
            tracks: Vec::new(),
        }
    }
    
    // Method that returns &mut self for chaining
    fn add_track(&mut self, track: &str) -> &mut Self {
        self.tracks.push(track.to_string());
        self
    }
    
    fn track_count(&self) -> usize {
        self.tracks.len()
    }
} 