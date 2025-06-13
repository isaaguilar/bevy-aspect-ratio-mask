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
| 0.1.0 | 0.16.x |

> This crate currently supports **Bevy 0.16**. Compatibility with future Bevy versions will follow stable releases as needed.

---

## Getting Started

### 1. Add the crate

```toml
# Cargo.toml
bevy_aspect_ratio_mask = "0.1"
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

To customize the target resolution:

```rust
use bevy_aspect_ratio_mask::{AspectRatioPlugin, Resolution};

.add_plugins(AspectRatioPlugin {
    resolution: Resolution { width: 1280.0, height: 720.0 }
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

