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
struct Player {
    global_pos: Vec2,
}

#[derive(Resource)]
struct Map {
    dimensions: Vec2,
    //create a matrix for the map
}

struct Object {}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let player_global_pos = Vec2::ZERO;
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(player_global_pos.x, player_global_pos.y, 0.0),
    ));
    commands.spawn((
        Player {
            global_pos: player_global_pos,
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
    mut transform: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&mut Transform, With<Player>>,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Player>,
    map: Res<Map>,
) {
    let speed: f32 = 200.0;
    let mut displacement: Vec3 = Vec3::ZERO;
    let map_dimensions_half = (map.dimensions.x / 2.0, map.dimensions.y / 2.0);

    if keyboard_input.pressed(KeyCode::KeyD) {
        displacement.x += (speed * time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        displacement.x -= (speed * time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        displacement.y += (speed * time.delta_secs());
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        displacement.y -= (speed * time.delta_secs());
    }

    //displace camera from keyboard inputs
    for mut camera_transform in transform.p0().iter_mut() {
        //check if player is at the edge of map

        //this code is so ugly. please fix this later.
        if (-(map_dimensions_half.0) >= camera_transform.translation.x)
            || ((map_dimensions_half.0) <= camera_transform.translation.x)
            || (-(map_dimensions_half.1) >= camera_transform.translation.y)
            || ((map_dimensions_half.1) <= camera_transform.translation.y)
        {
            if (-(map_dimensions_half.1) >= camera_transform.translation.x) {
                camera_transform.translation.x += 0.01;
                displacement = camera_transform.translation;
            }
            if ((map_dimensions_half.1) <= camera_transform.translation.x) {
                camera_transform.translation.x -= 0.01;
                displacement = camera_transform.translation;
            }
            if (-(map_dimensions_half.1) >= camera_transform.translation.y) {
                camera_transform.translation.y += 0.01;
                displacement = camera_transform.translation;
            }
            if ((map_dimensions_half.1) <= camera_transform.translation.y) {
                camera_transform.translation.y -= 0.01;
                displacement = camera_transform.translation;
            }
        } else {
            camera_transform.translation += displacement;
            displacement = camera_transform.translation;
        }
    }
    //displace player with camera displacement
    for mut player_transform in transform.p1().iter_mut() {
        player_transform.translation = displacement;
        player_transform.translation.z = 1.0;
    }

    //update global player pos
    for mut player_inst in player.iter_mut() {
        player_inst.global_pos = Vec2 {
            x: displacement.x,
            y: displacement.y,
        };
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
