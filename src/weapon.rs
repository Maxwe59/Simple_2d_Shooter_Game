use crate::user;
use bevy::prelude::*;
use rand::Rng;
use bevy::utils::Duration;

#[derive(Component, Clone, Copy)]
pub struct Rifle {
    //rifle display settings
    length: f32,
    radius: f32,
    color: Color,
    y_offset: f32,
    hand1: Vec2,
    hand2: Vec2,

    //rifle stats settings
    bullet_spread: f32, //angle of spread (from normal line to farthest angle, so half of full angle)
    bullet_radius: f32,
    bullet_range: f32,
    bullet_speed: f32,
    fire_rate: f32, //lower value is faster fire rate, measured in delay between shots (sec)

    //for animation purposes
    recoil_direction: bool,
}
#[derive(Component)]
pub struct FireRateTimer(Timer);

impl FireRateTimer{
    fn new(shoot_delay: f32)->Self{
        return FireRateTimer(Timer::new(Duration::from_secs_f32(shoot_delay), TimerMode::Repeating));
    }
}

#[derive(Component, Clone, Copy)]
pub struct Bullet {
    direction: Vec2,
    spawn_point: Vec2,
}

//spread angle is angle from normal vector, (represents half the total spread angle)
fn impl_spread(direction: Vec2, spread_angle: f32) -> Vec2 {
    //randomizes angle for bullet spread
    let mut rng = rand::thread_rng();
    let mut rand_angle: f32 = rng.gen_range(-spread_angle..spread_angle);
    rand_angle = rand_angle.to_radians();

    let mut direction_with_spread = direction.normalize();
    direction_with_spread.x =
        (direction_with_spread.x * rand_angle.cos()) - (direction_with_spread.y * rand_angle.sin());
    direction_with_spread.y =
        (direction_with_spread.x * rand_angle.sin()) + (direction_with_spread.y * rand_angle.cos());

    return direction_with_spread;
}

/*
TODO:
add bullet drag animation
add delay to firing/different fire modes
add flash effect (with bloom)
 */
pub fn spawn_bullets(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    key_inputs: Res<ButtonInput<MouseButton>>,
    rifle: Query<&Rifle>,
    time: Res<Time>,
    mut fire_rate: Query<&mut FireRateTimer>,
    player: Query<&user::Player>,
    mut transform_set: ParamSet<(
        Query<(Entity, &mut Transform, &Bullet)>,
        Query<&mut Transform, With<user::Player>>,
    )>,
) {
    //reset the timer once the rifle is despawned as well
    if key_inputs.pressed(MouseButton::Left) && !rifle.is_empty() && !fire_rate.is_empty(){
        fire_rate.single_mut().0.tick(time.delta());
    }

    //main logic for bullets, when left click is pressed
    if key_inputs.pressed(MouseButton::Left) && !rifle.is_empty() && fire_rate.single().0.just_finished(){
                
        let rifle_stats = rifle.single();
        let player_direction = player.single().direction.normalize();
        let player_position = transform_set.p1().single().translation;
        let direction_with_spread = impl_spread(player_direction, rifle_stats.bullet_spread);

        let scaled_direction =
            direction_with_spread * (rifle_stats.length + (rifle_stats.y_offset / 2.0));
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
            Mesh2d(meshes.add(Circle::new(rifle_stats.bullet_radius))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgba_u8(0, 0, 0, 255)))),
            Transform::from_xyz(
                bullet_component.spawn_point.x,
                bullet_component.spawn_point.y,
                0.0,
            ),
        );
        commands.spawn(bullet_bundle);
    }

    for (bullet_entity, mut bullet_transform, bullet) in transform_set.p0().iter_mut() {
        let rifle_stats = rifle.single();
        //launch bullet in player_direction vec
        bullet_transform.translation.x += rifle_stats.bullet_speed * bullet.direction.x * time.delta_secs();
        bullet_transform.translation.y += rifle_stats.bullet_speed * bullet.direction.y * time.delta_secs();

        //calculate the bullet's distance from its spawning point
        let x_displacement = bullet_transform.translation.x - bullet.spawn_point.x;
        let y_displacement = bullet_transform.translation.y - bullet.spawn_point.y;
        let distance_from_spawn =
            ((x_displacement * x_displacement) + (y_displacement * y_displacement)).sqrt();
        if distance_from_spawn >= rifle_stats.bullet_range {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn equip_rifle(
    //so many queries...
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    key_inputs: Res<ButtonInput<KeyCode>>,
    player_entity: Query<Entity, With<user::Player>>,
    mut get_hands: Query<&mut Transform, With<user::Hand>>,
    get_player: Query<&user::Player>,
    get_rifle: Query<&Rifle>,
    rifle_entity: Query<Entity, With<Rifle>>,
    mut fire_rate: Query<&mut FireRateTimer>
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
            bullet_spread: 2.0,
            bullet_radius: 5.0, 
            bullet_range: 450.0,
            bullet_speed: 1000.0,
            fire_rate: 0.15
        };
        let player_entity = player_entity.single();

        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                new_rifle,
                FireRateTimer::new(new_rifle.fire_rate),
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

        //reset rifle firerate timer
        fire_rate.single_mut().0.reset();

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

//unfinished function. fix by making rifle child of hands, transforming hands
fn shoot_rifle(
    mut rifle_transform: Query<(&mut Transform, &mut Rifle)>,
    rifle_entity: Query<Entity, With<Rifle>>,
    hand_entities: Query<Entity, With<user::Hand>>,
    player_entity: Query<Entity, With<user::Player>>,
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
