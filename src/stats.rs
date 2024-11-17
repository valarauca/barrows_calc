use std::{
    iter::Peekable,
    collections::btree_map::{BTreeMap,Iter},
    ops::AddAssign,
};


// using an external library for simple maths because I CBA
use average::{
    Skewness,
    //WeightedMeanWithError,
};

#[derive(Default)]
pub struct Stats {
    stats: BTreeMap<u64,u64>
}
impl Stats {

    pub fn add_kc(&mut self, kc: u64) {
        self.stats.entry(kc)
            .and_modify(|v: &mut u64| v.add_assign(1u64))
            .or_insert_with(|| 1u64);
    }
    pub fn merge(&mut self, other: Self) {
        // TODO: rewrite this when cursor get stablized
        let old_self = std::mem::replace(&mut self.stats, BTreeMap::default());
        self.stats = join_maps(
            old_self,
            other.stats,
            |a,b| a+b
        );
    }

    /*
     * Methods to get information about samples
     *
     */

    fn get_samples<'a>(&'a self) -> impl Iterator<Item=f64> + 'a {
        self.stats.iter()
            .map(|(kc,count): (&u64,&u64)| -> (f64,u64) {
                (*kc as f64, *count)
            })
            .flat_map(|(kc, count): (f64,u64)| std::iter::repeat(kc).take(count as usize))
    }

    fn get_total_samples(&self) -> u64 {
        self.stats.iter().map(|(_,x)| x).sum::<u64>()
    }
    fn get_min_kc(&self) -> f32 {
        let (kc, _samples) = self.stats.first_key_value().unwrap();
        kc.clone() as f32
    }

    fn get_max_kc(&self) -> f32 {
        let (kc, _samples) = self.stats.last_key_value().unwrap();
        kc.clone() as f32
    }

    pub fn basic_stats(&self) -> (f64,f64) {
        let samples = self.get_total_samples() as f64;
        
        let mut value_sum = 0.0_f64;
        let mut weight_sum = 0.0_f64;
        for (kc,count) in self.stats.iter() {
            let kc: f64 = *kc as f64;
            let count: f64 = *count as f64;
            value_sum += kc * count;
            weight_sum += count;
        }
        let mean = value_sum/weight_sum;

        let mut sum = 0.0_f64;
        for (kc,count) in self.stats.iter() {
            let kc: f64 = *kc as f64;
            let weight: f64 = *count as f64 / samples;
            sum += weight * (kc - mean).powi(2)
        }
        let stddev = sum.sqrt();
        (mean,stddev)
    }

    pub fn get_stats(&self) -> StatsData {
        let skew = self.get_samples()
            .collect::<Skewness>();

        let popvar = skew.population_variance();
        let stdev = popvar.sqrt();
        StatsData {
            total_samples: skew.len() as f32,
            min: self.get_min_kc(),
            max: self.get_max_kc(),
            mean: skew.mean() as f32,
            standard_deviation: stdev as f32,
        }
    }


    pub fn build_plot(&self, stats: &StatsData, sigma: f32) -> DataPlot {

        let mut vec: Vec<(f32,f32)> = Vec::with_capacity(self.stats.len());
        let (lower_bound,upper_bound) = stats.calc_bounds(sigma);
        let lower_bound = if lower_bound <= 0.0 { stats.min } else { lower_bound };

        let mut kc_max = std::f32::MIN;
        let mut count_max = std::f32::MIN;

        let mut kc_min = std::f32::MAX;
        let mut count_min = std::f32::MAX;

        for (kc,count) in self.stats.iter() {
            let kc: f32 = kc.clone() as f32;
            let count: f32 = count.clone() as f32;
            if kc < lower_bound || kc > upper_bound {
                // don't graph this sample
                continue;
            }

            vec.push( (kc, count) );

            if kc > kc_max {
                kc_max = kc;
            }
            if kc < kc_min {
                kc_min = kc;
            }

            if count > count_max {
                count_max = count;
            }
            if count < count_min {
                count_min = count;
            }
        }


        DataPlot {
            data: vec,
            x_min: kc_min,
            x_max: kc_max,
            y_min: count_min,
            y_max: count_max,
        }
    }
}

pub struct DataPlot {
    pub data: Vec<(f32,f32)>,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

#[derive(Clone)]
pub struct StatsData {
    pub total_samples: f32,
    pub min: f32,
    pub max: f32,
    pub mean: f32,
    pub standard_deviation: f32,
}
impl StatsData {

