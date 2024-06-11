use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    p: Vector3,
    q: Vector3,
}

impl Line {
    /// Construct a Line from its two vertices
    pub fn new(p: Vector3, q: Vector3) -> Line {
        Line { p, q }
    }

    /// Get the p-vertex
    pub fn p(&self) -> Vector3 {
        self.p
    }

    /// Get the q-vertex
    pub fn q(&self) -> Vector3 {
        self.q
    }
}

impl std::ops::Index<usize> for Line {
    type Output = Vector3;

    fn index(&self, index: usize) -> Self::Output {
        match index {
            0 => self.p,
            1 => self.q,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            _ => panic!("index out of range"),
        }
    }
}
