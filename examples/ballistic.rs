use phys_buzz::Particle;

use bevy::{prelude::*, render::camera::PerspectiveProjection};
use utilities::PhysBuzzDemoPlugin;

// Tracks whether a particle has been alive for a given length of time
struct LifeTimer(Timer);

// Global assets for all particles and shadows
struct ParticleAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    shadow_mesh: Handle<Mesh>,
    shadow_material: Handle<StandardMaterial>,
}

// Ammunition types with different properties
enum ShotType {
    PISTOL,
    ARTILLERY,
    FIREBALL,
    LASER,
}

// Marker type for the shot type text
struct ShotTypeText;

// An entity associated with a given shot entity
struct Shadow(Entity);

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysBuzzDemoPlugin)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(simulate.system())
        .add_system(click_spawn_projectile.system())
        .add_system(shadow_follow.system())
        .add_system(shot_type_select.system())
        .add_system(shot_type_text.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("PhysBuzz - Ballistic Demo".to_string());

    commands.spawn_bundle(UiCameraBundle::default());

    // An overhead light source
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // A camera starting near the launch point, viewing the path of projectiles
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-25.0, 8.0, 5.0)
            .looking_at(Vec3::new(0.0, 5.0, 22.0), Vec3::Y),
        perspective_projection: PerspectiveProjection {
            fov: std::f32::consts::PI / 3.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // A small sphere at the launch point
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 5,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.5, 0.0),
        ..Default::default()
    });

    // A small shadow below the launch point
    let shadow_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.75, 0.75, 0.75),
        unlit: true,
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 5,
        })),
        material: shadow_material.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, -1.5, 0.0),
            scale: Vec3::new(1.0, 0.1, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Scale lines along the projectile path
    let line_mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(0.25, 10.0),
        flip: true,
    }));
    let line_material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
    for i in 0..20 {
        let transform = Transform::from_xyz(0.0, 0.0, i as f32 * 10.0);
        commands.spawn_bundle(PbrBundle {
            mesh: line_mesh.clone(),
            material: line_material.clone(),
            transform: transform.looking_at(Vec3::Y + transform.translation, Vec3::X),
            ..Default::default()
        });
    }

    // Initialize the assets used for all particles and shadows
    commands.insert_resource(ParticleAssets {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.3,
            subdivisions: 5,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        shadow_mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.6,
            subdivisions: 5,
        })),
        shadow_material,
    });

    // Holds the current shot type
    commands.insert_resource(ShotType::PISTOL);

    // Text for instructions and current shot type
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_size = 20.0;
    let shot_type_text = Text {
        sections: vec![
            TextSection {
                value: "Click: Fire\n1-4: Select Ammo\n".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
            },
            TextSection {
                value: "Current Ammo: ".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size,
                    color: Color::rgb(0.0, 0.0, 0.0),
                },
            },
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font: font.clone(),
                    font_size,
                    color: Color::rgb(1.0, 0.5, 0.5),
                },
            },
        ],
        ..Default::default()
    };

    // Text in the top left
    commands
        .spawn_bundle(TextBundle {
            text: shot_type_text,
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ShotTypeText);
}

// Apply physics to all particles, and destroy them if necessary
fn simulate(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut LifeTimer)>,
) {
    let duration = time.delta_seconds();
    if duration <= 0.0 {
        return;
    }

    for (entity, mut particle, mut life_timer) in query.iter_mut() {
        let position = particle.get_position();
        if life_timer.0.tick(time.delta()).finished() || position.y < 0.0 || position.z > 200.0 {
            commands.entity(entity).despawn();
        } else {
            particle.integrate(duration);
        }
    }
}

// Spawn an instance of the current shot type when the user clicks
fn click_spawn_projectile(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
    shot_type: Res<ShotType>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mut particle = Particle::default();
        particle.set_position(0.0, 1.5, 0.0);

        let (mass, vel, grav, damping) = match *shot_type {
            ShotType::PISTOL => (2.0, (0.0, 35.0), -1.0, 0.99),
            ShotType::ARTILLERY => (200.0, (30.0, 40.0), -20.0, 0.99),
            ShotType::FIREBALL => (1.0, (0.0, 10.0), 0.6, 0.9),
            ShotType::LASER => (0.1, (0.0, 100.0), 0.0, 0.99),
        };
        particle.set_mass(mass);
        particle.set_velocity(0.0, vel.0, vel.1);
        particle.set_acceleration(0.0, grav, 0.0);
        particle.set_damping(damping);

        let particle_id = commands
            .spawn_bundle(PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                ..Default::default()
            })
            .insert(particle)
            .insert(LifeTimer(Timer::from_seconds(5.0, false)))
            .id();

        // A shadow below the particle
        commands
            .spawn_bundle(PbrBundle {
                mesh: assets.shadow_mesh.clone(),
                material: assets.shadow_material.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(1.0, 0.1, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Shadow(particle_id));
    }
}

// Follow particles with their respective shadows
fn shadow_follow(
    mut commands: Commands,
    mut query: Query<(Entity, &Shadow, &mut Transform)>,
    particles: Query<&Particle>,
) {
    for (entity, shadow, mut transform) in query.iter_mut() {
        let particle = particles.get(shadow.0);
        if let Ok(particle) = particle {
            let position = particle.get_position();
            transform.translation = Vec3::new(position.x, 0.0, position.z);
        } else {
            // Despawn shadows linked to nonexistent/invalid particles
            commands.entity(entity).despawn();
        }
    }
}

// Set the current shot type using keyboard input
fn shot_type_select(keyboard_input: Res<Input<KeyCode>>, mut shot_type: ResMut<ShotType>) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        *shot_type = ShotType::PISTOL;
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        *shot_type = ShotType::ARTILLERY;
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        *shot_type = ShotType::FIREBALL;
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        *shot_type = ShotType::LASER;
    }
}

// Keep the UI text updated with the current shot type
fn shot_type_text(shot_type: Res<ShotType>, mut text_query: Query<(&mut Text, &ShotTypeText)>) {
    let mut text = text_query.single_mut().unwrap().0;
    text.sections[2].value = match *shot_type {
        ShotType::PISTOL => "Pistol".to_string(),
        ShotType::ARTILLERY => "Artillery".to_string(),
        ShotType::FIREBALL => "Fireball".to_string(),
        ShotType::LASER => "Laser".to_string(),
    }
}
