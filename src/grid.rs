use bevy::prelude::*;

const CELLWIDTH:i32 = 32;
const CELLHEIGHT:i32 = 32;

#[derive(Component,DerefMut,Deref)]
pub struct GridPosition(pub IVec2);

pub fn to_world_coords(p: &GridPosition)-> Vec2{
    Vec2::new((p.x*CELLWIDTH) as f32,(p.y*CELLWIDTH) as f32)
}
pub fn snap_to_grid(pos: Vec2) -> IVec2 {
    let mut snapped = ((pos / CELLWIDTH as f32).ceil() * CELLWIDTH as f32).round();
    IVec2::new(snapped.x as i32, snapped.y as i32)
}
