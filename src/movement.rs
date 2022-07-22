use bevy::prelude::*;

use crate::{grid::{GridPosition, to_world_coords, snap_to_grid}, animation::AniState, enemy::Enemy};

const THRESHOLD:f32 = 0.01;

    
pub fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}

// store grid pos, or next move?
#[derive(Component)]
pub struct Movement{
    // the move dir, 0 if nothing
    pub next_move: IVec2,
    pub frame: f32
    
}
//time based lerp; make mv.frame/t = 1 where t = time to move between squares
fn movement(mut query:Query<(&mut GridPosition, &mut Movement, &mut Transform)>){
    for (mut grid_pos, mut mv, mut transform) in query.iter_mut(){

        
        if mv.next_move == IVec2::ZERO{
            return 
        }
        let next_pos = to_world_coords(&(grid_pos.0+mv.next_move));
        let cur_pos = transform.translation.truncate();
        if cur_pos.distance(next_pos)>THRESHOLD{
            transform.translation.x = lerp(transform.translation.x, next_pos.x, mv.frame/60.);
            transform.translation.y = lerp(transform.translation.y, next_pos.y, mv.frame/60.);
            mv.frame+=1.;
        }
        else{
            mv.frame = 0.;
            info!("finished");
            transform.translation = next_pos.extend(transform.translation.z);
            grid_pos.0 +=mv.next_move;
            mv.next_move = IVec2::ZERO;
        }
    }
}



pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(movement);
    }
    
}
