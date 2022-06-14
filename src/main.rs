#![allow(clippy::all)]
#![allow(warnings)]
// #![feature(slice_take)]

use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::{prelude::*, ecs::event::Events, window::WindowResized};

pub mod physics;
pub mod utils;
pub mod events;
pub mod prelude;
pub mod dialogue;
pub mod character;
pub mod graphics;
pub mod battle;

use prelude::*;


use physics::SyncHitboxSize;
pub use utils::logging::{Logger, logging_system};

use physics::collisions::HitboxBundle;

pub struct Deletor { 
    pub b: bool
}

impl Default for Deletor {
    fn default() -> Self {
        Deletor { 
            b: false
        }
    }
}

fn destroy_map(
    mut commands: Commands,
    mut deletor: ResMut<Deletor>,
    mut query: Query<Entity, With<Map>>,
) { 
    if deletor.b {
        deletor.b = false;
        for (mut el) in query.iter_mut() {
            commands.entity(el).despawn();
        }
    }
}

/// game setup
/// TODO: split into multiple setup functions
/// mostly for testing purposes rn
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, win: Res<WindowDescriptor>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let texture_atlas = TextureAtlas::from_grid(asset_server.load("3-walking.png"), Vec2::new(20., 30.), 2, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(MainCharacter::from(texture_atlas_handle));
    // Text bundle for the text box
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "DEFAULT VALUE",
            TextStyle {
                font: asset_server.load("fonts/detbubble.ttf"),
                font_size: 20.,
                color: Color::WHITE
            },
            TextAlignment { horizontal: HorizontalAlign::Center, ..Default::default() }
        ),
        transform: Transform::from_xyz(0.,-win.height/2.+150., 7.),
        ..Default::default()
    }).insert(Textbox::default());
    // Textbox sprite
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("textbox.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(win.width/1.5, 120.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0., -win.height/2.+120., 6.),
        ..Default::default()
    }).insert(Textbox::default());
    
}
/// prevent window resize from breaking the game
/// TODO change sizes relatively too
pub fn window_size_update(resize: Res<Events<WindowResized>>, mut win: ResMut<WindowDescriptor>, mut trq: Query<(&mut Transform)>) {
    let mut reader = resize.get_reader();
    for e in reader.iter(&resize) {
        for (mut tr) in trq.iter_mut() {
            tr.translation.x = (tr.translation.x/win.width)*e.width;
            tr.translation.y = (tr.translation.y/win.height)*e.height;
        }
        win.width = e.width;
        win.height = e.height;
    }
}

pub fn sync_hitbox_with_sprite(mut q: Query<(&mut physics::HitboxSize, &Sprite, &SyncHitboxSize)>) {
    for (mut hbsize, spr, sync) in q.iter_mut() {
        if sync.sync {
            if let Some(custom_size) = spr.custom_size {
                hbsize.size.width = custom_size[0];
                hbsize.size.height = custom_size[1];
            }
        }
    }
}

pub fn sync_hitbox_with_atlassprite(mut q: Query<(&mut physics::HitboxSize, &TextureAtlasSprite, &SyncHitboxSize)>) {
    for (mut hbsize, spr, sync) in q.iter_mut() {
        if sync.sync {
            if let Some(custom_size) = spr.custom_size {
                hbsize.size.width = custom_size[0];
                hbsize.size.height = custom_size[1];
            } 
        }   
    }
}

/// main function
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .init_resource::<Logger>()
        .init_resource::<Deletor>()
        .init_resource::<SceneUpdater>()
        .init_resource::<Battle>()
        .init_resource::<AssetHandles>()

        .insert_resource(NewTextboxText::new(
            0.1
        ))
        .insert_resource(
            ClearColor(Color::hsla(252., 0.33, 0.05, 1.0))
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(physics::player_movement)
        .add_system(destroy_map)
        .add_system(logging_system)
        .add_system(window_size_update)
        .add_system(sync_hitbox_with_sprite)
        .add_system(sync_textbox_text)
        .add_system(sync_hitbox_with_atlassprite)
        .add_system(sync_textbox_vis)
        .add_system(txb_tick)
        .add_system(oscillate)
        .add_system(graphics::animate_sprite)
        .add_system(graphics::anime_moving_char)
        .add_system(events::player_use_input)
        .add_system(check_bg_change)
        .add_system(spawn_scene_00)
        .add_system(spawn_scene_01)
        .add_system(spawn_battle_scene_00)
        .run();
}