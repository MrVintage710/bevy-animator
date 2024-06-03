pub mod id;
pub mod animator;

#[cfg(feature = "aseprite")]
pub mod aseprite;

use std::marker::PhantomData;
use bevy::prelude::*;
use id::Animation;

//=================================================================================
//    Animation Id
//=================================================================================

#[derive(Default)]
pub struct AnimationPlugin<A : Animation + Send + Sync + 'static>(PhantomData<A>);

impl <A : Animation + Send + Sync + 'static> Plugin for AnimationPlugin<A> {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "aseprite") {
            app.add_plugins(aseprite::AsepriteAnimationPlugin);
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
