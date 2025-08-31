// Closures & Iterators Examples with Music
// This file demonstrates Rust's functional programming concepts using music-related examples

use std::collections::HashMap;

fn main() {
    // Example 1: Basic closures
    let add_volume = |current: f32, increase: f32| current + increase;
    let new_volume = add_volume(50.0, 25.0);
    println!("New volume: {}", new_volume);
    
    // Example 2: Closures with environment capture
    let base_volume = 50.0;
    let volume_adjuster = |adjustment: f32| base_volume + adjustment;
    
    println!("Volume +10: {}", volume_adjuster(10.0));
    println!("Volume -20: {}", volume_adjuster(-20.0));
    
    // Example 3: Closures with move keyword
    let song_title = "Bohemian Rhapsody".to_string();
    let title_printer = move || println!("Song: {}", song_title);
    title_printer();
    // song_title is no longer accessible here due to move
    
    // Example 4: Closures as function parameters
    let songs = vec![
        "Bohemian Rhapsody".to_string(),
        "Stairway to Heaven".to_string(),
        "Hotel California".to_string(),
    ];
    
    let filtered_songs = filter_songs(&songs, |song| song.len() > 15);
    println!("Long songs: {:?}", filtered_songs);
    
    let processed_songs = process_songs(&songs, |song| format!("🎵 {}", song));
    println!("Processed songs: {:?}", processed_songs);
    
    // Example 5: Closures with different trait bounds
    let song_ratings = vec![8, 9, 10, 7, 9];
    let average_rating = calculate_average(&song_ratings);
    println!("Average rating: {}", average_rating);
    
    // Example 6: Iterator methods
    let song_durations = vec![360, 482, 391, 324, 450];
    
    let total_duration: u32 = song_durations.iter().sum();
    println!("Total duration: {} seconds", total_duration);
    
    let long_songs: Vec<&u32> = song_durations.iter().filter(|&&duration| duration > 400).collect();
    println!("Long songs (>400s): {:?}", long_songs);
    
    let formatted_durations: Vec<String> = song_durations
        .iter()
        .map(|&duration| format!("{}:{:02}", duration / 60, duration % 60))
        .collect();
    println!("Formatted durations: {:?}", formatted_durations);
    
    // Example 7: Chaining iterator methods
    let music_data = vec![
        ("Bohemian Rhapsody", "Queen", 1975, 360),
        ("Stairway to Heaven", "Led Zeppelin", 1971, 482),
        ("Hotel California", "Eagles", 1976, 391),
        ("Take Five", "Dave Brubeck", 1959, 324),
    ];
    
    let classic_rock_songs: Vec<String> = music_data
        .iter()
        .filter(|(_, _, year, _)| *year >= 1970 && *year <= 1980)
        .map(|(title, artist, _, _)| format!("{} by {}", title, artist))
        .collect();
    
    println!("Classic rock songs: {:?}", classic_rock_songs);
    
    // Example 8: Custom iterators
    let playlist = Playlist::new("Classic Rock");
    for song in playlist {
        println!("Playing: {}", song);
    }
    
    // Example 9: Iterator adaptors
    let song_titles = vec![
        "Bohemian Rhapsody",
        "Stairway to Heaven",
        "Hotel California",
        "Sweet Child O' Mine",
    ];
    
    let enumerated_songs: Vec<(usize, &str)> = song_titles.iter().enumerate().collect();
    println!("Enumerated songs: {:?}", enumerated_songs);
    
    let zipped_songs: Vec<(&str, &str)> = song_titles.iter()
        .zip(song_titles.iter().skip(1))
        .collect();
    println!("Zipped songs: {:?}", zipped_songs);
    
    // Example 10: Folding and reducing
    let song_lengths = vec![15, 20, 18, 22, 19];
    let total_length: usize = song_lengths.iter().fold(0, |acc, &len| acc + len);
    println!("Total title length: {}", total_length);
    
    let longest_title = song_lengths.iter().max().unwrap();
    println!("Longest title length: {}", longest_title);
    
    // Example 11: Closures with structs
    let music_processor = MusicProcessor::new();
    let result = music_processor.process_songs(&songs, |song| song.to_uppercase());
    println!("Uppercase songs: {:?}", result);
    
    // Example 12: Advanced iterator patterns
    let music_library = create_music_library();
    
    let rock_songs: Vec<&str> = music_library
        .iter()
        .filter(|(_, genre)| **genre == "Rock")
        .map(|(title, _)| *title)
        .collect();
    
    println!("Rock songs: {:?}", rock_songs);
    
    // Example 13: Iterator with state
    let mut music_stream = MusicStream::new();
    music_stream.add_song("Bohemian Rhapsody");
    music_stream.add_song("Stairway to Heaven");
    
    for song in &mut music_stream {
        println!("Streaming: {}", song);
    }
    
    // Example 14: Functional composition
    let pipeline = compose_functions(
        |s: &str| s.to_uppercase(),
        |s: String| format!("🎵 {}", s),
        |s: String| format!("{} 🎵", s),
    );
    
    let result = pipeline("Bohemian Rhapsody");
    println!("Pipelined result: {}", result);
}

