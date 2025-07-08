#[derive(Default, Clone)]
#[repr(usize)]
pub enum WalkerAnim {
    #[default]
    Idle,
    Run,
    Rise,
    Fall
}

impl WalkerAnim {
    pub const COUNT: usize = 4;
    pub fn usize(&self) -> usize {
        self.clone() as usize
    }
}
