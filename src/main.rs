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
//use bevy::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use rand::Rng;

#[derive(Component)]
struct Player {
    speed: f32,
    direction: Vec2,
}

#[derive(Resource)]
struct Map {
    grid_size: Vec2, //size of individual grid on the map
    //partition -> how many rects are in a row, number of rows
    partition: IVec2, //amount of individual grids on the map. 1 BASED COUNTING
    //create a matrix for the map, stores coords for each rectangle
    grid: Vec<Vec<Vec2>>, //[row][collumn]
    dimensions: (f32, f32),
}

impl Map {
    fn new_square(grid_length: f32, partition_sqre: i32) -> Self {
        Map {
            grid_size: Vec2 {
                x: grid_length,
                y: grid_length,
            },
            grid: Vec::new(),
            partition: IVec2 {
                x: partition_sqre,
                y: partition_sqre,
            },
            dimensions: (
                (grid_length * partition_sqre as f32),
                (grid_length * partition_sqre as f32),
            ),
        }
    }

    fn generate_grid(&mut self) {
        let mut tile_placement = Vec2 {
            x: -((self.dimensions.0 - self.grid_size.x) / 2.0),
            y: ((self.dimensions.1 - self.grid_size.y) / 2.0),
        };
        for _row in 0..self.partition.y {
            let mut temp_vec: Vec<Vec2> = Vec::new();
            for _collumn in 0..self.partition.x {
                temp_vec.push(tile_placement);
                tile_placement.x += self.grid_size.x;
            }
            self.grid.push(temp_vec);
            tile_placement.y -= self.grid_size.y;
            tile_placement.x = -((self.dimensions.0 - self.grid_size.x) / 2.0);
        }
        //generates one single row of the grid
        //should populate self.grid with coordinates for each rectangle
    }
}

//spawn map
fn spawn_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut map: ResMut<Map>,
) {
    map.generate_grid();
    for row in &map.grid {
        for item in row {
            commands.spawn((
                //spawns in a single block from the map grid
                Mesh2d(meshes.add(Rectangle::new(map.grid_size.x, map.grid_size.y))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
                Transform::from_xyz(item.x, item.y, 0.0), //FIX LATER
            ));
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    map: Res<Map>,
) {
    //random coord generation
    let mut rng = rand::thread_rng();
    let xrange = (map.dimensions.0 / 2.0) as i32;
    let yrange = (map.dimensions.1 / 2.0) as i32;
    let rand_coords: Vec3 = Vec3 {
        x: (rng.gen_range(-xrange..xrange)) as f32,
        y: (rng.gen_range(-yrange..yrange)) as f32,
        z: 1.0,
    };
    //coords player will spawn at
    let player_global_pos = rand_coords;
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(player_global_pos.x, player_global_pos.y, 0.0),
    ));
    commands.spawn((
        Player {
            speed: 200.0,
            direction: Vec2::new(1.0,1.0).normalize(),
        },
        Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))), //radius: 30
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::BLACK))),
        Transform {
            translation: player_global_pos,
            ..Default::default()
        },
    ));
}

fn rotate_player(
    mut transform: Query<(&mut Transform, &mut Player)>,
    mut cursor: EventReader<CursorMoved>,
) {
    let mut rotation_vec = Vec2::ZERO;
    for event in cursor.read() {
        rotation_vec = event.position.normalize();
    }
    for (mut player_transform, mut player) in transform.iter_mut(){
        let player_direction = player.direction;
        player_transform.rotate(Quat::from_rotation_arc_2d(player_direction, rotation_vec));
        player.direction = rotation_vec;
    }

}

fn move_player(
    mut transform: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&mut Transform, With<Player>>,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Res<Map>,
    player: Query<&Player>,
) {
    //get player speed
    let mut speed: f32 = 0.0;
    for item in player.iter() {
        speed = item.speed;
    }

    //displacement calculations
    let mut displacement: Vec3 = Vec3::ZERO;
    let map_dimensions_half = (map.dimensions.0 / 2.0, map.dimensions.1 / 2.0);

    if keyboard_input.pressed(KeyCode::KeyD) {
        displacement.x += speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        displacement.x -= speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        displacement.y += speed * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        displacement.y -= speed * time.delta_secs();
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
            if -(map_dimensions_half.1) >= camera_transform.translation.x {
                camera_transform.translation.x += 0.01;
                displacement = camera_transform.translation;
            }
            if (map_dimensions_half.1) <= camera_transform.translation.x {
                camera_transform.translation.x -= 0.01;
                displacement = camera_transform.translation;
            }
            if -(map_dimensions_half.1) >= camera_transform.translation.y {
                camera_transform.translation.y += 0.01;
                displacement = camera_transform.translation;
            }
            if (map_dimensions_half.1) <= camera_transform.translation.y {
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
}

fn main() {
    App::new()
        .insert_resource(Map::new_square(50.0, 10))
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .add_systems(Update, rotate_player)
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
