/* TODO:
components:
Player(speed, health, size, global_pos, inventory)
gun(range, fire rate, spread, )

Other implimentations:
Map rendering, collisions with objects
*/

/* TODAY'S IMPLIMENTATIONS:
player hands

*/
//USE: $env:WGPU_BACKEND = "vulkan"; cargo run
//use bevy::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;

#[derive(Component, Clone, Copy)]
struct Player {
    speed: f32,
    size: f32,
    hand_size: f32,
    direction: Vec2,
    left_hand: Hand,
    right_hand: Hand,
    body_color: Color,
}

#[derive(Component, Clone, Copy)]
struct Hand{
    offset: Vec2,
    color: Color
}

impl Player {
    fn new_symetric(
        speed: f32,
        size: f32,
        x_offset: f32,
        y_offset: f32,
        body_colour: Color,
        hand_colour: Color
    ) -> Self {
        Player {
            speed: speed,
            size: size,
            hand_size: size / 3.0,
            direction: Vec2::new(0.0, 1.0),
            left_hand: Hand{offset: Vec2::new(-x_offset, y_offset), color: hand_colour},
            right_hand: Hand{offset: Vec2::new(x_offset, y_offset), color: hand_colour},
            body_color: body_colour,
        }
    }
}

#[derive(Component, Clone, Copy)]
struct Rifle {
    length: f32,
    color: Color,
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
                Transform::from_xyz(item.x, item.y, -5.0), //FIX LATER
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
    let rand_coords: Vec2 = Vec2 {
        x: (rng.gen_range(-xrange..xrange)) as f32,
        y: (rng.gen_range(-yrange..yrange)) as f32,
    };
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(rand_coords.x, rand_coords.y, 0.0),
    ));
    let player_color = Color::srgb(0.2, 0.2, 0.2);
    let player_comp = Player::new_symetric(200.0, 30.0, 23.0, 20.0, player_color, Color::BLACK);
    commands
        .spawn((
            player_comp,
            Mesh2d(meshes.add(Circle::new(player_comp.size))), //Circle::new(30.0)
            MeshMaterial2d(materials.add(ColorMaterial::from_color(player_comp.body_color))),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .with_children(|parent| {
            parent.spawn(
                //left hand
                (player_comp.left_hand,
                    Mesh2d(meshes.add(Circle::new(player_comp.hand_size))),
                    MeshMaterial2d(
                        materials.add(ColorMaterial::from_color(player_comp.left_hand.color)),
                    ),
                    Transform::from_xyz(
                        player_comp.left_hand.offset.x,
                        player_comp.left_hand.offset.y,
                        1.0,
                    ),
                ),
            );
            parent.spawn(
                //right hand
                (player_comp.right_hand,
                    Mesh2d(meshes.add(Circle::new(player_comp.hand_size))),
                    MeshMaterial2d(
                        materials.add(ColorMaterial::from_color(player_comp.right_hand.color)),
                    ),
                    Transform::from_xyz(
                        player_comp.right_hand.offset.x,
                        player_comp.right_hand.offset.y,
                        1.0,
                    ),
                ),
            );
        });
}

fn rotate_player(
    mut transform: Query<(&mut Transform, &mut Player)>,
    mut cursor: EventReader<CursorMoved>,
    win_res: Single<&mut Window>,
) {
    let mut cursor_vec = Vec2::ZERO;
    let dimensions: Vec2 = Vec2::new(win_res.width(), win_res.height());
    for event in cursor.read() {
        cursor_vec = event.position;
        cursor_vec.x = cursor_vec.x - (dimensions.x / 2.0);
        cursor_vec.y = (dimensions.y / 2.0) - cursor_vec.y;
    }
    if cursor_vec == Vec2::ZERO {
        return;
    };
    cursor_vec = cursor_vec.normalize();

    for (mut player_transform, mut player) in transform.iter_mut() {
        let player_vec = player.direction.normalize();
        player_transform.rotate(Quat::from_rotation_arc_2d(player_vec, cursor_vec));
        player.direction = cursor_vec;
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
                camera_transform.translation.x += 0.1;
                displacement = camera_transform.translation;
            }
            if (map_dimensions_half.1) <= camera_transform.translation.x {
                camera_transform.translation.x -= 0.1;
                displacement = camera_transform.translation;
            }
            if -(map_dimensions_half.1) >= camera_transform.translation.y {
                camera_transform.translation.y += 0.1;
                displacement = camera_transform.translation;
            }
            if (map_dimensions_half.1) <= camera_transform.translation.y {
                camera_transform.translation.y -= 0.1;
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

fn use_rifle( //so many queries...
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    player_entity: Query<Entity, With<Player>>,
    mut get_hands: Query<&mut Transform, With<Hand>>,
    get_player: Query<&Player>,
    rifle_entity: Query<Entity, With<Rifle>>,
) {
    let new_rifle: Rifle;
    let hand_position1 = Vec2::new(0.0,32.0);
    let hand_position2 = Vec2::new(7.5,65.0);

    //if 1 keybind is pressed: deploy weapon, transform player hands
    if key_inputs.just_pressed(KeyCode::Digit1) && rifle_entity.is_empty() {
        new_rifle = Rifle {
            length: 65.0,
            color: Color::BLACK,
        };
        let player_entity = player_entity.single();
        
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                new_rifle,
                Mesh2d(meshes.add(Capsule2d::new(5.5, new_rifle.length))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(new_rifle.color))),
                Transform::from_xyz(0.0, 55.0, -1.0),
            ));
        });

        //switch hand orientation
        let player_comp = get_player.single();
        for mut hand in get_hands.iter_mut(){
            let current_hands = hand.translation.truncate(); //converts to Vec2
            let z_pos = hand.translation.z;
            //checks if current hand is right hand
            if player_comp.right_hand.offset == current_hands{
                hand.translation = hand_position1.extend(z_pos);
            }
            //checks if current hand is left hand
            if player_comp.left_hand.offset == current_hands{
                hand.translation = hand_position2.extend(z_pos);
            }

           
            
        }
    }
    //if 1 keybind is pressed and weapon is deployed: revert to default transform
    else if (key_inputs.just_pressed(KeyCode::Digit1)) && (!rifle_entity.is_empty()) {
        //despawn entity
        let rifle_despawn = rifle_entity.single();
        commands.entity(rifle_despawn).despawn();

        //reset hand position
        for mut hand in get_hands.iter_mut(){
            let original_position_left = get_player.single().left_hand.offset;
            let original_position_right = get_player.single().right_hand.offset;
            if hand_position1 == hand.translation.truncate(){
                hand.translation = Vec3::new(original_position_left.x, original_position_left.y, hand.translation.z);
            }
            if hand_position2 == hand.translation.truncate(){
                hand.translation = Vec3::new(original_position_right.x, original_position_right.y, hand.translation.z);
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Map::new_square(50.0, 10))
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .add_systems(Update, rotate_player)
        .add_systems(Update, use_rifle)
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
