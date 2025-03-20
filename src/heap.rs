#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

pub struct TFixedIHeapClass<T>(Vec<T>);

pub enum InsertError{
    ReachedCapacity,
}

impl<T> TFixedIHeapClass<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            0: Vec::with_capacity(capacity),
        }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), InsertError> {
        if self.len() == self.0.capacity() {
            return Err(InsertError::ReachedCapacity);
        }

        self.0.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Was Alloc(void) in C++ code
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), InsertError> {
        if self.len() == self.0.capacity() {
            return Err(InsertError::ReachedCapacity);
        }
        self.0.insert(index, element);
        Ok(())
    }

    /// Was Free(T * pointer) in C++ code
    pub fn try_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        Some(self.0.remove(index))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }
}
