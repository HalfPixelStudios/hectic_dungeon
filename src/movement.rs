use bevy::prelude::*;

use crate::{
    animation::AniState,
    enemy::Enemy,
    grid::{snap_to_grid, to_world_coords, GridPosition},
};

const THRESHOLD: f32 = 0.01;

pub fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}

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
        return Movement {
            next_move: IVec2::ZERO,
            frame: 0.,
        };
    }
}

//time based lerp; make mv.frame/t = 1 where t = time to move between squares
fn movement(mut query: Query<(&mut GridPosition, &mut Movement, &mut Transform)>) {
    for (mut grid_pos, mut mv, mut transform) in query.iter_mut() {
        if mv.next_move == IVec2::ZERO {
            return;
        }
        let next_pos = to_world_coords(&(grid_pos.pos() + mv.next_move));
        let cur_pos = transform.translation.truncate();
        if cur_pos.distance(next_pos) > THRESHOLD {
            transform.translation.x = lerp(transform.translation.x, next_pos.x, mv.frame / 60.);
            transform.translation.y = lerp(transform.translation.y, next_pos.y, mv.frame / 60.);
            mv.frame += 1.;
        } else {
            mv.frame = 0.;
            transform.translation = next_pos.extend(transform.translation.z);
            grid_pos.move_relative(&mv.next_move);
            mv.next_move = IVec2::ZERO;
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement);
    }
}