    fn calc_bounds(&self, sigma: f32) -> (f32,f32) {
        let lower_bound: f32 = self.mean - (self.standard_deviation * sigma.abs());
        let upper_bound: f32 = self.mean + (self.standard_deviation * sigma.abs());
        (lower_bound, upper_bound)
    }

    pub fn generate_log_normal(&self) -> Box<dyn Fn(f32)->f32> {
        let stddev: f64 = self.standard_deviation.clone() as f64;
        let mean: f64 = self.mean.clone() as f64;
        let val: f64 = (2.0_f64 * std::f64::consts::PI).sqrt();
        Box::new(move |x: f32| -> f32 {
            let x: f64 = x as f64;

            let output: f64 = {
                (
                    (1.0_f64) / ( x * stddev * val)
                )
                *
                (
                    -(
                        (x.ln() - mean).powi(2)
                            /
                        (2.0_f64 * stddev.powi(2))
                    )
                ).exp() 
            };
            output as f32
        })
    }
}


/*
 * Boilerplate to join maps in O(m+n) time
 *
 */
fn join_maps<K,V,F>(
    a: BTreeMap<K,V>,
    b: BTreeMap<K,V>,
    lambda: F
) -> BTreeMap<K,V>
where
    K: Ord+Clone,
    V: Clone,
    F: Fn(V,V) -> V,
{
    let j = Joiner::new(&a,&b, lambda);
    BTreeMap::from_iter(j)
}

#[test]
fn test_join_maps() {
    let a: BTreeMap<usize,usize> = vec![(1,1),(2,2),(15,1)].into_iter().collect();
    let b: BTreeMap<usize,usize> = vec![(1,1),(16,2),(15,2),(25,6)].into_iter().collect();

    let out = join_maps(a,b,|a,b|a+b);
    assert_eq!(out.get(&1).unwrap(), &2usize);
    assert_eq!(out.get(&2).unwrap(), &2usize);
    assert_eq!(out.get(&15).unwrap(), &3usize);
    assert_eq!(out.get(&16).unwrap(), &2usize);
    assert_eq!(out.get(&25).unwrap(), &6usize);
    assert_eq!(out.len(), 5);
}

struct Joiner<'a, K: Ord+Clone,V:Clone, F: Fn(V,V) -> V> {
    min_length: usize,
    max_length: usize,
    finished: bool,
    a: Peekable<Iter<'a,K,V>>,
    b: Peekable<Iter<'a,K,V>>,
    lambda: F,
}
impl<'a, K: Ord+Clone, V: Clone, F: Fn(V,V) -> V> Joiner<'a,K,V,F> {
    fn new(
        map_a: &'a BTreeMap<K,V>,
        map_b: &'a BTreeMap<K,V>,
        lambda: F,
    ) -> Joiner<'a,K,V,F> {
        let min_length = std::cmp::max(map_a.len(), map_b.len());
        let max_length = map_a.len() + map_b.len();
        Self {
            min_length,
            max_length,
            finished: false,
            a: map_a.iter().peekable(),
            b: map_b.iter().peekable(),
            lambda,
        }
    }
}
impl<'a, K: Ord+Clone, V: Clone, F: Fn(V,V)->V> Iterator for Joiner<'a,K,V,F> {
    type Item = (K,V);

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.min_length, Some(self.max_length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match (self.a.peek(),self.b.peek()) {
            (Option::None,Option::None) => {
                self.finished = true;
                None
            }
            (Option::Some(_),Option::None) => {
                wtf(self.a.next())
            }
            (Option::None,Option::Some(_)) => {
                wtf(self.b.next())
            }
            (Option::Some(&(a_key,_)),Option::Some(&(b_key,_))) => {
                if a_key < b_key {
                    wtf(self.a.next())
                } else if b_key < a_key {
                    wtf(self.b.next())
                } else {
                    let (a_key,a_val) = self.a.next().unwrap();
                    let (_,b_val) = self.b.next().unwrap();
                    Some((a_key.clone(), (self.lambda)(a_val.clone(),b_val.clone())))
                }
            }
        }
    }
}


fn wtf<'a,A:Clone,B:Clone>(x: Option<(&'a A, &'a B)>) -> Option<(A,B)> {
    x.map(|(a,b)| (a.clone(),b.clone()))
}
