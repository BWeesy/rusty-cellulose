#[derive(Debug)]
pub struct Gene (pub u32, pub u32, pub u32, pub u32);

#[derive(Debug)]
pub struct Genome {
    pub genes: Vec<Gene>
}