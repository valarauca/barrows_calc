
use std::num::NonZeroUsize;
use clap::{App,Arg,ArgMatches};

use crate::{
    barrows_state::TombState,
    clog::Predicate,
    reader::parse,
};

pub fn app() -> App<'static,'static> {
    App::new("barrows_calc")
        .author("WCL <codylaeder@gmail.com>")
        .version("1.0.0")
        .about("Calculates information about drops at barrows")
        .arg(Arg::with_name("leagues")
             .short("m")
             .long("leagues-modifier")
             .value_name("LEAGUES_MODIFIER")
             .help("multipler for leagues")
             .default_value("1")
             .validator(validate_number))
        .arg(Arg::with_name("threads")
             .long("threads")
             .takes_value(true)
             .value_name("THREADS")
             .help("state number of threads to use")
             .default_value("4")
             .validator(validate_number))
        .arg(Arg::with_name("sigma")
             .short("s")
             .long("sigma")
             .takes_value(true)
             .value_name("SIGMA")
             .help("how many standard deviations do you want to graph")
             .default_value("3")
             .validator(validate_number))
        .arg(Arg::with_name("trials")
             .long("trials")
             .takes_value(true)
             .value_name("THREADS")
             .help("number of trails to run")
             .default_value("1000000")
             .validator(validate_number))
        .arg(Arg::with_name("brothers")
             .short("b")
             .long("brothers")
             .takes_value(true)
             .multiple(true)
             .default_value("all")
             .help("which brothers are you slaying")
             .possible_values(&["all","ahrim", "dharok", "guthan", "karil", "torag", "verac"]))
        .arg(Arg::with_name("want")
             .short("w")
             .long("want")
             .takes_value(true)
             .required(true)
             .validator(validate_wants)
             .help("what do you want"))
}

pub fn from_args(arg: &ArgMatches<'_>) -> (usize,usize,Option<NonZeroUsize>,TombState,Predicate) {
    let multipler = usize::from_str_radix(arg.value_of("leagues").unwrap(),10).unwrap();
    let multipler = if multipler == 0 || multipler == 1 {
        None 
    } else {
        Some(NonZeroUsize::new(multipler).unwrap())
    };
    let threads = usize::from_str_radix(arg.value_of("threads").unwrap(),10).unwrap();
    let trials = usize::from_str_radix(arg.value_of("trials").unwrap(),10).unwrap();
    let mut state = TombState::default();
    for brother in arg.values_of_lossy("brothers").into_iter().flat_map(|x| x) {
        match brother.as_str() {
            "all" => {
                state.ahrim = true;
                state.dharok = true;
                state.guthan = true;
                state.karil = true;
                state.torag = true;
                state.verac = true;
            }
            "ahrim" => {
                state.ahrim = true;
            }
            "dharok" => {
                state.dharok = true;
            }
            "guthan" => {
                state.guthan = true;
            }
            "karil" => {
                state.karil = true;
            }
            "torag" => {
                state.torag = true;
            }
            "verac" => {
                state.verac = true;
            }
            _ => { }
        }
    };

    if state.number_of_brothers_slain() == 0 {
        panic!("you cannot slay 0 brothers and use this calc");
    }
   
    let pred = parse(arg.value_of("want").unwrap()).unwrap();
    (threads, trials, multipler,state, pred)
}

fn validate_number(arg: String) -> Result<(),String> {
    match usize::from_str_radix(&arg,10) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("cannot convert value:'{}' into unsigned int, error:'{:?}'", &arg, e))
    }
}

fn validate_wants(arg: String) -> Result<(),String> {
    match parse(&arg) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}
