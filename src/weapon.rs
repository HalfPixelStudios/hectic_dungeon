use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use serde::Deserialize;

use crate::attack::AttackPattern;

#[derive(Deserialize,Clone)]
pub enum Damage {
    Fixed(u32),
    Range(u32, u32),
}
impl Damage{
    pub fn value(&self)->u32{
        match &self{
            Damage::Fixed(x) => *x,
            Damage::Range(x,y) => (x+y)/2


        }
    }
}

#[derive(Deserialize,Clone)]
pub struct WeaponPrefab {
    pub display_name: Option<String>,
    pub attack_pattern: AttackPattern,
    pub damage: Damage,
}

#[derive(Component,Deref,DerefMut)]
pub struct CurrentWeapon(pub WeaponPrefab);

const RON_STRING: &str = r#"
{
    "hammer": (
        attack_pattern: Hammer,
        damage: Fixed(1),
    ),
    "dagger": (
        attack_pattern: StraightOne,
        damage: Fixed(1),
    ),
    "twin_blade": (
        attack_pattern: TwinBlade,
        damage: Fixed(1),
    ),
    "steel_sword": (
        attack_pattern: StraightTwo,
        damage: Fixed(1),
    ),
    "arrow_trap": (
        attack_pattern: StraightSix,
        damage: Fixed(1),
    ),
}
"#;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<WeaponPrefab>::new(RON_STRING));
    }
}
