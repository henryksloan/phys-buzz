use crate::math::{Real, Vector3};
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

/// Generates a force emulating to a stiff spring
pub struct ParticleFakeSpring {
    anchor: Vector3,
    spring_constant: Real,
    damping: Real,
}

impl ParticleForceGenerator for ParticleFakeSpring {
    fn update_force(&mut self, particle: &mut Particle, duration: Real) {
        if !particle.has_finite_mass() {
            return;
        }

        let difference = particle.get_position() - self.anchor;

        // Calculate and bounds-check constants
        let gamma = 0.5 * (4.0 * self.spring_constant * self.damping.powi(2)).sqrt();
        if gamma == 0.0 {
            return;
        }
        let c =
            difference * (self.damping / (2.0 * gamma)) + particle.get_velocity() * (1.0 / gamma);

        let target = (difference * (gamma * duration).cos() + c * (gamma * duration).sin())
            * (-0.5 * duration * self.damping).exp();
        let accel =
            (target - difference) * (1.0 / duration.powi(2)) - particle.get_velocity() * duration;
        particle.add_force(&(accel * particle.get_mass()));
    }
}

impl ParticleFakeSpring {
    pub fn new(anchor: Vector3, spring_constant: Real, damping: Real) -> Self {
        Self {
            anchor,
            spring_constant,
            damping,
        }
    }
}
