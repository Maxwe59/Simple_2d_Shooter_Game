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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "shooter_2d".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
