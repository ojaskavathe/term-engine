use num::cast::ToPrimitive;

use std::ops::{
    Add, Sub, Mul, Div, Neg
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i16,
    pub y: i16
}

impl Vec2 {
    pub fn zero() -> Vec2 {
        Self { x: 0, y: 0 }
    }

    pub fn new(x: impl ToPrimitive, y: impl ToPrimitive) -> Vec2 {
        Self { 
            x: x.to_i16().unwrap(), 
            y: y.to_i16().unwrap()  
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T: ToPrimitive> Mul<T> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs.to_i16().unwrap(),
            y: self.y * rhs.to_i16().unwrap()
        }
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    
    fn div(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl<T: ToPrimitive> Div<T> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x / rhs.to_i16().unwrap(),
            y: self.y / rhs.to_i16().unwrap()
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}
