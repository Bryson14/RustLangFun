#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const MAX_ENEMIES: u32 = 8;
const MAX_FORMATION_MEMBERS: u32 = 2;
const PLAYER_RESPAWN_DELAY: f64 = 1.5;

// resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
}

struct WinSize {
    w: f32,
    h: f32,
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders!".to_string(),
            width: 600.00,
            height: 680.00,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // creating camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window
    let mut window = windows.get_primary_mut().unwrap();
    // window.set_position(IVec2::new(3870, 4830));

    // insert window size into a resource
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // creates the resource once and loads it into bevy engine
    // better for performance
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
    })
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    // spawn main character
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(SpriteBundle {
        material: materials.player_materials.clone(),
        transform: Transform {
            // bottom plus half the height of ferris plus some padding
            translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
