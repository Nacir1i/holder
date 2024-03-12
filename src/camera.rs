use std::f32::consts::PI;

use bevy::{
    input::{
        gamepad::{GamepadConnection::*, *},
        mouse::MouseMotion,
    },
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use leafwing_input_manager::{prelude::*, user_input::InputKind};

#[derive(Component)]
pub struct MainCamera {
    pub mouse_sensitivity: f32,
    pub focus: Vec3,
    pub cursor_lock_active: bool,
    pub gamepad_settings: CustomGamepadSettings,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.6,
            focus: Vec3::ZERO,
            cursor_lock_active: true,
            gamepad_settings: CustomGamepadSettings::default(),
        }
    }
}

#[derive(Resource)]
pub struct GamepadResource(pub Gamepad);

#[derive(Component)]
pub struct CustomGamepadSettings {
    pub x_sensitivity: f32,
    pub y_sensitivity: f32,
}

impl Default for CustomGamepadSettings {
    fn default() -> Self {
        Self {
            x_sensitivity: 7.0,
            y_sensitivity: 4.0,
        }
    }
}

pub struct GamePadPlugin;

impl Plugin for GamePadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                connections,
                orbit_gamepad
                    .run_if(resource_exists::<GamepadResource>().and_then(orbit_condition)),
            ),
        );
    }
}

fn connections(
    mut cmds: Commands,
    gamepad_res: Option<Res<GamepadResource>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.read() {
        match &ev.connection {
            Connected(_info) => {
                if gamepad_res.is_none() {
                    cmds.insert_resource(GamepadResource(Gamepad::new(0)));
                }
            }
            Disconnected => cmds.remove_resource::<GamepadResource>(),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum CameraAction {
    ToggleCursor,
}

impl CameraAction {
    pub fn default_keyboard_mouse_input(action: CameraAction) -> UserInput {
        match action {
            Self::ToggleCursor => UserInput::Single(InputKind::Keyboard(KeyCode::Escape)),
        }
    }

    pub fn default_gamepad_input(action: CameraAction) -> UserInput {
        match action {
            Self::ToggleCursor => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Start))
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct MainCameraTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamePadPlugin)
            .add_systems(Update, orbit_mouse.run_if(orbit_condition))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (sync_player_camera, toggle_cursor));
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut input_map = InputMap::default();

    for action in CameraAction::variants() {
        input_map.insert(CameraAction::default_keyboard_mouse_input(action), action);
        input_map.insert(CameraAction::default_gamepad_input(action), action);
    }

    commands.spawn((
        Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 1.5, 0.0),
                rotation: Quat::from_rotation_x(-15.0_f32.to_radians()),
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: f32::to_radians(80.0),
                ..default()
            }),
            ..default()
        },
        MainCamera::default(),
        InputManagerBundle::<CameraAction> {
            input_map,
            ..default()
        },
    ));
}

fn sync_player_camera(
    player_q: Query<&Transform, With<MainCameraTarget>>,
    mut cam_q: Query<(&mut MainCamera, &mut Transform), Without<MainCameraTarget>>,
) {
    let Ok(player) = player_q.get_single() else {
        return;
    };
    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

    let desired_translation = cam.focus + rotation_matrix.mul_vec3(Vec3::new(0.0, 0.7, 1.8));

    let delta = player.translation - cam.focus;
    cam_transform.translation = desired_translation + delta;
}

fn toggle_cursor(
    mut cam_q: Query<&mut MainCamera>,
    mut window_q: Query<&mut Window, With<PrimaryWindow>>,
    action_q: Query<&ActionState<CameraAction>, With<MainCamera>>,
) {
    let action_state = action_q.single();

    let Ok(mut cam) = cam_q.get_single_mut() else {
        return;
    };

    let mut window = window_q.get_single_mut().unwrap();

    if window.focused {
        if action_state.just_pressed(CameraAction::ToggleCursor) {
            cam.cursor_lock_active = !cam.cursor_lock_active;
        }

        if cam.cursor_lock_active {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        } else {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    } else {
        cam.cursor_lock_active = false;
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

fn orbit_condition(cam_q: Query<&MainCamera>) -> bool {
    let Ok(cam) = cam_q.get_single() else {
        return true;
    };
    return cam.cursor_lock_active;
}

pub fn orbit_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&MainCamera, &mut Transform), With<MainCamera>>,
    mut mouse_evr: EventReader<MouseMotion>,
) {
    let mut rotation = Vec2::ZERO;

    for ev in mouse_evr.read() {
        rotation = ev.delta;
    }

    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    rotation *= cam.mouse_sensitivity;

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = rotation.x / window.width() * std::f32::consts::PI;
        let delta_y = rotation.y / window.height() * PI;

        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation;

        let new_rotation = cam_transform.rotation * pitch;

        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }
}

pub fn orbit_gamepad(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut cam_q: Query<(&MainCamera, &mut Transform), With<MainCamera>>,
    axis: Res<Axis<GamepadAxis>>,
    gamepad_res: Option<Res<GamepadResource>>,
) {
    let gamepad = if let Some(gp) = gamepad_res {
        gp.0
    } else {
        return;
    };

    let Ok((cam, mut cam_transform)) = cam_q.get_single_mut() else {
        return;
    };

    let x_axis = GamepadAxis::new(gamepad, GamepadAxisType::RightStickX);
    let y_axis = GamepadAxis::new(gamepad, GamepadAxisType::RightStickY);

    let deadzone = 0.5;
    let mut rotation = Vec2::ZERO;
    if let (Some(x), Some(y)) = (axis.get(x_axis), axis.get(y_axis)) {
        if x.abs() > deadzone || y.abs() > deadzone {
            rotation = Vec2::new(x, y);
        }
    }

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width()
                * std::f32::consts::PI
                * 2.0
                * cam.gamepad_settings.x_sensitivity;
            delta
        };
        let delta_y = -rotation.y / window.height() * PI * cam.gamepad_settings.y_sensitivity;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation;

        let new_rotation = cam_transform.rotation * pitch;

        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }
}
