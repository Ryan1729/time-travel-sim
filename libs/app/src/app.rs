use game::Splat;
use gfx::{Commands};
#[allow(unused_imports)]
use platform_types::{command, sprite, unscaled, Button, Input, Speaker, SFX};
pub use platform_types::StateParams;

pub struct State {
    pub game_state: game::State,
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

        let mut game_state = game::State::new(seed);
        //game_state.add_splat();

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
    if input.pressed_this_frame(Button::UP) {
        state.move_up();
    } else if input.pressed_this_frame(Button::DOWN) {
        state.move_down();
    } else if input.pressed_this_frame(Button::LEFT) {
        state.move_left();
    } else if input.pressed_this_frame(Button::RIGHT) {
        state.move_right();
    }
//
    //state.advance_time();
}

#[inline]
fn render(commands: &mut Commands, state: &game::State) {
    let X_OFFSET: unscaled::W = unscaled::W((command::WIDTH - (game::xy::MAX_W_INNER as unscaled::Inner)) / 2);
    let Y_OFFSET: unscaled::H = unscaled::H((command::HEIGHT - (game::xy::MAX_H_INNER as unscaled::Inner)) / 2);

    commands.draw_box(
        unscaled::Rect {
            x: unscaled::X(0) + X_OFFSET - unscaled::W(1),
            y: unscaled::Y(0) + Y_OFFSET - unscaled::H(1),
            w: unscaled::W(game::xy::MAX_W_INNER.into()) + unscaled::W(2),
            h: unscaled::H(game::xy::MAX_H_INNER.into()) + unscaled::H(2),
        },
        0
    );
    let &Splat { x, y, .. } = &state.player;
    commands.draw_pixel(
        x.get() + X_OFFSET,
        y.get() + Y_OFFSET,
        6
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
