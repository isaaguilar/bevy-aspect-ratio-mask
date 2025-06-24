use bevy::{
    color::palettes::css::ORANGE, prelude::*, render::camera::ScalingMode, window::WindowResolution,
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
                    RESOLUTION_WIDTH * 1.3, // Window size doesn't matter here. It can be resized and the aspect ratio is kept with the defined resolution
                    RESOLUTION_HEIGHT * 1.3,
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
