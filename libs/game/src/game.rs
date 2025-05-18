use models::{Card, gen_card};
use platform_types::{command, unscaled};
use xs::{Xs, Seed};

#[derive(Clone, Default)]
pub struct Splat {
    pub x: unscaled::X,
    pub y: unscaled::Y,
}

#[derive(Clone, Default)]
pub struct State {
    pub rng: Xs,
    pub splats: Vec<Splat>,
}

impl State {
    pub fn new(seed: Seed) -> State {
        let rng = xs::from_seed(seed);

        State {
            rng,
            .. <_>::default()
        }
    }

    pub fn add_splat(&mut self) {
        let rng = &mut self.rng;

        let x = unscaled::X(xs::range(rng, 0..command::WIDTH as u32) as command::Inner);
        let y = unscaled::Y(xs::range(rng, 0..command::HEIGHT as u32) as command::Inner);

        self.splats.push(Splat {
            x,
            y,
        });
    }
}
