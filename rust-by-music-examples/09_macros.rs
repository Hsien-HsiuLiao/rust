// Macros Examples with Music
// This file demonstrates Rust's macro system using music-related examples

fn main() {
    // Example 1: Declarative macro (macro_rules!)
    let song = create_song!("Bohemian Rhapsody", "Queen", 1975);
    println!("{}", song);
    
    let songs = create_songs![
        "Stairway to Heaven" => "Led Zeppelin" => 1971,
        "Hotel California" => "Eagles" => 1976,
        "Sweet Child O' Mine" => "Guns N' Roses" => 1987
    ];
    
    for song in songs {
        println!("{}", song);
    }
    
    // Example 2: Procedural macro (derive macro simulation)
    let playlist = Playlist::new("Classic Rock");
    println!("Playlist: {}", playlist);
    
    // Example 3: Function-like macro
    let duration = format_duration!(360);
    println!("Duration: {}", duration);
    
    let rating = calculate_rating!(10, 9, 8, 7);
    println!("Average rating: {}", rating);
    
    // Example 4: Macro with repetition
    let song_list = song_list![
        "Bohemian Rhapsody",
        "Stairway to Heaven", 
        "Hotel California"
    ];
    println!("Songs: {:?}", song_list);
    
    // Example 5: Macro with conditional compilation
    if cfg!(feature = "debug") {
        debug_print!("Debug mode enabled");
    }
    
    // Example 6: Macro with different token types
    let song_info = song_info! {
        title: "Bohemian Rhapsody",
        artist: "Queen",
        year: 1975,
        duration: 360
    };
    println!("Song info: {:?}", song_info);
    
    // Example 7: Macro with custom syntax
    let playlist = playlist! {
        "Classic Rock" => [
            "Bohemian Rhapsody",
            "Stairway to Heaven",
            "Hotel California"
        ]
    };
    println!("Playlist: {:?}", playlist);
    
    // Example 8: Macro with error handling
    let result = safe_parse_duration!("360");
    match result {
        Ok(duration) => println!("Parsed duration: {} seconds", duration),
        Err(e) => println!("Parse error: {}", e),
    }
    
    // Example 9: Macro with type inference
    let song = typed_song!("Bohemian Rhapsody", "Queen", 1975);
    println!("Typed song: {:?}", song);
    
    // Example 10: Macro with multiple patterns
    let result = match_song_type!("Bohemian Rhapsody");
    println!("Song type: {}", result);
}

// Declarative macro using macro_rules!
macro_rules! create_song {
    ($title:expr, $artist:expr, $year:expr) => {
        format!("{} by {} ({})", $title, $artist, $year)
    };
}

// Macro with repetition
macro_rules! create_songs {
    ($($title:expr => $artist:expr => $year:expr),*) => {
        vec![
            $(
                create_song!($title, $artist, $year)
            ),*
        ]
    };
}

// Macro with different token types
macro_rules! song_list {
    ($($song:expr),*) => {
        vec![
            $($song.to_string()),*
        ]
    };
}

// Function-like macro
macro_rules! format_duration {
    ($seconds:expr) => {
        {
            let minutes = $seconds / 60;
            let remaining_seconds = $seconds % 60;
            format!("{}:{:02}", minutes, remaining_seconds)
        }
    };
}

// Macro with arithmetic operations
macro_rules! calculate_rating {
    ($($rating:expr),*) => {
        {
            let ratings = vec![$($rating),*];
            let sum: u32 = ratings.iter().sum();
            sum / ratings.len() as u32
        }
    };
}

// Macro with conditional compilation
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug")]
        println!($($arg)*);
    };
}

// Macro with custom syntax for song info
macro_rules! song_info {
    ($($key:ident: $value:expr),*) => {
        {
            let mut info = std::collections::HashMap::new();
            $(
                info.insert(stringify!($key).to_string(), $value.to_string());
            )*
            info
        }
    };
}

// Macro with custom syntax for playlists
macro_rules! playlist {
    ($name:expr => [$($song:expr),*]) => {
        {
            let songs = vec![$($song.to_string()),*];
            Playlist {
                name: $name.to_string(),
                songs,
            }
        }
    };
}

// Macro with error handling
macro_rules! safe_parse_duration {
    ($duration:expr) => {
        {
            $duration.parse::<u32>()
                .map_err(|_| "Invalid duration format".to_string())
        }
    };
}

// Macro with type inference
macro_rules! typed_song {
    ($title:expr, $artist:expr, $year:expr) => {
        TypedSong {
            title: $title.to_string(),
            artist: $artist.to_string(),
            year: $year,
        }
    };
}

// Macro with multiple patterns
macro_rules! match_song_type {
    ("Bohemian Rhapsody") => { "Epic Rock Opera" };
    ("Stairway to Heaven") => { "Progressive Rock" };
    ("Hotel California") => { "Soft Rock" };
    ($other:expr) => { "Unknown Genre" };
}

