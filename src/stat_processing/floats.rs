
/// Float is a wrapper around the `f64` type
///
/// This prevents constructing a float with non-compariable types
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
pub struct Float {
    f: f64,
}
impl Float {
    pub fn new(f: f64) -> Option<Self> {
        use std::num::FpCategory;
        match f.classify() {
            FpCategory::Nan => None,
            FpCategory::Infinite => None,
            _ => Some(Self { f })
        }
    }
    pub fn get_float(&self) -> f64 {
        self.f
    }
}
impl Eq for Float { }
impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering  {
        self.f.partial_cmp(&other.f).unwrap()
    }
}

