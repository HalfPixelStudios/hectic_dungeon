
use std::ops::Range;
use enum_map::*;
use bevy::{prelude::*, sprite};
use serde::*;

use crate::movement::Movement;

#[derive(Debug, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Enum, Deserialize, PartialEq, Eq)]
pub enum AniState {
    Idle,
    Walk,
    Attack
}
#[derive(Deserialize, Debug, Clone)]
pub struct AnimationPrefab {
    pub frames: EnumMap<AniState,Range<usize>>,
    pub speed: f32,
}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub state: AniState,
    pub frames: EnumMap<AniState,Range<usize>>,
    pub index: usize,
    pub played_once: bool,
    pub facing: Facing
}
impl Animation {
    pub fn new(prefab:&AnimationPrefab) -> Self {
        Self {
            frames:prefab.frames.clone(),
            index: prefab.frames[AniState::Idle].start,
            state: AniState::Idle,
            timer: Timer::from_seconds(0.1, true),
            played_once: false,
            facing:Facing::Left
        }
    }
    pub fn set_state(&mut self, s:AniState){
        if self.state == s{
            return 
        }
        self.index = self.frames[s].start;
        self.state = s;
        self.played_once = false;
        self.timer.reset();
        
    }
}
//returns the value of non zero element
fn iv2_sum(v: IVec2)->i32{
    v.x+v.y
}
pub fn state(mut query: Query<(&Movement, &mut Animation)>){
    for (mv, mut anim) in query.iter_mut(){
        if iv2_sum(mv.next_move)!=0{
            anim.set_state(AniState::Walk);
            if mv.next_move.x==1{
                anim.facing = Facing::Right;
            }else if mv.next_move.x==-1{
                
                anim.facing = Facing::Left;
            }

        }
        else{
            anim.set_state(AniState::Idle);
        }
    }

}
pub fn animate(time: Res<Time>, mut animations: Query<(&mut Animation, &mut TextureAtlasSprite)>) {

    for (mut ani, mut sprite) in animations.iter_mut() {
        let ani_range = ani.frames[ani.state].clone();

        ani.timer.tick(time.delta());
        if ani.timer.just_finished() {
            if !ani_range.contains(&ani.index) {
                ani.index = ani_range.start;
                ani.played_once = true;
            }
            sprite.index = ani.index as usize;
            ani.index += 1;

        }

        match ani.facing{
            Facing::Left => sprite.flip_x = false,
            Facing::Right => sprite.flip_x = true
        }
        
    }
}

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate)
            .add_system(state);
    }
}
