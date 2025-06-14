use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                handle_input,
                update_tool_display,
                camera_follow_player,
                execute_animations,
            ),
        );
    }
}

const PLAYER_SPEED: f32 = 100.;

#[derive(Component)]
struct ToolDisplay;

#[derive(Component)]
struct Player;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Component)]
enum AnimationType {
    Idle,
    Walking,
    Acting,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ToolAction {
    None,
    Chop,
    Hoe,
    Water,
}

#[derive(Component)]
struct AnimatedSprite {
    direction: Direction,
    state: AnimationType,
    action: ToolAction,
}

#[derive(Component)]
struct AnimationConfig {
    frames_per_row: usize,
    fps: u8,
    timer: Timer,
}

impl AnimationConfig {
    fn new(frames_per_row: usize, fps: u8) -> Self {
        Self {
            frames_per_row,
            fps,
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / fps as f32),
                TimerMode::Repeating,
            ),
        }
    }
}

fn handle_input(
    mut animated_query: Query<&mut AnimatedSprite>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        if let Ok(mut animated) = animated_query.single_mut() {
            // === Tool selection ===
            if input.just_pressed(KeyCode::Digit1) {
                animated.action = ToolAction::Chop;
            } else if input.just_pressed(KeyCode::Digit2) {
                animated.action = ToolAction::Hoe;
            } else if input.just_pressed(KeyCode::Digit3) {
                animated.action = ToolAction::Water;
            } else if input.just_pressed(KeyCode::Digit4) {
                animated.action = ToolAction::None;
            }

            let mut direction = Vec2::ZERO;

            if input.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if input.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }
            if input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }

            let is_moving = direction != Vec2::ZERO;
            let is_acting = input.pressed(KeyCode::Space) && animated.action != ToolAction::None;

            if is_acting {
                animated.state = AnimationType::Acting;
            } else if is_moving {
                animated.state = AnimationType::Walking;

                animated.direction = match direction {
                    d if d.y > 0.0 => Direction::Up,
                    d if d.y < 0.0 => Direction::Down,
                    d if d.x > 0.0 => Direction::Right,
                    d if d.x < 0.0 => Direction::Left,
                    _ => animated.direction,
                };

                let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
                transform.translation += move_delta.extend(0.);
            } else {
                animated.state = AnimationType::Idle;
            }
        }
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &AnimatedSprite, &mut Sprite)>,
) {
    for (mut config, animated, mut sprite) in &mut query {
        config.timer.tick(time.delta());

        if config.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let row_index = match animated.state {
                    AnimationType::Idle => match animated.direction {
                        Direction::Down => 0,
                        Direction::Up => 1,
                        Direction::Right => 2,
                        Direction::Left => 3,
                    },
                    AnimationType::Walking => match animated.direction {
                        Direction::Down => 4,
                        Direction::Up => 5,
                        Direction::Right => 6,
                        Direction::Left => 7,
                    },
                    AnimationType::Acting => match animated.action {
                        ToolAction::Chop => match animated.direction {
                            Direction::Down => 16,
                            Direction::Up => 17,
                            Direction::Right => 18,
                            Direction::Left => 19,
                        },
                        ToolAction::Hoe => match animated.direction {
                            Direction::Down => 12,
                            Direction::Up => 13,
                            Direction::Right => 14,
                            Direction::Left => 15,
                        },
                        ToolAction::Water => match animated.direction {
                            Direction::Down => 20,
                            Direction::Up => 21,
                            Direction::Right => 22,
                            Direction::Left => 23,
                        },
                        ToolAction::None => 0,
                    },
                };

                let row_offset = row_index * config.frames_per_row;
                let current = atlas.index % config.frames_per_row;
                let next = (current + 1) % config.frames_per_row;

                atlas.index = row_offset + next;
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("character.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 8, 24, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(0., 0., 1.0)),
        AnimatedSprite {
            direction: Direction::Down,
            state: AnimationType::Idle,
            action: ToolAction::None,
        },
        AnimationConfig::new(8, 10),
        Player,
    ));

    commands.spawn((
        Text::new("Action: None"),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::default(),
        ToolDisplay,
    ));
}

fn update_tool_display(
    animated_query: Query<&AnimatedSprite, With<Player>>,
    mut writer: TextUiWriter,
    query: Query<Entity, With<ToolDisplay>>,
) {
    if let Ok(animated) = animated_query.single() {
        if let Ok(entity) = query.single() {
            let label = match animated.action {
                ToolAction::Chop => "Chop",
                ToolAction::Hoe => "Hoe",
                ToolAction::Water => "Water",
                ToolAction::None => "None",
            };
            *writer.text(entity, 0) = format!("Action: {}", label);
        }
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let cam_speed = 5.0;
            camera_transform.translation.x = camera_transform.translation.x.lerp(
                player_transform.translation.x,
                cam_speed * time.delta_secs(),
            );
            camera_transform.translation.y = camera_transform.translation.y.lerp(
                player_transform.translation.y,
                cam_speed * time.delta_secs(),
            );
        }
    }
}
