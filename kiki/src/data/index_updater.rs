#[derive(Debug)]
pub struct IndexUpdater {
    index_map: Vec<usize>,
}

impl IndexUpdater {
    pub fn from_map(index_map: Vec<usize>) -> Self {
        Self { index_map }
    }
}

impl IndexUpdater {
    pub fn update(&self, i: usize) -> usize {
        self.index_map[i]
    }
}
