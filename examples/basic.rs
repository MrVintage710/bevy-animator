//==============================================================================
// This example shows a basic implementation of the Aseprite Animator, and how 
// to give it state. Use the arrow keys or WASD to make the charcter walk in a 
// given direction. Hold shift to run. 
//==============================================================================


use bevy::{prelude::*, window::close_on_esc};
use bevy_animator::{animation::{AnimationPlugin, Animator}, aseprite::{Aseprite, AsepriteStateAnimation}, AnimatorPlugin, InitAnimationCommand};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//==============================================================================
//           Entry Point
//==============================================================================

pub fn main() {
    
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // Add the default plugins for sprite rendering
        .add_plugins(AnimatorPlugin) // Add the animator plugin
        .add_plugins(AnimationPlugin::<CharacterAnimation>::default()) // Register the animator defined later by adding a plguin for it.
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
        .insert(Transform::from_scale(Vec3::splat(10.0)));
    
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

#[derive(Component, Default)]
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

impl AsepriteStateAnimation for CharacterAnimation {
    type State = PlayerCharacter;

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

    fn get_anchor_pixel() -> Vec2 {
        Vec2::ZERO 
    }

    fn update_state(&mut self, item : &Self::State) {
        if item.looking_direction.x.abs() > item.looking_direction.y.abs() {
            if item.looking_direction.x > 0.0 {
                if item.is_running {
                    *self = CharacterAnimation::RunRight;
                } else if item.is_walking {
                    *self = CharacterAnimation::WalkRight;
                } else {
                    *self = CharacterAnimation::IdleRight;
                }
            } else {
                if item.is_running {
                    *self = CharacterAnimation::RunLeft;
                } else if item.is_walking {
                    *self = CharacterAnimation::WalkLeft;
                } else {
                    *self = CharacterAnimation::IdleLeft;
                }
            }
        } else {
            if item.looking_direction.y > 0.0 {
                if item.is_running {
                    *self = CharacterAnimation::RunUp;
                } else if item.is_walking {
                    *self = CharacterAnimation::WalkUp;
                } else {
                    *self = CharacterAnimation::IdleUp;
                }
            } else {
                if item.is_running {
                    *self = CharacterAnimation::RunDown;
                } else if item.is_walking {
                    *self = CharacterAnimation::WalkDown;
                } else {
                    *self = CharacterAnimation::IdleDown;
                }
            }
        }
    }
}