// Function that takes a closure as parameter
fn filter_songs<F>(songs: &[String], predicate: F) -> Vec<String>
where
    F: Fn(&String) -> bool,
{
    songs.iter().filter(|song| predicate(song)).cloned().collect()
}

// Function that processes songs using a closure
fn process_songs<F>(songs: &[String], processor: F) -> Vec<String>
where
    F: Fn(&String) -> String,
{
    songs.iter().map(|song| processor(song)).collect()
}

// Function that calculates average using a closure
fn calculate_average<F>(numbers: &[u32]) -> f64
where
    F: Fn(u32) -> f64,
{
    let sum: u32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

// Custom iterator implementation
struct Playlist {
    songs: Vec<String>,
    current_index: usize,
}

impl Playlist {
    fn new(name: &str) -> Self {
        Playlist {
            songs: vec![
                format!("{} - Bohemian Rhapsody", name),
                format!("{} - Stairway to Heaven", name),
                format!("{} - Hotel California", name),
            ],
            current_index: 0,
        }
    }
}

impl Iterator for Playlist {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.songs.len() {
            let song = self.songs[self.current_index].clone();
            self.current_index += 1;
            Some(song)
        } else {
            None
        }
    }
}

// Struct with closure-based processing
struct MusicProcessor {
    default_processor: Box<dyn Fn(&str) -> String>,
}

impl MusicProcessor {
    fn new() -> Self {
        MusicProcessor {
            default_processor: Box::new(|song| format!("Processed: {}", song)),
        }
    }
    
    fn process_songs<F>(&self, songs: &[String], processor: F) -> Vec<String>
    where
        F: Fn(&String) -> String,
    {
        songs.iter().map(|song| processor(song)).collect()
    }
    
    fn process_with_default(&self, song: &str) -> String {
        (self.default_processor)(song)
    }
}

// Function to create a music library
fn create_music_library() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Bohemian Rhapsody", "Rock"),
        ("Stairway to Heaven", "Rock"),
        ("Hotel California", "Rock"),
        ("Take Five", "Jazz"),
        ("Moonlight Sonata", "Classical"),
    ]
}

// Iterator with state
struct MusicStream {
    songs: Vec<String>,
    current_index: usize,
}

impl MusicStream {
    fn new() -> Self {
        MusicStream {
            songs: Vec::new(),
            current_index: 0,
        }
    }
    
    fn add_song(&mut self, song: &str) {
        self.songs.push(song.to_string());
    }
}

impl Iterator for MusicStream {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.songs.len() {
            let song = self.songs[self.current_index].clone();
            self.current_index += 1;
            Some(song)
        } else {
            None
        }
    }
}

// Function composition
fn compose_functions<A, B, C, F, G>(f: F, g: G, h: impl Fn(String) -> String) -> impl Fn(&str) -> String
where
    F: Fn(&str) -> String,
    G: Fn(String) -> String,
{
    move |x| h(g(f(x)))
}

// Advanced closure examples

