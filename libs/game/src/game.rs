use models::{Card, gen_card};
use platform_types::{command, unscaled, PaletteIndex};
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

#[derive(Clone, Copy, Default)]
pub struct Player {
    pub x: X,
    pub y: Y,
}

#[derive(Clone, Copy, Default)]
pub struct Splat {
    pub x: X,
    pub y: Y,
    pub colour: PaletteIndex,
}

pub const SPLAT_COUNT: u16 = u8::MAX as u16 + 1;
pub type SplatIndex = u8;

#[derive(Clone)]
pub struct Instant {
    pub splats: [Splat; SPLAT_COUNT as _],
    pub one_past_last: SplatIndex,
}

impl Default for Instant {
    fn default() -> Self {
        Self {
            splats: [Splat::default(); SPLAT_COUNT as _],
            one_past_last: 0,
        }
    }
}

pub const INSTANT_COUNT: u32 = u16::MAX as u32 + 1;
pub type InstantIndex = u16;

#[derive(Clone, Copy, Default)]
pub enum AdvanceOutcome {
    #[default]
    Success,
    OutOfInstants,
    OutOfSplats,
}

#[derive(Clone, Copy, Default)]
pub struct TimeInput {
    current: InstantIndex,
    initial: InstantIndex,
}

impl TimeInput {
    pub fn saturating_add(&mut self, to_add: InstantIndex) {
        self.current = self.current.saturating_add(to_add);
    }

    pub fn saturating_sub(&mut self, to_sub: InstantIndex) {
        self.current = self.current.saturating_sub(to_sub);
    }

    pub fn reset(&mut self) {
        self.current = self.initial;
    }

    pub fn get_value(&self) -> InstantIndex {
        self.current
    }
}

#[derive(Clone, Copy, Default)]
#[must_use]
pub struct CollisionError {
    // TODO? The location where the collision happened?
}

#[derive(Clone, Copy, Default)]
pub enum TimeMode {
    // TODO? a dev feature to skip the main menu? Maybe just a cli arg
    #[default]
    MainMenu,
    Flowing,
    Manipulating(TimeInput),
    Collision(CollisionError)
}

#[derive(Clone)]
pub struct State {
    pub rng: Xs,
    pub instants: [Instant; INSTANT_COUNT as _],
    pub current: InstantIndex,
    pub player: Player,
    pub last_outcome: AdvanceOutcome,
    pub time_mode: TimeMode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rng: Xs::default(),
            instants: core::array::from_fn(|_| Instant::default()),
            current: 0,
            player: <_>::default(),
            last_outcome: <_>::default(),
            time_mode: <_>::default(),
        }
    }
}

impl State {
    pub fn new(seed: Seed) -> Box<State> {
        let mut rng = xs::from_seed(seed);

        let x = xy::x(xs::range(&mut rng, 0..xy::MAX_W_INNER as u32) as xy::Inner);
        let y = xy::y(xs::range(&mut rng, 0..xy::MAX_H_INNER as u32) as xy::Inner);

        let mut output: Box<State> = <_>::default();

        output.rng = rng;
        output.player = Player {
            x,
            y,
        };

        output
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
        self.last_outcome = self.advance_time_inner();
    }

    fn advance_time_inner(&mut self) -> AdvanceOutcome {
        // If we are at the last index already
        if u32::from(self.current) == (INSTANT_COUNT - 1) {
            return AdvanceOutcome::OutOfInstants
        }
        let new_splats: &mut [Splat; SPLAT_COUNT as _] = &mut self.instants[self.current as usize].splats;
        let one_past_last = &mut self.instants[self.current as usize].one_past_last;

        // If we are at the last index already
        if u16::from(*one_past_last) == SPLAT_COUNT - 1 {
            return AdvanceOutcome::OutOfSplats
        }
        new_splats[*one_past_last as usize] = Splat {
            x: self.player.x,
            y: self.player.y,
            colour: 6,
        };

        *one_past_last += 1;

        self.current += 1;

        AdvanceOutcome::Success
    }

    // TODO? Make collision detection optional?
    pub fn check_collision(&self) -> Result<(), CollisionError> {
        for splat in self.current_non_player_splats() {
            if self.player.x.get() == splat.x.get()
            && self.player.y.get() == splat.y.get() {
                return Err(CollisionError{})
            }
        }

        Ok(())
    }

    fn current_non_player_splats(&self) -> &[Splat] {
        use TimeMode::*;
        let current = match self.time_mode {
            MainMenu | Collision(_) => {
                return &[]
            },
            Flowing => self.current,
            Manipulating(ref time_input) => time_input.get_value(),
        };

        let instant: &Instant = &self.instants[current as usize];

        &instant.splats[0..instant.one_past_last as usize]
    }

    pub fn current_splats(&self) -> (&[Splat], Splat) {
        use TimeMode::*;
        (
            self.current_non_player_splats(),
            Splat {
                x: self.player.x,
                y: self.player.y,
                colour: match (self.check_collision(), self.time_mode) {
                    (Err(_), _) => 2,
                    (Ok(()), MainMenu | Collision(_)) => 0,
                    (Ok(()), Flowing) => 6,
                    (Ok(()), Manipulating(_)) => 1,
                },
            }
        )
    }

    pub fn fresh_time_input(&self) -> TimeInput {
        TimeInput {
            current: self.current,
            initial: self.current,
        }
    }
}
