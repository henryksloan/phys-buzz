use crate::math::Real;
use crate::particle::Particle;

pub trait ParticleForceGenerator {
    fn update_force(&mut self, particle: &mut Particle, duration: Real);
}
