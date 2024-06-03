//=================================================================================
// This is the animator, namesake component that will handle animating components.
// It is generic for any type of animations. It also will have the ability to have
// an animation state machine to that animations can be dynamically changed.
//=================================================================================

use bevy::prelude::*;

use crate::id::Animation;

//=================================================================================
//    Animator
//=================================================================================

pub struct Animater<A : Animation> {
    current_state: A,
}

//=================================================================================
//    AnimationContext
//=================================================================================

pub struct AnimationContext {
    progress : f32,
    
}