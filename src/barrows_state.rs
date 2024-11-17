
use std::num::NonZeroUsize;

use rand::Rng;

use crate::{
    barrows_items::{Brother},
};

#[derive(Default)]
pub struct TombState {
    pub ahrim: bool,
    pub dharok: bool,
    pub guthan: bool,
    pub karil: bool,
    pub torag: bool,
    pub verac: bool,
}
impl FromIterator<Brother> for TombState {
    fn from_iter<I: IntoIterator<Item=Brother>>(iter: I) -> Self {
        let mut s = TombState::default();
        for x in iter {
            match x {
                Brother::Ahrim => {
                    s.ahrim = true;
                },
                Brother::Dharok => {
                    s.dharok = true;
                }
                Brother::Guthan => {
                    s.guthan = true;
                }
                Brother::Karil => {
                    s.karil = true;
                }
                Brother::Torag => {
                    s.torag = true;
                }
                Brother::Verac => {
                    s.verac = true;
                }
            };
        }
        s
    }
}
impl TombState {

    pub fn get_brothers_finished(&self) -> impl Iterator<Item=Brother> {
        let brothers: [Option<Brother>; 6] = [
            self.ahrim.then(|| Brother::Ahrim),
            self.dharok.then(|| Brother::Dharok),
            self.guthan.then(|| Brother::Guthan),
            self.karil.then(|| Brother::Karil),
            self.torag.then(|| Brother::Torag),
            self.verac.then(|| Brother::Verac),
        ];
        brothers.into_iter().filter_map(|x: Option<Brother>| -> Option<Brother> { x })
    }

    /*
     * Make some reason about number killed & number of rolls
     *
     */

    pub fn number_of_brothers_slain(&self) -> u32 {
        self.ahrim as u32
            +
        self.dharok as u32
            +
        self.guthan as u32
            +
        self.karil as u32
            +
        self.torag as u32
            +
        self.verac as u32
    }

    pub fn get_number_of_rolls(&self) -> u32 {
        let rolls: u32 = self.number_of_brothers_slain() + 1u32;
        debug_assert!(rolls >= 1u32, "cli requires at least 1 brother slain, rolls:'{:?}' should be >=1", rolls);
        debug_assert!(rolls <= 7u32, "data structure only allows you to slay 6 brothers, rolls:'{:?}' cannot be >7", rolls);
        rolls
    }

    pub fn get_successful_items_from_chest<R: Rng>(&self, rng: &mut R, denom: u32, rolls: u32) -> Option<NonZeroUsize> {
        let slain = self.number_of_brothers_slain();
        debug_assert!(slain >=1, "slain:'{:?}' cli requires slaying at least 1 brother", slain);
        debug_assert!(slain <=6, "slain:'{:?}', you cannot slay more than 6 brothers", slain);
        let mut count = 0usize;
        let ratio: f64 = 1.0_f64 / (denom as f64);
        debug_assert!(ratio <= 1.0_f64);
        for _ in 0..rolls {
            if rng.gen_bool(ratio) {
                count += 1;
            }
        }
        NonZeroUsize::new(count)
    }
}

fn calc_denom(number_slain: u32) -> u32 {
    if number_slain == 0 || number_slain > 6 {
        panic!("this function should be called if number_slain is out-of-bounds");
    }
    450 - (58 * number_slain)
}

#[test]
fn assert_denominator_calc_is_correct() {
    assert_eq!(392, calc_denom(1));
    assert_eq!(334, calc_denom(2));
    assert_eq!(276, calc_denom(3));
    assert_eq!(218, calc_denom(4));
    assert_eq!(160, calc_denom(5));
    assert_eq!(102, calc_denom(6));
}
