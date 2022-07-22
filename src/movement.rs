use bevy::prelude::*;

use crate::{grid::{GridPosition, to_world_coords, snap_to_grid}, animation::AniState};

const THRESHOLD:f32 = 4.;

    
// store grid pos, or next move?
#[derive(Component)]
pub struct Movement{
    // time to get to next grid cell
    pub timer: Timer,
    // the move dir, 0 if nothing
    pub next_move: IVec2,
    
}
fn movement(mut query:Query<(&mut GridPosition, &mut Movement, &mut Transform)>){
    for (mut grid_pos, mut mv, mut transform) in query.iter_mut(){
        if mv.next_move == IVec2::ZERO{
            return 
        }
        let next_pos = to_world_coords(&(grid_pos.0+mv.next_move));
        let cur_pos = transform.translation.truncate();
        if cur_pos.distance(next_pos)>THRESHOLD{
            transform.translation += mv.next_move.as_vec2().extend(0.)*1.8;
        }
        else{
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
