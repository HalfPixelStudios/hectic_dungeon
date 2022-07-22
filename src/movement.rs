use bevy::prelude::*;

use crate::{grid::{GridPosition, to_world_coords, snap_to_grid}, animation::AniState, enemy::Enemy};

const THRESHOLD:f32 = 4.;

    
// store grid pos, or next move?
#[derive(Component)]
pub struct Movement{
    // delay from next_move being set to movement
    pub delay: Timer,
    // the move dir, 0 if nothing
    pub next_move: IVec2,
    
}
fn movement(time: Res<Time>, mut query:Query<(&mut GridPosition, &mut Movement, &mut Transform),With<Enemy>>){
    for (mut grid_pos, mut mv, mut transform) in query.iter_mut(){
        info!("enemy");

        
        if mv.next_move == IVec2::ZERO{
            mv.delay.reset();
            return 
        }else{
            mv.delay.tick(time.delta());
            if(!mv.delay.finished()){
                info!("finished?");

                return
            }
        }
        let next_pos = to_world_coords(&(grid_pos.0+mv.next_move));
        let cur_pos = transform.translation.truncate();
        if cur_pos.distance(next_pos)>THRESHOLD{
            transform.translation += mv.next_move.as_vec2().extend(0.)*1.8;
        }
        else{
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
