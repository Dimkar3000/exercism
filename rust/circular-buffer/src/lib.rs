#[derive(Clone)]
pub struct CircularBuffer<T>
where
    T: Clone,
{
    fields: Vec<Option<T>>,
    index: usize,
    capacity: usize,
    read_index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            fields: vec![None; capacity],
            index: 0,
            capacity,
            read_index: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.fields[self.index].is_none() {
            self.fields[self.index] = Some(element);
            self.index = (self.index + 1) % self.capacity;
            return Ok(());
        }
        Err(Error::FullBuffer)
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.read_index >= self.capacity {
            return Err(Error::EmptyBuffer);
        }
        let result = std::mem::replace(&mut self.fields[self.read_index], None);
        self.read_index = (self.read_index + 1) % self.capacity;

        if let Some(x) = result {
            return Ok(x);
        }
        Err(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.fields = vec![None; self.capacity];
    }

    pub fn overwrite(&mut self, element: T) {
        let result = std::mem::replace(&mut self.fields[self.index], Some(element));
        self.index = (self.index + 1) % self.capacity;
        if result.is_some() {
            self.read_index = (self.read_index + 1) % self.capacity;
        }
    }
}
