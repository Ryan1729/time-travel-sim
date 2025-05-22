use models::{Card, gen_card};
use platform_types::{command, unscaled};
use xs::{Xs, Seed};

pub mod xy {
    use super::*;

    pub type Inner = u8;

    #[derive(Clone, Copy, Default)]
    pub struct X(Inner);

    /// Clamps to the valid range
    pub fn x(x: Inner) -> X {
        X(if x > MAX_W_INNER { MAX_W_INNER } else { x })
    }

    pub const MAX_W_INNER: Inner = 0x80;

    impl X {
        pub const ONE: Self = Self(1);

        pub fn get(self) -> unscaled::X {
            unscaled::X(self.0.into())
        }
    }

    impl core::ops::AddAssign for X {
        fn add_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > MAX_W_INNER {
                self.0 = MAX_W_INNER;
            }
        }
    }

    impl core::ops::SubAssign for X {
        fn sub_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }

    #[derive(Clone, Copy, Default)]
    pub struct Y(Inner);

    pub const MAX_H_INNER: Inner = 0x40;

    /// Clamps to the valid range
    pub fn y(y: Inner) -> Y {
        Y(if y > MAX_H_INNER { MAX_H_INNER } else { y })
    }

    impl Y {
        pub const ONE: Self = Self(1);

        pub fn get(self) -> unscaled::Y {
            unscaled::Y(self.0.into())
        }
    }

    impl core::ops::AddAssign for Y {
        fn add_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > MAX_H_INNER {
                self.0 = MAX_H_INNER;
            }
        }
    }

    impl core::ops::SubAssign for Y {
        fn sub_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_sub(other.0);
        }
    }
}
pub use xy::{X, Y};


#[derive(Clone, Default)]
pub struct Splat {
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Default)]
struct Instant {
    pub splats: Vec<Splat>,
}

#[derive(Clone, Default)]
pub struct State {
    pub rng: Xs,
    // TODO change to
    // pub instants: [Instant; INSTANT_COUNT as _],
    // pub instant_index: InstantIndex,
    pub instants: Vec<Instant>,
    pub instant_index: usize,
    pub player: Splat,
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut rng = xs::from_seed(seed);

        let x = xy::x(xs::range(&mut rng, 0..xy::MAX_W_INNER as u32) as xy::Inner);
        let y = xy::y(xs::range(&mut rng, 0..xy::MAX_H_INNER as u32) as xy::Inner);

        State {
            rng,
            player: Splat {
                x,
                y,
            },
            .. <_>::default()
        }
    }

    pub fn move_up(&mut self) {
        self.player.y -= Y::ONE;
    }

    pub fn move_down(&mut self) {
        self.player.y += Y::ONE;
    }
    pub fn move_left(&mut self) {
        self.player.x -= X::ONE;
    }
    pub fn move_right(&mut self) {
        self.player.x += X::ONE;
    }

    pub fn advance_time(&mut self) {
        self.instant_index += 1;

        let new_splats: &mut Vec<Splat> =
            if let Some(instant) = self.instants.get_mut(self.instant_index) {
                &mut instant.splats
            } else {
                self.instants.push(Instant::default());
                self.instant_index = self.instants.len() - 1;

                &mut self.instants[self.instant_index].splats
            };
        new_splats.push(self.player.clone());
    }

    pub fn current_splats(&self) -> Box<dyn Iterator<Item = &Splat> + '_> {
        if let Some(instant) = self.instants.get(self.instant_index) {
            Box::new(instant.splats.iter().chain(std::iter::once(&self.player)))
        } else {
            Box::new(std::iter::empty())
        }
    }
}
