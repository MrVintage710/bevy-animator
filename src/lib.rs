pub mod animation;

#[cfg(feature = "aseprite")]
pub mod aseprite;

use std::marker::PhantomData;
use bevy::prelude::*;
use animation::Animation;

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
