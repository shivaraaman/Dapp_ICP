use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::cell::RefCell;

#[derive(Clone, CandidType, Deserialize)]
struct Student {
    name: String,
    total: u32,
    subjects: u32,
    average: f32,
    grade: String,
}

thread_local! {
    static STUDENTS: RefCell<Vec<Student>> = RefCell::new(Vec::new());
}

fn calculate_grade(avg: f32) -> String {
    if avg >= 90.0 {
        "A".to_string()
    } else if avg >= 75.0 {
        "B".to_string()
    } else if avg >= 60.0 {
        "C".to_string()
    } else {
        "D".to_string()
    }
}

#[update]
fn add_student(name: String, total: u32, subjects: u32) {
    let average = total as f32 / subjects as f32;
    let grade = calculate_grade(average);
    let student = Student { name, total, subjects, average, grade };
    STUDENTS.with(|s| s.borrow_mut().push(student));
}

#[query]
fn get_students() -> Vec<Student> {
    STUDENTS.with(|s| s.borrow().clone())
}
