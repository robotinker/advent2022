use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul}, fmt::Debug, str::FromStr, num::ParseIntError};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct V2 {
    pub x: i32,
    pub y: i32
}

impl V2 {
    pub const ZERO: V2 = V2{x: 0, y: 0};
    pub const ONE: V2 = V2{x: 1, y: 1};
    pub const RIGHT: V2 = V2{x: 1, y: 0};
    pub const LEFT: V2 = V2{x: -1, y: 0};
    pub const UP: V2 = V2{x: 0, y: 1};
    pub const DOWN: V2 = V2{x: 0, y: -1};

    pub fn squish(&self) -> V2 {
        V2{x: if self.x == 0 {0} else {self.x / self.x.abs()}, y: if self.y == 0 {0} else {self.y / self.y.abs()}}
    }

    pub fn add(&self, other: &V2) -> V2 {
        V2{x: self.x + other.x, y: self.y + other.y}
    }

    pub fn sub(&self, other: &V2) -> V2 {
        V2{x: self.x - other.x, y: self.y - other.y}
    }
}

impl FromStr for V2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('(') || !s.ends_with(')') || !s.contains(',') {
            panic!("Cannot parse V2")
        }
        let sides = s[1..s.len() - 1].split(", ").collect::<Vec<_>>();
        Ok(V2{x: sides[0].parse::<i32>().expect("Couldn't parse left side of V2"), y: sides[1].parse::<i32>().expect("Couldn't parse right side of V2")})
    }
}

impl Add for V2 {
    type Output = V2;

    fn add(self, rhs: Self) -> Self::Output {
        V2{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        V2{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for V2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for V2 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        V2{x: self.x * rhs.x, y: self.y * rhs.y}
    }
}

impl Mul<i32> for V2 {
    type Output = V2;

    fn mul(self, rhs: i32) -> Self::Output {
        V2{x: self.x * rhs, y: self.y * rhs}
    }
}

impl Mul<V2> for i32 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        V2{x: self * rhs.x, y: self * rhs.y}
    }
}

impl ToString for V2 {
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        s.push_str(&"(".to_string());
        s.push_str(&self.x.to_string());
        s.push_str(&", ".to_string());
        s.push_str(&self.y.to_string());
        s.push_str(&")".to_string());
        s
    }
}

pub fn to_string(vs: &[V2]) -> String {
    let xs = vs.iter().map(|v| v.x).collect::<Vec<_>>();
    let ys = vs.iter().map(|v| v.y).collect::<Vec<_>>();
    let (x_min, x_max, y_min, y_max) = (xs.iter().min().unwrap(), xs.iter().max().unwrap(), ys.iter().min().unwrap(), ys.iter().max().unwrap());
    let row = vec!('.'; (*x_max - *x_min + 1) as usize);
    let mut grid = vec!(row; (*y_max - *y_min + 1) as usize);
    (*x_min..=*x_max).for_each(|x| {
        (*y_min..=*y_max).for_each(|y| if vs.contains(&V2{x, y}) {grid[(y - y_min) as usize][(x - x_min) as usize] = '#'})
    });
    let mut output = Vec::new();
    grid.iter().rev().for_each(|row| output.push(row.iter().collect::<String>()));
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_v2() {
        let new_point = V2{x: 1, y: 1} + V2{x: 1, y: 0};
        assert_eq!(new_point.x, 2);
        assert_eq!(new_point.y, 1);
    }
}
