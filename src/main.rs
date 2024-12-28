/* TODO:
components:
Player(speed, health, size, global_pos, inventory)
gun(range, fire rate, spread, )

Other implimentations:
Map rendering, collisions with objects
*/


/* TODAY'S IMPLIMENTATIONS:
player movement (circle with no rotation),
map movement and offset
*/

use bevy::prelude::*;

#[derive(Component)]
struct Player{
    global_pos: Vec3
}

#[derive(Component)]
struct Map{
    dimensions: Vec2,
    
}


fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>){
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(45.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::BLACK))),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}


fn main() {
    App::new()
        .add_systems(Startup, spawn_player)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "shooter_2d".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
