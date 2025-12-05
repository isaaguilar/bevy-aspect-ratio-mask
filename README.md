# Bevy Aspect Ratio Mask

A lightweight Bevy plugin that maintains a fixed virtual resolution across all screen sizes, applying dynamic letterboxing and consistent UI scaling. Perfect for 2D games with pixel-perfect layouts or tight design constraints.

---

## Features

- Viewport letterboxing (black bars) for non-matching aspect ratios  
- Centered, consistently scaled UI on any screen size  
- Automatically responds to `WindowResized` events  
- Works out-of-the-box with a single plugin line  
- Fully configurable design resolution (default: `960 × 540`)  

---

## Compatibility

| bevy_aspect_ratio_mask | Bevy Version |
|-|-|
| 0.3.0 | 0.17.x |
| 0.2.0 | 0.16.x |

---

## Getting Started

### 1. Add the crate

```toml
# Cargo.toml
bevy_aspect_ratio_mask = "0.3"
```

### 2. Register the plugin

```rust
use bevy::prelude::*;
use bevy_aspect_ratio_mask::AspectRatioPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AspectRatioPlugin::default()) // Optional: customize resolution
        .add_systems(Startup, setup)
        .run();
}
```

To customize the target resolution or other options:

```rust
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};

.add_plugins(AspectRatioPlugin {
    resolution: Resolution { width: 1280.0, height: 720.0 }, 
    mask: AspectRatioMask::default(),
})
```

### 3. Add your own Camera2dBundle

> Required for proper scaling behavior

```rust
Camera2d::default(),
Projection::from(OrthographicProjection {
    scaling_mode: ScalingMode::AutoMin {
        min_width: 1280.0,
        min_height: 720.0,
    },
    ..OrthographicProjection::default_2d()
}),
```


## Usage

In your Startup system or any other system, attach UI or game content to the HUD:

```rust
use bevy_aspect_ratio_mask::Hud;

fn setup(mut commands: Commands, hud: Res<Hud>) {
 commands.entity(hud.0).with_children(|parent| {
       parent
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    top: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|p| {
                p.spawn(Text("Hello".into()));
            });
    });
}
```

All content spawned as children of the HUD entity will scale and position correctly with the defined resolution and black bars.

## Full Example

Run the examples: `cargo run --example simple`. 

```rust
use bevy::{
    camera::ScalingMode, color::palettes::css::ORANGE, prelude::*, window::WindowResolution,
};
use bevy_aspect_ratio_mask::{AspectRatioPlugin, Hud, Resolution};

const RESOLUTION_WIDTH: f32 = 600.0;
const RESOLUTION_HEIGHT: f32 = 480.0;
const HALF_WIDTH_SPRITE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aspect Ratio Mask".into(),
                name: Some("Aspect Ratio Mask".into()),
                resolution: WindowResolution::new(
                    (RESOLUTION_WIDTH * 1.3) as u32, // Window size doesn't matter here. It can be resized and the aspect ratio is kept with the defined resolution
                    (RESOLUTION_HEIGHT * 1.3) as u32,
                ),
                ..default()
            }),
            ..default()
        }))
        // Add the custom aspect ratio plugin to enforce resolution scaling behavior
        .add_plugins(AspectRatioPlugin {
            resolution: Resolution {
                width: RESOLUTION_WIDTH,
                height: RESOLUTION_HEIGHT,
            },
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, arrow_move)
        .run();
}

fn setup(mut commands: Commands, hud: Res<Hud>) {
    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: RESOLUTION_WIDTH,
                min_height: RESOLUTION_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.entity(hud.0).with_children(|parent| {
        parent
            .spawn((Node {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                top: Val::Px(55.0),
                align_items: AlignItems::Center,
                ..default()
            },))
            .with_children(|p| {
                p.spawn(Text("Press Left / Right To Move\n\n".into()));
                p.spawn(Text("Resizing window maintains aspect ratio".into()));
            });
    });

    commands.spawn(Sprite {
        color: ORANGE.into(),
        custom_size: Some(Vec2::splat(HALF_WIDTH_SPRITE * 2.)),
        ..default()
    });
}

fn arrow_move(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite: Query<&mut Transform, With<Sprite>>,
) {
    if let Ok(mut transform) = sprite.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if transform.translation.x > HALF_WIDTH_SPRITE + RESOLUTION_WIDTH / 2. {
                transform.translation.x = -HALF_WIDTH_SPRITE - RESOLUTION_WIDTH / 2.;
            } else {
                transform.translation.x += 100. * time.delta_secs();
            }
        } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if transform.translation.x < -HALF_WIDTH_SPRITE - RESOLUTION_WIDTH / 2. {
                transform.translation.x = HALF_WIDTH_SPRITE + RESOLUTION_WIDTH / 2.;
            } else {
                transform.translation.x -= 100. * time.delta_secs();
            }
        }
    }
}
```

## When to Use This

You're targeting a fixed virtual resolution and don’t want content leaking outside it
You want clean black bars (letterboxing) instead of stretching
You want your UI to stay visually centered and properly scaled
You're making a retro, puzzle, or pixel-art game where aspect ratio precision matters

## Internals

Spawns a viewport mask with 4 sides (top/bottom/left/right) using dark overlays
Injects a Hud resource pointing to the UI root entity
Scales UI and game visuals using Bevy’s UiScale and position margins

## Questions / Contributing

Open an issue, submit a PR, or start a discussion! Feedback and improvements welcome.

