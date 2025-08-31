// Generic Types Examples with Music
// This file demonstrates Rust's generic type system using music-related examples

fn main() {
    // Example 1: Generic function with single type parameter
    let song_titles = vec!["Bohemian Rhapsody", "Stairway to Heaven", "Hotel California"];
    let first_title = get_first_element(&song_titles);
    println!("First title: {}", first_title);
    
    let song_durations = vec![360, 482, 391];
    let first_duration = get_first_element(&song_durations);
    println!("First duration: {} seconds", first_duration);
    
    // Example 2: Generic struct with single type parameter
    let string_playlist = Playlist::new("Classic Rock");
    let int_playlist = Playlist::new(42);
    
    println!("String playlist: {}", string_playlist.name);
    println!("Int playlist: {}", int_playlist.name);
    
    // Example 3: Generic struct with multiple type parameters
    let song = Song::new("Bohemian Rhapsody", "Queen", 1975);
    let album = Album::new("A Night at the Opera", 12);
    
    println!("Song: {}", song.title);
    println!("Album: {}", album.title);
    
    // Example 4: Generic struct with constraints
    let music_box = MusicBox::new("Rock Music");
    music_box.play();
    
    let number_box = NumberBox::new(42);
    number_box.display();
    
    // Example 5: Generic enum
    let success_result: MusicResult<String, String> = MusicResult::Success("Bohemian Rhapsody".to_string());
    let error_result: MusicResult<String, String> = MusicResult::Error("Song not found".to_string());
    
    match success_result {
        MusicResult::Success(song) => println!("Success: {}", song),
        MusicResult::Error(err) => println!("Error: {}", err),
    }
    
    // Example 6: Generic trait implementation
    let song_processor = SongProcessor;
    let processed_title = song_processor.process("Bohemian Rhapsody".to_string());
    let processed_duration = song_processor.process(360);
    
    println!("Processed title: {}", processed_title);
    println!("Processed duration: {}", processed_duration);
    
    // Example 7: Generic methods with different constraints
    let music_library = MusicLibrary::new();
    music_library.add_item("Bohemian Rhapsody".to_string());
    music_library.add_item(360);
    
    // Example 8: Generic function with multiple constraints
    let songs = vec![
        Song::new("Bohemian Rhapsody", "Queen", 1975),
        Song::new("Stairway to Heaven", "Led Zeppelin", 1971),
    ];
    
    let total_duration = calculate_total_duration(&songs);
    println!("Total duration: {} seconds", total_duration);
    
    // Example 9: Generic associated types
    let string_storage = StringStorage::new();
    let id1 = string_storage.store("Bohemian Rhapsody".to_string());
    let stored_song = string_storage.retrieve(id1);
    println!("Stored song: {:?}", stored_song);
    
    // Example 10: Generic function with where clause
    let playlist = create_playlist("Classic Rock", vec!["Bohemian Rhapsody", "Stairway to Heaven"]);
    println!("Playlist: {}", playlist.name);
    
    // Example 11: Phantom data for type safety
    let rock_playlist: TypedPlaylist<Rock> = TypedPlaylist::new("Rock Music");
    let jazz_playlist: TypedPlaylist<Jazz> = TypedPlaylist::new("Jazz Music");
    
    rock_playlist.play();
    jazz_playlist.play();
}

// Generic function with single type parameter
fn get_first_element<T>(list: &[T]) -> &T {
    &list[0]
}

// Generic struct with single type parameter
struct Playlist<T> {
    name: T,
    songs: Vec<String>,
}

impl<T> Playlist<T> {
    fn new(name: T) -> Self {
        Playlist {
            name,
            songs: Vec::new(),
        }
    }
}

// Generic struct with multiple type parameters
struct Song<T, U> {
    title: T,
    artist: U,
    year: u32,
}

impl<T, U> Song<T, U> {
    fn new(title: T, artist: U, year: u32) -> Self {
        Song { title, artist, year }
    }
}

struct Album<T, U> {
    title: T,
    tracks: U,
}

impl<T, U> Album<T, U> {
    fn new(title: T, tracks: U) -> Self {
        Album { title, tracks }
    }
}

// Generic struct with trait bounds
struct MusicBox<T: Playable> {
    content: T,
}

