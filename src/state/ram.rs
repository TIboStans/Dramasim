pub type RAM = RandomAccessMemory;

pub struct RandomAccessMemory {
    inner: [usize; 10_000]
}

impl RAM {
    pub fn get(&self, index: usize) -> usize {
        self.inner[index]
    }

    pub fn new() -> Self {
        RAM {
            inner: [0; 10_000]
        }
    }
}