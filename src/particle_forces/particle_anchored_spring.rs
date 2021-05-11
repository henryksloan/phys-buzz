use crate::math::{Real, Vector3};
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

/// Generates a spring force relative to a fixed anchor
pub struct ParticleAnchoredSpring {
    anchor: Vector3,
    spring_constant: Real,
    rest_length: Real,
}

impl ParticleForceGenerator for ParticleAnchoredSpring {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let difference = particle.get_position() - self.anchor;
        let direction = {
            let mut temp = difference;
            temp.normalize();
            temp
        };
        let distance = difference.magnitude();

        // A spring acts to pull the particle towards the rest length
        // with force proportional to the spring constant
        let magnitude = self.spring_constant * (distance - self.rest_length).abs();
        particle.add_force(&(direction * -magnitude));
    }
}

impl ParticleAnchoredSpring {
    pub fn new(anchor: Vector3, spring_constant: Real, rest_length: Real) -> Self {
        Self {
            anchor,
            spring_constant,
            rest_length,
        }
    }
}
