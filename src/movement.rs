use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{prelude::*, utils::lerp};

const THRESHOLD: f32 = 0.001;

// store grid pos, or next move?
#[derive(Component)]
pub struct Movement {
    // the move dir, 0 if nothing
    pub next_move: IVec2,
    // TODO: not very nice that animation logic is bunched together with movement
    pub frame: f32,
}

impl Movement {
    pub fn new() -> Self {
        Movement {
            next_move: IVec2::ZERO,
            frame: 0.,
        }
    }
}

//time based lerp; make mv.frame/t = 1 where t = time to move between squares
fn movement(mut query: Query<(&mut GridEntity, &mut Movement, &mut Transform)>) {
    for (mut grid_entity, mut mv, mut transform) in &mut query {
        if mv.next_move != IVec2::ZERO {
            grid_entity.pos += mv.next_move;
            mv.next_move = IVec2::ZERO;
        }
        let next_pos = to_world_coords(&grid_entity.pos);
        let cur_pos = transform.translation.truncate();
        if cur_pos.distance(next_pos) > THRESHOLD {
            transform.translation.x = lerp(transform.translation.x, next_pos.x, mv.frame / 60.);
            transform.translation.y = lerp(transform.translation.y, next_pos.y, mv.frame / 60.);
            mv.frame += 1.;
            grid_entity.pos += mv.next_move;
        } else {
            mv.frame = 0.;
            transform.translation = next_pos.extend(transform.translation.z);
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ScreenState::Ingame)
                .with_system(movement)
                .into(),
        );
    }
}
