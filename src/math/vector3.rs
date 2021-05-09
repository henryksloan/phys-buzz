use crate::math::Real;

use std::ops::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Holds a vector with three components
/// The default constructor create a zero vector
#[derive(Default)]
pub struct Vector3 {
    x: Real,
    y: Real,
    z: Real,
}

impl Vector3 {
    /// Creates a vector with the given components
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }

    /// Negates each component, creating an equal and opposite vector
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// Returns the magnitude of the vector
    pub fn magnitude(&self) -> Real {
        self.square_magnitude().sqrt()
    }

    /// Returns the square of the magnitude of the vector,
    /// using simpler math than getting the magnitude then squaring it
    pub fn square_magnitude(&self) -> Real {
        self.x.powi(2) + self.y.powi(2) + self.y.powi(2)
    }

    /// Converts the vector to a unit vector in the same direction
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        assert_ne!(magnitude, 0.0, "attempted to normalize a zero vector");
        *self *= 1.0 / magnitude;
    }

    /// Adds a vector, scaled by a scalar, to this vector
    pub fn add_scaled_vector(&mut self, other: &Vector3, scalar: Real) {
        self.x += other.x * scalar;
        self.y += other.y * scalar;
        self.z += other.z * scalar;
    }

    /// Creates a new vector containing each component's product
    pub fn component_product(&self, other: &Vector3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    /// Multiplies each component by that of another vector
    pub fn component_product_update(&mut self, other: &Vector3) {
        *self = self.component_product(other);
    }

    /// Calculates the scalar (aka dot, inner) product of two vectors
    pub fn scalar_product(&self, other: &Vector3) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn vector_product(&self, other: &Vector3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.z,
        }
    }
}

/// Multiplies each component by a scalar
impl Mul<Real> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<Real> for Vector3 {
    fn mul_assign(&mut self, rhs: Real) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Adds two vectors by adding each component
impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// Subtracts two vectors by subtracting each component
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Calculates the scalar (aka dot, inner) product of two vectors
impl Mul for Vector3 {
    type Output = Real;

    fn mul(self, rhs: Self) -> Self::Output {
        self.scalar_product(&rhs)
    }
}

/// Calculates the vector (aka cross) product of two vectors
impl Rem for Vector3 {
    type Output = Vector3;

    fn rem(self, rhs: Self) -> Self::Output {
        self.vector_product(&rhs)
    }
}

impl RemAssign for Vector3 {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.vector_product(&rhs);
    }
}
