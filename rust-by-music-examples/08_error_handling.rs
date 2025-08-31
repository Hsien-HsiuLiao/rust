// Error Handling Examples with Music
// This file demonstrates Rust's error handling using music-related examples

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    // Example 1: Basic Result usage
    match search_song("Bohemian Rhapsody") {
        Ok(song_info) => println!("Found: {}", song_info),
        Err(error) => println!("Error: {}", error),
    }
    
    // Example 2: Using ? operator
    if let Err(e) = process_song_file("song_data.txt") {
        println!("Failed to process song file: {}", e);
    }
    
    // Example 3: Custom error types
    let result = create_playlist("Classic Rock", 10);
    match result {
        Ok(playlist) => println!("Created playlist: {}", playlist.name),
        Err(PlaylistError::InvalidName) => println!("Invalid playlist name"),
        Err(PlaylistError::InvalidCapacity(cap)) => println!("Invalid capacity: {}", cap),
    }
    
    // Example 4: Error propagation with context
    let result = load_song_with_context("bohemian_rhapsody.mp3");
    if let Err(e) = result {
        println!("Failed to load song: {}", e);
        // Print the error chain
        let mut source = e.source();
        while let Some(err) = source {
            println!("Caused by: {}", err);
            source = err.source();
        }
    }
    
    // Example 5: Combining different error types
    let result = process_music_data("music_data.json");
    match result {
        Ok(data) => println!("Processed music data: {:?}", data),
        Err(e) => println!("Failed to process music data: {}", e),
    }
    
    // Example 6: Error handling with Option
    let song_duration = get_song_duration("Bohemian Rhapsody");
    if let Some(duration) = song_duration {
        println!("Song duration: {} seconds", duration);
    } else {
        println!("Song duration not found");
    }
    
    // Example 7: Using unwrap_or_else for default values
    let rating = get_song_rating("Unknown Song").unwrap_or_else(|_| 5);
    println!("Song rating: {}/10", rating);
    
    // Example 8: Error handling in async-like scenarios
    let result = simulate_async_song_search("Bohemian Rhapsody");
    match result {
        Ok(song) => println!("Async search result: {}", song),
        Err(e) => println!("Async search failed: {}", e),
    }
    
    // Example 9: Error handling with collections
    let songs = vec!["Bohemian Rhapsody", "Stairway to Heaven", "Hotel California"];
    let results: Vec<Result<String, MusicError>> = songs
        .iter()
        .map(|&song| search_song_detailed(song))
        .collect();
    
    for (song, result) in songs.iter().zip(results.iter()) {
        match result {
            Ok(info) => println!("{}: {}", song, info),
            Err(e) => println!("{}: Error - {}", song, e),
        }
    }
    
    // Example 10: Error handling with file operations
    if let Err(e) = write_song_to_file("bohemian_rhapsody.txt", "Bohemian Rhapsody by Queen") {
        println!("Failed to write song to file: {}", e);
    }
}

// Basic function returning Result
fn search_song(query: &str) -> Result<String, String> {
    if query == "Bohemian Rhapsody" {
        Ok("Found: Bohemian Rhapsody by Queen (1975)".to_string())
    } else {
        Err(format!("Song '{}' not found in database", query))
    }
}

// Function using ? operator for error propagation
fn process_song_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Custom error types
#[derive(Debug)]
enum PlaylistError {
    InvalidName,
    InvalidCapacity(u32),
}

impl fmt::Display for PlaylistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlaylistError::InvalidName => write!(f, "Invalid playlist name"),
            PlaylistError::InvalidCapacity(cap) => write!(f, "Invalid capacity: {}", cap),
        }
    }
}

impl Error for PlaylistError {}

// Function returning custom error type
fn create_playlist(name: &str, capacity: u32) -> Result<Playlist, PlaylistError> {
    if name.is_empty() {
        return Err(PlaylistError::InvalidName);
    }
    
    if capacity == 0 || capacity > 1000 {
        return Err(PlaylistError::InvalidCapacity(capacity));
    }
    
    Ok(Playlist {
        name: name.to_string(),
        capacity,
        songs: Vec::new(),
    })
}

struct Playlist {
    name: String,
    capacity: u32,
    songs: Vec<String>,
}

// Custom error type with context
#[derive(Debug)]
struct SongLoadError {
    message: String,
    source: Option<Box<dyn Error + 'static>>,
}

impl SongLoadError {
    fn new(message: &str) -> Self {
        SongLoadError {
            message: message.to_string(),
            source: None,
        }
    }
    
    fn with_source(message: &str, source: Box<dyn Error + 'static>) -> Self {
        SongLoadError {
            message: message.to_string(),
            source: Some(source),
        }
    }
}

