
pub mod barrows_items;
pub mod barrows_state;
pub mod rng;
pub mod avaliable_items;
pub mod joiner;
pub mod stats;
pub mod rolls;
pub mod clog;
pub mod report;
pub mod cli;
pub mod term_size;
pub mod stat_processing;
pub mod reader;

fn main() {

    let args = cli::app().get_matches();
    let (threads, trials, leagues, state, pred) = cli::from_args(&args);
    println!("wants: {}", &pred);

    let term_size = term_size::TerminalSize::from_values(49,196);
    let mut items = state.get_brothers_finished().collect::<avaliable_items::AvaliableItems>();
    items.set_leagues_multipler(leagues);
    let stats = rolls::get_stats(&state, &items, &pred, trials, threads);
    report::build_plot(&stats,&term_size);
}


