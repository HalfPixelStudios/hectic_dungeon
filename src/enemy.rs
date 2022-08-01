use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheets},
    grid::{CellType, Grid, GridPosition},
    movement::Movement,
    player::PlayerMovedEvent,
};

#[derive(Component)]
pub struct Enemy;
pub struct SpawnEnemyEvent {
    pub spawn_pos: Vec2,
}
pub struct EnemyUpdateEvent;
fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>,
) {
    for SpawnEnemyEvent { spawn_pos } in events.iter() {
        let enemy = beings.get(prefab_data.get("archer").unwrap()).unwrap();
        cmd.spawn()
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    color: Color::ORANGE,
                    ..default()
                },
                texture_atlas: asset_sheet.get("archer").unwrap().clone(),
                transform: Transform {
                    translation: spawn_pos.extend(0.),
                    ..default()
                },
                ..default()
            })
            .insert(Animation::new(&enemy.anim))
            .insert(GridPosition::new(spawn_pos))
            .insert(Movement::new())
            .insert(Enemy);
    }
}
//TODO player cant move if go neg or enemy on top
//Maybe both are knocked back when two beings try to move into the same cell
fn ai(
    mut query: Query<(&Transform, &mut GridPosition, &mut Movement), With<Enemy>>,
    mut events: EventReader<PlayerMovedEvent>,
    grid: Res<Grid>,
) {
    let player_pos = grid.find(CellType::Player);

    for (transform, mut grid_pos, mut mv) in query.iter_mut() {
        if let Some(p) = player_pos {
            let diff = p - grid_pos.0;
            if diff.x.abs() > diff.y.abs() {
                mv.next_move = diff.x.signum() * IVec2::X;
            } else {
                mv.next_move = diff.y.signum() * IVec2::Y;
            }
        } else {
            mv.next_move = IVec2::ZERO;
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_event::<EnemyUpdateEvent>()
            .add_system(spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_on_event::<EnemyUpdateEvent>()
                    .with_system(ai)
                    .into(),
            );
    }
}
