use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub trait RunConditionsInputExtras: ConditionHelpers {
    fn run_on_mouse_press(
        self,
        mouse_butt: MouseButton,
    ) -> Self {
        self.run_if(move |input: Res<Input<MouseButton>>| {
            input.just_pressed(mouse_butt)
        })
    }
    fn run_on_mouse_release(
        self,
        mouse_butt: MouseButton,
    ) -> Self {
        self.run_if(move |input: Res<Input<MouseButton>>| {
            input.just_released(mouse_butt)
        })
    }
    fn run_on_key_press(
        self,
        key: KeyCode,
    ) -> Self {
        self.run_if(move |input: Res<Input<KeyCode>>| {
            input.just_pressed(key)
        })
    }
    fn run_on_key_release(
        self,
        key: KeyCode,
    ) -> Self {
        self.run_if(move |input: Res<Input<KeyCode>>| {
            input.just_released(key)
        })
    }
    fn run_on_scancode_press(
        self,
        key: ScanCode,
    ) -> Self {
        self.run_if(move |input: Res<Input<ScanCode>>| {
            input.just_pressed(key)
        })
    }
    fn run_on_scancode_release(
        self,
        key: ScanCode,
    ) -> Self {
        self.run_if(move |input: Res<Input<ScanCode>>| {
            input.just_released(key)
        })
    }
}

impl<T: ConditionHelpers> RunConditionsInputExtras for T {}

