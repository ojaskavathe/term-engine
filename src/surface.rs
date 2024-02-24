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

    pub fn set(&mut self, pos: Vec2, val: Element) -> Option<&Element> {
        let elem = self.elem_mut(pos);

        match elem {
            Some(x) => {
                *x = val;
                Some(x)
            },
            None => None
        }
    }

    pub fn size(&self) -> Vec2 {
        self.dim
    }

    pub fn state(&self) -> &Vec<Element> {
        &self.data
    }

    pub fn print_str(&mut self, s: &str, x: u16, y: u16) {
        for (i, c) in s.chars().enumerate() {
            let elem = self.elem_mut(Vec2{ x:(x + i as u16) as i16, y:y as i16 }).unwrap();
            elem.value = c;
        }
    }

    pub fn draw_line(&mut self, p1: Vec2, p2: Vec2, val: Element) {
        // Bresenham
        let mut p: Vec2;
        let (mut dx, mut dy): (i16, i16);
        let (incx, incy): (i16, i16);
        let mut balance: i16;
   
        if p2.x > p1.x {
            dx = p2.x - p1.x;
            incx = 1;
        } else {
            dx = p1.x - p2.x;
            incx = -1
        }
   
        if p2.y > p1.y {
            dy = p2.y - p1.y;
            incy = 1;
        } else {
            dy = p1.y - p2.y;
            incy = -1
        }

        p = p1;

        if dx >= dy {
            dy <<= 1;
            balance = dy - dx;
            dx <<= 1;

            while p.x != p2.x {
                self.set(p, val).unwrap();
                if balance >= 0 {
                    p.y += incy;
                    balance -= dx;
                }
                balance += dy;
                p.x += incx;
            } 
            self.set(p, val).unwrap();
        }

        if dy >= dx {
            dx <<= 1;
            balance = dx - dy;
            dy <<= 1;

            while p.y != p2.y {
                self.set(p, val).unwrap();
                if balance >= 0 {
                    p.x += incx;
                    balance -= dy;
                }
                balance += dx;
                p.y += incy;
            } 
            self.set(p, val).unwrap();
        }
    }

    pub fn clear(&mut self) {
        self.data.iter_mut().map(|x| *x = Element{ value: ' ' }).count();
    }
}
