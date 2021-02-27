use super::topic::Topic;

pub struct Subject {
    name: Topic,
    pub score: i32,
}

impl Subject {
    pub fn new(name: Topic, score: i32) -> Subject {
        Subject { name, score }
    }

    pub fn clone(&self) -> Subject {
        return Subject {
            name: self.name,
            score: self.score,
        };
    }
}
