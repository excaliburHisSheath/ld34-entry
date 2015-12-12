#![feature(augmented_assignments)]

extern crate gunship;

use gunship::*;
use std::collections::HashMap;

pub fn do_main() {
    let mut engine = Engine::new();
    game_init(&mut engine);
    engine.main_loop();
}

macro_rules! game_setup {
    (
        setup: $setup:ident,

        managers:
        $($manager:ty => $manager_instance:expr),*

        systems:
        $($system_instance:expr),*

        models:
        $($model:expr),*
    ) => {
        #[no_mangle]
        pub fn game_init(engine: &mut Engine) {
            $(engine.scene_mut().register_manager($manager_instance);)*
            $(engine.register_system($system_instance);)*

            $(engine.scene().resource_manager().load_resource_file($model).unwrap();)*

            $setup(engine.scene_mut());
        }

        #[no_mangle]
        pub fn game_reload(old_engine: &Engine, engine: &mut Engine) {
            $(engine.scene_mut().reload_manager::<$manager>(old_engine.scene());)*
            $(engine.register_system($system_instance);)*
        }
    }
}

game_setup! {
    setup: scene_setup,

    managers:
        GameManager => GameManager::new(GameData {
            grid: HashMap::new(),
            selected: GridPos::new(0, 0),
            cursor: Point::new(0.0, 0.0, 0.0),
        })

    systems:
        manager_update

    models:
        "meshes/cube.dae",
        "meshes/sphere.dae"
}

fn scene_setup(scene: &Scene) {
    let mut transform_manager = scene.get_manager_mut::<TransformManager>();
    let camera_manager = scene.get_manager::<CameraManager>();

    // Create camera.
    let camera_entity = scene.create_entity();
    {
        let mut camera_transform = transform_manager.assign(camera_entity);
        camera_transform.set_position(Point::new(0.0, 0.0, 30.0));
        camera_transform.look_at(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

        camera_manager.assign(camera_entity, Camera::default());
    }


}

const CELL_SIZE: f32 = 5.0;
const MOUSE_SPEED: f32 = 0.1;

type GameManager = SingletonComponentManager<GameData>;

#[derive(Debug, Clone)]
pub struct GameData {
    /// A map between a grid coordinate and its contents.
    grid: HashMap<GridPos, Entity>,

    /// The grid cell currently selected by the player.
    selected: GridPos,

    /// The position in world space of the game cursor. Used to move the selected grid cell in
    /// discrete increments.
    cursor: Point,
}

/// Represents a coordinate in the the 2D game grid.
///
/// The game grid is oriented along the global x-y plane, with positive z being up. A grid
/// coordinate represents the minimum point of the cell, so a grid pos (5, 3) represents the space
/// between (5.0, 3.0) and (6.0, 4.0) if the grid cells are 1.0x1.0.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GridPos {
    x: isize,
    y: isize,
}

impl GridPos {
    fn new(x: isize, y: isize) -> GridPos {
        GridPos {
            x: x,
            y: y,
        }
    }

    fn from_world(point: Point) -> GridPos {
        GridPos {
            x: (point.x / CELL_SIZE).floor() as isize,
            y: (point.y / CELL_SIZE).floor() as isize,
        }
    }

    fn to_world(&self) -> Point {
        Point::new(self.x as f32 * CELL_SIZE, self.y as f32 * CELL_SIZE, 0.0)
    }
}

fn manager_update(scene: &Scene, delta: f32) {
    let mut game_manager = scene.get_manager_mut::<GameManager>();

    // Handle mouse movement to move the cursor and selected grid cell.
    {
        let (mouse_x, mouse_y) = scene.input.mouse_delta();
        game_manager.cursor += Vector3::new(mouse_x as f32 * MOUSE_SPEED, -mouse_y as f32 * MOUSE_SPEED, 0.0);
        game_manager.selected = GridPos::from_world(game_manager.cursor);
    }

    // Visualize cursor.
    debug_draw::sphere(game_manager.cursor, 0.25);

    // Draw the selected world point.
    // TODO: Do this with a real wireframe or something, not debug drawing.
    {
        let min = game_manager.selected.to_world();
        let max = min + Vector3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE);

        debug_draw::box_min_max(min, max);
    }
}
