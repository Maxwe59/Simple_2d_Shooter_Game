use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    grid_size: Vec2, //size of individual grid on the map
    //partition -> how many rects are in a row, number of rows
    partition: IVec2, //amount of individual grids on the map. 1 BASED COUNTING
    //create a matrix for the map, stores coords for each rectangle
    grid: Vec<Vec<Vec2>>, //[row][collumn]
    pub dimensions: (f32, f32),
}

impl Map {
    pub fn new_square(grid_length: f32, partition_sqre: i32) -> Self {
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
pub fn spawn_map(
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
