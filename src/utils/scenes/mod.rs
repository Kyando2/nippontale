use bevy::prelude::*;
use crate::prelude::*;

pub mod scene_00;
pub mod scene_01;
pub mod battle_scene_00;

pub struct SceneUpdater {
    pub num: u32,
    pub b: bool,
    pub transitioning: bool,
    // current time (in frames)
    pub current: f32,
    // current time (in frames)
    pub length: f32,
    
    pub transitioned: bool,
}

impl Default for SceneUpdater {
    fn default() -> Self {
        SceneUpdater { num:  0, b: true, transitioning: false, current: 0., length: 240., transitioned: false }
    }
}

pub fn check_bg_change(
    mut commands: Commands,
    mut battle: ResMut<Battle>,
    asset_server: Res<AssetServer>,
    mut q: Query<Entity, With<BG>>,
    screen: Res<WindowDescriptor>,
) {
    if battle.state == 2 && battle.change {
        battle.change = false;
        for e in q.iter_mut() {
            commands.entity(e).despawn();
        };
        let battle_asset = asset_server.load(match battle.choice {
            0 => "0-battle.png",
            1 => "1-choice-fight.png",
            2 => "2-choice-act.png",
            3 => "3-choice-item.png",
            4 => "4-choice-mercy.png",
            5 => "5-battle-in-progress.png",
            _ => "",
        });
        if battle.choice == 0 {
            spawn_image(&mut commands, 0., -100., 1., 550., 180.,asset_server.load("fight-bg.png"));
            spawn_oscillate_bar(&mut commands, -280., -100., 2., 200., 200.,asset_server.load("bar-light.png"));
        }
        spawn_background(&mut commands, &screen, battle_asset.clone());
    }
}

#[derive(Component)]
pub struct Oscillate {
    max: f32,
    min: f32,
    direction: i8
}

pub fn oscillate(mut q: Query<(&mut Transform, &mut Oscillate)>) {
    for (mut tr, mut osc) in q.iter_mut() {
        if tr.translation.x >= osc.max {
            osc.direction = -1;
        } else if tr.translation.x <= osc.min {
            osc.direction = 1;
        }
        tr.translation.x+=osc.direction as f32*10.;
    }
}


pub fn spawn_oscillate_bar(mut commands: &mut Commands, x: f32, y: f32, z: f32, width: f32,height: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: tat,
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        })
        .insert(Oscillate {
            min: x,
            max: -x,
            direction: 1
        })
        .insert(Map {})
        .insert(BG {});
}

#[derive(Debug)]
pub struct AssetHandles {
    handles: Vec<Handle<Image>>,
    scene_saved: u32,
}

impl Default for AssetHandles {
    fn default() -> Self {
        AssetHandles { handles: Vec::new(), scene_saved: 0 }
    }
}

pub fn spawn_savepoint(mut commands: &mut Commands, x: f32, y: f32, tat: Handle<TextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: tat,
            transform: Transform::from_xyz(x, y, 0.),
            sprite: TextureAtlasSprite { custom_size: Some(Vec2::new(64., 64.)), ..Default::default()},
            ..default()
        })
        // deleted as part of the map
        .insert(Map {})
        // HitboxBundle to take care of player - entity collisions
        // this will auto sync with the texture atlas sprite's size
        // so we simply use default.
        .insert(HitboxSize { size: Size { width: 52., height: 52.} , xdelta: 0., ydelta: 0. })
        // save point event marker, marks this entity 
        // as a save point so it can be used as so
        // by the player.
        .insert(events::OnTouch::svpt())
        // animated bundle to animate the spritesheet 
        // changes sprite every (duration)s 
        // and repeats if (repeating) is set to true
        .insert_bundle(graphics::AnimatedBundle::from_seconds(0.3, true));
}

pub fn spawn_loading_zone(mut commands: &mut Commands, x: f32, y: f32, width: f32, height: f32, scene_to: u32, transition: bool) {
    commands
        // spawn the loading zone
        .spawn()
        // inserted a map component so it's destroyed when changing scenes
        .insert(Map {})
        // inserted a transform component for its position
        .insert(Transform::from_xyz((x.abs()+width)*(x/x.abs()), y, -10.))

        .insert(HitboxSize { size: Size {width, height}, xdelta: 0., ydelta: 0.})

        .insert(events::OnTouch::scene(scene_to, transition));
}

pub fn spawn_pass_tile(mut commands: &mut Commands, x: f32, y: f32, z: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
        texture: tat,
        sprite: Sprite {
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(x, y, z),
        ..Default::default()
        })
        // deleted as part of the map
        .insert(Map {});
}

pub fn spawn_pass_big(mut commands: &mut Commands, x: f32, y: f32, z: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
        texture: tat,
        sprite: Sprite {
            custom_size: Some(Vec2::new(128., 128.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(x, y, z),
        ..Default::default()
        })
        // deleted as part of the map
        .insert(Map {});
}


pub fn spawn_wall_tile(mut commands: &mut Commands, x: f32, y: f32, z: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
        texture: tat,
        sprite: Sprite {
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(x, y, z),
        ..Default::default()
        })
        // deleted as part of the map
        .insert(Map {})
        .insert(HitboxSize { size: Size { width: 52., height: 52.}, xdelta: 0., ydelta: 32.});
}

pub fn spawn_screen_cover(mut commands: &mut Commands, screen: &Res<WindowDescriptor>, opacity: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: tat,
            sprite: Sprite {
                custom_size: Some(Vec2::new(screen.width, screen.height)),
                color: Color::rgba(0., 0., 0., opacity),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 10.),
            ..Default::default()
        })
        .insert(Cover {});

}

pub fn spawn_background(mut commands: &mut Commands, screen: &Res<WindowDescriptor>, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: tat,
            sprite: Sprite {
                custom_size: Some(Vec2::new(screen.width, screen.height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(Map {})
        .insert(BG {});
}

pub fn spawn_image(mut commands: &mut Commands, x: f32, y: f32, z: f32, width: f32,height: f32, tat: Handle<Image>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: tat,
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        })
        .insert(Map {})
        .insert(BG {});
}