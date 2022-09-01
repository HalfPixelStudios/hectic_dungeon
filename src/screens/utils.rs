use bevy::prelude::*;

use crate::utils::ok_or_return;

pub const FONT_PATH: &str = "fonts/arcadeclassic.ttf";

/// Marker component for the root node of each screen
#[derive(Component)]
pub struct UIRoot;

/// Clean up UI when switching screens
pub fn destroy_ui(mut cmd: Commands, query: Query<Entity, With<UIRoot>>) {
    let e = ok_or_return!(query.get_single());
    cmd.entity(e).despawn_recursive();
}

/// Similar to `destroy_ui` but takes in a generic tag component
pub fn destroy_ui_tag<C: Component>(mut cmd: Commands, query: Query<Entity, With<C>>) {
    let e = ok_or_return!(query.get_single());
    cmd.entity(e).despawn_recursive();
}
