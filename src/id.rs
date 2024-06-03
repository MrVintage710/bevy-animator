//=================================================================================
// The animation ID will tell the animator what animation to play on the component.
// This is the trait that you will define for your animation file. For now we only
// handle 2D animations for now but plan for 3D later.
//=================================================================================

use bevy::{ecs::{query::{QueryData, WorldQuery}, system::SystemParam}, prelude::*};

//=================================================================================
//    Animation
//=================================================================================

pub trait Animation {
    
    type AsociatedAsset : Asset;
    
    type Query<'w, 's> : QueryData;
    
    fn apply(
        &self, 
        items : &mut <Self::Query<'_, '_> as WorldQuery>::Item<'_>, 
        asset : &Self::AsociatedAsset,
        time : &Time,
    );
}

//=================================================================================
//    Animation Plugin
//=================================================================================


