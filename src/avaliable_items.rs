use std::num::NonZeroUsize;

use rand::{
    Rng,
};

use crate::{
    barrows_items::{Item,Brother},
    barrows_state::{TombState},
};

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct AvaliableItems {
    avaliable: usize,
    number_of_brothers: u32,
    number_of_rolls: u32,
    denom: u32,
    data: [Option<Item>;24]
}
impl Default for AvaliableItems {
    fn default() -> Self {
        Self {
            avaliable: 0,
            data: [None;24],
            number_of_brothers: 0,
            number_of_rolls: 0,
            denom: 0,
        }
    }
}

struct ItemIter<'a, R: Rng> {
    rolls: usize,
    items: Option<Box<AvaliableItems>>,
    rng: &'a mut R,
}
impl<'a,R: Rng> Iterator for ItemIter<'a,R> {
    type Item = Item;
    fn next(&mut self) -> Option<Item> {
        if self.rolls == 0 {
            self.items = None;
            return None;
        }
        if self.items.is_none() {
            self.rolls = 0;
            return None;
        }
        match &mut self.items {
            &mut Option::None => None,
            &mut Option::Some(ref mut items) => {
                if self.rolls == 0 {
                    None
                } else if items.avaliable == 0 {
                    None
                } else {
                    self.rolls -= 1;
                    items.pick_item(self.rng)
                }
            }
        }
    }
}

impl AvaliableItems {

    pub fn build_chest_roll<'a, R: Rng>(&self, tomb_state: &TombState, rng: &'a mut R) -> impl Iterator<Item=Item> + 'a {
        let (count,b) = match tomb_state.get_successful_items_from_chest(rng, self.denom, self.number_of_rolls) {
            Option::None => (0,None),
            Option::Some(x) => {
                let x: usize = x.get();
                assert!(x <= 7);
                (x, Some(Box::new(self.clone())))
            }
        };
        ItemIter {
            rolls: count,
            items: b,
            rng: rng,
        }

        /*
        ItemIter::new(
            tomb_state.get_successful_items_from_chest(rng)
                .map(|x: usize| -> (usize,Box<AvaliableItems>) {
                    (x,Box::new(self.clone()))
                }),
            rng
        )
        */
    }


    /// Pick item handles the semantics of removing that item from avaliable items
    /// when it is choosen
    fn pick_item<R: Rng>(&mut self, rng: &mut R) -> Option<Item> {
        if self.avaliable == 0 {
            return None;
        }
        let idx = rng.gen_range(0,self.avaliable);
        if idx < self.avaliable {
            let item = std::mem::replace(&mut self.data[idx], None);
            debug_assert!(item.is_some());
            self.cleanup_state();
            item
        } else {
            None
        }
    }


    /*
     * Logic to handle building this from an iterator
     *
     */
    fn cleanup_state(&mut self) {
        // push all Option::None to the end
        self.data.as_mut_slice().sort_unstable_by_key(|x: &Option<Item>| -> usize {
            match x {
                &Option::None => 25,
                &Option::Some(ref x) => x.get_index(),
            }
        });
        // set how many are present
        self.avaliable = self.count_avaliable();

        // count brothers slain
        self.number_of_brothers = self.count_brothers_slain() as u32;
        self.number_of_rolls = self.number_of_brothers + 1 as u32;

        if self.number_of_brothers > 6 {
            panic!("this function should be called if number_slain is out-of-bounds");
        }
    
        self.denom = 450 - (58 * self.number_of_brothers);
    }

    pub (in crate) fn set_leagues_multipler(&mut self, multi: Option<NonZeroUsize>) {
        match multi {
            Option::None => { }
            Option::Some(x)  => {
                let x: usize = x.get();
                if x != 1 {
                    self.denom = self.denom / x as u32;
                }
            }
        };
    }

    fn count_brothers_slain(&self) -> usize {
        let mut vec = self.data.iter()
            .filter(|x| x.is_some())
            .map(|x: &Option<Item>| -> Brother { x.clone().unwrap().get_brother() })
            .collect::<Vec<Brother>>();
        vec.sort_unstable();
        vec.dedup();
        vec.len()
    }


    fn count_avaliable(&self) -> usize {
        self.data.iter().map(|x: &Option<Item>| -> usize { x.is_some() as usize }).sum::<usize>()
    }
}
impl FromIterator<Item> for AvaliableItems {
    fn from_iter<I: IntoIterator<Item=Item>>(iter: I) -> Self {
        let mut data = Self::default();
        for item in iter {
            data.data[item.get_index()] = Some(item);
        }
        data.cleanup_state();
        data
    }
}
impl FromIterator<Brother> for AvaliableItems {
    fn from_iter<I: IntoIterator<Item=Brother>>(iter: I) -> Self {
        iter.into_iter()
            .map(|bro: Brother| -> [Item;4] { bro.get_items() })
            .flat_map(|x| x)
            .collect::<AvaliableItems>()
    }
}

#[test]
fn test_avaliable_items() {
    const TWO_BROTHERS: [Brother;2] = [Brother::Ahrim,Brother::Dharok];
    let arg: [Item;8] = [
        Item::AhrimsHood,
        Item::AhrimsRobeTop,
        Item::AhrimsSkirt,
        Item::AhrimsStaff,
        Item::DharoksHelm,
        Item::DharoksPlateBody,
        Item::DharoksPlateLegs,
        Item::DharoksGreatAxe,
    ];


    let tombs: TombState = TWO_BROTHERS.clone().into_iter().collect();
    let a: AvaliableItems = TWO_BROTHERS.clone().into_iter().collect();
    assert_eq!(a.number_of_brothers, 2);
    assert_eq!(a.number_of_rolls, 3);
    assert_eq!(a.denom, 334);
    let b: AvaliableItems = tombs.get_brothers_finished().collect();
    assert_eq!(a,b);

    let mut r = crate::rng::produce_rng().unwrap();

    // run iterator test
    let iter = ItemIter {
        rolls: 8,
        items: Some(Box::new(b.clone())),
        rng: &mut r,
    };

    let mut v = iter.collect::<Vec<Item>>();
    v.sort();
    assert_eq!(v.len(), 8);
    assert_eq!(v.as_slice(), arg.as_slice());

    // run iterator test for greater than item
    let iter = ItemIter {
        rolls: 12,
        items: Some(Box::new(b.clone())),
        rng: &mut r,
    };

    let mut v = iter.collect::<Vec<Item>>();
    v.sort();
    assert_eq!(v.len(), 8);
    assert_eq!(v.as_slice(), arg.as_slice());

    // run another test where we have no items
    let iter = ItemIter {
        rolls: 0,
        items: Some(Box::new(b.clone())),
        rng: &mut r,
    };
    let v2 = iter.collect::<Vec<Item>>();
    assert_eq!(v2.len(), 0);

    // another test where the iter should be empty
    let iter = ItemIter {
        rolls: 10,
        items: None,
        rng: &mut r,
    };
    let v3 = iter.collect::<Vec<Item>>();
    assert_eq!(v3.len(), 0);
}
