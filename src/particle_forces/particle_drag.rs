use crate::math::Real;
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

/// Generates drag on a particle depending directly and squarely on its velocity
pub struct ParticleDrag {
    k1: Real,
    k2: Real,
}

impl ParticleForceGenerator for ParticleDrag {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let velocity = particle.get_velocity();
        let direction = {
            let mut temp = velocity;
            temp.normalize();
            temp
        };
        let speed = velocity.magnitude();

        let drag_coeff = self.k1 * speed + self.k2 * speed.powi(2);
        particle.add_force(&(direction * -drag_coeff));
    }
}

impl ParticleDrag {
    pub fn new(k1: Real, k2: Real) -> Self {
        Self { k1, k2 }
    }
}
