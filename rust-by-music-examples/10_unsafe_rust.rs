// Unsafe Rust Examples with Music
// This file demonstrates Rust's unsafe features using music-related examples
// WARNING: This code contains unsafe operations for educational purposes only!

fn main() {
    // Example 1: Raw pointers
    let song_title = "Bohemian Rhapsody";
    let raw_ptr = song_title as *const str;
    
    unsafe {
        // Dereferencing raw pointers is unsafe
        let dereferenced = &*raw_ptr;
        println!("Dereferenced: {}", dereferenced);
    }
    
    // Example 2: Unsafe functions
    let song_data = "Queen,Bohemian Rhapsody,1975,360";
    let parsed_song = unsafe { parse_song_unsafe(song_data) };
    println!("Parsed song: {:?}", parsed_song);
    
    // Example 3: Unsafe traits
    let music_player = UnsafeMusicPlayer::new();
    unsafe {
        music_player.play_unsafe("Bohemian Rhapsody");
    }
    
    // Example 4: Unsafe blocks with FFI simulation
    let result = unsafe { call_external_music_api("Bohemian Rhapsody") };
    println!("External API result: {}", result);
    
    // Example 5: Unsafe code with memory manipulation
    let mut buffer = [0u8; 100];
    unsafe {
        write_song_to_buffer(&mut buffer, "Bohemian Rhapsody");
    }
    println!("Buffer content: {:?}", &buffer[..20]);
    
    // Example 6: Unsafe code with static mut
    unsafe {
        GLOBAL_MUSIC_STATE.current_song = "Bohemian Rhapsody";
        println!("Global state: {}", GLOBAL_MUSIC_STATE.current_song);
    }
    
    // Example 7: Unsafe code with unions
    let music_data = unsafe {
        let mut union = MusicDataUnion { bytes: [0; 8] };
        union.song_id = 12345;
        union
    };
    println!("Song ID: {}", unsafe { music_data.song_id });
    
    // Example 8: Unsafe code with transmute
    let song_bytes = "Queen".as_bytes();
    let song_chars = unsafe { std::mem::transmute::<&[u8], &[char]>(song_bytes) };
    println!("Transmuted chars: {:?}", song_chars);
    
    // Example 9: Unsafe code with manual memory management
    let music_buffer = unsafe { allocate_music_buffer(1024) };
    unsafe {
        write_to_buffer(music_buffer, "Bohemian Rhapsody");
        println!("Buffer content: {}", std::str::from_utf8_unchecked(music_buffer));
        deallocate_music_buffer(music_buffer);
    }
    
    // Example 10: Unsafe code with thread safety
    let shared_data = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let data_clone = shared_data.clone();
    
    std::thread::spawn(move || {
        unsafe {
            let mut data = data_clone.lock().unwrap();
            *data = "Bohemian Rhapsody".to_string();
        }
    });
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("Shared data: {}", shared_data.lock().unwrap());
}

// Unsafe function that parses song data without bounds checking
unsafe fn parse_song_unsafe(data: &str) -> ParsedSong {
    let parts: Vec<&str> = data.split(',').collect();
    
    // This is unsafe because we're not checking array bounds
    let artist = *parts.get_unchecked(0);
    let title = *parts.get_unchecked(1);
    let year = parts.get_unchecked(2).parse().unwrap_or(0);
    let duration = parts.get_unchecked(3).parse().unwrap_or(0);
    
    ParsedSong {
        artist: artist.to_string(),
        title: title.to_string(),
        year,
        duration,
    }
}

#[derive(Debug)]
struct ParsedSong {
    artist: String,
    title: String,
    year: u32,
    duration: u32,
}

// Unsafe trait implementation
trait UnsafePlayable {
    unsafe fn play_unsafe(&self, song: &str);
}

struct UnsafeMusicPlayer;

impl UnsafePlayable for UnsafeMusicPlayer {
    unsafe fn play_unsafe(&self, song: &str) {
        // Simulating unsafe operation
        let song_ptr = song.as_ptr();
        let song_len = song.len();
        
        // This is unsafe because we're working with raw pointers
        let song_slice = std::slice::from_raw_parts(song_ptr, song_len);
        let song_str = std::str::from_utf8_unchecked(song_slice);
        
        println!("Playing (unsafe): {}", song_str);
    }
}

impl UnsafeMusicPlayer {
    fn new() -> Self {
        UnsafeMusicPlayer
    }
}

// Simulating FFI call
unsafe fn call_external_music_api(query: &str) -> String {
    // In real FFI, this would call C functions
    // For demonstration, we'll simulate unsafe operations
    
    let query_bytes = query.as_bytes();
    let query_ptr = query_bytes.as_ptr();
    let query_len = query_bytes.len();
    
    // Simulating unsafe memory access
    let result_slice = std::slice::from_raw_parts(query_ptr, query_len);
    let result_str = std::str::from_utf8_unchecked(result_slice);
    
    format!("External API result: {}", result_str)
}

// Unsafe function that writes to a buffer
unsafe fn write_song_to_buffer(buffer: &mut [u8], song: &str) {
    let song_bytes = song.as_bytes();
    let song_len = song_bytes.len().min(buffer.len());
    
    // Copy bytes without bounds checking (unsafe)
    std::ptr::copy_nonoverlapping(
        song_bytes.as_ptr(),
        buffer.as_mut_ptr(),
        song_len
    );
}