impl fmt::Display for SongLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SongLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

// Function demonstrating error context
fn load_song_with_context(filename: &str) -> Result<String, SongLoadError> {
    File::open(filename)
        .map_err(|e| SongLoadError::with_source(
            &format!("Failed to open song file: {}", filename),
            Box::new(e)
        ))?
        .read_to_string(&mut String::new())
        .map_err(|e| SongLoadError::with_source(
            "Failed to read song file contents",
            Box::new(e)
        ))
        .map(|_| "Song loaded successfully".to_string())
}

// Generic error type for music operations
#[derive(Debug)]
enum MusicError {
    Io(io::Error),
    Parse(String),
    NotFound(String),
    InvalidFormat(String),
}

impl fmt::Display for MusicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MusicError::Io(e) => write!(f, "IO error: {}", e),
            MusicError::Parse(e) => write!(f, "Parse error: {}", e),
            MusicError::NotFound(item) => write!(f, "Not found: {}", item),
            MusicError::InvalidFormat(format) => write!(f, "Invalid format: {}", format),
        }
    }
}

impl Error for MusicError {}

impl From<io::Error> for MusicError {
    fn from(err: io::Error) -> Self {
        MusicError::Io(err)
    }
}

// Function combining different error types
fn process_music_data(filename: &str) -> Result<String, MusicError> {
    let mut file = File::open(filename)?; // Uses From<io::Error> for MusicError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    if contents.is_empty() {
        return Err(MusicError::Parse("Empty file".to_string()));
    }
    
    Ok(contents)
}

// Function returning Option for cases where absence is not an error
fn get_song_duration(title: &str) -> Option<u32> {
    match title {
        "Bohemian Rhapsody" => Some(360),
        "Stairway to Heaven" => Some(482),
        "Hotel California" => Some(391),
        _ => None,
    }
}

// Function using unwrap_or_else
fn get_song_rating(title: &str) -> Result<u8, String> {
    match title {
        "Bohemian Rhapsody" => Ok(10),
        "Stairway to Heaven" => Ok(9),
        "Hotel California" => Ok(8),
        _ => Err("Song not found".to_string()),
    }
}

// Function simulating async-like error handling
fn simulate_async_song_search(query: &str) -> Result<String, MusicError> {
    // Simulate some async operation that could fail
    if query.is_empty() {
        return Err(MusicError::InvalidFormat("Empty query".to_string()));
    }
    
    if query == "Bohemian Rhapsody" {
        Ok("Found: Bohemian Rhapsody by Queen".to_string())
    } else {
        Err(MusicError::NotFound(query.to_string()))
    }
}

// Function returning detailed search results
fn search_song_detailed(query: &str) -> Result<String, MusicError> {
    match query {
        "Bohemian Rhapsody" => Ok("Queen - 1975 - 6:00".to_string()),
        "Stairway to Heaven" => Ok("Led Zeppelin - 1971 - 8:02".to_string()),
        "Hotel California" => Ok("Eagles - 1976 - 6:31".to_string()),
        _ => Err(MusicError::NotFound(query.to_string())),
    }
}

// Function demonstrating file write error handling
fn write_song_to_file(filename: &str, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Function demonstrating error handling with multiple fallback strategies
fn get_song_info_with_fallback(title: &str) -> Result<String, MusicError> {
    // Try primary source
    match search_song_detailed(title) {
        Ok(info) => Ok(info),
        Err(_) => {
            // Try fallback source
            match get_basic_song_info(title) {
                Ok(info) => Ok(format!("Fallback: {}", info)),
                Err(e) => Err(e),
            }
        }
    }
}

fn get_basic_song_info(title: &str) -> Result<String, MusicError> {
    if title == "Bohemian Rhapsody" {
        Ok("Queen - Classic Rock".to_string())
    } else {
        Err(MusicError::NotFound(title.to_string()))
    }
}

// Function demonstrating error handling with custom error conversion
fn process_song_metadata(metadata: &str) -> Result<SongMetadata, MusicError> {
    let parts: Vec<&str> = metadata.split(',').collect();
    
    if parts.len() != 3 {
        return Err(MusicError::InvalidFormat(
            format!("Expected 3 parts, got {}", parts.len())
        ));
    }
    
    let duration = parts[2].parse::<u32>()
        .map_err(|_| MusicError::Parse("Invalid duration".to_string()))?;
    
    Ok(SongMetadata {
        title: parts[0].to_string(),
        artist: parts[1].to_string(),
        duration,
    })
}

struct SongMetadata {
    title: String,
    artist: String,
    duration: u32,
} 