use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Savepoint;

pub fn player_use_input(keys: Res<Input<KeyCode>>, mut q: Query<(&PlayerControlled, &mut Touching)>, mut q2: Query<(&mut Textbox, &mut Visibility, Option<&mut Text>)>, mut logger: ResMut<Logger>, mut ntt: ResMut<NewTextboxText>) {
    for (ply, mut tch) in q.iter_mut() {
        if ply.controlled && tch.savepoint && keys.just_pressed(KeyCode::E) && !tch.in_scene {
            tch.savepoint = false;
            tch.in_scene = true;
            ntt.new_text("You are filled with pride and honor!", 40.);
            ntt.text = String::from("");
            for (mut txb, mut vis, mut txt) in q2.iter_mut() {
                txb.active = true;
                vis.is_visible = txb.active;
                if let Some(mut tx) = txt {
                    tx.sections[0].value = String::from("");
                } 
            }
        } else if keys.just_pressed(KeyCode::E) && ntt.complete == ntt.text && tch.in_scene {
            for (mut txb, mut vis, txt) in q2.iter_mut() {
                txb.active = false;
                vis.is_visible = txb.active;
            }
            ntt.is_done = false;
            tch.in_scene = false;
        } else if keys.just_pressed(KeyCode::E) {
            ntt.text = ntt.complete.clone();
            ntt.i = ntt.complete.len();
            ntt.is_done = true;
        }
    }
}