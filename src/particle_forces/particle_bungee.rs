use crate::math::Real;
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

use std::cell::RefCell;
use std::rc::Rc;

/// Generates a spring force relative to another particle,
/// but only if they are past a certain distance
pub struct ParticleBungee {
    other: Rc<RefCell<Particle>>,
    spring_constant: Real,
    rest_length: Real,
}

impl ParticleForceGenerator for ParticleBungee {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let difference = particle.get_position() - self.other.borrow_mut().get_position();
        let direction = {
            let mut temp = difference;
            temp.normalize();
            temp
        };
        let distance = difference.magnitude();

        if distance <= self.rest_length {
            return;
        }

        // A spring acts to pull the particle towards the rest length
        // with force proportional to the spring constant
        let magnitude = self.spring_constant * (distance - self.rest_length);
        particle.add_force(&(direction * -magnitude));
    }
}

impl ParticleBungee {
    pub fn new(other: Rc<RefCell<Particle>>, spring_constant: Real, rest_length: Real) -> Self {
        Self {
            other,
            spring_constant,
            rest_length,
        }
    }
}
