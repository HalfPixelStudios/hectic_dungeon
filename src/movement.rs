use bevy::prelude::*;

use crate::grid::{GridPosition, to_world_coords};

const THRESHOLD:f32 = 2.;

    
// }
//2 ideas
//calculate distance from current position to grid position. if dist is greater
//than a threshold, keep moving towards the target. 
//Second idea, keep a track of last position, 
// Also have constant speed with timer?
fn movement(mut query:Query<(&GridPosition, &mut Transform)>){
    for (grid_pos, mut transform) in query.iter_mut(){
        let pos = to_world_coords(grid_pos);
        if transform.translation.truncate().distance(pos)>THRESHOLD{
        }
        else{
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }


    }


}



pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(movement);
    }
    
}
