use bevy::prelude::*;

/// Marker component for the root node of each screen
#[derive(Component)]
pub struct UIRoot;

/// Clean up UI when switching screens
pub fn destroy_ui(mut cmd: Commands, query: Query<Entity, With<UIRoot>>) {
    let e = query.single();
    cmd.entity(e).despawn_recursive();
}
