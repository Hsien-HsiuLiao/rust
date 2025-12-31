// Ownership and Borrowing Tests for TDD Practice
// Run with: cargo test ownership_borrowing

#[cfg(test)]
mod ownership_borrowing_tests {
    use std::collections::HashMap;

    // Test 1: Basic Ownership Transfer
    #[test]
    fn test_ownership_transfer() {
        // TODO: Implement a function that transfers ownership of a song title
        // The function should take a String and return ownership to the caller
        
        let song_title = String::from("Bohemian Rhapsody");
        let transferred_title = transfer_ownership(song_title);
        
        assert_eq!(transferred_title, "Bohemian Rhapsody");
        // song_title should no longer be usable here
    }

    // Test 2: Borrowing (Immutable Reference)
    #[test]
    fn test_immutable_borrowing() {
        // TODO: Implement a function that borrows a song title without taking ownership
        
        let song_title = String::from("Stairway to Heaven");
        let length = get_song_length(&song_title);
        
        assert_eq!(length, 17);
        assert_eq!(song_title, "Stairway to Heaven"); // Should still be usable
    }

    // Test 3: Mutable Borrowing
    #[test]
    fn test_mutable_borrowing() {
        // TODO: Implement a function that mutably borrows a playlist
        
        let mut playlist = vec![
            "Hotel California".to_string(),
            "Sweet Child O' Mine".to_string(),
        ];
        
        add_song_to_playlist(&mut playlist, "November Rain".to_string());
        
        assert_eq!(playlist.len(), 3);
        assert_eq!(playlist[2], "November Rain");
    }

    // Test 4: Multiple Immutable Borrows
    #[test]
    fn test_multiple_immutable_borrows() {
        // TODO: Implement a function that takes multiple immutable references
        
        let song1 = String::from("Bohemian Rhapsody");
        let song2 = String::from("Stairway to Heaven");
        
        let result = compare_song_lengths(&song1, &song2);
        
        assert_eq!(result, "Stairway to Heaven is longer");
        // Both songs should still be usable
        assert_eq!(song1, "Bohemian Rhapsody");
        assert_eq!(song2, "Stairway to Heaven");
    }

    // Test 5: Ownership in Structs
    #[test]
    fn test_ownership_in_structs() {
        // TODO: Implement a Song struct with owned data
        
        let song = Song::new("Hotel California", "Eagles", 1976);
        
        assert_eq!(song.title, "Hotel California");
        assert_eq!(song.artist, "Eagles");
        assert_eq!(song.year, 1976);
    }

    // Test 6: Borrowing in Structs
    #[test]
    fn test_borrowing_in_structs() {
        // TODO: Implement a function that borrows a Song struct
        
        let song = Song::new("Take Five", "Dave Brubeck", 1959);
        let description = get_song_description(&song);
        
        assert_eq!(description, "Take Five by Dave Brubeck (1959)");
        // song should still be usable
        assert_eq!(song.title, "Take Five");
    }

    // Test 7: Moving Out of Structs
    #[test]
    fn test_moving_out_of_structs() {
        // TODO: Implement a function that moves data out of a struct
        
        let song = Song::new("Moonlight Sonata", "Beethoven", 1801);
        let title = extract_title(song);
        
        assert_eq!(title, "Moonlight Sonata");
        // song should no longer be usable here
    }

    // Test 8: Borrowing with Collections
    #[test]
    fn test_borrowing_with_collections() {
        // TODO: Implement functions that work with borrowed collections
        
        let songs = vec![
            Song::new("Bohemian Rhapsody", "Queen", 1975),
            Song::new("Stairway to Heaven", "Led Zeppelin", 1971),
            Song::new("Hotel California", "Eagles", 1976),
        ];
        
        let total_songs = count_songs(&songs);
        let rock_songs = filter_rock_songs(&songs);
        
        assert_eq!(total_songs, 3);
        assert_eq!(rock_songs.len(), 3);
        // songs should still be usable
        assert_eq!(songs.len(), 3);
    }

    // Test 9: Lifetime Annotations
    #[test]
    fn test_lifetime_annotations() {
        // TODO: Implement a function with explicit lifetime annotations
        
        let song_title = "Bohemian Rhapsody";
        let artist_name = "Queen";
        let result = longest_string(song_title, artist_name);
        
        assert_eq!(result, "Bohemian Rhapsody");
    }

    // Test 10: Complex Ownership Scenarios
    #[test]
    fn test_complex_ownership_scenarios() {
        // TODO: Implement a music library with complex ownership
        
        let mut library = MusicLibrary::new();
        
        let song1 = Song::new("Bohemian Rhapsody", "Queen", 1975);
        let song2 = Song::new("Stairway to Heaven", "Led Zeppelin", 1971);
        
        library.add_song(song1);
        library.add_song(song2);
        
        assert_eq!(library.song_count(), 2);
        
        let found_song = library.find_song("Bohemian Rhapsody");
        assert!(found_song.is_some());
        assert_eq!(found_song.unwrap().artist, "Queen");
    }

    // Helper structs and functions to implement
    // TODO: Implement these structs and functions to make the tests pass

    #[derive(Debug, PartialEq)]
    struct Song {
        title: String,
        artist: String,
        year: u32,
    }

    impl Song {
        fn new(title: &str, artist: &str, year: u32) -> Self {
            // TODO: Implement this constructor
            todo!()
        }
    }

    struct MusicLibrary {
        songs: Vec<Song>,
    }

    impl MusicLibrary {
        fn new() -> Self {
            // TODO: Implement this constructor
            todo!()
        }

        fn add_song(&mut self, song: Song) {
            // TODO: Implement this method
            todo!()
        }

        fn song_count(&self) -> usize {
            // TODO: Implement this method
            todo!()
        }

        fn find_song(&self, title: &str) -> Option<&Song> {
            // TODO: Implement this method
            todo!()
        }
    }

    // TODO: Implement these functions
    fn transfer_ownership(song: String) -> String {
        todo!()
    }

    fn get_song_length(song: &String) -> usize {
        todo!()
    }

    fn add_song_to_playlist(playlist: &mut Vec<String>, song: String) {
        todo!()
    }

    fn compare_song_lengths(song1: &String, song2: &String) -> &str {
        todo!()
    }

    fn get_song_description(song: &Song) -> String {
        todo!()
    }

    fn extract_title(song: Song) -> String {
        todo!()
    }

    fn count_songs(songs: &[Song]) -> usize {
        todo!()
    }

    fn filter_rock_songs(songs: &[Song]) -> Vec<&Song> {
        todo!()
    }

    fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        todo!()
    }
}
