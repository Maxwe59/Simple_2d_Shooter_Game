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
struct Hand {
    offset: Vec2,
    color: Color,
}

#[derive(Component, Clone, Copy)]
struct Bullet {
    direction: Vec2,
    spawn_point: Vec2,
}

impl Player {
    fn new_symetric(
        speed: f32,
        size: f32,
        x_offset: f32,
        y_offset: f32,
        body_colour: Color,
        hand_colour: Color,
    ) -> Self {
        Player {
            speed: speed,
            size: size,
            hand_size: size / 3.0,
            direction: Vec2::new(0.0, 1.0),
            left_hand: Hand {
                offset: Vec2::new(-x_offset, y_offset),
                color: hand_colour,
            },
            right_hand: Hand {
                offset: Vec2::new(x_offset, y_offset),
                color: hand_colour,
            },
            body_color: body_colour,
        }
    }
}

#[derive(Component, Clone, Copy)]
struct Rifle {
    length: f32,
    radius: f32,
    color: Color,
    y_offset: f32,
    hand1: Vec2,
    hand2: Vec2,

    //for animation purposes
    recoil_direction: bool,
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
                (
                    player_comp.left_hand,
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
                (
                    player_comp.right_hand,
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

//unfinished function. fix by making rifle child of hands, transforming hands
fn shoot_rifle(
    mut rifle_transform: Query<(&mut Transform, &mut Rifle)>,
    rifle_entity: Query<Entity, With<Rifle>>,
    hand_entities: Query<Entity, With<Hand>>,
    player_entity: Query<Entity, With<Player>>,
    keyboard_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    //checks if no rifle is being spawned. if a rifle is spawned, play the animation
    if !rifle_transform.is_empty() && keyboard_input.pressed(MouseButton::Left) {
        //add hands as child of rifle so hands transform with rifle movement.

        for hand in hand_entities.iter() {
            commands.entity(rifle_entity.single()).add_child(hand);
        }

        let mut rifle = rifle_transform.single_mut();
        let fire_speed = 250.0;
        if rifle.1.y_offset <= rifle.0.translation.y {
            rifle.1.recoil_direction = false;
        } else if rifle.0.translation.y <= 50.0 {
            //change this number later
            rifle.1.recoil_direction = true;
        }

        if !rifle.1.recoil_direction {
            rifle.0.translation.y -= fire_speed * time.delta_secs(); //recoil in y direction
        } else if rifle.1.recoil_direction {
            //change value later
            rifle.0.translation.y += fire_speed * time.delta_secs();
        }
    }

    if keyboard_input.just_released(MouseButton::Left) && !rifle_transform.is_empty() {
        //remove hands as children of rifle1

        commands.entity(rifle_entity.single()).clear_children();
        if !hand_entities.is_empty() {
            for hand in hand_entities.iter() {
                commands.entity(player_entity.single()).add_child(hand);
            }
        }

        let mut rifle = rifle_transform.single_mut();
        //reset rifle position
        rifle.0.translation = Vec3::new(
            rifle.0.translation.x,
            rifle.1.y_offset,
            rifle.0.translation.z,
        );
    }
}

fn equip_rifle(
    //so many queries...
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    player_entity: Query<Entity, With<Player>>,
    mut get_hands: Query<&mut Transform, With<Hand>>,
    get_player: Query<&Player>,
    get_rifle: Query<&Rifle>,
    rifle_entity: Query<Entity, With<Rifle>>,
) {
    let new_rifle: Rifle;

    //if 1 keybind is pressed: deploy weapon, transform player hands
    if key_inputs.just_pressed(KeyCode::Digit1) && rifle_entity.is_empty() {
        new_rifle = Rifle {
            radius: 5.5,
            length: 65.0,
            y_offset: 55.0,
            color: Color::BLACK,
            hand1: Vec2::new(0.0, 32.0),
            hand2: Vec2::new(7.5, 65.0),
            recoil_direction: false,
        };
        let player_entity = player_entity.single();

        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                new_rifle,
                Mesh2d(meshes.add(Capsule2d::new(new_rifle.radius, new_rifle.length))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(new_rifle.color))),
                Transform::from_xyz(0.0, new_rifle.y_offset, -1.0),
            ));
        });

        //switch hand orientation
        let player_comp = get_player.single();
        for mut hand in get_hands.iter_mut() {
            let current_hands = hand.translation.truncate(); //converts to Vec2
            let z_pos = hand.translation.z;
            //checks if current hand is right hand
            if player_comp.right_hand.offset == current_hands {
                hand.translation = new_rifle.hand1.extend(z_pos);
            }
            //checks if current hand is left hand
            if player_comp.left_hand.offset == current_hands {
                hand.translation = new_rifle.hand2.extend(z_pos);
            }
        }
    }
    //if 1 keybind is pressed and weapon is deployed: revert to default transform
    else if (key_inputs.just_pressed(KeyCode::Digit1)) && (!rifle_entity.is_empty()) {
        //despawn entity
        let rifle_despawn = rifle_entity.single();
        commands.entity(rifle_despawn).despawn();

        //reset hand position
        let rifle_comp = get_rifle.single();
        for mut hand in get_hands.iter_mut() {
            if rifle_comp.hand1 == hand.translation.truncate() {
                let original_position_left = get_player.single().left_hand.offset;
                hand.translation = original_position_left.extend(hand.translation.z);
            }
            if rifle_comp.hand2 == hand.translation.truncate() {
                let original_position_right = get_player.single().right_hand.offset;
                hand.translation = original_position_right.extend(hand.translation.z);
            }
        }
    }
}

