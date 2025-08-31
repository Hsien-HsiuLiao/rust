// Lifetimes Examples with Music
// This file demonstrates Rust's lifetime system using music-related examples

fn main() {
    // Example 1: Basic lifetime annotation
    let song_title = "Bohemian Rhapsody";
    let artist_name = "Queen";
    let result = longest_title(song_title, artist_name);
    println!("Longest: {}", result);
    
    // Example 2: Struct with lifetime parameter
    let song = Song {
        title: "Stairway to Heaven",
        artist: "Led Zeppelin",
        year: 1971,
    };
    
    let playlist = Playlist {
        name: "Classic Rock",
        songs: vec![&song],
    };
    
    println!("Playlist: {}", playlist.name);
    println!("First song: {}", playlist.songs[0].title);
    
    // Example 3: Function with multiple lifetime parameters
    let album_title = "A Night at the Opera";
    let band_name = "Queen";
    let release_year = "1975";
    
    let album_info = create_album_info(album_title, band_name, release_year);
    println!("{}", album_info);
    
    // Example 4: Lifetime elision rules
    let song1 = "Bohemian Rhapsody";
    let song2 = "Stairway to Heaven";
    let longest = get_longest_song(song1, song2);
    println!("Longest song title: {}", longest);
    
    // Example 5: 'static lifetime
    let static_song = get_static_song();
    println!("Static song: {}", static_song);
    
    // Example 6: Lifetime bounds
    let songs = vec![
        "Bohemian Rhapsody",
        "Stairway to Heaven",
        "Hotel California",
    ];
    
    let filtered = filter_songs_by_length(&songs, 15);
    println!("Songs with title length > 15: {:?}", filtered);
    
    // Example 7: Multiple lifetimes in complex scenarios
    let concert_info = ConcertInfo {
        venue: "Madison Square Garden",
        headliner: "Queen",
        opening_act: "Led Zeppelin",
    };
    
    let description = concert_info.get_description();
    println!("{}", description);
}

// Function with explicit lifetime annotation
fn longest_title<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime parameter
struct Song<'a> {
    title: &'a str,
    artist: &'a str,
    year: u32,
}

// Struct that contains references to Songs
struct Playlist<'a> {
    name: &'a str,
    songs: Vec<&'a Song<'a>>,
}

// Function with multiple lifetime parameters
fn create_album_info<'a, 'b, 'c>(
    title: &'a str,
    artist: &'b str,
    year: &'c str,
) -> String {
    format!("{} by {} ({})", title, artist, year)
}

// Function demonstrating lifetime elision rules
// This is equivalent to: fn get_longest_song<'a>(x: &'a str, y: &'a str) -> &'a str
fn get_longest_song(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function returning a reference with 'static lifetime
fn get_static_song() -> &'static str {
    "Bohemian Rhapsody - A timeless classic that will never go out of style"
}

// Function with lifetime bounds
fn filter_songs_by_length<'a>(songs: &'a [&str], min_length: usize) -> Vec<&'a str> {
    songs.iter()
        .filter(|&&song| song.len() > min_length)
        .copied()
        .collect()
}

// Struct with multiple lifetime parameters
struct ConcertInfo<'a, 'b> {
    venue: &'a str,
    headliner: &'a str,
    opening_act: &'b str,
}

impl<'a, 'b> ConcertInfo<'a, 'b> {
    // Method with lifetime parameters
    fn get_description(&self) -> String {
        format!(
            "Concert at {} featuring {} with opening act {}",
            self.venue, self.headliner, self.opening_act
        )
    }
    
    // Method that returns a reference with the same lifetime as self
    fn get_venue(&self) -> &'a str {
        self.venue
    }
}

// Function demonstrating lifetime subtyping
fn process_concert<'a, 'b>(concert: &'a ConcertInfo<'a, 'b>) -> &'a str {
    concert.venue
}

// Example of lifetime bounds with traits
trait MusicDatabase {
    fn get_song_info(&self) -> &str;
}

struct LocalDatabase {
    song_info: String,
}

impl MusicDatabase for LocalDatabase {
    fn get_song_info(&self) -> &str {
        &self.song_info
    }
}

// Function with trait bounds and lifetimes
fn print_song_info<'a, T: MusicDatabase>(database: &'a T) {
    println!("Song info: {}", database.get_song_info());
}

// Example of lifetime in closures
fn create_song_processor<'a>() -> impl Fn(&'a str) -> String {
    |song_title| format!("Processing: {}", song_title)
}

// Example of lifetime in iterators
fn process_song_titles<'a>(titles: impl Iterator<Item = &'a str>) -> Vec<String> {
    titles.map(|title| format!("Processed: {}", title)).collect()
} 