use csv::ReaderBuilder;
use std::{
    collections::HashMap,
    error::Error,
};
const ENROLMENTS_PATH: &str = "enrolments.psv";

#[derive(serde::Deserialize)]
struct Student {
    course_code: String,
    student_number: String,
    name: String,
    program: String,
    plan: String,
    wam: f32,
    session: String,
    birthdate: String,
    sex: String,
}

fn main() {
    let mut stu_info: HashMap<String, Student> = HashMap::new();
    let mut course: HashMap<String, i32> = HashMap::new();
    let _ = process(
        &mut stu_info,
        &mut course,
        ENROLMENTS_PATH,
    );

    
}

fn process(
    stu_info: &mut HashMap<String, Student>,
    course: &mut HashMap<String, i32>,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut rd = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(path)?;

    let mut enrollments: Vec<Student> = Vec::new();

    for result in rd.records() {
        let student: Student = result?.deserialize(None)?;
        enrollments.push(student);
    }

    enrollments.into_iter().for_each(|stu| {
        course
            .entry(stu.course_code.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        stu_info.entry(stu.student_number.clone()).or_insert(stu);
    });

    let max_course = course.iter().max_by_key(|(_, count)| *count);
    let min_course = course.iter().min_by_key(|(_, count)| *count);

    println!("Number of students: {}", stu_info.len());
    println!(
        "Most common course: {} with {} students",
        max_course.unwrap().0,
        max_course.unwrap().1
    );
    println!(
        "Least common course: {} with {} students",
        min_course.unwrap().0,
        min_course.unwrap().1
    );


    let mut total_wam:f32 = 0.0;
    stu_info.into_iter().for_each(|(_, stu)| {
        total_wam += stu.wam;
    });

    println!("Average WAM: {:.2}",total_wam/stu_info.len() as f32);
  
    Ok(())
}