/*
TODO:
add bullet drag animation
add delay to firing/different fire modes
add flash effect (with bloom)
 */
fn spawn_bullets(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    key_inputs: Res<ButtonInput<MouseButton>>,
    rifle: Query<&Rifle>,
    time: Res<Time>,
    player: Query<&Player>,
    mut transform_set: ParamSet<(
        Query<(Entity, &mut Transform, &Bullet)>,
        Query<&mut Transform, With<Player>>,
    )>,
) {
    let bullet_spread = 5.0; //angle of spread (from normal line to farthest angle, so half of full angle)
    let bullet_radius = 5.0;
    let bullet_range: f32 = 450.0;
    let bullet_speed: f32 = 600.0;

    let player_direction = player.single().direction;
    let player_position = transform_set.p1().single().translation;
    //calculates bullet's spread, based on randomized angle given above
    let mut rng = rand::thread_rng();
    let mut rand_angle: f32 = rng.gen_range(-bullet_spread..bullet_spread);
    rand_angle = rand_angle.to_radians();
    let mut direction_with_spread = player_direction.normalize();
    direction_with_spread.x =
        (direction_with_spread.x * rand_angle.cos()) - (direction_with_spread.y * rand_angle.sin());
    direction_with_spread.y =
        (direction_with_spread.x * rand_angle.sin()) + (direction_with_spread.y * rand_angle.cos());

    //main logic for bullets, when left click is pressed
    if key_inputs.pressed(MouseButton::Left) && !rifle.is_empty() {
        let rifle_info = rifle.single();
        let scaled_direction = direction_with_spread * (rifle_info.length + (rifle_info.y_offset / 2.0));
        let bullet_component = Bullet {
            direction: direction_with_spread.normalize(),
            spawn_point: Vec2::new(
                player_position.x + scaled_direction.x,
                player_position.y + scaled_direction.y,
            ),
        };
        //single shot
        let bullet_bundle = (
            bullet_component,
            Mesh2d(meshes.add(Circle::new(bullet_radius))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Color::srgba_u8(0, 0, 0, 255))),
            ),
            Transform::from_xyz(
                bullet_component.spawn_point.x,
                bullet_component.spawn_point.y,
                0.0,
            ),
        );
        commands.spawn(bullet_bundle);
    }

    for (bullet_entity, mut bullet_transform, bullet) in transform_set.p0().iter_mut() {
        //launch bullet in player_direction vec
        bullet_transform.translation.x += bullet_speed * bullet.direction.x * time.delta_secs();
        bullet_transform.translation.y += bullet_speed * bullet.direction.y * time.delta_secs();

        //calculate the bullet's distance from its spawning point
        let x_displacement = bullet_transform.translation.x - bullet.spawn_point.x;
        let y_displacement = bullet_transform.translation.y - bullet.spawn_point.y;
        let distance_from_spawn =
            ((x_displacement * x_displacement) + (y_displacement * y_displacement)).sqrt();
        if distance_from_spawn >= bullet_range {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Map::new_square(250.0, 2))
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_player)
        .add_systems(Update, rotate_player)
        .add_systems(Update, equip_rifle)
        .add_systems(Update, spawn_bullets)
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
