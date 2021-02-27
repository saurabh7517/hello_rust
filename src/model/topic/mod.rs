pub enum Topic {
    Maths,
    English,
    Science,
    Hindi,
    Social,
}

impl Copy for Topic {}
impl Clone for Topic {
    fn clone(&self) -> Topic {
        *self
    }
}
