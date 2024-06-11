pub mod animation;
pub mod state;
pub mod util;

#[cfg(feature = "aseprite")]
pub mod aseprite;

use bevy::{ecs::system::EntityCommands, prelude::*};
use animation::Animation;

pub mod prelude {
    pub use crate::AnimatorPlugin;
    pub use crate::animation::{Animation, Animator, AnimationPlugin};
    pub use crate::state::{AnimationState, AnimationStatePlugin};
    pub use crate::{InitAnimationCommand, InsertAnimationCommand};
    
    #[cfg(feature = "aseprite")]
    pub use crate::aseprite::{Aseprite, AsepriteAnimation};
}

//=================================================================================
//    AnimatorPLugin
//=================================================================================

/// The Animator Plugin. This will add the animator plugins to the app based on what flags are set.
pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "aseprite") {
            app
                .add_plugins(aseprite::AsepriteAnimationPlugin)
            ;
        }
    }
}

//=================================================================================
//    Animation Spawn Commands
//=================================================================================

pub trait InitAnimationCommand {
    fn init_animation<A : Animation + FromWorld + Send + Sync + 'static>(&mut self, path : &str) -> EntityCommands;
}

impl <'w, 's> InitAnimationCommand for Commands<'w, 's> {
    fn init_animation<A : Animation + FromWorld + Send + Sync + 'static>(&mut self, path : &str) -> EntityCommands {
        let entity = self.spawn_empty().id();
        let path = path.to_string();
        self.add(move |world : &mut World| {
            A::spawn(None, world, path, entity);
        });
        self.entity(entity)
    }
}

pub trait InsertAnimationCommand {
    fn insert_animation<A : Animation + Send + Sync + 'static>(&mut self, animation : A, path : &str) -> EntityCommands;
}

impl <'w, 's> InsertAnimationCommand for Commands<'w, 's> {
    fn insert_animation<A : Animation + Send + Sync + 'static>(&mut self, animation : A, path : &str) -> EntityCommands {
        let entity = self.spawn_empty().id();
        let path = path.to_string();
        self.add(move |world : &mut World| {
            A::spawn(Some(animation), world, path, entity);
        });
        self.entity(entity)
    }
}
