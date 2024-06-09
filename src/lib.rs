pub mod animation;
pub mod state;

#[cfg(feature = "aseprite")]
pub mod aseprite;

use std::{any::{type_name, TypeId}, marker::PhantomData};
use bevy::{asset::AssetPath, ecs::system::EntityCommands, prelude::*};
use animation::{Animation, Animator};

//=================================================================================
//    AnimatorPLugin
//=================================================================================

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "aseprite") {
            app.add_plugins(aseprite::AsepriteAnimationPlugin);
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
            A::spawn(world, path, entity);
        });
        self.entity(entity)
    }
}

pub trait InsertAnimationCommand<A : Animation> {
    fn insert_animation<'p>(&mut self, animation : A, path : impl Into<AssetPath<'p>>);
}

impl <'w, 's, A : Animation + FromWorld> InsertAnimationCommand<A> for Commands<'w, 's> {
    fn insert_animation<'p>(&mut self, animation : A, path : impl Into<AssetPath<'p>>) {
        
    }
}
