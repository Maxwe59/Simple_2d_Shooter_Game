use crate::user;
use bevy::prelude::*;
use bevy::utils::Duration;
use rand::Rng;

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

impl Rifle{
    fn assault_rifle()->Self{
        return Rifle {
            radius: 5.5,
            length: 65.0,
            y_offset: 55.0,
            color: Color::BLACK,
            hand1: Vec2::new(0.0, 32.0),
            hand2: Vec2::new(7.5, 65.0),
            recoil_direction: false,
            bullet_spread: 4.0,
            bullet_radius: 5.0,
            bullet_range: 1000.0,
            bullet_speed: 1000.0,
            fire_rate: 0.09
        };
    }
}
#[derive(Component)]
pub struct FireRateTimer(Timer);


impl FireRateTimer {
    fn new(shoot_delay: f32) -> Self {
        let mut temp_timer = Timer::new(
            Duration::from_secs_f32(shoot_delay),
            TimerMode::Once,
        );
        temp_timer.set_elapsed(Duration::from_secs_f32(shoot_delay));

        return FireRateTimer(temp_timer);
    }

}

#[derive(Component, Clone, Copy)]
pub struct Bullet {
    direction: Vec2,
    spawn_point: Vec2,
    range: f32,
    speed: f32,
    radius: f32,
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
    if !rifle.is_empty() && !fire_rate.is_empty() {
        fire_rate.single_mut().0.tick(time.delta());
    }

    //main logic for bullets, when left click is pressed
    if key_inputs.pressed(MouseButton::Left)
        && !rifle.is_empty() && fire_rate.single().0.finished()
    {
        fire_rate.single_mut().0.reset();
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
            range: rifle_stats.bullet_range,
            speed: rifle_stats.bullet_speed,
            radius: rifle_stats.radius,
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
        //launch bullet in player_direction vec
        bullet_transform.translation.x += bullet.speed * bullet.direction.x * time.delta_secs();
        bullet_transform.translation.y += bullet.speed * bullet.direction.y * time.delta_secs();

        //calculate the bullet's distance from its spawning point
        let x_displacement = bullet_transform.translation.x - bullet.spawn_point.x;
        let y_displacement = bullet_transform.translation.y - bullet.spawn_point.y;
        let distance_from_spawn =
            ((x_displacement * x_displacement) + (y_displacement * y_displacement)).sqrt();
        if distance_from_spawn >= bullet.range {
            commands.entity(bullet_entity).despawn_recursive();
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
    mut get_player: Query<&mut user::Player>,
    get_rifle: Query<&Rifle>,
    rifle_entity: Query<Entity, With<Rifle>>,
    mut fire_rate: Query<&mut FireRateTimer>,
) {
    let new_rifle: Rifle;

    //if 1 keybind is pressed: deploy weapon, transform player hands
    if key_inputs.just_pressed(KeyCode::Digit1) && rifle_entity.is_empty() {
        new_rifle = Rifle::assault_rifle();
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
        
        let mut player_comp = get_player.single_mut();
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
        get_player.single_mut().fists_mode(get_hands);
    }
}


pub fn bullet_drag(
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    //spawn bullet pieces only under certain conditions
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
        let x_translation = bullet_transform.translation.x;
        let y_translation = bullet_transform.translation.y;
        if x_translation >= bullet.spawn_point.x - 1.0
            && 1.0 + bullet.spawn_point.x >= x_translation &&
            y_translation >= bullet.spawn_point.y - 1.0
            && 1.0 + bullet.spawn_point.y >= y_translation
        {
            let max_drag = 80;
            for i in 1..max_drag-1 {
                let drag_pos =  - ((i as f32) * 2.0 * bullet.direction.normalize());
                let drag_piece_bundle = (
                    Mesh2d(meshes.add(Circle::new(bullet.radius))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgba(
                        0.0,
                        0.0,
                        0.0,
                        1.0/(i as f32), //(((i as f32)/(max_drag as f32)))
                    )))),
                    Transform::from_xyz(drag_pos.x, drag_pos.y, 0.0), 
                );
                commands.entity(bullet_entity).with_child(drag_piece_bundle);
            }
        }
        
    }

    
}

//unfinished function. fix by making rifle child of hands, transforming hands
pub fn shoot_rifle(
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


