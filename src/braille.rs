use std::ops::{Index, IndexMut};

use anyhow::bail;
use anyhow::Result;
pub struct BasicMatrix<T> {
    vec: Vec<T>,
    m: usize,
    n: usize,
}

impl<T> BasicMatrix<T> {
    pub fn _new(vec: Vec<T>, m: usize, n: usize) -> Result<BasicMatrix<T>> {
        if vec.len() != m * n || vec.is_empty() {
            bail!("Invalid matrix size provided: {}x{}", m, n);
        } else {
            Ok(BasicMatrix { vec, m, n })
        }
    }

    pub fn generate(
        m: usize,
        n: usize,
        mut f: impl FnMut(usize, usize) -> T,
    ) -> Result<BasicMatrix<T>> {
        if m == 0 || n == 0 {
            bail!("Invalid matrix size provided: {}x{}", m, n);
        } else {
            let mut vec = Vec::with_capacity(m * n);
            for i in 0..m {
                for j in 0..n {
                    vec.push(f(i, j));
                }
            }

            Ok(BasicMatrix { vec, m, n })
        }
    }

    pub fn _get(&self, i: usize, j: usize) -> Result<&T> {
        if i >= self.m || j >= self.n {
            bail!("Invalid matrix indices: ({}, {})", i, j);
        } else {
            Ok(&self[(i, j)])
        }
    }

    pub fn _get_mut(&mut self, i: usize, j: usize) -> Result<&mut T> {
        if i >= self.m || j >= self.n {
            bail!("Invalid matrix indices: ({}, {})", i, j);
        } else {
            Ok(&mut self[(i, j)])
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.m, self.n)
    }
}

impl<T> Index<(usize, usize)> for BasicMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vec[index.0 + index.1 * self.m]
    }
}

impl<T> IndexMut<(usize, usize)> for BasicMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.0 + index.1 * self.m]
    }
}

impl<T: Clone> Clone for BasicMatrix<T> {
    fn clone(&self) -> Self {
        BasicMatrix {
            vec: self.vec.clone(),
            m: self.m,
            n: self.n,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct BrailleChar {
    pub bits: [[bool; 4]; 2],
}

impl Into<char> for BrailleChar {
    fn into(self) -> char {
        let mut base: u32 = 0x2800;
        base += self.bits[0][0] as u32;
        base += (self.bits[0][1] as u32) << 1;
        base += (self.bits[0][2] as u32) << 2;
        base += (self.bits[1][0] as u32) << 3;
        base += (self.bits[1][1] as u32) << 4;
        base += (self.bits[1][2] as u32) << 5;
        base += (self.bits[0][3] as u32) << 6;
        base += (self.bits[1][3] as u32) << 7;

        if base == 0x2800 {
            base += 1;
        }

        char::from_u32(base).unwrap()
    }
}