// Global mutable state (unsafe)
static mut GLOBAL_MUSIC_STATE: GlobalMusicState = GlobalMusicState {
    current_song: "No song playing",
    volume: 0.0,
};

struct GlobalMusicState {
    current_song: &'static str,
    volume: f32,
}

// Union for different data types (unsafe)
union MusicDataUnion {
    bytes: [u8; 8],
    song_id: u64,
    duration: f64,
}

// Manual memory management (unsafe)
unsafe fn allocate_music_buffer(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
    std::alloc::alloc(layout)
}

unsafe fn write_to_buffer(buffer: *mut u8, content: &str) {
    let content_bytes = content.as_bytes();
    let content_len = content_bytes.len();
    
    std::ptr::copy_nonoverlapping(
        content_bytes.as_ptr(),
        buffer,
        content_len
    );
}

unsafe fn deallocate_music_buffer(buffer: *mut u8) {
    let layout = std::alloc::Layout::from_size_align(1024, 8).unwrap();
    std::alloc::dealloc(buffer, layout);
}

// Unsafe code with interior mutability
struct UnsafeMusicLibrary {
    songs: *mut Vec<String>,
}

impl UnsafeMusicLibrary {
    fn new() -> Self {
        let songs = Box::into_raw(Box::new(Vec::new()));
        UnsafeMusicLibrary { songs }
    }
    
    unsafe fn add_song(&self, song: String) {
        (*self.songs).push(song);
    }
    
    unsafe fn get_songs(&self) -> &[String] {
        &*self.songs
    }
}

impl Drop for UnsafeMusicLibrary {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.songs);
        }
    }
}

// Unsafe code with raw slices
unsafe fn create_raw_slice(data: &[u8]) -> *const [u8] {
    std::slice::from_raw_parts(data.as_ptr(), data.len())
}

// Unsafe code with function pointers
type MusicCallback = unsafe extern "C" fn(song: *const u8, len: usize);

unsafe extern "C" fn music_callback(song: *const u8, len: usize) {
    let song_slice = std::slice::from_raw_parts(song, len);
    let song_str = std::str::from_utf8_unchecked(song_slice);
    println!("Callback received: {}", song_str);
}

// Unsafe code with atomic operations
use std::sync::atomic::{AtomicPtr, Ordering};

struct AtomicMusicPlayer {
    current_song: AtomicPtr<u8>,
}

impl AtomicMusicPlayer {
    fn new() -> Self {
        AtomicMusicPlayer {
            current_song: AtomicPtr::new(std::ptr::null_mut()),
        }
    }
    
    unsafe fn set_song(&self, song: &str) {
        let song_bytes = song.as_bytes();
        let song_ptr = song_bytes.as_ptr() as *mut u8;
        self.current_song.store(song_ptr, Ordering::SeqCst);
    }
    
    unsafe fn get_song(&self) -> Option<&str> {
        let song_ptr = self.current_song.load(Ordering::SeqCst);
        if song_ptr.is_null() {
            None
        } else {
            // This is unsafe because we don't know the length
            Some(std::str::from_utf8_unchecked(std::slice::from_raw_parts(song_ptr, 20)))
        }
    }
}

// Unsafe code with SIMD operations (simulated)
unsafe fn process_audio_samples(samples: &[f32]) -> Vec<f32> {
    let mut result = Vec::with_capacity(samples.len());
    
    // Simulating unsafe SIMD operations
    for chunk in samples.chunks(4) {
        let mut processed = [0.0f32; 4];
        
        // In real SIMD, this would use intrinsics
        for (i, &sample) in chunk.iter().enumerate() {
            processed[i] = sample * 1.5; // Amplify
        }
        
        result.extend_from_slice(&processed);
    }
    
    result
}

// Unsafe code with custom allocator
struct MusicAllocator;

unsafe impl std::alloc::GlobalAlloc for MusicAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        // Delegate to the default allocator
        std::alloc::System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        std::alloc::System.dealloc(ptr, layout);
    }
}

// Unsafe code with thread-local storage
use std::cell::UnsafeCell;

struct ThreadLocalMusicState {
    data: UnsafeCell<String>,
}

impl ThreadLocalMusicState {
    fn new() -> Self {
        ThreadLocalMusicState {
            data: UnsafeCell::new(String::new()),
        }
    }
    
    unsafe fn set_song(&self, song: String) {
        *self.data.get() = song;
    }
    
    unsafe fn get_song(&self) -> &str {
        &*self.data.get()
    }
}

// Unsafe code with raw memory operations
unsafe fn copy_music_data(src: *const u8, dst: *mut u8, len: usize) {
    std::ptr::copy_nonoverlapping(src, dst, len);
}

unsafe fn zero_music_buffer(buffer: *mut u8, len: usize) {
    std::ptr::write_bytes(buffer, 0, len);
}

// Unsafe code with volatile operations
use std::ptr;

unsafe fn volatile_write_music_register(register: *mut u32, value: u32) {
    ptr::write_volatile(register, value);
}

unsafe fn volatile_read_music_register(register: *const u32) -> u32 {
    ptr::read_volatile(register)
} 