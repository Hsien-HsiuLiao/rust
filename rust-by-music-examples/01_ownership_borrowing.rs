// Ownership & Borrowing Examples with Music
// This file demonstrates Rust's ownership system using music-related examples

fn main() {
    // Example 1: Basic Ownership Transfer
    let song_title = String::from("Bohemian Rhapsody");
    let moved_title = song_title; // song_title is moved to moved_title
    // println!("{}", song_title); // This would cause a compile error!
    println!("Song: {}", moved_title);

    // Example 2: Borrowing (Referencing)
    let album_name = String::from("A Night at the Opera");
    print_album_name(&album_name); // We borrow album_name
    println!("Original album: {}", album_name); // Still accessible

    // Example 3: Mutable Borrowing
    let mut playlist = vec![
        "Stairway to Heaven".to_string(),
        "Hotel California".to_string(),
    ];
    add_song(&mut playlist, "Sweet Child O' Mine".to_string());
    println!("Updated playlist: {:?}", playlist);

    // Example 4: Multiple Immutable Borrows
    let artist = String::from("Queen");
    let reference1 = &artist;
    let reference2 = &artist;
    println!("Artist: {}", reference1);
    println!("Artist: {}", reference2);

    // Example 5: Ownership in Functions
    let original_song = create_song("Imagine", "John Lennon");
    let song_info = get_song_info(original_song); // original_song is consumed
    println!("{}", song_info);
}

// Function that borrows (doesn't take ownership)
fn print_album_name(album: &String) {
    println!("Album: {}", album);
}

// Function that mutably borrows
fn add_song(playlist: &mut Vec<String>, song: String) {
    playlist.push(song);
}

// Function that takes ownership and returns ownership
fn create_song(title: &str, artist: &str) -> String {
    format!("{} by {}", title, artist)
}

// Function that consumes the String and returns a new one
fn get_song_info(song: String) -> String {
    format!("Song information: {}", song)
} 