use crate::math::{Real, Vector3};

/// A simple particle with basic physical properties
#[derive(Default)]
pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,

    /// The resultant force acting at the next integration step
    force_accum: Vector3,

    /// The degree of velocity damping.
    /// This ensures small numerical instability doesn't
    /// add unwanted motion to the particle.
    /// Equal to the proportion of velocity kept at each update
    /// (0.0 = none, 0.995 = almost all, 1.0 = all)
    damping: Real,

    /// Holds the inverse of the particle's mass.
    /// This simplifies math, and allows for infinite mass
    inverse_mass: Real,
}

impl Particle {
    /// Update the kinetic properties of the particle
    /// by integrating over a duration of time
    pub fn integrate(&mut self, duration: Real) {
        assert!(
            duration > 0.0,
            "attempted to integrate over a zero or negative duration",
        );

        // Update the position by the linear velocity
        self.position.add_scaled_vector(&self.velocity, duration);

        // Update the acceleration by the force
        let mut resultant = self.acceleration;
        resultant.add_scaled_vector(&self.force_accum, duration);

        // Update the velocity by the acceleration
        self.velocity.add_scaled_vector(&resultant, duration);

        // Impose drag and velocity damping
        self.velocity *= self.damping.powf(duration);

        // Clear the force accumulator
        self.clear_accumulator();
    }

    pub fn add_force(&mut self, force: &Vector3) {
        self.force_accum += *force;
    }

    /// Returns the mass, or infinity if the inverse mass is zero
    pub fn get_mass(&self) -> Real {
        if self.inverse_mass == 0.0 {
            Real::INFINITY
        } else {
            1.0 / self.inverse_mass
        }
    }

    /// Sets the mass, panicking if the given value is zero
    pub fn set_mass(&mut self, mass: Real) {
        assert_ne!(mass, 0.0, "attempted to set mass to zero");
        self.inverse_mass = 1.0 / mass;
    }

    /// Gets the inverse of the particle's mass
    pub fn get_inverse_mass(&self) -> Real {
        self.inverse_mass
    }

    /// Sets the inverse of the particle's mass
    pub fn set_inverse_mass(&mut self, inverse_mass: f32) {
        self.inverse_mass = inverse_mass;
    }

    pub fn has_finite_mass(&self) -> bool {
        self.inverse_mass != 0.0
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }

    pub fn get_acceleration(&self) -> Vector3 {
        self.acceleration
    }

    pub fn set_position(&mut self, x: Real, y: Real, z: Real) {
        self.position.x = x;
        self.position.y = y;
        self.position.z = z;
    }

    pub fn set_velocity(&mut self, x: Real, y: Real, z: Real) {
        self.velocity.x = x;
        self.velocity.y = y;
        self.velocity.z = z;
    }

    pub fn set_acceleration(&mut self, x: Real, y: Real, z: Real) {
        self.acceleration.x = x;
        self.acceleration.y = y;
        self.acceleration.z = z;
    }

    pub fn get_damping(&mut self) -> Real {
        self.damping
    }

    pub fn set_damping(&mut self, damping: Real) {
        self.damping = damping;
    }

    pub fn clear_accumulator(&mut self) {
        self.force_accum.x = 0.0;
        self.force_accum.y = 0.0;
        self.force_accum.z = 0.0;
    }
}
