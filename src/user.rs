use crate::map;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone, Copy)]
pub struct Player {
    speed: f32,
    size: f32,
    hand_size: f32,
    body_color: Color,
    pub direction: Vec2,
    pub left_hand: Hand,
    pub right_hand: Hand,
}

impl Player {
    fn new_symetric() -> Self {
        let size = 30.0;
        let x_offset = 23.0;
        let y_offset = 20.0;
        let hand_colour = Color::BLACK;
        let body_colour = Color::srgb(0.2, 0.2, 0.2);
        Player {
            speed: 200.0,
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
    //Make setter methods for changing player stats (speed, size)

    fn fists_mode(&mut self, mut hand_transform: Query<&mut Transform, With<Hand>>) {
        self.direction = self.direction; //do not change
        self.speed = 200.0;
        self.size = 30.0;
        self.hand_size = self.size / 3.0;
        self.body_color = Color::srgb(0.2, 0.2, 0.2);
        self.left_hand = Hand {
            offset: Vec2::new(-23.0, 20.0),
            color: Color::BLACK,
        };
        self.right_hand = Hand {
            offset: Vec2::new(23.0, 20.0),
            color: Color::BLACK,
        };

        for (index, mut hand) in hand_transform.iter_mut().enumerate() {
            hand.translation = if (index == 0) {
                self.left_hand.offset.extend(1.0)
            } else {
                self.right_hand.offset.extend(1.0)
            };
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Hand {
    pub offset: Vec2,
    color: Color,
}

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    map: Res<map::Map>,
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
    let player_comp = Player::new_symetric();
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

pub fn rotate_player(
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

pub fn move_player(
    mut transform: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&mut Transform, With<Player>>,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Res<map::Map>,
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
