//USE: $env:WGPU_BACKEND = "vulkan"; cargo run
//use bevy::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod map;
mod user;
mod weapon;

fn main() {
    App::new()
        .insert_resource(map::Map::new_square(250.0, 2))
        .add_systems(Startup, map::spawn_map)
        .add_systems(Startup, user::spawn_player)
        .add_systems(Update, user::move_player)
        .add_systems(Update, user::rotate_player)
        .add_systems(Update, weapon::equip_rifle)
        .add_systems(Update, weapon::spawn_bullets)
        .add_systems(Update, weapon::bullet_drag)
        //.add_systems(Update, shoot_rifle)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "shooter_2d".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
