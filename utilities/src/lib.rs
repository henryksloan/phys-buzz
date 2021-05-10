use bevy::prelude::*;
use phys_buzz::Particle;

pub struct PhysBuzzDemoPlugin;

impl Plugin for PhysBuzzDemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_transforms.system());
    }
}

fn update_transforms(mut query: Query<(&mut Transform, &Particle)>) {
    for (mut transform, particle) in query.iter_mut() {
        let particle_position = particle.get_position();
        transform.translation.x = particle_position.x;
        transform.translation.y = particle_position.y;
        transform.translation.z = particle_position.z;
    }
}
