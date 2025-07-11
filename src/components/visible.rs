#[derive(Clone)]
pub struct Visible {
    pub value: bool
}

impl Default for Visible {
    fn default() -> Self {
        Self { value: true }
    }
}
