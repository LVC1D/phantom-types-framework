use std::marker::PhantomData;
use std::fs::File;
use std::io::Read;

// 1. State markers
pub struct Open;
pub struct Closed;

// 2. Define FileHandle
pub struct FileHandle<S> {
    path: String,
    _state: PhantomData<S>,
}

// 3. Methods available on ANY state
impl<S> FileHandle<S> {
    pub fn path(&self) -> &str {
        // Your implementation
        &self.path
    }
}

// 4. Methods ONLY on Closed
impl FileHandle<Closed> {
    pub fn new(path: impl Into<String>) -> Self {
        // Your implementation
        Self {
            path: path.into(),
            _state: PhantomData,
        }
    }
    
    pub fn open(self) -> Result<FileHandle<Open>, std::io::Error> {
        // Your implementation - just simulate for now
        // Hint: You need to consume self and return FileHandle<Open>
        Ok(
            FileHandle {
                path: self.path,
                _state: PhantomData,
            }
        )
    }
}

// 5. Methods ONLY on Open
impl FileHandle<Open> {
    pub fn read(&self) -> Result<String, std::io::Error> {
        // Your implementation - simulate reading
        let mut f = File::open(self.path())?;
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        Ok(data)
    }
    
    pub fn close(self) -> FileHandle<Closed> {
        // Your implementation
        FileHandle {
            path: self.path,
            _state: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_state_transitions() {
        // Create a test file
        fs::write("test.txt", "Hello, phantom types!").unwrap();
        
        // Start closed
        let file = FileHandle::<Closed>::new("test.txt");
        
        // Can access path in any state
        assert_eq!(file.path(), "test.txt");
        
        // Open it (transition: Closed -> Open)
        let file = file.open().unwrap();
        
        // Can read when open
        let contents = file.read().unwrap();
        assert_eq!(contents, "Hello, phantom types!");
        
        // Close it (transition: Open -> Closed)
        let file = file.close();
        
        // Still can access path
        assert_eq!(file.path(), "test.txt");
        
        // Cleanup
        fs::remove_file("test.txt").unwrap();
    }

    // This test intentionally does NOT compile - proving type safety
    // Uncommenting will produce compile error: method `read` not found
    /*
    #[test]
    fn test_cannot_read_closed_file() {
        let file = FileHandle::<Closed>::new("test.txt");
        file.read();  // ERROR: method not found in `FileHandle<Closed>`
    }
    */
}
