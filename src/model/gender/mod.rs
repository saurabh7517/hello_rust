use std::fmt;
pub enum Gender {
    Male,
    Female,
}

impl Copy for Gender {}
impl Clone for Gender {
    fn clone(&self) -> Gender {
        *self
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "{}", "Male"),
            Gender::Female => write!(f, "{}", "Female"),
        }
    }
}