// Closure that captures and modifies state
fn create_volume_controller() -> impl FnMut(f32) -> f32 {
    let mut current_volume = 50.0;
    move |change| {
        current_volume = (current_volume + change).max(0.0).min(100.0);
        current_volume
    }
}

// Closure that returns different types based on input
fn create_music_analyzer() -> impl Fn(&str) -> Box<dyn std::fmt::Display> {
    move |song| {
        if song.len() > 15 {
            Box::new(format!("Long song: {} characters", song.len()))
        } else {
            Box::new(format!("Short song: {} characters", song.len()))
        }
    }
}

// Iterator that generates infinite sequence
struct InfiniteMusicSequence {
    current: u32,
}

impl InfiniteMusicSequence {
    fn new() -> Self {
        InfiniteMusicSequence { current: 0 }
    }
}

impl Iterator for InfiniteMusicSequence {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        let song = format!("Generated Song {}", self.current);
        self.current += 1;
        Some(song)
    }
}

// Lazy evaluation with iterators
fn create_lazy_playlist() -> impl Iterator<Item = String> {
    (1..=5).map(|i| format!("Lazy Song {}", i))
}

// Closure with multiple captures
fn create_music_filter(min_length: usize, max_length: usize) -> impl Fn(&str) -> bool {
    move |song| {
        let length = song.len();
        length >= min_length && length <= max_length
    }
}

// Iterator with custom state
struct MusicQueue {
    songs: Vec<String>,
    shuffle_mode: bool,
}

impl MusicQueue {
    fn new() -> Self {
        MusicQueue {
            songs: vec![
                "Bohemian Rhapsody".to_string(),
                "Stairway to Heaven".to_string(),
                "Hotel California".to_string(),
            ],
            shuffle_mode: false,
        }
    }
    
    fn toggle_shuffle(&mut self) {
        self.shuffle_mode = !self.shuffle_mode;
    }
}

impl Iterator for MusicQueue {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.songs.is_empty() {
            None
        } else if self.shuffle_mode {
            // Simple shuffle: take from random position
            let index = (self.songs.len() as f64 * 0.5) as usize;
            Some(self.songs.remove(index))
        } else {
            // Normal order: take from front
            Some(self.songs.remove(0))
        }
    }
}

// Functional programming utilities
fn map_reduce<T, U, F, G>(items: &[T], mapper: F, reducer: G) -> U
where
    F: Fn(&T) -> U,
    G: Fn(U, U) -> U,
    U: Clone,
{
    items.iter().map(mapper).fold(None, |acc, item| {
        match acc {
            Some(current) => Some(reducer(current, item)),
            None => Some(item),
        }
    }).unwrap()
}

// Example usage of advanced features
fn demonstrate_advanced_features() {
    // Volume controller
    let mut volume_controller = create_volume_controller();
    println!("Volume: {}", volume_controller(10.0));
    println!("Volume: {}", volume_controller(-5.0));
    
    // Music analyzer
    let analyzer = create_music_analyzer();
    println!("Analysis: {}", analyzer("Bohemian Rhapsody"));
    println!("Analysis: {}", analyzer("Short"));
    
    // Infinite sequence (take first 5)
    let infinite = InfiniteMusicSequence::new();
    let first_five: Vec<String> = infinite.take(5).collect();
    println!("First five: {:?}", first_five);
    
    // Lazy playlist
    let lazy_playlist: Vec<String> = create_lazy_playlist().collect();
    println!("Lazy playlist: {:?}", lazy_playlist);
    
    // Music filter
    let filter = create_music_filter(10, 20);
    let songs = ["Short", "Medium Length Song", "Very Long Song Title"];
    let filtered: Vec<&str> = songs.iter().filter(|&&song| filter(song)).collect();
    println!("Filtered songs: {:?}", filtered);
    
    // Music queue with shuffle
    let mut queue = MusicQueue::new();
    queue.toggle_shuffle();
    let shuffled: Vec<String> = queue.collect();
    println!("Shuffled queue: {:?}", shuffled);
    
    // Map-reduce example
    let numbers = vec![1, 2, 3, 4, 5];
    let result = map_reduce(&numbers, |&x| x * 2, |a, b| a + b);
    println!("Map-reduce result: {}", result);
} 