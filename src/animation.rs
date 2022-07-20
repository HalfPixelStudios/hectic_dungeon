
use std::ops::Range;
use enum_map::*;
use bevy::prelude::*;
use serde::*;

pub enum Facing {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Enum, Deserialize)]
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
    pub played_once: bool
}
impl Animation {
    pub fn new(prefab:&AnimationPrefab) -> Self {
        info!(prefab.speed);
        Self {
            frames:prefab.frames.clone(),
            index: prefab.frames[AniState::Idle].start,
            state: AniState::Idle,
            timer: Timer::from_seconds(prefab.speed, true),
            played_once: false,
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

            info!("{}", ani.index);
        }
        
    }
}

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate);
    }
}
