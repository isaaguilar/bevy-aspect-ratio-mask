use bevy::color::palettes::tailwind::GRAY_950;
use bevy::prelude::*;

pub struct AspectRatioPlugin {
    pub resolution: Resolution,
}

impl Default for AspectRatioPlugin {
    fn default() -> Self {
        Self {
            resolution: Resolution::default(),
        }
    }
}

impl Plugin for AspectRatioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.resolution);
        plugin(app);
    }
}

#[derive(Resource, Clone, Copy)]
pub struct Resolution {
    pub width: f32,
    pub height: f32,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 960.0,
            height: 540.0,
        }
    }
}

#[derive(Component)]
pub struct AspectRatioHud;

#[derive(Component)]
pub enum AspectRatioMask {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Resource)]
pub struct Hud(pub Entity);

fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(
        Update,
        aspect_ratio_hud_scaler
            .chain()
            .run_if(on_event::<bevy::window::WindowResized>),
    );
}

fn setup(mut commands: Commands, resolution: Res<Resolution>) {
    commands.spawn(aspect_ratio_mask());

    let hud = commands.spawn(aspect_ratio_hud(resolution)).id();
    let mut base = commands.spawn(aspect_ratio_hud_parent());
    base.add_child(hud);

    commands.insert_resource(Hud(hud));
}

pub fn aspect_ratio_hud_scaler(
    windows: Query<&Window>,
    resolution: Res<Resolution>,
    mut ui_scale: ResMut<UiScale>,
    mut aspect_ratio_hud: Query<&mut Node, With<AspectRatioHud>>,
    mut masks: Query<(&AspectRatioMask, &mut Node), Without<AspectRatioHud>>,
) {
    let scale_x = windows.single().unwrap().resolution.size().x / resolution.width;
    let scale_y = windows.single().unwrap().resolution.size().y / resolution.height;

    let normalized_width = resolution.width * scale_x / scale_y;
    let normalized_height = resolution.height * scale_y / scale_x;

    let min_scale = scale_x.min(scale_y);

    let Ok(mut node) = aspect_ratio_hud.single_mut() else {
        return;
    };

    let dx = normalized_width - resolution.width;
    if scale_x > min_scale {
        node.margin.left = Val::Px(dx / 2.0);
    } else if scale_x <= min_scale {
        node.margin.left = Val::Px(0.0);
    }

    let dy = normalized_height - resolution.height;
    if scale_y > min_scale {
        node.margin.top = Val::Px(dy / 2.0);
    } else if scale_y <= min_scale {
        node.margin.top = Val::Px(0.0);
    }

    for (mask, mut node) in masks.iter_mut() {
        match mask {
            AspectRatioMask::Left => {
                node.width = Val::Px(dx);
                node.left = Val::Px(-dx / 2.0);
            }
            AspectRatioMask::Right => {
                node.width = Val::Px(dx);
                node.left = Val::Px(normalized_width - dx / 2.0);
            }
            AspectRatioMask::Top => {
                node.height = Val::Px(dy);
                node.top = Val::Px(-dy / 2.0);
            }
            AspectRatioMask::Bottom => {
                node.height = Val::Px(dy);
                node.top = Val::Px(normalized_height - dy / 2.0);
            }
        }
    }

    ui_scale.0 = min_scale;
}

pub fn aspect_ratio_hud_parent() -> impl Bundle {
    (
        Name::new("Aspect Ratio Hud Parent"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        Pickable::IGNORE,
    )
}

pub fn aspect_ratio_hud(resolution: Res<Resolution>) -> impl Bundle {
    (
        Name::new("Aspect Ratio Hud"),
        AspectRatioHud,
        Node {
            width: Val::Px(resolution.width),
            height: Val::Px(resolution.height),
            position_type: PositionType::Absolute,
            ..default()
        },
        // BackgroundColor(GRAY_500.into()),
    )
}

pub fn aspect_ratio_mask() -> impl Bundle {
    (
        aspect_ratio_hud_parent(),
        children![
            (
                AspectRatioMask::Left,
                Name::new("Aspect Ratio Mask"),
                Node {
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(GRAY_950.into()),
            ),
            (
                AspectRatioMask::Right,
                Name::new("Aspect Ratio Mask"),
                Node {
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(GRAY_950.into()),
            ),
            (
                AspectRatioMask::Top,
                Name::new("Aspect Ratio Mask"),
                Node {
                    width: Val::Percent(100.0),
                    top: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(GRAY_950.into()),
            ),
            (
                AspectRatioMask::Bottom,
                Name::new("Aspect Ratio Mask"),
                Node {
                    width: Val::Percent(100.0),
                    top: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(GRAY_950.into()),
            )
        ],
    )
}
