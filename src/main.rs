use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, execute_animations))
        .run();
}

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
}

#[derive(Component)]
struct AnimatedSprite {
    direction: Direction,
    state: AnimationType,
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
            timer: Timer::new(Duration::from_secs_f32(1.0 / fps as f32), TimerMode::Repeating),
        }
    }
}

fn handle_input(mut query: Query<&mut AnimatedSprite>, input: Res<ButtonInput<KeyCode>>) {
    for mut animated in &mut query {
        let mut new_state = AnimationType::Idle;

        if input.pressed(KeyCode::ArrowUp) {
            animated.direction = Direction::Up;
            new_state = AnimationType::Walking;
        } else if input.pressed(KeyCode::ArrowDown) {
            animated.direction = Direction::Down;
            new_state = AnimationType::Walking;
        } else if input.pressed(KeyCode::ArrowLeft) {
            animated.direction = Direction::Left;
            new_state = AnimationType::Walking;
        } else if input.pressed(KeyCode::ArrowRight) {
            animated.direction = Direction::Right;
            new_state = AnimationType::Walking;
        }

        animated.state = new_state;
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
    commands.spawn(Camera2d);

    let texture = asset_server.load("character.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 8, 8, None, None);
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
        Transform::from_scale(Vec3::splat(6.0)),
        AnimatedSprite {
            direction: Direction::Down,
            state: AnimationType::Idle,
        },
        AnimationConfig::new(8, 10), // 8 frames per row, 10 FPS
    ));
}
