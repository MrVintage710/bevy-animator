pub mod animation;

#[cfg(feature = "aseprite")]
pub mod aseprite;

use std::{any::{type_name, TypeId}, marker::PhantomData};
use bevy::{asset::AssetPath, prelude::*};
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
//    SpawnAnimationCommand
//=================================================================================

pub trait SpawnAnimationCommand {
    fn spawn_animation_default<A : Animation + Default>(&mut self, animation : A, handle : Handle<A::AsociatedAsset>, duration : f32);
    
    fn spawn_animation<A : Animation + Default>(&mut self, animation : A, handle : Handle<A::AsociatedAsset>, duration : f32);
}

impl <'w, 's> SpawnAnimationCommand for Commands<'w, 's> {
    fn spawn_animation_default<A : Animation + Default>(&mut self, animation : A, handle : Handle<A::AsociatedAsset>, duration : f32) {
        // let path = file.into();
        self.add(move |world : &mut World| {
            
            // let asset_server = world
            //     .get_resource::<AssetServer>()
            //     .expect("Error while spawning animation: Asset server not initialized. Make sure you have the 'AssetPlugin' added to your app.");
            
            // let assets = world
            //     .get_resource::<Assets<A::AsociatedAsset>>()
            //     .expect(format!("Error while spawning animation: Animation type '{0}' not setup. Setup by adding the 'AnimationPlugin::<{0}>::default()' plugin to your app.", type_name::<A>()).as_str());
        
            // world.spawn((handle, Animator::<A>::default_with_duration(duration)));
            
            // if asset_server.is_loaded_with_dependencies(handle.id()) {
            //     let asset = assets.get(handle.id()).unwrap();
            // } else {
                
            // }
        });
    }

    fn spawn_animation<A : Animation + Default>(&mut self, animation : A, handle : Handle<A::AsociatedAsset>, duration : f32) {
        todo!()
    }
}
