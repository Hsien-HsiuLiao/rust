// Smart Pointers Examples with Music
// This file demonstrates Rust's smart pointer types using music-related examples

use std::rc::Rc;
use std::sync::Arc;
use std::cell::{RefCell, Cell};
use std::sync::Mutex;
use std::collections::HashMap;

fn main() {
    // Example 1: Box<T> - Heap allocation
    let song = Box::new(Song {
        title: "Bohemian Rhapsody".to_string(),
        artist: "Queen".to_string(),
        duration: 360,
    });
    
    println!("Boxed song: {}", song.title);
    
    // Example 2: Rc<T> - Reference counting for single-threaded scenarios
    let playlist = Rc::new(Playlist {
        name: "Classic Rock".to_string(),
        songs: vec![
            "Bohemian Rhapsody".to_string(),
            "Stairway to Heaven".to_string(),
            "Hotel California".to_string(),
        ],
    });
    
    // Clone the Rc (increases reference count)
    let playlist_clone1 = Rc::clone(&playlist);
    let playlist_clone2 = Rc::clone(&playlist);
    
    println!("Playlist: {}", playlist.name);
    println!("Reference count: {}", Rc::strong_count(&playlist));
    
    // Example 3: Arc<T> - Atomic reference counting for multi-threaded scenarios
    let shared_playlist = Arc::new(Playlist {
        name: "Shared Rock".to_string(),
        songs: vec![
            "Sweet Child O' Mine".to_string(),
            "November Rain".to_string(),
        ],
    });
    
    let shared_clone1 = Arc::clone(&shared_playlist);
    let shared_clone2 = Arc::clone(&shared_playlist);
    
    // Spawn threads that can access the shared data
    let handle1 = std::thread::spawn(move || {
        println!("Thread 1: {}", shared_clone1.name);
        println!("Songs: {:?}", shared_clone1.songs);
    });
    
    let handle2 = std::thread::spawn(move || {
        println!("Thread 2: {}", shared_clone2.name);
        println!("Songs: {:?}", shared_clone2.songs);
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("Main thread: {}", shared_playlist.name);
    
    // Example 4: RefCell<T> - Interior mutability for single-threaded scenarios
    let music_player = RefCell::new(MusicPlayer {
        current_song: "No song playing".to_string(),
        volume: 50.0,
        playlist: vec![],
    });
    
    // Borrow mutably through RefCell
    {
        let mut player = music_player.borrow_mut();
        player.current_song = "Bohemian Rhapsody".to_string();
        player.volume = 75.0;
        player.playlist.push("Stairway to Heaven".to_string());
    }
    
    // Borrow immutably
    let player = music_player.borrow();
    println!("Current song: {}", player.current_song);
    println!("Volume: {}", player.volume);
    
    // Example 5: Cell<T> - Interior mutability for Copy types
    let volume_control = Cell::new(50.0);
    volume_control.set(75.0);
    println!("Volume: {}", volume_control.get());
    
    // Example 6: Mutex<T> - Interior mutability for multi-threaded scenarios
    let shared_music_state = Arc::new(Mutex::new(MusicState {
        current_song: "No song".to_string(),
        is_playing: false,
        volume: 50.0,
    }));
    
    let state_clone1 = Arc::clone(&shared_music_state);
    let state_clone2 = Arc::clone(&shared_music_state);
    
    let handle1 = std::thread::spawn(move || {
        let mut state = state_clone1.lock().unwrap();
        state.current_song = "Bohemian Rhapsody".to_string();
        state.is_playing = true;
        state.volume = 80.0;
        println!("Thread 1 updated state");
    });
    
    let handle2 = std::thread::spawn(move || {
        let state = state_clone2.lock().unwrap();
        println!("Thread 2 read: {} is playing: {}", 
                state.current_song, state.is_playing);
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    // Example 7: Combining smart pointers
    let music_library = Rc::new(RefCell::new(MusicLibrary {
        songs: HashMap::new(),
        playlists: HashMap::new(),
    }));
    
    // Add songs to the library
    {
        let mut library = music_library.borrow_mut();
        library.songs.insert(
            "Bohemian Rhapsody".to_string(),
            SongInfo {
                artist: "Queen".to_string(),
                year: 1975,
                duration: 360,
            },
        );
        
        library.playlists.insert(
            "Classic Rock".to_string(),
            vec!["Bohemian Rhapsody".to_string(), "Stairway to Heaven".to_string()],
        );
    }
    
    // Read from the library
    let library = music_library.borrow();
    if let Some(song_info) = library.songs.get("Bohemian Rhapsody") {
        println!("Song: Bohemian Rhapsody by {} ({})", song_info.artist, song_info.year);
    }
    
    // Example 8: Custom smart pointer
    let custom_song = CustomBox::new(Song {
        title: "Custom Song".to_string(),
        artist: "Custom Artist".to_string(),
        duration: 300,
    });
    
    println!("Custom song: {}", custom_song.title);
    
    // Example 9: Weak references to prevent reference cycles
    let album = Rc::new(Album {
        title: "Greatest Hits".to_string(),
        songs: RefCell::new(Vec::new()),
    });
    
    let song1 = Rc::new(Song {
        title: "Song 1".to_string(),
        artist: "Artist 1".to_string(),
        duration: 240,
    });
    
    let song2 = Rc::new(Song {
        title: "Song 2".to_string(),
        artist: "Artist 2".to_string(),
        duration: 300,
    });
    
    // Add songs to album
    album.songs.borrow_mut().push(Rc::downgrade(&song1));
    album.songs.borrow_mut().push(Rc::downgrade(&song2));
    
    // Access songs through weak references
    for song_ref in album.songs.borrow().iter() {
        if let Some(song) = song_ref.upgrade() {
            println!("Album song: {}", song.title);
        }
    }
    
    // Example 10: Smart pointers with trait objects
    let playable_items: Vec<Box<dyn Playable>> = vec![
        Box::new(Song {
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            duration: 360,
        }),
        Box::new(Album {
            title: "A Night at the Opera".to_string(),
            songs: RefCell::new(vec![]),
        }),
    ];
    
    for item in &playable_items {
        item.play();
    }
}

// Basic structs for examples
struct Song {
    title: String,
    artist: String,
    duration: u32,
}

struct Playlist {
    name: String,
    songs: Vec<String>,
}

struct MusicPlayer {
    current_song: String,
    volume: f32,
    playlist: Vec<String>,
}

struct MusicState {
    current_song: String,
    is_playing: bool,
    volume: f32,
}

struct SongInfo {
    artist: String,
    year: u32,
    duration: u32,
}

struct MusicLibrary {
    songs: HashMap<String, SongInfo>,
    playlists: HashMap<String, Vec<String>>,
}

struct Album {
    title: String,
    songs: RefCell<Vec<Rc<Song>>>,
}

// Custom smart pointer implementation
struct CustomBox<T> {
    ptr: *mut T,
}

impl<T> CustomBox<T> {
    fn new(value: T) -> Self {
        let layout = std::alloc::Layout::new::<T>();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        
        unsafe {
            std::ptr::write(ptr, value);
        }
        
        CustomBox { ptr }
    }
}

impl<T> std::ops::Deref for CustomBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> std::ops::DerefMut for CustomBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.ptr);
            let layout = std::alloc::Layout::new::<T>();
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

// Trait for playable items
trait Playable {
    fn play(&self);
}

impl Playable for Song {
    fn play(&self) {
        println!("Playing {} by {}", self.title, self.artist);
    }
}

impl Playable for Album {
    fn play(&self) {
        println!("Playing album: {}", self.title);
        for song_ref in self.songs.borrow().iter() {
            if let Some(song) = song_ref.upgrade() {
                song.play();
            }
        }
    }
}

// Example with Arc<Mutex<T>> for shared mutable state
struct SharedMusicQueue {
    queue: Arc<Mutex<Vec<String>>>,
}

impl SharedMusicQueue {
    fn new() -> Self {
        SharedMusicQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn add_song(&self, song: String) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(song);
    }
    
    fn get_next_song(&self) -> Option<String> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop()
    }
    
    fn get_queue_length(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
}

// Example with Rc<RefCell<T>> for single-threaded shared mutable state
struct LocalMusicQueue {
    queue: Rc<RefCell<Vec<String>>>,
}

impl LocalMusicQueue {
    fn new() -> Self {
        LocalMusicQueue {
            queue: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    fn add_song(&self, song: String) {
        let mut queue = self.queue.borrow_mut();
        queue.push(song);
    }
    
    fn get_next_song(&self) -> Option<String> {
        let mut queue = self.queue.borrow_mut();
        queue.pop()
    }
    
    fn get_queue_length(&self) -> usize {
        let queue = self.queue.borrow();
        queue.len()
    }
}

// Example demonstrating the difference between Rc and Arc
fn demonstrate_rc_vs_arc() {
    // Rc - Single-threaded reference counting
    let rc_playlist = Rc::new(vec!["Song 1".to_string(), "Song 2".to_string()]);
    let rc_clone1 = Rc::clone(&rc_playlist);
    let rc_clone2 = Rc::clone(&rc_playlist);
    
    println!("Rc reference count: {}", Rc::strong_count(&rc_playlist));
    
    // Arc - Multi-threaded reference counting
    let arc_playlist = Arc::new(vec!["Song 1".to_string(), "Song 2".to_string()]);
    let arc_clone1 = Arc::clone(&arc_playlist);
    let arc_clone2 = Arc::clone(&arc_playlist);
    
    println!("Arc reference count: {}", Arc::strong_count(&arc_playlist));
    
    // This would work with Arc but not with Rc
    let handle = std::thread::spawn(move || {
        println!("Thread playing: {:?}", arc_clone1);
    });
    
    handle.join().unwrap();
}

// Example with Cell for simple interior mutability
struct VolumeControl {
    volume: Cell<f32>,
    max_volume: Cell<f32>,
}

impl VolumeControl {
    fn new(initial_volume: f32, max_volume: f32) -> Self {
        VolumeControl {
            volume: Cell::new(initial_volume),
            max_volume: Cell::new(max_volume),
        }
    }
    
    fn set_volume(&self, new_volume: f32) {
        let max = self.max_volume.get();
        let clamped_volume = new_volume.max(0.0).min(max);
        self.volume.set(clamped_volume);
    }
    
    fn get_volume(&self) -> f32 {
        self.volume.get()
    }
    
    fn increase_volume(&self, amount: f32) {
        let current = self.volume.get();
        self.set_volume(current + amount);
    }
    
    fn decrease_volume(&self, amount: f32) {
        let current = self.volume.get();
        self.set_volume(current - amount);
    }
} 