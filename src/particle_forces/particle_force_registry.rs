use crate::math::Real;
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

use std::cell::RefCell;
use std::rc::Rc;

pub struct ParticleForceRegistry {
    registrations: Vec<(
        Rc<RefCell<Particle>>,
        Rc<RefCell<dyn ParticleForceGenerator>>,
    )>,
}

impl ParticleForceRegistry {
    pub fn new() -> Self {
        Self {
            registrations: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        particle: Rc<RefCell<Particle>>,
        generator: Rc<RefCell<dyn ParticleForceGenerator>>,
    ) {
        self.registrations.push((particle, generator));
    }

    pub fn remove(
        &mut self,
        particle: Rc<RefCell<Particle>>,
        generator: Rc<RefCell<dyn ParticleForceGenerator>>,
    ) {
        if let Some(index) = self
            .registrations
            .iter()
            .position(|reg| Rc::ptr_eq(&reg.0, &particle) && Rc::ptr_eq(&reg.1, &generator))
        {
            self.registrations.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.registrations.clear();
    }

    pub fn update_forces(&mut self, duration: Real) {
        for registration in &self.registrations {
            let mut particle = registration.0.borrow_mut();
            registration
                .1
                .borrow_mut()
                .update_force(&mut *particle, duration);
        }
    }
}
