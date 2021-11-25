use std::ops::{Index, IndexMut};

use anyhow::bail;
use anyhow::Result;
pub struct BasicMatrix<T> {
    vec: Vec<T>,
    m: usize,
    n: usize,
}

impl<T> BasicMatrix<T> {
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

impl From<BrailleChar> for char {
    fn from(other: BrailleChar) -> char {
        let mut base: u32 = 0x2800;
        base += other.bits[0][0] as u32;
        base += (other.bits[0][1] as u32) << 1;
        base += (other.bits[0][2] as u32) << 2;
        base += (other.bits[1][0] as u32) << 3;
        base += (other.bits[1][1] as u32) << 4;
        base += (other.bits[1][2] as u32) << 5;
        base += (other.bits[0][3] as u32) << 6;
        base += (other.bits[1][3] as u32) << 7;

        if base == 0x2800 {
            // the empty braille char has alignment issues,
            // so we convert it into the first non-empty one instead
            base += 1;
        }

        // SAFETY: the only possible values of `base` are definitely valid UTF-8 characters
        // corresponding to braille.
        unsafe { char::from_u32_unchecked(base) }
    }
}
