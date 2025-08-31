// Traits Examples with Music
// This file demonstrates Rust's trait system using music-related examples

fn main() {
    // Example 1: Basic trait implementation
    let song = Song {
        title: "Bohemian Rhapsody".to_string(),
        artist: "Queen".to_string(),
        duration: 360,
    };
    
    song.play();
    song.pause();
    println!("Duration: {} seconds", song.get_duration());
    
    // Example 2: Default trait implementations
    let album = Album {
        title: "A Night at the Opera".to_string(),
        tracks: 12,
    };
    
    album.play(); // Uses default implementation
    album.stop(); // Uses default implementation
    
    // Example 3: Trait as parameters
    let playlist = Playlist {
        name: "Classic Rock".to_string(),
        songs: vec![song],
    };
    
    play_media(&playlist);
    play_media(&album);
    
    // Example 4: Trait bounds
    let songs = vec![
        Song {
            title: "Stairway to Heaven".to_string(),
            artist: "Led Zeppelin".to_string(),
            duration: 482,
        },
        Song {
            title: "Hotel California".to_string(),
            artist: "Eagles".to_string(),
            duration: 391,
        },
    ];
    
    let total_duration = calculate_total_duration(&songs);
    println!("Total duration: {} seconds", total_duration);
    
    // Example 5: Associated types
    let music_player = MusicPlayer::new();
    let song_id = music_player.add_song("Bohemian Rhapsody".to_string());
    let song_info = music_player.get_song(song_id);
    println!("Song info: {:?}", song_info);
    
    // Example 6: Trait objects
    let media_items: Vec<Box<dyn Playable>> = vec![
        Box::new(Song {
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            duration: 360,
        }),
        Box::new(Album {
            title: "A Night at the Opera".to_string(),
            tracks: 12,
        }),
    ];
    
    for item in &media_items {
        item.play();
    }
    
    // Example 7: Supertraits
    let concert = Concert {
        venue: "Madison Square Garden".to_string(),
        date: "2024-06-15".to_string(),
        capacity: 20000,
    };
    
    concert.schedule();
    concert.perform();
    
    // Example 8: Trait extensions
    let song_title = "Bohemian Rhapsody";
    let formatted = song_title.format_for_display();
    println!("{}", formatted);
    
    // Example 9: Conditional trait implementations
    let rock_song = RockSong {
        title: "Stairway to Heaven".to_string(),
        artist: "Led Zeppelin".to_string(),
        duration: 482,
        guitar_solos: 2,
    };
    
    rock_song.play();
    rock_song.rock_out();
}

// Basic trait definition
trait Playable {
    fn play(&self);
    fn pause(&self);
    fn stop(&self);
    fn get_duration(&self) -> u32;
}

// Trait with default implementations
trait MediaPlayer {
    fn play(&self) {
        println!("Playing media...");
    }
    
    fn stop(&self) {
        println!("Stopping media...");
    }
}

// Supertrait example
trait Event {
    fn schedule(&self);
}

trait Performance: Event {
    fn perform(&self);
}

// Trait with associated types
trait MusicLibrary {
    type SongId;
    type SongInfo;
    
    fn add_song(&mut self, song: String) -> Self::SongId;
    fn get_song(&self, id: Self::SongId) -> Option<Self::SongInfo>;
}

// Struct implementing Playable trait
struct Song {
    title: String,
    artist: String,
    duration: u32,
}

impl Playable for Song {
    fn play(&self) {
        println!("Playing {} by {}", self.title, self.artist);
    }
    
    fn pause(&self) {
        println!("Pausing {} by {}", self.title, self.artist);
    }
    
    fn stop(&self) {
        println!("Stopping {} by {}", self.title, self.artist);
    }
    
    fn get_duration(&self) -> u32 {
        self.duration
    }
}

// Struct implementing MediaPlayer trait (uses defaults)
struct Album {
    title: String,
    tracks: u32,
}

impl MediaPlayer for Album {}

