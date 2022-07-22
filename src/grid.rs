use bevy::prelude::*;

const CELLWIDTH:i32 = 16;
const CELLHEIGHT:i32 = 16;

#[derive(Component,DerefMut,Deref)]
pub struct GridPosition(pub IVec2);

impl GridPosition{
    pub fn new(v: &Vec2) -> GridPosition{
        GridPosition(snap_to_grid(v))
    }
}

pub fn to_world_coords(p: &IVec2)-> Vec2{
    Vec2::new((p.x*CELLWIDTH) as f32,(p.y*CELLWIDTH) as f32)
}
pub fn snap_to_grid(p: &Vec2) -> IVec2 {
    info!("{:?}",p);
    Vec2::new(p.x/CELLWIDTH as f32,p.y*CELLWIDTH as f32).as_ivec2()


}
