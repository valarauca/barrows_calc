

/*
pub fn convert_to_normalized(
    kc_buckets: &BTreeMap<u64,u64>,
    stats: &NonNormalizedStats,
) -> BTreeMap<f64,f64> {
}
*/

#[derive(Clone,Debug)]
pub struct NonNormalizedStats {
    pub min: f64,
    pub max: f64,
    pub samples: f64,
    pub mean: f64,
    pub stddev: f64,
}
impl NonNormalizedStats {

    /// calculate non-normalized stats
    pub fn new(kc_buckets: &BTreeMap<u64,u64>) -> Self {
        let (min_kc, _samples) = kc_buckets.first_key_value().unwrap();
        let (max_kc, _samples) = kc_buckets.last_key_value().unwrap();

        let samples: f64 = kc_buckets.iter().map(|(_,x)| x).sum::<u64>() as f64;

        let mut value_sum = 0.0_f64;
        let mut weight_sum = 0.0_f64;
        for (kc,count) in self.stats.iter() {
            let kc: f64 = *kc as f64;
            let count: f64 = *count as f64;
            value_sum += kc * count;
            weight_sum += count;
        }
        let mean: f64 = value_sum/weight_sum;

        let mut sum = 0.0_f64;
        for (kc,count) in self.stats.iter() {
            let kc: f64 = *kc as f64;
            let weight: f64 = *count as f64 / samples;
            sum += weight * (kc - mean).powi(2)
        }
        let stddev = sum.sqrt();

        Self {
            min: min_kc as f64,
            max: max_kc as f64,
            samples: samples,
            mean: mean,
            stddev: stddev,
        }
    }
}
