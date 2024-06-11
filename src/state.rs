//=================================================================================
// Animation State is a trait that allows you to define functionality that will allow
// the state of an animation to be updated based on the world.
//=================================================================================

use std::marker::PhantomData;
use bevy::{ecs::query::{ReadOnlyQueryData, WorldQuery}, prelude::*};
use crate::animation::{update_animators, Animation, Animator};

//=================================================================================
//    AnimationState Plugin
//=================================================================================

pub struct AnimationStatePlugin<A : AnimationState>(PhantomData<A>);

impl <A : AnimationState + Send + Sync + 'static> Default for AnimationStatePlugin<A> {
    fn default() -> Self {
        AnimationStatePlugin(PhantomData)
    }
}

impl <A : AnimationState + Send + Sync + 'static> Plugin for AnimationStatePlugin<A> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, update_states::<A>.before(update_animators::<A>))
        ;
    }
}

//=================================================================================
//    AnimationState Systems
//=================================================================================


pub(crate) fn update_states<A : AnimationState + Send + Sync + 'static>(
    mut states : Query<(&mut Animator<A>, A::StateQuery<'_, '_>)>,
) {
    for (mut state, item) in states.iter_mut() {
        A::update_state(state.as_mut(), &item);
    }
}

//=================================================================================
//    Animation State
//=================================================================================

/// This trait is used to allow animations to update their state based on the compenent the animation is attached to.
pub trait AnimationState : Animation {
    
    /// The query that will allow the animation to read data from the component it is attached to.
    type StateQuery<'w, 's> : ReadOnlyQueryData;
    
    /// This method will update the state of the animation based on the data from the component it is attached to.
    fn update_state(animator : &mut Animator<Self>, data : & <Self::StateQuery<'_, '_> as WorldQuery>::Item<'_>);
}