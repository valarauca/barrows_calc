

use std::{
    iter::Peekable,
    collections::btree_map::{BTreeMap,Iter},
};


pub fn join_maps<K,V,F>(
    a: BTreeMap<K,V>,
    b: BTreeMap<K,V>,
    lambda: F
) -> BTreeMap<K,V>
where
    K: Ord+Clone,
    V: Clone,
    F: Fn(V,V) -> V,
{
    let j = Joiner {
        a: a.iter().peekable(),
        b: b.iter().peekable(),
        lambda: lambda,
    };
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
    a: Peekable<Iter<'a,K,V>>,
    b: Peekable<Iter<'a,K,V>>,
    lambda: F,
}
impl<'a, K: Ord+Clone, V:Clone, F: Fn(V,V)->V> Iterator for Joiner<'a,K,V,F> {
    type Item = (K,V);
    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.peek(),self.b.peek()) {
            (Option::None,Option::None) => None,
            (Option::Some(_),Option::None) => wtf(self.a.next()),
            (Option::None,Option::Some(_)) => wtf(self.b.next()),
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
