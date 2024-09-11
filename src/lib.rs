use std::{time::Duration, usize};

use bevy::prelude::*;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut animation_timer, mut atlas) in &mut query {
        animation_timer.tick(time.delta());
        if atlas.index > indices.last || atlas.index < indices.first {
            atlas.index = indices.first;
            return;
        }
        if animation_timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

pub fn set_animation(
    animation: (&mut AnimationIndices, &mut AnimationTimer),
    first: usize,
    last: usize,
    duration: f32,
) {
    let duration = Duration::from_secs_f32(duration);
    animation.0.first = first;
    animation.0.last = last;
    animation.1.set_duration(duration);
}

#[derive(Bundle)]
pub struct AnimationBundle {
    timer: AnimationTimer,
    indices: AnimationIndices,
    atlas: TextureAtlas,
}

impl AnimationBundle {
    pub fn new(
        duration: f32,
        first: usize,
        last: usize,
        layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        Self {
            timer: AnimationTimer(Timer::from_seconds(duration, TimerMode::Repeating)),
            indices: AnimationIndices { first, last },
            atlas: TextureAtlas {
                layout,
                index: first,
            },
        }
    }
}
