use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("vageena");
    }
}

pub struct LoadingState<'a> {
    msg: &'a str,
}

impl Default for LoadingState<'_> {
    fn default() -> Self {
        LoadingState {
            msg: "bewbs",
        }
    }
}

impl SimpleState for LoadingState<'_> {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("{}", self.msg);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("bewbs away...");
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                // Trans::Quit
                Trans::Replace(Box::new(GameState {}))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
