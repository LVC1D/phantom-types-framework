mod file_handle;
pub use file_handle::*;

use std::marker::PhantomData;

// 1. Define length marker types
struct Three;
struct Five;

// 2. Define LengthVec
#[derive(Debug, PartialEq)]
pub struct LengthVec<T, N> {
    data: Vec<T>,
    _length: PhantomData<N>,
}

// 3. Implement constructors for specific lengths
impl<T> LengthVec<T, Three> {
    pub fn new(data: [T; 3]) -> Self {
        // Your implementation
        Self {
            data: Vec::from(data),
            _length: PhantomData,
        }
    }
}

impl<T> LengthVec<T, Five> {
    pub fn new(data: [T; 5]) -> Self {
        // Your implementation
        Self {
            data: Vec::from(data),
            _length: PhantomData,
        }
    }
}

// 4. Implement zip for any length N
impl<T, N> LengthVec<T, N> {
    pub fn zip<U>(self, other: LengthVec<U, N>) -> LengthVec<(T, U), N> {
        // Your implementation
        let data: Vec<_> = self.data.into_iter().zip(other.data).collect();

        LengthVec {
            data,
            _length: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_length_zip() {
        let v1 = LengthVec::<i32, Three>::new([1, 2, 3]);
        let v2 = LengthVec::<&str, Three>::new(["a", "b", "c"]);
        
        let v3 = v1.zip(v2);
        // If this compiles, our type safety works!
        let expected = vec![(1, "a"), (2, "b"), (3, "c")];
        assert_eq!(v3.data, expected)
    }

    // This test intentionally does NOT compile - proving type safety works
    // Uncommenting will produce compile error: mismatched types
    /*
    #[test]
    fn test_different_length_zip() {
        let v1 = LengthVec::<i32, Three>::new([1, 2, 3]);
        let v2 = LengthVec::<i32, Five>::new([1, 2, 3, 4, 5]);
    
        v1.zip(v2);  // ERROR: expected LengthVec<_, Three>, found LengthVec<_, Five>
    }
    */
}

