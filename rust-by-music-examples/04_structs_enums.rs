// Structs & Enums Examples with Music
// This file demonstrates Rust's structs and enums using music-related examples

fn main() {
    // Basic struct usage
    let song = Song {
        title: "Bohemian Rhapsody".to_string(),
        artist: "Queen".to_string(),
        year: 1975,
        duration: 360, // 6 minutes in seconds
        genre: Genre::Rock,
    };
    
    println!("Song: {}", song.title);
    println!("Artist: {}", song.artist);
    println!("Genre: {:?}", song.genre);
    
    // Struct update syntax
    let song2 = Song {
        title: "Stairway to Heaven".to_string(),
        artist: "Led Zeppelin".to_string(),
        year: 1971,
        duration: 482, // 8 minutes 2 seconds
        genre: Genre::Rock,
    };
    
    // Create a new song using some fields from song2
    let song3 = Song {
        title: "Hotel California".to_string(),
        artist: "Eagles".to_string(),
        ..song2 // Use remaining fields from song2
    };
    
    println!("Song 3: {} by {} ({})", song3.title, song3.artist, song3.year);
    
    // Tuple structs
    let coordinates = Point(3, 4);
    println!("Point: ({}, {})", coordinates.0, coordinates.1);
    
    // Unit struct
    let _music_player = MusicPlayer;
    
    // Enums with data
    let event = MusicEvent::Concert {
        venue: "Madison Square Garden".to_string(),
        date: "2024-06-15".to_string(),
        capacity: 20000,
    };
    
    match event {
        MusicEvent::Concert { venue, date, capacity } => {
            println!("Concert at {} on {} with capacity {}", venue, date, capacity);
        }
        MusicEvent::AlbumRelease { title, artist } => {
            println!("New album: {} by {}", title, artist);
        }
        MusicEvent::Award { category, winner } => {
            println!("Award for {}: {}", category, winner);
        }
    }
    
    // Enum methods
    let album = AlbumType::Studio {
        title: "A Night at the Opera".to_string(),
        recording_studio: "Rockfield Studios".to_string(),
    };
    
    println!("Album: {}", album.get_title());
    
    // Pattern matching with enums
    let songs = vec![
        Song {
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            year: 1975,
            duration: 360,
            genre: Genre::Rock,
        },
        Song {
            title: "Take Five".to_string(),
            artist: "Dave Brubeck".to_string(),
            year: 1959,
            duration: 324,
            genre: Genre::Jazz,
        },
        Song {
            title: "Moonlight Sonata".to_string(),
            artist: "Beethoven".to_string(),
            year: 1801,
            duration: 900,
            genre: Genre::Classical,
        },
    ];
    
    // Filter songs by genre
    let rock_songs: Vec<&Song> = songs.iter().filter(|song| song.genre == Genre::Rock).collect();
    println!("Rock songs: {:?}", rock_songs.iter().map(|s| &s.title).collect::<Vec<_>>());
    
    // Using if let for single pattern matching
    let maybe_concert = Some(MusicEvent::Concert {
        venue: "Royal Albert Hall".to_string(),
        date: "2024-07-20".to_string(),
        capacity: 5000,
    });
    
    if let Some(MusicEvent::Concert { venue, .. }) = maybe_concert {
        println!("Concert venue: {}", venue);
    }
}

// Basic struct
struct Song {
    title: String,
    artist: String,
    year: u32,
    duration: u32, // in seconds
    genre: Genre,
}

// Tuple struct
struct Point(i32, i32);

// Unit struct
struct MusicPlayer;

// Enum with variants that hold data
enum MusicEvent {
    Concert {
        venue: String,
        date: String,
        capacity: u32,
    },
    AlbumRelease {
        title: String,
        artist: String,
    },
    Award {
        category: String,
        winner: String,
    },
}

// Enum with different types of data
enum AlbumType {
    Studio {
        title: String,
        recording_studio: String,
    },
    Live {
        title: String,
        venue: String,
        date: String,
    },
    Compilation {
        title: String,
        songs: Vec<String>,
    },
}

// Enum with methods
impl AlbumType {
    fn get_title(&self) -> &str {
        match self {
            AlbumType::Studio { title, .. } => title,
            AlbumType::Live { title, .. } => title,
            AlbumType::Compilation { title, .. } => title,
        }
    }
}

// Simple enum
#[derive(Debug, PartialEq)]
enum Genre {
    Rock,
    Jazz,
    Classical,
    Pop,
    HipHop,
    Country,
    Electronic,
}

// Struct with lifetime parameters
struct Playlist<'a> {
    name: String,
    songs: Vec<&'a Song>,
}

impl<'a> Playlist<'a> {
    fn new(name: &str) -> Self {
        Playlist {
            name: name.to_string(),
            songs: Vec::new(),
        }
    }
    
    fn add_song(&mut self, song: &'a Song) {
        self.songs.push(song);
    }
    
    fn get_songs_by_genre(&self, genre: Genre) -> Vec<&'a Song> {
        self.songs.iter().filter(|song| song.genre == genre).copied().collect()
    }
} 