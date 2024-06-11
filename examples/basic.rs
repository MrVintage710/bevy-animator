//==============================================================================
// This example shows a basic implementation of the Aseprite Animator, and how 
// to give it state. Use the arrow keys or WASD to make the charcter walk in a 
// given direction. Hold shift to run. 
//==============================================================================


use bevy_animator::{animation::{AnimationPlugin, Animator}, aseprite::{Aseprite, AsepriteAnimation}, state::{AnimationState, AnimationStatePlugin}, AnimatorPlugin, InitAnimationCommand};
use bevy::{prelude::*, render::texture::ImagePlugin, window::close_on_esc, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//==============================================================================
//           Entry Point
//==============================================================================

pub fn main() {
    
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Add the default plugins for sprite rendering
        .add_plugins(AnimatorPlugin) // Add the animator plugin
        .add_plugins(AnimationPlugin::<CharacterAnimation>::default())
        .add_plugins(AnimationStatePlugin::<CharacterAnimation>::default())// Register the animator defined later by adding a plguin for it.
        .add_plugins(WorldInspectorPlugin::new()) // You don't need this, the is just for debugging
        
        .register_type::<PlayerCharacter>() // I am registering the player state so we can see it during runs
    
        .add_systems(Startup, initialize) // Startup System
    
        .add_systems(Update, close_on_esc) // For debuging, allows us to close the window with the escape key
        .add_systems(Update, walk) // This update the player state based on the input. See function later.
    ;
    
    app.run();
}

//==============================================================================
//           Systems
//==============================================================================

/// This system initializes the game by adding a camera and the player character.
pub fn initialize(
    mut commands : Commands,
) {
    
    commands.spawn(Camera2dBundle::default());
    commands.init_animation::<CharacterAnimation>("character.aseprite")
        .insert(Transform::from_scale(Vec3::splat(10.0)))
        .insert(PlayerCharacter::default())
    ;
    
}

pub fn walk(
    keys : Res<ButtonInput<KeyCode>>,
    mut player : Query<&mut PlayerCharacter>,
) {
    let Ok(mut player) = player.get_single_mut() else { return };
    let mut direction = player.looking_direction;
    
    let left_pressed = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let right_pressed = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    let up_pressed = keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp);
    let down_pressed = keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown);
    let shift_pressed = keys.pressed(KeyCode::ShiftLeft);
    
    if left_pressed || right_pressed || up_pressed || down_pressed {
        if shift_pressed {
            player.is_running = true;
        } else {
            player.is_running = false;
        }
       direction = Vec2::ZERO;
       player.is_walking = true;
    } else {
        player.is_walking = false;
        player.is_running = false;
    }
    
    if up_pressed {
        direction.y = 1.0;
    } else if down_pressed {
        direction.y = -1.0;
    }
    
    if left_pressed {
        direction.x = -1.0;
    } else if right_pressed {
        direction.x = 1.0;
    }
    
    player.looking_direction = direction;
}

//==============================================================================
//           Player State
//==============================================================================

#[derive(Component, Default, Reflect)]
pub struct PlayerCharacter {
    looking_direction : Vec2,
    is_walking : bool,
    is_running : bool,
}

//==============================================================================
//           Character Animation Definition
//==============================================================================

#[derive(Default)]
pub enum CharacterAnimation {
    #[default]
    IdleRight,
    IdleLeft,
    IdleUp,
    IdleDown,
    WalkRight,
    WalkLeft,
    WalkUp,
    WalkDown,
    RunRight,
    RunLeft,
    RunUp,
    RunDown,
}

impl AsepriteAnimation for CharacterAnimation {
    fn get_tag_name(&self) -> &str {
        match self {
            CharacterAnimation::IdleRight => "idle-right",
            CharacterAnimation::IdleLeft => "idle-left",
            CharacterAnimation::IdleUp => "idle-up",
            CharacterAnimation::IdleDown => "idle-down",
            CharacterAnimation::WalkRight => "walk-right",
            CharacterAnimation::WalkLeft => "walk-left",
            CharacterAnimation::WalkUp => "walk-up",
            CharacterAnimation::WalkDown => "walk-down",
            CharacterAnimation::RunRight => "run-right",
            CharacterAnimation::RunLeft => "run-left",
            CharacterAnimation::RunUp => "run-up",
            CharacterAnimation::RunDown => "run-down",
        }
    }

    fn get_anchor_pixel() -> Vec2 { Vec2::new(8.5, 14.5) }

    fn get_dimensions() -> UVec2 {
        UVec2::new(16, 16)
    }
}

impl AnimationState for CharacterAnimation {
    type StateQuery<'w, 's> = &'w PlayerCharacter;

    fn update_state(animator : &mut Animator<Self>, data : & <Self::StateQuery<'_, '_> as bevy::ecs::query::WorldQuery>::Item<'_>) {
        
        // let animation = animator.get_current_animation();
        if data.looking_direction.x.abs() > data.looking_direction.y.abs() {
            if data.looking_direction.x > 0.0 {
                if data.is_running {
                    animator.set_animation(CharacterAnimation::RunRight);
                } else if data.is_walking {
                    animator.set_animation(CharacterAnimation::WalkRight);
                } else {
                    animator.set_animation(CharacterAnimation::IdleRight);
                }
            } else {
                if data.is_running {
                    animator.set_animation(CharacterAnimation::RunLeft);
                } else if data.is_walking {
                    animator.set_animation(CharacterAnimation::WalkLeft);
                } else {
                    animator.set_animation(CharacterAnimation::IdleLeft);
                }
            }
        } else {
            if data.looking_direction.y > 0.0 {
                if data.is_running {
                    animator.set_animation(CharacterAnimation::RunUp);
                } else if data.is_walking {
                    animator.set_animation(CharacterAnimation::WalkUp);
                } else {
                    animator.set_animation(CharacterAnimation::IdleUp);
                }
            } else {
                if data.is_running {
                    animator.set_animation(CharacterAnimation::RunDown);
                } else if data.is_walking {
                    animator.set_animation(CharacterAnimation::WalkDown);
                } else {
                    animator.set_animation(CharacterAnimation::IdleDown);
                }
            }
        }
    }
}