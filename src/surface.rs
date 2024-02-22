use crate::vec::Vec2;

#[derive(Clone, Copy)]
pub struct Element {
    pub value: char
}

pub struct Surface {
    dim: Vec2,
    data: Vec<Element>,
}

impl Surface {
    pub fn new(dim: Vec2) -> Self {
        let mut data = Vec::new();
        data.resize((dim.x * dim.y) as usize, Element { value: ' ' });
        Self {
            dim,
            data
        }
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x < self.dim.x &&
        pos.y < self.dim.y
    }

    pub fn elem(&self, pos: Vec2) -> Option<&Element> {
        if self.contains(pos) {
            return Some(&self.data[( (pos.y * self.dim.x) + pos.x ) as usize])
        }
        None
    }

    pub fn elem_mut(&mut self, pos: Vec2) -> Option<&mut Element> {
        if self.contains(pos) {
            return Some(&mut self.data[( (pos.y * self.dim.x) + pos.x ) as usize])
        }
        None
    }

    pub fn size(&self) -> Vec2 {
        self.dim
    }

    pub fn state(&self) -> &Vec<Element> {
        &self.data
    }
}
