
use std::{
    thread::{self,},
    sync::mpsc::{channel},
};

use rand::Rng;
use crate::{
    rng::{produce_rng},
    barrows_state::{TombState},
    clog::{Clog,Predicate},
    avaliable_items::AvaliableItems,
    stats::Stats,
};


pub fn get_stats(
    state: &TombState,
    avaliable: &AvaliableItems,
    pred: &Predicate,
    trials: usize,
    threads: usize,
) -> Stats {

    thread::scope(|s| {
        let (snd,rcv) = channel();
        let trials: usize = trials / threads;
        for id in 0..threads {
            let handle: thread::ScopedJoinHandle<Stats> = thread::Builder::new()
                .name(format!("barrows-calc-thread-{:?}", id))
                .stack_size(1024 * 1024)
                .spawn_scoped(s, move || -> Stats {
                    let mut rng = produce_rng().unwrap();
                    collect_stats(state,avaliable, pred, &mut rng, trials)
                })
                .unwrap();
            snd.send(handle).unwrap();
        }

        let mut stats = Stats::default();
        for _ in 0..threads {
            let id = rcv.recv().unwrap();
            stats.merge(id.join().unwrap());
        }
        stats
    })
}

fn collect_stats<R: Rng>(
    state: &TombState,
    avaliable: &AvaliableItems,
    pred: &Predicate,
    rng: &mut R,
    trials: usize,
) -> Stats {
    let mut stats = Stats::default();
    for _ in 0..trials {
        let kc = kc_to_predicate(state,avaliable,pred,rng);
        stats.add_kc(kc);
    }
    stats
}

fn kc_to_predicate<R: Rng>(
    state: &TombState,
    avaliable: &AvaliableItems,
    pred: &Predicate,
    rng: &mut R,
) -> u64 {
    let mut clog = Clog::new();
    loop {
        perform_single_roll(state, avaliable, &mut clog, rng);
        if clog.process_predicate(pred) {
            return clog.get_kc();
        }
    }
}

fn perform_single_roll<R: Rng>(
    state: &TombState,
    avaliable: &AvaliableItems,
    clog: &mut Clog,
    rng: &mut R
) {
    clog.add_chest(avaliable.build_chest_roll(state, rng));
}

