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
        X(if x > X_MAX_INNER { X_MAX_INNER } else { x })
    }

    pub const X_MAX_INNER: Inner = 0x80;

    impl X {
        pub const ONE: Self = Self(1);

        pub fn get(self) -> unscaled::X {
            unscaled::X(self.0.into())
        }
    }

    impl core::ops::AddAssign for X {
        fn add_assign(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            if self.0 > X_MAX_INNER {
                self.0 = X_MAX_INNER;
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

    pub const Y_MAX_INNER: Inner = 0x40;

    /// Clamps to the valid range
    pub fn y(y: Inner) -> Y {
        Y(if y > Y_MAX_INNER { Y_MAX_INNER } else { y })
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
            if self.0 > Y_MAX_INNER {
                self.0 = Y_MAX_INNER;
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
pub struct State {
    pub rng: Xs,
    pub player: Splat,
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut rng = xs::from_seed(seed);

        let x = xy::x(xs::range(&mut rng, 0..xy::X_MAX_INNER as u32) as xy::Inner);
        let y = xy::y(xs::range(&mut rng, 0..xy::X_MAX_INNER as u32) as xy::Inner);

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
}
