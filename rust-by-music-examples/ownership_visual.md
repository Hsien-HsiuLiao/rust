# Ownership Transfer Visual Example

## Example 1: Basic Ownership Transfer

```rust
let song_title = String::from("Bohemian Rhapsody");
let moved_title = song_title; // song_title is moved to moved_title
// println!("{}", song_title); // This would cause a compile error!
println!("Song: {}", moved_title);
```

## Memory Visualization

### Step 1: Initial Assignment
```
Stack                    Heap
┌─────────────────┐     ┌─────────────────────────────┐
│ song_title      │────▶│ "Bohemian Rhapsody"         │
│ (pointer)       │     │ (length: 19, capacity: 19)  │
│                 │     │                             │
└─────────────────┘     └─────────────────────────────┘
```

**What happens:**
- `song_title` is a variable on the stack
- It contains a pointer to the string data on the heap
- The heap stores the actual string "Bohemian Rhapsody" with metadata

### Step 2: Ownership Transfer (Move)
```
Stack                    Heap
┌─────────────────┐     ┌─────────────────────────────┐
│ song_title      │     │ "Bohemian Rhapsody"         │
│ (INVALID)       │     │ (length: 19, capacity: 19)  │
│                 │     │                             │
└─────────────────┘     └─────────────────────────────┘
         │
         ▼
┌─────────────────┐
│ moved_title     │────▶│ (same heap location)
│ (pointer)       │
│                 │
└─────────────────┘
```

**What happens:**
- The pointer value is copied from `song_title` to `moved_title`
- `song_title` becomes invalid (cannot be used)
- `moved_title` now owns the string data
- Only one variable can own the data at a time

### Step 3: After Move (Compile Error)
```
Stack                    Heap
┌─────────────────┐     ┌─────────────────────────────┐
│ song_title      │     │ "Bohemian Rhapsody"         │
│ (INVALID)       │     │ (length: 19, capacity: 19)  │
│                 │     │                             │
└─────────────────┘     └─────────────────────────────┘
         │
         ▼
┌─────────────────┐
│ moved_title     │────▶│ (same heap location)
│ (VALID)         │
│                 │
└─────────────────┘
```

**Compile Error:**
```rust
// This line would cause a compile error:
println!("{}", song_title);
//        ^^^^^^^^^^^ value moved here
```

## ⚠️ IMPORTANT: What Happens to `song_title`?

**`song_title` is NOT deleted!** Here's what actually happens:

### What Stays:
- ✅ **Variable exists**: `song_title` still exists on the stack
- ✅ **Memory location**: The stack space is still allocated
- ✅ **Variable name**: You can still reference the variable name

### What Changes:
- ❌ **Pointer becomes invalid**: The pointer value is no longer valid
- ❌ **Cannot access data**: Rust prevents you from using the variable
- ❌ **Compile-time error**: Any attempt to use it fails compilation

### Visual Representation of What Remains:
```
Stack                    Heap
┌─────────────────┐     ┌─────────────────────────────┐
│ song_title      │     │ "Bohemian Rhapsody"         │
│ (INVALID ptr)   │     │ (length: 19, capacity: 19)  │
│ [0x00000000]    │     │                             │
│                 │     │                             │
└─────────────────┘     └─────────────────────────────┘
         │
         ▼
┌─────────────────┐
│ moved_title     │────▶│ (same heap location)
│ (VALID ptr)     │
│ [0x12345678]    │
└─────────────────┘
```

**Key Point**: `song_title` still exists but contains an invalid pointer (often `0x00000000` or similar "null" value).

## Detailed Explanation

### Before Move
- `song_title` owns the String data
- `song_title` is a valid, usable variable
- The String data lives on the heap

### During Move
- Rust copies the pointer value (not the data)
- `moved_title` becomes the new owner
- `song_title` becomes invalid
- No data is actually copied or moved in memory

### After Move
- Only `moved_title` can access the String data
- `song_title` cannot be used (compile-time error)
- The heap data remains in the same location

## Why This Happens

1. **Memory Safety**: Prevents multiple variables from accessing the same heap data
2. **No Double Free**: Only one owner means only one cleanup
3. **Compile-time Guarantees**: Rust catches ownership violations at compile time

## Alternative Approaches

### Option 1: Clone (Deep Copy)
```rust
let song_title = String::from("Bohemian Rhapsody");
let cloned_title = song_title.clone(); // Creates a new copy
println!("Original: {}", song_title);  // Still valid
println!("Clone: {}", cloned_title);   // New copy
```

### Option 2: Reference (Borrowing)
```rust
let song_title = String::from("Bohemian Rhapsody");
let borrowed_title = &song_title;      // Borrows, doesn't own
println!("Original: {}", song_title);  // Still valid
println!("Borrowed: {}", borrowed_title); // Reference
```

## Visual Summary

```
┌─────────────────────────────────────────────────────────────┐
│                    OWNERSHIP TRANSFER                      │
├─────────────────────────────────────────────────────────────┤
│  Before: song_title owns "Bohemian Rhapsody"              │
│  During: song_title → moved_title (ownership moves)       │
│  After:  moved_title owns "Bohemian Rhapsody"             │
│          song_title is INVALID (but not deleted!)          │
└─────────────────────────────────────────────────────────────┘

Key Points:
• Only ONE owner at a time
• Move = transfer ownership
• Original variable becomes invalid (not deleted)
• No data copying, just pointer transfer
• Variable still exists on stack, just unusable
```

## Common Ownership Patterns

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   OWNERSHIP     │     │    BORROWING    │     │     CLONING     │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ song → moved    │     │ song → &song    │     │ song → song     │
│ (transfer)      │     │ (reference)     │     │ (copy)          │
│                 │     │                 │     │                 │
│ moved valid     │     │ both valid      │     │ both valid      │
│ song invalid    │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Memory Cleanup Timeline

```
┌─────────────────────────────────────────────────────────────┐
│                    MEMORY LIFECYCLE                        │
├─────────────────────────────────────────────────────────────┤
│ 1. song_title created → owns heap data                     │
│ 2. Ownership transferred → song_title becomes invalid      │
│ 3. moved_title now owns heap data                          │
│ 4. When moved_title goes out of scope → heap data freed    │
│ 5. When song_title goes out of scope → stack space freed   │
└─────────────────────────────────────────────────────────────┘

Note: song_title's stack space is freed when it goes out of scope,
not when ownership is transferred!
```

This visual representation shows how Rust's ownership system works at the memory level, making it clear why the original variable becomes invalid after a move operation. 