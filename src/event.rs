use crate::{
    constants::DEFAULT_BLOCK_SIZE,
    enums::{Direction, Stage},
    render::State,
};
use bracket_lib::terminal::VirtualKeyCode;

pub fn on_key_pressed(key: Option<VirtualKeyCode>, state: &mut State) {
    if let Some(key) = key {
        let result = match key {
            VirtualKeyCode::Up | VirtualKeyCode::W => {
                if state.stage == Stage::GAMING {
                    state.data.move_box(Direction::UP)
                } else {
                    false
                }
            }
            VirtualKeyCode::Down | VirtualKeyCode::S => {
                if state.stage == Stage::GAMING {
                    state.data.move_box(Direction::DOWN)
                } else {
                    false
                }
            }
            VirtualKeyCode::Left | VirtualKeyCode::A => {
                if state.stage == Stage::GAMING {
                    state.data.move_box(Direction::LEFT)
                } else {
                    false
                }
            }
            VirtualKeyCode::Right | VirtualKeyCode::D => {
                if state.stage == Stage::GAMING {
                    state.data.move_box(Direction::RIGHT)
                } else {
                    false
                }
            }
            VirtualKeyCode::Key2 => {
                state.stage = Stage::GAMING;
                state.reset(2);
                false
            }
            VirtualKeyCode::Key3 => {
                state.stage = Stage::GAMING;
                state.reset(3);
                false
            }
            VirtualKeyCode::Key4 => {
                state.stage = Stage::GAMING;
                state.reset(4);
                false
            }
            VirtualKeyCode::Key5 => {
                state.stage = Stage::GAMING;
                state.reset(5);
                false
            }
            VirtualKeyCode::Key6 => {
                state.stage = Stage::GAMING;
                state.reset(6);
                false
            }
            VirtualKeyCode::Key7 => {
                state.stage = Stage::GAMING;
                state.reset(7);
                false
            }
            VirtualKeyCode::B | VirtualKeyCode::Back => {
                state.stage = Stage::HOME;
                false
            }
            VirtualKeyCode::R => {
                state.stage = Stage::GAMING;
                state.reset(DEFAULT_BLOCK_SIZE);
                false
            }
            VirtualKeyCode::Q => {
                state.quit_game = true;
                false
            }
            VirtualKeyCode::Return => {
                if state.stage == Stage::HOME {
                    state.stage = Stage::GAMING;
                    state.reset(DEFAULT_BLOCK_SIZE);
                }
                false
            }
            _ => false,
        };

        if result {
            if state.data.check_succeed() {
                state.succeed = true;
            } else {
                state.succeed = false;
            }
        }
    }
}
