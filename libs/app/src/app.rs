use game::{AdvanceOutcome, Splat};
use gfx::{Commands};
#[allow(unused_imports)]
use platform_types::{command, sprite, unscaled, Button, Input, Speaker, SFX};
pub use platform_types::StateParams;

pub struct State {
    pub game_state: Box<game::State>,
    pub commands: Commands,
    pub input: Input,
    pub speaker: Speaker,
}

impl State {
    pub fn new((seed, logger, error_logger): StateParams) -> Self {
        unsafe {
            features::GLOBAL_LOGGER = logger;
            features::GLOBAL_ERROR_LOGGER = error_logger;
        }

        // We always want to log the seed, if there is a logger available, so use the function,
        // not the macro.
        features::log(&format!("{:?}", seed));

        let game_state = game::State::new(seed);

        Self {
            game_state,
            commands: Commands::default(),
            input: Input::default(),
            speaker: Speaker::default(),
        }
    }
}

impl platform_types::State for State {
    fn frame(&mut self) -> (&[platform_types::Command], &[SFX]) {
        self.commands.clear();
        self.speaker.clear();
        update_and_render(
            &mut self.commands,
            &mut self.game_state,
            self.input,
            &mut self.speaker,
        );

        self.input.previous_gamepad = self.input.gamepad;

        (self.commands.slice(), self.speaker.slice())
    }

    fn press(&mut self, button: Button) {
        if self.input.previous_gamepad.contains(button) {
            //This is meant to pass along the key repeat, if any.
            //Not sure if rewriting history is the best way to do this.
            self.input.previous_gamepad.remove(button);
        }

        self.input.gamepad.insert(button);
    }

    fn release(&mut self, button: Button) {
        self.input.gamepad.remove(button);
    }
}

fn update(state: &mut game::State, input: Input, speaker: &mut Speaker) {
    use game::TimeMode::*;
    match &mut state.time_mode {
        // TODO? Main menu?
        Flowing => {
            if input.pressed_this_frame(Button::START) {
                state.time_mode = Manipulating(state.fresh_time_input());
            } else {
                if input.pressed_this_frame(Button::UP) {
                    state.move_up();
                } else if input.pressed_this_frame(Button::DOWN) {
                    state.move_down();
                } else if input.pressed_this_frame(Button::LEFT) {
                    state.move_left();
                } else if input.pressed_this_frame(Button::RIGHT) {
                    state.move_right();
                }

                state.advance_time();
            }
        },
        Manipulating(ref mut time_input) => {
            if input.pressed_this_frame(Button::START) {
                state.current = time_input.get_value();
                state.time_mode = Flowing
            } else if input.pressed_this_frame(Button::UP) {
                time_input.saturating_add(10);
            } else if input.pressed_this_frame(Button::DOWN) {
                time_input.saturating_sub(10);
            } else if input.pressed_this_frame(Button::LEFT) {
                time_input.saturating_sub(1);
            } else if input.pressed_this_frame(Button::RIGHT) {
                time_input.saturating_add(1);
            } else if input.pressed_this_frame(Button::A) {
                time_input.saturating_add(100);
            } else if input.pressed_this_frame(Button::B) {
                time_input.saturating_sub(100);
            } else if input.pressed_this_frame(Button::SELECT) {
                time_input.reset();
            }
        }
    }
}

#[inline]
fn render(commands: &mut Commands, state: &game::State) {
    use game::TimeMode::*;
    const X_OFFSET: unscaled::W = unscaled::W((command::WIDTH - (game::xy::MAX_W_INNER as unscaled::Inner)) / 2);
    const Y_OFFSET: unscaled::H = unscaled::H((command::HEIGHT - (game::xy::MAX_H_INNER as unscaled::Inner)) / 2);

    // TODO? Bother making these const?
    let box_rect = unscaled::Rect {
        x: unscaled::X(0) + X_OFFSET - unscaled::W(1),
        y: unscaled::Y(0) + Y_OFFSET - unscaled::H(1),
        w: unscaled::W(game::xy::MAX_W_INNER.into()) + unscaled::W(2),
        h: unscaled::H(game::xy::MAX_H_INNER.into()) + unscaled::H(2),
    };
    let time_y = box_rect.y - (gfx::CHAR_H + unscaled::H(2));
    let manipulated_time_y = time_y - (gfx::CHAR_H + unscaled::H(2));
    let error_y = manipulated_time_y - (gfx::CHAR_H + unscaled::H(2));

    commands.draw_box(box_rect, 0);

    for &Splat { x, y, .. } in state.current_splats() {
        commands.draw_pixel(
            x.get() + X_OFFSET,
            y.get() + Y_OFFSET,
            6
        );
    }

    match state.last_outcome {
        AdvanceOutcome::Success => {}
        AdvanceOutcome::OutOfInstants => {
            commands.print(
                b"64k instants ought to be enough for anybody!",
                unscaled::X(0) + gfx::CHAR_W,
                error_y,
                6,
            );
        },
        AdvanceOutcome::OutOfSplats => {
            commands.print(
                b"256 selves ought to be enough for anybody!",
                unscaled::X(0) + gfx::CHAR_W,
                error_y,
                6,
            );
        },
    }

    match state.time_mode {
        Flowing => {},
        Manipulating(ref time_input) => {
            commands.print(
                format!("{}", time_input.get_value()).as_bytes(),
                box_rect.x,
                manipulated_time_y,
                2,
            );
        },
    }

    commands.print(
        format!("{}", state.current).as_bytes(),
        box_rect.x,
        time_y,
        6,
    );
}

#[inline]
fn update_and_render(
    commands: &mut Commands,
    state: &mut game::State,
    input: Input,
    speaker: &mut Speaker,
) {
    update(state, input, speaker);
    render(commands, state);
}