// Struct implementing Playable trait
impl Playable for Album {
    fn play(&self) {
        println!("Playing album: {} ({} tracks)", self.title, self.tracks);
    }
    
    fn pause(&self) {
        println!("Pausing album: {}", self.title);
    }
    
    fn stop(&self) {
        println!("Stopping album: {}", self.title);
    }
    
    fn get_duration(&self) -> u32 {
        self.tracks * 240 // Assume average 4 minutes per track
    }
}

// Struct implementing Playable trait
struct Playlist {
    name: String,
    songs: Vec<Song>,
}

impl Playable for Playlist {
    fn play(&self) {
        println!("Playing playlist: {}", self.name);
        for song in &self.songs {
            song.play();
        }
    }
    
    fn pause(&self) {
        println!("Pausing playlist: {}", self.name);
    }
    
    fn stop(&self) {
        println!("Stopping playlist: {}", self.name);
    }
    
    fn get_duration(&self) -> u32 {
        self.songs.iter().map(|song| song.get_duration()).sum()
    }
}

// Function that takes any type implementing Playable
fn play_media(media: &impl Playable) {
    media.play();
}

// Function with trait bounds
fn calculate_total_duration<T: Playable>(items: &[T]) -> u32 {
    items.iter().map(|item| item.get_duration()).sum()
}

// Struct implementing MusicLibrary trait
struct MusicPlayer {
    songs: Vec<String>,
    next_id: u32,
}

impl MusicPlayer {
    fn new() -> Self {
        MusicPlayer {
            songs: Vec::new(),
            next_id: 1,
        }
    }
}

impl MusicLibrary for MusicPlayer {
    type SongId = u32;
    type SongInfo = String;
    
    fn add_song(&mut self, song: String) -> Self::SongId {
        let id = self.next_id;
        self.songs.push(song);
        self.next_id += 1;
        id
    }
    
    fn get_song(&self, id: Self::SongId) -> Option<Self::SongInfo> {
        if id > 0 && id <= self.songs.len() as u32 {
            Some(self.songs[(id - 1) as usize].clone())
        } else {
            None
        }
    }
}

// Struct implementing Event and Performance traits
struct Concert {
    venue: String,
    date: String,
    capacity: u32,
}

impl Event for Concert {
    fn schedule(&self) {
        println!("Concert scheduled at {} on {}", self.venue, self.date);
    }
}

impl Performance for Concert {
    fn perform(&self) {
        println!("Performing concert at {}!", self.venue);
    }
}

// Trait extension (implementing a trait for an existing type)
trait DisplayFormat {
    fn format_for_display(&self) -> String;
}

impl DisplayFormat for &str {
    fn format_for_display(&self) -> String {
        format!("🎵 {} 🎵", self)
    }
}

// Conditional trait implementation
struct RockSong {
    title: String,
    artist: String,
    duration: u32,
    guitar_solos: u32,
}

impl Playable for RockSong {
    fn play(&self) {
        println!("Rocking out to {} by {}!", self.title, self.artist);
    }
    
    fn pause(&self) {
        println!("Pausing rock song: {}", self.title);
    }
    
    fn stop(&self) {
        println!("Stopping rock song: {}", self.title);
    }
    
    fn get_duration(&self) -> u32 {
        self.duration
    }
}

// Additional methods for RockSong
impl RockSong {
    fn rock_out(&self) {
        println!("🤘 {} has {} guitar solos! 🤘", self.title, self.guitar_solos);
    }
}

// Trait with generic parameters
trait MusicProcessor<T> {
    fn process(&self, input: T) -> T;
}

struct EchoProcessor;

impl MusicProcessor<String> for EchoProcessor {
    fn process(&self, input: String) -> String {
        format!("{}...{}...{}", input, input, input)
    }
}

impl MusicProcessor<u32> for EchoProcessor {
    fn process(&self, input: u32) -> u32 {
        input * 2 // Double the duration
    }
} 