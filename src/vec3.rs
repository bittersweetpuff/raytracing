use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: (f64, f64, f64),
}

impl Vec3 {
    /// Parameterless constructor
    pub fn new_zeros() -> Vec3 {
        Vec3 { e: (0.0, 0.0, 0.0) }
    }

    /// Constructor
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: (e0, e1, e2) }
    }

    pub fn x(self) -> f64 {
        self.e.0
    }

    pub fn y(self) -> f64 {
        self.e.1
    }

    pub fn z(self) -> f64 {
        self.e.2
    }

    pub fn length_squared(self) -> f64 {
        (self.e.0 * self.e.0) + (self.e.1 * self.e.1) + (self.e.2 * self.e.2)
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn print(self) {
        println!("{} {} {}", self.e.0, self.e.1, self.e.2);
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.e.0,
            1 => &self.e.1,
            2 => &self.e.2,
            _ => panic!("Tried to access an index of {} in 3D vector", i),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.e.0,
            1 => &mut self.e.1,
            2 => &mut self.e.2,
            _ => panic!("Tried to access an index of {} in 3D vector", i),
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Self {
            e: (
                self.e.0 - other.e.0,
                self.e.1 - other.e.1,
                self.e.2 - other.e.2,
            ),
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            e: (self.e.0 * t, self.e.1 * t, self.e.2 * t),
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Self {
            e: (
                self.e.0 - other.e.0,
                self.e.1 - other.e.1,
                self.e.2 - other.e.2,
            ),
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Self {
            e: (self.e.0 * t, self.e.1 * t, self.e.2 * t),
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Self {
            e: (
                self.e.0 * other.e.0,
                self.e.1 * other.e.1,
                self.e.2 * other.e.2,
            ),
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: (self * other.e.0, self * other.e.1, self * other.e.2),
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Self {
            e: (self.e.0 * 1.0 / t, self.e.1 * 1.0 / t, self.e.2 * 1.0 / t),
        }
    }
}

/// Returns a dot product of two Vec3 vectors
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    (u.e.0 * v.e.0) + (u.e.1 * v.e.1) + (u.e.2 * v.e.2)
}

/// Returns a cross product of two Vec3 vectors
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: (
            u.e.1 * v.e.2 - u.e.2 * v.e.1,
            u.e.2 * v.e.0 - u.e.0 * v.e.2,
            u.e.0 * v.e.1 - u.e.1 * v.e.0,
        ),
    }
}

/// Returns a unit vector having the same direction as an argument
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
