//! A Bevy plugin that enforces a fixed virtual resolution with dynamic letterboxing and UI scaling.
//!
//! Add `AspectRatioPlugin` to your app, and use the injected `Hud` resource to spawn your UI.
//! Requires a `Camera2dBundle` with `ScalingMode::AutoMin`.
//!
//! ---
//!
//! ### Example
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_aspect_ratio_mask::{AspectRatioPlugin, Hud};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(AspectRatioPlugin::default())
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands, hud: Res<Hud>) {
//!     commands.entity(hud.0).with_children(|parent| {
//!         parent
//!             .spawn((
//!                 Node {
//!                     width: Val::Percent(100.0),
//!                     top: Val::Percent(10.0),
//!                     position_type: PositionType::Absolute,
//!                     justify_content: JustifyContent::Center,
//!                     ..default()
//!                 },
//!             ))
//!             .with_children(|p| {
//!                 p.spawn(Text {
//!                     sections: vec![TextSection::new("Hello", TextStyle::default())],
//!                     ..default()
//!                 });
//!             });
//!     });
//! }
//! ```
use bevy::color::palettes::tailwind::GRAY_950;
use bevy::prelude::*;

/// A Bevy plugin that enforces a fixed virtual resolution with black bar masking and UI scaling.
///
/// This plugin centers and scales all UI content while hiding out-of-bounds regions
/// using dynamically positioned black bars. It works best with a 2D camera using
/// `ScalingMode::AutoMin`.
pub struct AspectRatioPlugin {
    /// The target virtual resolution (default is 960×540).
    pub resolution: Resolution,
    pub mask: AspectRatioMask,
}

impl Default for AspectRatioPlugin {
    fn default() -> Self {
        Self {
            resolution: Resolution::default(),
            mask: AspectRatioMask::default(),
        }
    }
}

impl Plugin for AspectRatioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.resolution)
            .insert_resource(self.mask);
        plugin(app);
    }
}

/// Represents the background color used for the letterboxing "mask" regions
/// that appear outside the target virtual resolution.
///
/// This color fills the black bars (or any custom color you choose)
/// when the window's aspect ratio doesn't match the desired resolution.
/// It's used internally by `AspectRatioPlugin` to visually isolate the game area.
#[derive(Resource, Clone, Copy)]
pub struct AspectRatioMask {
    pub color: Color,
}

impl Default for AspectRatioMask {
    fn default() -> Self {
        Self {
            color: GRAY_950.into(),
        }
    }
}

/// The virtual resolution used to maintain a consistent aspect ratio.
///
/// This should match your game's design resolution. If the window doesn't
/// match this ratio, the crate will apply letterboxing and UI scaling automatically.
#[derive(Resource, Clone, Copy)]
pub struct Resolution {
    /// The target width of the virtual resolution.
    pub width: f32,
    /// The target height of the virtual resolution.
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

/// Marker component for the UI node that defines the HUD's layout space.
///
/// Any entities spawned as children of this node will scale and center relative
/// to the defined virtual resolution.
#[derive(Component)]
struct AspectRatioHud;

/// Enum identifying one of the four aspect ratio masking regions.
///
/// These are spawned automatically as dark overlays ("black bars") to hide
/// any extra viewport space when the window aspect ratio deviates.
#[derive(Component)]
enum AspectRatioMaskSide {
    Left,
    Right,
    Top,
    Bottom,
}

/// Resource pointing to the root `Entity` of the aspect-ratio-scaled HUD.
///
/// Use `hud.0` in a system to spawn child nodes that auto-scale and stay centered.
#[derive(Resource)]
pub struct Hud(pub Entity);

/// Adds all internal systems for applying aspect ratio masking and UI scaling.
///
/// This is automatically invoked via `AspectRatioPlugin`—you generally don't call this yourself.
fn plugin(app: &mut App) {
    app.add_systems(PreStartup, setup); // PreStartup to register Hud so it can be used in Startup

    app.add_systems(
        Update,
        aspect_ratio_hud_scaler
            .chain()
            .run_if(on_event::<bevy::window::WindowResized>),
    );
}

fn setup(
    mut commands: Commands,
    resolution: Res<Resolution>,
    aspect_ration_mask: Res<AspectRatioMask>,
) {
    commands.spawn(aspect_ratio_mask_setup(aspect_ration_mask.color));

    let hud = commands.spawn(aspect_ratio_hud(resolution)).id();
    let mut base = commands.spawn(aspect_ratio_hud_parent());
    base.add_child(hud);

    commands.insert_resource(Hud(hud));
}

/// Updates UI margins and black bars when the window is resized.
///
/// Called only when a `WindowResized` event occurs.
fn aspect_ratio_hud_scaler(
    windows: Query<&Window>,
    resolution: Res<Resolution>,
    mut ui_scale: ResMut<UiScale>,
    mut aspect_ratio_hud: Query<&mut Node, With<AspectRatioHud>>,
    mut masks: Query<(&AspectRatioMaskSide, &mut Node), Without<AspectRatioHud>>,
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
            AspectRatioMaskSide::Left => {
                node.width = Val::Px(dx);
                node.left = Val::Px(-dx / 2.0);
            }
            AspectRatioMaskSide::Right => {
                node.width = Val::Px(dx);
                node.left = Val::Px(normalized_width - dx / 2.0);
            }
            AspectRatioMaskSide::Top => {
                node.height = Val::Px(dy);
                node.top = Val::Px(-dy / 2.0);
            }
            AspectRatioMaskSide::Bottom => {
                node.height = Val::Px(dy);
                node.top = Val::Px(normalized_height - dy / 2.0);
            }
        }
    }

    ui_scale.0 = min_scale;
}

/// Spawns a 100% sized container node for holding HUD content.
///
/// This node remains centered and scaled using the aspect ratio logic.
fn aspect_ratio_hud_parent() -> impl Bundle {
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

/// Creates a HUD node with fixed pixel dimensions based on the configured resolution.
///
/// This should be added as a child of the parent node returned by `aspect_ratio_hud_parent()`.
fn aspect_ratio_hud(resolution: Res<Resolution>) -> impl Bundle {
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

/// Spawns four masking nodes that surround the viewport to simulate black bars.
///
/// These are automatically sized based on the window and resolution mismatch.
fn aspect_ratio_mask_setup(color: Color) -> impl Bundle {
    (
        aspect_ratio_hud_parent(),
        children![
            (
                AspectRatioMaskSide::Left,
                Name::new("Aspect Ratio Mask"),
                Node {
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(color),
            ),
            (
                AspectRatioMaskSide::Right,
                Name::new("Aspect Ratio Mask"),
                Node {
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(color),
            ),
            (
                AspectRatioMaskSide::Top,
                Name::new("Aspect Ratio Mask"),
                Node {
                    width: Val::Percent(100.0),
                    top: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(color),
            ),
            (
                AspectRatioMaskSide::Bottom,
                Name::new("Aspect Ratio Mask"),
                Node {
                    width: Val::Percent(100.0),
                    top: Val::Px(0.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(color),
            )
        ],
    )
}
