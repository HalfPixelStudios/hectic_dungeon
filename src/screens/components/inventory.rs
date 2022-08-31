use autodefault::autodefault;
use bevy::prelude::*;

use crate::{
    player::{inventory::Inventory, Player},
    utils::ok_or_return,
};

#[derive(Component)]
enum SlotType {
    PrimaryWeapon,
    SecondaryWeapon,
    Ability,
    Armor,
}

pub struct InventoryDisplayPlugin;

impl Plugin for InventoryDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

#[autodefault]
#[allow(non_snake_case)]
pub fn InventoryDisplay(cmd: &mut ChildBuilder, assets: Res<AssetServer>) -> Entity {
    cmd.spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
            },
        })
        .with_children(|parent| {
            for slot_type in vec![
                SlotType::PrimaryWeapon,
                SlotType::SecondaryWeapon,
                SlotType::Ability,
                SlotType::Armor,
            ] {
                parent
                    .spawn_bundle(ImageBundle {
                        image: UiImage(assets.load("tilesheet/inventory_bg.png")),
                    })
                    .insert(slot_type);
            }
        })
        .id()
}

#[autodefault]
fn update(
    mut cmd: Commands,
    player_query: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    slot_query: Query<(Entity, &SlotType), Without<Player>>,
    assets: Res<AssetServer>,
) {
    let inventory = ok_or_return!(player_query.get_single());

    for (entity, slot) in &slot_query {
        cmd.entity(entity).despawn_descendants();

        let img = match slot {
            SlotType::PrimaryWeapon => match &inventory.weapon_primary {
                Some(id) => "",
                None => "tilesheet/weapon_slot.png",
            },
            SlotType::SecondaryWeapon => match &inventory.weapon_secondary {
                Some(id) => "",
                None => "tilesheet/weapon_slot.png",
            },
            SlotType::Ability => match &inventory.ability {
                Some(id) => "",
                None => "tilesheet/ability_slot.png",
            },
            SlotType::Armor => match &inventory.armor {
                Some(id) => "",
                None => "tilesheet/inventory_slot.png",
            },
        };

        cmd.entity(entity).with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: UiImage(assets.load(img)),
            });
        });
    }
}
