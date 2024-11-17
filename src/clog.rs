
use std::collections::BTreeMap;

use crate::barrows_items::Item;

#[derive(Default)]
pub struct Clog {
    kc: u64,
    data: BTreeMap<Item,u64>,
}
impl Clog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_chest<I: IntoIterator<Item=Item>>(&mut self, iter: I) {
        use std::ops::AddAssign;
        self.kc += 1;
        for item in iter.into_iter() {
            self.data.entry(item)
                .and_modify(|value: &mut u64| value.add_assign(1u64))
                .or_insert_with(|| 1u64);
        }
    }

    pub fn contains_item(&self, item: &Item) -> bool {
        self.data.contains_key(item)
    }

    pub fn process_predicate(&self, pred: &Predicate) -> bool {
        pred.calc(self)
    }

    pub fn get_kc(&self) -> u64 {
        self.kc
    }
}

#[test]
fn ensure_clog_works() {
    let mut clog = Clog::default();
    clog.add_chest([Item::AhrimsHood,Item::ToragsHelm]);
    assert_eq!(clog.get_kc(), 1u64);

    assert!(clog.contains_item(&Item::AhrimsHood));
    assert!(clog.contains_item(&Item::ToragsHelm));
    assert!(! clog.contains_item(&Item::VeracsFlail));
}

#[derive(Clone,Debug)]
pub enum Predicate {
    Item(Item),
    And(Box<Self>,Box<Self>),
    Or(Box<Self>,Box<Self>),
}
impl Predicate {
    pub fn calc(&self, clog: &Clog) -> bool {
        match self {
            &Self::Item(ref item) => clog.contains_item(item),
            &Self::And(ref a, ref b) => a.calc(clog) && b.calc(clog),
            &Self::Or(ref a, ref b) => a.calc(clog) || b.calc(clog),
        }
    }
}
impl std::fmt::Display for Predicate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Item(ref i) => fmt.write_str(i.as_str()),
            &Self::And(ref a, ref b) => fmt.write_fmt(format_args!("( {} & {} )", a, b)),
            &Self::Or(ref a, ref b) => fmt.write_fmt(format_args!("( {} | {} )", a, b)),
        }
    }
}

