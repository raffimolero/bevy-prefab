use bevy::{prelude::*, window::close_on_esc};
use bevy_prefab::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tick)
        .add_system(close_on_esc)
        .run();
}

fn new_sprite(x: f32, y: f32, w: f32, h: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2 { x: w, y: h }),
            ..default()
        },
        transform: Transform {
            translation: Vec3 { x, y, z: default() },
            rotation: default(),
            scale: Vec3::splat(0.9),
        },
        ..default()
    }
}

fn square(x: f32, color: Color) -> SpriteBundle {
    new_sprite(x, 0.0, 50.0, 50.0, color)
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // usual Bevy API
    commands
        .spawn(square(-360.0, Color::PINK))
        .with_children(|parent| {
            parent.spawn(square(60.0, Color::VIOLET));
            parent.spawn(square(120.0, Color::PURPLE));
            parent
                .spawn(square(180.0, Color::BLUE))
                .with_children(|parent| {
                    parent.spawn(square(60.0, Color::CYAN));
                    parent.spawn(square(120.0, Color::AQUAMARINE));
                });
        });

    // prefab API
    let prefab = square(0.0, Color::RED)
        .child(square(60.0, Color::ORANGE_RED))
        .child(square(120.0, Color::ORANGE))
        .child(
            square(180.0, Color::YELLOW)
                .child(square(60.0, Color::YELLOW_GREEN))
                .child(square(120.0, Color::GREEN)),
        );

    // Explicit type of the prefab:
    // ParentNode<SpriteBundle, SiblingsNode<SiblingsNode<
    //    SpriteBundle,
    //    SpriteBundle>,
    //    ParentNode<SpriteBundle, SiblingsNode<
    //        SpriteBundle,
    //        SpriteBundle>>>>
    //

    commands.spawn_prefab(prefab);
}

fn tick(time: Res<Time>, mut movers: Query<&mut Transform, Without<Camera>>) {
    let delta = time.delta_seconds() * 10.0;
    movers.iter_mut().for_each(|mut transform| {
        transform.translation.y += delta;
    })
}
