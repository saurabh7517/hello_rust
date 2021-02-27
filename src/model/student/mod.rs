use std::fmt;

use super::gender::Gender;
use super::subject::Subject;
// use Gender;
// use Subject;

pub struct Student {
    pub name: String,
    pub gender: Gender,
    pub grade: String,
    subjects: Vec<Subject>,
}

// impl Copy for Student {}
// impl Clone for Student {
//     fn clone(&self) -> Student {
//         *self
//     }
// }

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Name : {}, Gender:{}, Grade : {}, Score : {})",
            self.name,
            self.gender,
            self.grade,
            self.calculate_score()
        )
    }
}

impl fmt::Debug for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Name : {}, Gender:{}, Grade : {}, Score : {})",
            self.name,
            self.gender,
            self.grade,
            self.calculate_score()
        )
    }
}

impl Student {
    pub fn new(name: String, gender: Gender, grade: String, subjects: Vec<Subject>) -> Self {
        Student {
            name,
            gender,
            grade,
            subjects,
        }
    }

    pub fn clone(&self) -> Student {
        return Student {
            name: self.name.clone(),
            gender: self.gender.clone(),
            grade: self.grade.clone(),
            subjects: {
                let mut subject_list: Vec<Subject> = Vec::new();
                for subject in self.subjects.iter() {
                    subject_list.push(subject.clone());
                }
                subject_list
            },
        };
    }

    pub fn get_score(&self) -> i32 {
        return self.calculate_score();
    }

    fn calculate_score(&self) -> i32 {
        let mut total: i32 = 0;
        for subject in self.subjects.iter() {
            total = total + subject.score;
        }
        return (total * 100) / 500;
    }

    // pub fn set_name(&mut self, name: String) {
    //     self.name = name;
    // }

    // pub fn set_gender(&mut self, gender: Gender) {
    //     self.gender = gender;
    // }

    pub fn calculate_grade(&mut self, score: &i32) {
        if score >= &60 && score <= &70 {
            self.grade = "B".to_string();
        } else if score >= &71 && score <= &80 {
            self.grade = "B+".to_string();
        } else if score >= &81 && score <= &90 {
            self.grade = "A".to_string();
        } else if score >= &91 && score <= &100 {
            self.grade = "A+".to_string();
        } else {
            self.grade = "F".to_string();
        }
    }
}
