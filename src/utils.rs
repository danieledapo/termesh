use std::collections::BTreeMap;

pub fn btree_minmax<K, V>(btree: &BTreeMap<K, V>) -> Option<(&K, &K)> {
    // it is guaranteed to be sorted
    let mut it = btree.keys();
    let min = it.next()?;

    // if next_back fails it means there was a single element which is both the
    // min and the max
    let max = it.next_back().unwrap_or(min);

    Some((min, max))
}
