use crate::data::IndexUpdater;

pub fn sort_and_get_index_updater<T: Ord>(v: Vec<T>) -> (Vec<T>, IndexUpdater) {
    let indexed = get_sorted_indexed(v);
    let updater = get_index_updater(&indexed);
    let sorted = indexed.into_iter().map(|(_, item)| item).collect();
    (sorted, updater)
}

fn get_sorted_indexed<T: Ord>(v: Vec<T>) -> Vec<(usize, T)> {
    let mut indexed: Vec<(usize, T)> = v.into_iter().enumerate().collect();
    indexed.sort_by(|(_, a), (_, b)| a.cmp(b));
    indexed
}

fn get_index_updater<T: Ord>(indexed: &[(usize, T)]) -> IndexUpdater {
    let mut changes = get_index_changes(indexed);
    changes.sort_by_key(|change| change.old);
    let index_map = changes.into_iter().map(|change| change.new).collect();
    IndexUpdater::from_map(index_map)
}

#[derive(Debug, Clone, Copy)]
struct IndexChange {
    old: usize,
    new: usize,
}

fn get_index_changes<T: Ord>(indexed: &[(usize, T)]) -> Vec<IndexChange> {
    indexed
        .iter()
        .enumerate()
        .map(|(new_index, (old_index, _))| IndexChange {
            old: *old_index,
            new: new_index,
        })
        .collect()
}