impl<T: Playable> MusicBox<T> {
    fn new(content: T) -> Self {
        MusicBox { content }
    }
    
    fn play(&self) {
        self.content.play();
    }
}

struct NumberBox<T: Display> {
    content: T,
}

impl<T: Display> NumberBox<T> {
    fn new(content: T) -> Self {
        NumberBox { content }
    }
    
    fn display(&self) {
        println!("Content: {}", self.content);
    }
}

// Generic enum
enum MusicResult<T, E> {
    Success(T),
    Error(E),
}

// Generic trait
trait Processor<T> {
    fn process(&self, input: T) -> T;
}

// Generic struct implementing generic trait
struct SongProcessor;

impl Processor<String> for SongProcessor {
    fn process(&self, input: String) -> String {
        format!("Processed: {}", input)
    }
}

impl Processor<u32> for SongProcessor {
    fn process(&self, input: u32) -> u32 {
        input * 2
    }
}

// Generic struct with methods that have different constraints
struct MusicLibrary {
    items: Vec<Box<dyn std::any::Any>>,
}

impl MusicLibrary {
    fn new() -> Self {
        MusicLibrary { items: Vec::new() }
    }
    
    fn add_item<T: 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }
}

// Generic function with multiple constraints
fn calculate_total_duration<T>(items: &[T]) -> u32
where
    T: HasDuration,
{
    items.iter().map(|item| item.get_duration()).sum()
}

// Generic trait with associated types
trait Storage {
    type Item;
    type Id;
    
    fn store(&mut self, item: Self::Item) -> Self::Id;
    fn retrieve(&self, id: Self::Id) -> Option<&Self::Item>;
}

struct StringStorage {
    items: Vec<String>,
    next_id: u32,
}

impl StringStorage {
    fn new() -> Self {
        StringStorage {
            items: Vec::new(),
            next_id: 1,
        }
    }
}

impl Storage for StringStorage {
    type Item = String;
    type Id = u32;
    
    fn store(&mut self, item: Self::Item) -> Self::Id {
        let id = self.next_id;
        self.items.push(item);
        self.next_id += 1;
        id
    }
    
    fn retrieve(&self, id: Self::Id) -> Option<&Self::Item> {
        if id > 0 && id <= self.items.len() as u32 {
            Some(&self.items[(id - 1) as usize])
        } else {
            None
        }
    }
}

// Generic function with where clause
fn create_playlist<T>(name: T, songs: Vec<&str>) -> Playlist<T>
where
    T: Clone,
{
    let mut playlist = Playlist::new(name);
    for song in songs {
        playlist.songs.push(song.to_string());
    }
    playlist
}

// Phantom data for type safety
use std::marker::PhantomData;

struct TypedPlaylist<Genre> {
    name: String,
    _phantom: PhantomData<Genre>,
}

impl<Genre> TypedPlaylist<Genre> {
    fn new(name: &str) -> Self {
        TypedPlaylist {
            name: name.to_string(),
            _phantom: PhantomData,
        }
    }
    
    fn play(&self) {
        println!("Playing {} playlist", self.name);
    }
}

// Marker types for different genres
struct Rock;
struct Jazz;
struct Classical;

// Generic trait for items that have duration
trait HasDuration {
    fn get_duration(&self) -> u32;
}

// Generic trait for items that can be played
trait Playable {
    fn play(&self);
}

// Generic trait for items that can be displayed
trait Display {
    fn fmt(&self) -> String;
}

impl Display for u32 {
    fn fmt(&self) -> String {
        self.to_string()
    }
}

impl Display for String {
    fn fmt(&self) -> String {
        self.clone()
    }
}

// Generic function with complex constraints
fn process_music_items<T, U>(items: &[T], processor: &U) -> Vec<T>
where
    T: Clone + HasDuration,
    U: Processor<T>,
{
    items.iter().map(|item| processor.process(item.clone())).collect()
}

// Generic struct with lifetime parameters
struct MusicReference<'a, T> {
    reference: &'a T,
    _phantom: PhantomData<T>,
}

impl<'a, T> MusicReference<'a, T> {
    fn new(reference: &'a T) -> Self {
        MusicReference {
            reference,
            _phantom: PhantomData,
        }
    }
    
    fn get_reference(&self) -> &'a T {
        self.reference
    }
} 