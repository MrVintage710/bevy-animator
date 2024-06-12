//==============================================================================
// This example shows a basic implementation of the Aseprite Animator. This will
// play an animation that can be changed by mutating the Animator component.
//==============================================================================

use bevy_animator::prelude::*;
use bevy::{prelude::*, render::texture::ImagePlugin, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//==============================================================================
//           Entry Point
//==============================================================================

pub fn main() {
    
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Add the default plugins for sprite rendering
        .add_plugins(AnimatorPlugin) // Add the animator plugin
        .add_plugins(bevy_animator::prelude::AnimationPlugin::<KnightAnimation>::default())
        .add_plugins(WorldInspectorPlugin::new()) // You don't need this, the is just for debugging
    
        .add_systems(Startup, initialize) // Startup System
    ;
    
    app.run();
}

/// This system initializes the game by adding a camera and the animation.
pub fn initialize(
    mut commands : Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.init_animation::<KnightAnimation>("knight.aseprite")
        .insert(Transform::from_scale(Vec3::splat(5.0)))
    ;
}

#[derive(Default)]
pub enum KnightAnimation {
    #[default]
    Run
}

impl AsepriteAnimation for KnightAnimation {
    fn get_tag_name(&self) -> &str {
        match self {
            KnightAnimation::Run => "run",
        }
    }

    fn get_dimensions() -> UVec2 {
        UVec2::new(48, 48)
    }

    fn get_anchor_pixel() -> Vec2 { Vec2::new(24.5, 24.5)}
}