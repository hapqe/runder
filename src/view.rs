use getset::Getters;

#[derive(Getters, Clone, Copy)]
pub struct ViewInfo {
    #[getset(get = "pub")]
    pub lenght: u64,
    #[getset(get = "pub")]
    pub offset: u64,
}

impl ViewInfo {
    pub fn new(lenght: u64, offset: u64) -> Self {
        Self { lenght, offset }
    }
}

#[derive(Clone, Copy)]
pub enum ViewType {
    Vertex(ViewInfo),
    Index(ViewInfo),
}
