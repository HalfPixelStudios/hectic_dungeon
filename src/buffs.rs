use bevy::prelude::*;
use bevy_bobs::component::health::Health;

//Types of effects
//Overtime ticking effect ex. Poison
//Buffs ex. +10 dmg
//Triggered effect -
trait Effect {
    fn apply_effect() {}
}
struct Modifier {
    timer: Timer,
    value: f32,
    var: String,
}
impl Modifier {
    fn apply(&self, mut stats: ModifiedStats) -> ModifiedStats {
        stats
    }
}
#[derive(Clone, Copy)]
pub struct ModifiedStats {
    health: Health,
    armor: i32,
    speed: i32,
    damage: i32,
    crit: f32,
}
impl ModifiedStats {}

#[derive(Component)]
pub struct Stats {
    health: Health,
    armor: i32,
    speed: i32,
    damage: i32,
    crit: f32,
    modifiers: Vec<Modifier>,
}
impl Stats {
    pub fn apply_modifiers(self) -> ModifiedStats {
        let mut m_stats = ModifiedStats {
            health: self.health,
            armor: self.armor,
            speed: self.speed,
            damage: self.damage,
            crit: self.crit,
        };
        for modifier in self.modifiers {
            modifier.apply(m_stats);
        }
        m_stats
    }
}
fn tick_modifiers(time: Res<Time>, mut stat_query: Query<&mut Stats>) {
    for mut stat in stat_query.iter_mut() {
        stat.modifiers.retain_mut(|modifier| {
            modifier.timer.tick(time.delta());
            !modifier.timer.just_finished()
        })
    }
}