// Struct for macro examples
#[derive(Debug)]
struct Playlist {
    name: String,
    songs: Vec<String>,
}

impl Playlist {
    fn new(name: &str) -> Self {
        Playlist {
            name: name.to_string(),
            songs: Vec::new(),
        }
    }
}

// Implement Display for Playlist
impl std::fmt::Display for Playlist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Playlist: {} ({} songs)", self.name, self.songs.len())
    }
}

// Struct for typed song macro
#[derive(Debug)]
struct TypedSong {
    title: String,
    artist: String,
    year: u32,
}

// Advanced macro examples

// Macro that creates a builder pattern
macro_rules! builder_struct {
    ($name:ident { $($field:ident: $type:ty),* }) => {
        struct $name {
            $($field: $type),*
        }
        
        impl $name {
            fn new() -> Self {
                $name {
                    $($field: Default::default()),*
                }
            }
            
            $(
                fn $field(mut self, $field: $type) -> Self {
                    self.$field = $field;
                    self
                }
            )*
        }
    };
}

// Use the builder macro
builder_struct!(SongBuilder {
    title: String,
    artist: String,
    year: u32,
    duration: u32
});

// Macro that creates enum variants
macro_rules! create_music_types {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        enum MusicType {
            $($variant),*
        }
        
        impl MusicType {
            fn all_types() -> Vec<MusicType> {
                vec![
                    $(MusicType::$variant),*
                ]
            }
        }
    };
}

// Use the music types macro
create_music_types!(Rock, Jazz, Classical, Pop, HipHop);

// Macro with complex pattern matching
macro_rules! process_song {
    (Rock $title:expr) => {
        format!("🤘 {} - Rock anthem! 🤘", $title)
    };
    (Jazz $title:expr) => {
        format!("🎷 {} - Smooth jazz vibes 🎷", $title)
    };
    (Classical $title:expr) => {
        format!("🎻 {} - Timeless classical 🎻", $title)
    };
    ($genre:ident $title:expr) => {
        format!("🎵 {} - {} music 🎵", $title, stringify!($genre))
    };
}

// Macro that creates test functions
macro_rules! create_tests {
    ($($test_name:ident: $test_fn:expr),*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            
            $(
                #[test]
                fn $test_name() {
                    $test_fn();
                }
            )*
        }
    };
}

// Use the test macro
create_tests!(
    test_create_song: || {
        let song = create_song!("Test Song", "Test Artist", 2024);
        assert_eq!(song, "Test Song by Test Artist (2024)");
    },
    test_format_duration: || {
        let duration = format_duration!(125);
        assert_eq!(duration, "2:05");
    }
);

// Macro with hygienic variables
macro_rules! count_songs {
    ($($song:expr),*) => {
        {
            let count = 0;
            $(
                let count = count + 1;
            )*
            count
        }
    };
}

// Macro that generates documentation
macro_rules! document_music_function {
    ($name:ident, $description:expr) => {
        #[doc = $description]
        fn $name() {
            println!("Function: {}", stringify!($name));
        }
    };
}

// Use the documentation macro
document_music_function!(
    play_music,
    "Plays the currently selected music track"
);

document_music_function!(
    pause_music,
    "Pauses the currently playing music track"
);

// Macro with recursive patterns
macro_rules! sum_durations {
    () => { 0 };
    ($first:expr) => { $first };
    ($first:expr, $($rest:expr),*) => {
        $first + sum_durations!($($rest),*)
    };
}

// Macro with conditional compilation and features
macro_rules! feature_gated_function {
    ($feature:expr, $name:ident, $body:block) => {
        #[cfg(feature = $feature)]
        fn $name() $body
    };
}

// Use the feature-gated macro
feature_gated_function!("advanced", advanced_music_processing, {
    println!("Advanced music processing enabled");
});

// Macro that creates a music library
macro_rules! create_music_library {
    ($($song:expr => $artist:expr),*) => {
        {
            let mut library = std::collections::HashMap::new();
            $(
                library.insert($song.to_string(), $artist.to_string());
            )*
            library
        }
    };
}

// Macro with custom error types
macro_rules! define_music_error {
    ($error_name:ident, $($variant:ident($($field:ty),*)),*) => {
        #[derive(Debug)]
        enum $error_name {
            $($variant($($field),*)),*
        }
        
        impl std::fmt::Display for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        $error_name::$variant($($field),*) => {
                            write!(f, "{}: {:?}", stringify!($variant), ($($field),*))
                        }
                    ),*
                }
            }
        }
        
        impl std::error::Error for $error_name {}
    };
}

// Use the error definition macro
define_music_error!(MusicError,
    NotFound(String),
    InvalidFormat(String),
    ParseError(std::num::ParseIntError)
); 