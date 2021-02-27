use std::collections::HashMap;
// use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod model;

use model::gender::Gender;
use model::student::Student;
use model::subject::Subject;
use model::topic::Topic;

fn main() {
    let path = "D:\\rust_projects\\school\\student_marks.csv";
    let input = File::open(path);
    let mut student_list: Vec<Student> = Vec::new();
    match input {
        Ok(file) => {
            let buffered = BufReader::new(file);
            for line in buffered.lines() {
                match line {
                    Ok(l) => {
                        let string_array: Vec<&str> = l.split(",").collect();
                        let mut subject_list: Vec<Subject> = Vec::with_capacity(5);

                        subject_list
                            .push(Subject::new(Topic::Maths, extract_integer(string_array[2])));
                        subject_list.push(Subject::new(
                            Topic::English,
                            extract_integer(string_array[3]),
                        ));
                        subject_list.push(Subject::new(
                            Topic::Science,
                            extract_integer(string_array[4]),
                        ));
                        subject_list
                            .push(Subject::new(Topic::Hindi, extract_integer(string_array[5])));
                        subject_list.push(Subject::new(
                            Topic::Social,
                            extract_integer(string_array[6]),
                        ));

                        let gender: Gender;

                        if string_array[1] == "female" {
                            gender = Gender::Female
                        } else {
                            gender = Gender::Male
                        }

                        let mut student = Student::new(
                            String::from(string_array[0]),
                            gender,
                            String::default(),
                            subject_list,
                        );
                        let total_score = student.get_score();
                        student.calculate_grade(&total_score);
                        student_list.push(student);
                    }
                    Err(error) => println!("{}", error),
                }
            }
        }
        Err(error) => println!("Path for the file doesnot exist {}", error),
    }

    // println!("{:?}", student_list);

    student_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    // for student in student_list.iter() {
    //     println!("{}", student);
    // }
    let mut gender_map: HashMap<String, Vec<&Student>> = HashMap::with_capacity(2);
    for student in student_list.iter() {
        // let student_gender = &student.gender.to_string();
        // let grade_map_list: Vec<GradeMap>;
        let stud_list: Vec<&Student> = Vec::new();
        let val = gender_map
            .entry(student.gender.to_string())
            .or_insert(stud_list);
        val.push(student);
    }

    // println!("{:?}", genderMap);
    let mut mapofmap: HashMap<String, HashMap<String, Vec<Student>>> = HashMap::new();
    for (key, value) in &gender_map {
        let mut temp_map: HashMap<String, Vec<Student>> = HashMap::new();
        for x in value.iter() {
            let temp_stu = *x;
            if temp_stu.grade == "A+" {
                let stud_list: Vec<Student> = Vec::new();
                let list = temp_map.entry(x.grade.to_string()).or_insert(stud_list);
                list.push(temp_stu.clone());
            // list.sort_by(|a, b| a.get_score().cmp(&b.get_score()));
            } else if x.grade == "A" {
                let stud_list: Vec<Student> = Vec::new();
                let list = temp_map.entry(x.grade.to_string()).or_insert(stud_list);
                list.push(temp_stu.clone());
            // list.sort_by(|a, b| a.get_score().cmp(&b.get_score()));
            } else if x.grade == "B+" {
                let stud_list: Vec<Student> = Vec::new();
                let list = temp_map.entry(x.grade.to_string()).or_insert(stud_list);
                list.push(temp_stu.clone());
            // list.sort_by(|a, b| a.get_score().cmp(&b.get_score()));
            } else if x.grade == "B" {
                let stud_list: Vec<Student> = Vec::new();
                let list = temp_map.entry(x.grade.to_string()).or_insert(stud_list);
                list.push(temp_stu.clone());

            // list.sort_by(|a, b| a.get_score().cmp(&b.get_score()));
            } else {
                let stud_list: Vec<Student> = Vec::new();
                let list = temp_map.entry(x.grade.to_string()).or_insert(stud_list);
                list.push(temp_stu.clone());
                // list.sort_by(|a, b| a.get_score().cmp(&b.get_score()));
            }
        }
        mapofmap.insert(key.to_string(), temp_map);
    }

    let mut gender_map_list: Vec<GenderMap> = Vec::new();

    // println!("{:?}", mapofmap);
    for (key, val) in &mapofmap {
        let internal_map = val;
        let mut temp_grade_list: Vec<GradeMap> = Vec::new();

        for (x, y) in internal_map.iter() {
            let mut stud_list: Vec<&Student> = Vec::new();
            for student in y.iter() {
                stud_list.push(student);
            }
            temp_grade_list.push(GradeMap {
                grade: x.to_string(),
                student_list: stud_list,
            });
        }
        temp_grade_list.sort_by(|a, b| a.grade.cmp(&b.grade));

        gender_map_list.push(GenderMap {
            gender: key.to_string(),
            grade_list: temp_grade_list,
        })
    }
    gender_map_list.sort_by(|a, b| a.gender.cmp(&b.gender));
}

// fn crate_map_of_map(genderMap: &HashMap<String, Vec<Student>>) {
//     let mut mapofmap: HashMap<String, HashMap<String, Vec<Student>>> = HashMap::new();
// }

fn extract_integer(input: &str) -> i32 {
    // let tempVal : i32 = 0;
    match input.parse::<i32>() {
        Ok(result) => return result,
        Err(_) => return 20,
    }
}

struct GenderMap<'a> {
    gender: String,
    grade_list: Vec<GradeMap<'a>>,
}

// impl Hash for GenderMap {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.gender.to_string().hash(state);
//     }
// }

struct GradeMap<'a> {
    grade: String,
    student_list: Vec<&'a Student>,
}

// struct Teacher {
//     name: String,
//     subjects: Vec<Topic>,
// }

// impl Teacher {
//     pub fn new(name: String, subjects: Vec<Topic>) -> Self {
//         return Teacher { name, subjects };
//     }
// }

// fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "({}, {})", self.mapper(), self.score)
// }

// fn mapper(&self) -> String {
//     match self.name {
//         Topic::English => String::from("English"),
//         Topic::Hindi => String::from("Hindi"),
//         Topic::Maths => String::from("Maths"),
//         Topic::Science => String::from("Science"),
//         Topic::Social => String::from("Social"),
//         _ => String::default(),
//     }
// }

// pub trait StringComp: Ord + PartialOrd + PartialEq {
//     fn cmp(&self, other: &Self) -> Ordering;
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
//     fn eq(&self, other: &Self) -> bool;
// }

// impl Ord for Student {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.name.cmp(&other.name)
//     }
// }

// impl PartialEq for Student{
//     fn
// }

// impl StringComp for Student {
//     fn cmp(&self, other: &Self) -> Ordering {
//         return self.name.to_lowercase().cmp(&other.name.to_lowercase());
//     }
// }
