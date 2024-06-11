//==============================================================================
// This file contains utility methods that will help you map animations to controls,
// and control the animations in a way that is easy to use. This is a work in progress!!!
//==============================================================================

#[allow(dead_code)]
pub mod controls {
    use bevy::{input::{gamepad::{GamepadAxisType, GamepadButton}, keyboard::KeyCode}, math::Vec2};

    pub struct Axis2d {
        control_mode : ControlMode,
        state : Axis2dDirection
    }
    
    pub enum Axis2dDirection {
        North,
        NorthWest,
        West,
        SouthWest,
        South,
        SouthEast,
        East,
        NorthEast,
        Middle,
        Exact(Vec2),
    }
    
    pub enum ControlMode {
        GamepadAxis { axis : GamepadAxisType },
        GamePadButtons {
            north : GamepadButton, 
            west : GamepadButton, 
            south : GamepadButton, 
            east : GamepadButton,
        },
        Keyboard {
            north : KeyCode,
            west : KeyCode,
            south : KeyCode,
            east : KeyCode,
        }
    }
}