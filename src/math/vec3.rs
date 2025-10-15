#[derive(Debug, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}
impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn from<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Vec3 {
            e: [x.into(), y.into(), z.into()],
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|x| x * x).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.e
            .iter()
            .enumerate()
            .fold(0.0, |acc, (i, ele)| acc + (ele * v.e[i]))
    }

    pub fn scale<T: Into<f64>>(&self, scalar: T) -> Self {
        let scalar = scalar.into();
        Self {
            e: array::from_fn(|i| self.e[i] * scalar),
        }
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Vec3::from(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Self {
        self * (1.0 / self.length())
    }
}
use std::{
    array,
    ops::{Add, Mul, Sub},
};
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: std::array::from_fn(|i| self.e[i] + rhs.e[i]),
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: std::array::from_fn(|i| self.e[i] + rhs.e[i]),
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: std::array::from_fn(|i| self.e[i] - rhs.e[i]),
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: std::array::from_fn(|i| self.e[i] - rhs.e[i]),
        }
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs: f64 = rhs.into();
        Self {
            e: std::array::from_fn(|i| self.e[i] * rhs),
        }
    }
}

impl<T: Into<f64>> Mul<T> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs: f64 = rhs.into();
        Vec3 {
            e: std::array::from_fn(|i| self.e[i] * rhs),
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        return self.e == other.e;
    }
}

impl Eq for Vec3 {}

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn vec_init() {
        let a = Vec3::from(1, 2, 3);
        let b = Vec3::from(1.0, 2.0, 3.0);
        assert_eq!(a, b)
    }

    #[test]
    fn vec_dot() {
        let a = Vec3::from(1, 2, 3);
        let b = Vec3::from(2, 3, 4);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn vec_add() {
        let a = Vec3::from(1, 2, 3);
        let b = Vec3::from(2, 3, 4);
        assert_eq!(a + b, Vec3::from(3, 5, 7));
    }

    #[test]
    fn vec_sub() {
        let a = Vec3::from(1, 2, 3);
        let b = Vec3::from(2, 3, 4);
        assert_eq!(a - b, Vec3::from(-1, -1, -1));
    }

    #[test]
    fn scalar_mult() {
        let a = Vec3::from(1, 2, 3);
        let mut b = 4;
        assert_eq!(&a * b, Vec3::from(4, 8, 12));
        assert_eq!(a.scale(b), Vec3::from(4, 8, 12));
        b = 0;
        assert_eq!(a * b, Vec3::from(0, 0, 0));
    }

    #[test]
    fn unit_vector() {
        let a = Vec3::from(1, 2, 3);
        assert_eq!(
            a.unit_vector(),
            Vec3::from(
                1.0 / (14 as f64).sqrt(),
                2.0 / (14 as f64).sqrt(),
                3.0 / (14 as f64).sqrt()
            )
        );

        let b = Vec3::from(5, 0, 0);
        assert_eq!(b.unit_vector(), Vec3::from(1, 0, 0));
    }
}
