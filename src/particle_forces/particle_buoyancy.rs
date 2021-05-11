use crate::math::{Real, Vector3};
use crate::particle::Particle;
use crate::particle_forces::ParticleForceGenerator;

/// Simulate a simple buoyancy force relative to a plane of liquid
pub struct ParticleBuoyancy {
    /// The depth after which the generator produces its maximal force
    max_depth: Real,

    /// The volume of the object being submerged
    volume: Real,

    /// The height of the plane of liquid above y=0
    liquid_height: Real,

    /// The density of the liquid (e.g. water = 1000 kg/m^3)
    liquid_density: Real,
}

impl ParticleForceGenerator for ParticleBuoyancy {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let depth = particle.get_position().y;

        // Return if the particle is out of the water
        if depth >= self.liquid_height + self.max_depth {
            return;
        }

        let force = if depth <= self.liquid_height - self.max_depth {
            // If at or below maximum depth, apply maximum force
            self.liquid_density * self.volume
        } else {
            // Otherwise, the force depends on depth
            self.liquid_density
                * self.volume
                * ((depth - self.max_depth - self.liquid_height) / 2.0)
                * self.max_depth
        };
        particle.add_force(&Vector3::new(0.0, force, 0.0));
    }
}

impl ParticleBuoyancy {
    pub fn new(max_depth: Real, volume: Real, liquid_height: Real, liquid_density: Real) -> Self {
        Self {
            max_depth,
            volume,
            liquid_height,
            liquid_density,
        }
    }

    pub fn new_water(max_depth: Real, volume: Real, liquid_height: Real) -> Self {
        Self::new(max_depth, volume, liquid_height, 1000.0)
    }
}
