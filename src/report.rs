
#[allow(unused_imports)]
use textplots::{
    Chart,Plot,Shape,
    LabelFormat,
    LineStyle,
    TickDisplay,
    TickDisplayBuilder,
    LabelBuilder,
    AxisBuilder,
};

use crate::{
    stats::Stats,
    term_size::TerminalSize,
};

#[allow(dead_code)]
pub fn build_plot(
    buckets: &Stats,
    _term: &TerminalSize,
) {
    let stats = buckets.get_stats();
    println!("total_samples: {:?}", stats.total_samples);
    println!("min_kc: {:?} max_kc: {:?}", stats.min, stats.max);
    println!("mean: {:?}", stats.mean);
    println!("stddev: {:?}", stats.standard_deviation);
}


