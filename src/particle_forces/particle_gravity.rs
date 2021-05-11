use crate::math::{Real, Vector3};
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

/// Generates a constant gravitational force on a particle
pub struct ParticleGravity {
    gravity: Vector3,
}

impl ParticleForceGenerator for ParticleGravity {
    fn update_force(&mut self, particle: &mut Particle, duration: Real) {
        if particle.has_finite_mass() {
            particle.add_force(&(self.gravity * particle.get_mass()));
        }
    }
}

impl ParticleGravity {
    pub fn new(gravity: Vector3) -> Self {
        Self { gravity }
    }
}
