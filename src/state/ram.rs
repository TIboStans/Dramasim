use std::ops::{Index, IndexMut};

pub type RAM = RandomAccessMemory;

pub struct RandomAccessMemory {
    inner: [isize; 10_000]
}

impl RAM {
    pub fn new() -> Self {
        RAM {
            inner: [0; 10_000]
        }
    }
}

impl Index<usize> for RAM {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl Index<isize> for RAM {
    type Output = isize;

    fn index(&self, index: isize) -> &Self::Output {
        &self.inner[address(index)]
    }
}

impl IndexMut<usize> for RAM {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl IndexMut<isize> for RAM {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        &mut self.inner[address(index)]
    }
}

pub fn address(number: isize) -> usize {
    let a = number % 10_000;
    if a < 0 { (a + 10_000) as usize } else { a as usize }
}

pub fn expand(a: usize) -> isize {
    if a >= 5_000 { a as isize - 10_000 } else { a as isize }
}