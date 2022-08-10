use bevy::{input::mouse::*, prelude::*};

#[derive(Debug, Component)]
struct MainCamera;

#[derive(Component)]
pub struct CameraFollow;

pub struct Cursor(pub Vec2);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Cursor(Vec2::ZERO))
            .add_system(cursor_system)
            .add_system(camera_controller);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            scale: 0.4,
            ..default()
        },
        ..OrthographicCameraBundle::new_2d()
    })
    .insert(PanCam::default())
    .insert(MainCamera);
}
fn camera_controller(
    entity_query: Query<&mut GlobalTransform, (With<CameraFollow>, Without<MainCamera>)>,
    mut camera_query: Query<
        (&mut Camera, &mut GlobalTransform),
        (With<MainCamera>, Without<CameraFollow>),
    >,
    mut cursor: ResMut<Cursor>,
) {
    let (mut camera, mut cam_transform) = camera_query.single_mut();
    let mut pos: Vec2 = Vec2::ZERO;
    let mut query_len = 0.;
    for (transform) in entity_query.iter() {
        pos.x += transform.translation.x;
        pos.y += transform.translation.y;
        query_len += 1.;
    }
    if (query_len == 0.) {
        return;
    }
    pos /= query_len;
    cam_transform.translation.x = lerp(cam_transform.translation.x, pos.x, 0.1);
    cam_transform.translation.y = lerp(cam_transform.translation.y, pos.y, 0.1);
}
fn camera_zoom(
    mut query: Query<(&PanCam, &mut OrthographicProjection)>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let pixels_per_line = 100.; // Maybe make configurable?
    let scroll = scroll_events
        .iter()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y,
            MouseScrollUnit::Line => ev.y * pixels_per_line,
        })
        .sum::<f32>();

    if scroll == 0. {
        return;
    }

    for (cam, mut projection) in query.iter_mut() {
        if cam.enabled {
            projection.scale = (projection.scale * (1. + -scroll * 0.001)).max(0.00001);
        }
    }
}
fn camera_movement(
    mut windows: ResMut<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut query: Query<(&PanCam, &mut Transform, &OrthographicProjection)>,
    mut last_pos: Local<Option<Vec2>>,
) {
    let window = windows.get_primary_mut().unwrap();

    // Use position instead of MouseMotion, otherwise we don't get acceleration movement
    let current_pos = match window.cursor_position() {
        Some(current_pos) => current_pos,
        None => return,
    };
    let delta = current_pos - last_pos.unwrap_or(current_pos);

    for (cam, mut transform, projection) in query.iter_mut() {
        if cam.enabled
            && cam
                .grab_buttons
                .iter()
                .any(|btn| mouse_buttons.pressed(*btn))
        {
            let scaling = Vec2::new(
                window.width() / (projection.right - projection.left),
                window.height() / (projection.top - projection.bottom),
            ) * projection.scale;

            transform.translation -= (delta * scaling).extend(0.);
        }
    }
    *last_pos = Some(current_pos);
}
// from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn cursor_system(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    let (camera, transform) = query.single();

    let win = windows.get_primary().unwrap();

    if let Some(pos) = win.cursor_position() {
        let window_size = Vec2::new(win.width() as f32, win.height() as f32);
        let ndc = (pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        cursor.0 = world_pos.truncate();
    }
}
pub fn lerp(x: f32, y: f32, by: f32) -> f32 {
    x * (1. - by) + y * by
}
#[derive(Component)]
pub struct PanCam {
    pub grab_buttons: Vec<MouseButton>,
    pub enabled: bool,
}

impl Default for PanCam {
    fn default() -> Self {
        Self {
            grab_buttons: vec![MouseButton::Left, MouseButton::Right, MouseButton::Middle],
            enabled: true,
        }
    }
}
