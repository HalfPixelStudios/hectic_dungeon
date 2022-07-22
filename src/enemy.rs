use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{assets::{SpriteSheets, PrefabData, BeingPrefab}, animation::Animation, grid::GridPosition, movement::Movement, player::PlayerMovedEvent, game::GameState};


#[derive(Component)]
pub struct Enemy;
pub struct SpawnEnemyEvent{
    pub spawn_pos: Vec2
}
fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>
) {
    for SpawnEnemyEvent { spawn_pos } in events.iter() {


        let enemy = beings.get(prefab_data.get("archer").unwrap()).unwrap();
        cmd.spawn()
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index:0,
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
            .insert(Movement{
                next_move: IVec2::ZERO,
                frame: 0.
            })
            .insert(Enemy);

            

    }
}
fn ai(mut query: Query<(&Transform, &mut GridPosition, &mut Movement),With<Enemy>>,
      mut events:EventReader<PlayerMovedEvent>){

    for (transform, mut grid_pos, mut mv) in query.iter_mut(){
        info!("move{}",transform.translation);
        
        
        mv.next_move = -IVec2::X;


    }

    
}


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_system(spawn)
            .add_system_set(
                ConditionSet::new()
                .run_on_event::<PlayerMovedEvent>() 
                .with_system(ai)
                .into()
                );
    }
    
}
