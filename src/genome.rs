#[derive(Debug)]
pub struct Gene {
    pub up: u32,
    pub left: u32,
    pub right: u32,
    pub down: u32
}

impl IntoIterator for Gene {
    type Item = u32;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [self.up, self.left, self.right, self.down].into_iter()
    }
}

#[derive(Debug)]
pub struct Genome {
    pub genes: Vec<Gene>
}