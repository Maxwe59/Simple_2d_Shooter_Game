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

use bevy::{prelude::*};

#[derive(Component)]
struct Player {
    global_pos: Vec3,
}

#[derive(Resource)]
struct Map {
    dimensions: Vec2,
}

struct Object {}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 0.0)));
    commands.spawn((
        Player {
            global_pos: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        Mesh2d(meshes.add(Circle::new(45.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::BLACK))),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

//spawn map
fn spawn_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    map: Res<Map>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(map.dimensions.x, map.dimensions.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_player(
    mut transform: ParamSet<(Query<&mut Transform, With<Camera2d>>, Query<&mut Transform, With<Player>>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    let mut final_pos: Vec3 = Vec3{x: 0.0, y: 0.0, z: 0.0};
    for mut camera in transform.p0().iter_mut(){
        if keyboard_input.pressed(KeyCode::KeyD){
            camera.translation.x += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA){
            camera.translation.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW){
            camera.translation.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS){
            camera.translation.y -= 10.0;
        }
        final_pos = camera.translation;
    }

    for mut player in transform.p1().iter_mut(){
        player.translation = final_pos;
        player.translation.z = 1.0;
    }
    
}

fn main() {
    App::new()
        .insert_resource(Map {
            dimensions: Vec2 {
                x: 1000.0,
                y: 1000.0,
            },
        })
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "shooter_2d".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
