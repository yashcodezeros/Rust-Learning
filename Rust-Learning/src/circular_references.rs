//e.g - Students have take courses. Or Courses assigned to students (many-to-many relationships)
//Rc and RefCell

use std::cell::RefCell;
use std::rc::Rc;

// struct Student<'a> {
//     name: String,
//     courses: Vec<&'a Course<'a>> //to overcome problem with lifetime we have use Rc and RefCell
// }

// impl<'a> Student<'a> {
//     fn new(name: &str) -> Student<'a> {
//         Student {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }

// struct Course<'a> {
//     name: String,
//     students: Vec<&'a Student<'a>>
// }

// impl<'a> Course<'a> {
//     fn new(name: &str) -> Course<'a> {
//         Course {
//             name: name.into(),
//             students: Vec::new(),
//         }
//     }
//    fn add_students(&'a mut self, student: &'a mut Student<'a>) {
//       student.courses.push(self);
//       self.student.push(student);
//    }
// }

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>,
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new(),
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>,
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new(),
        }
    }

    fn add_students(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student.clone());
        course.borrow_mut().students.push(student);
    }
}

//a new option i.e like - database normalization.
//no need to take Student , Course
//create a vector of Enrollment of student, course and you will get the data of both struct like Vec<Enrollment{course,student}>

struct StudentNew {
    name: String,
}

impl StudentNew {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct CourseNew {
    name: String,
}

struct Enrollment<'a> {
    student: &'a StudentNew,
    course: &'a CourseNew,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a StudentNew, course: &'a CourseNew) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new(),
        }
    }
    fn enroll(&mut self, student: &'a StudentNew, course: &'a CourseNew) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

pub fn run() {
    let johny = Rc::new(RefCell::new(Student::new("Johnathan")));
    let jane = Rc::new(RefCell::new(Student::new("Jane")));
    let course = Rc::new(RefCell::new(Course::new("Rust")));

    // course.add_students("john"); //Rc

    Course::add_students(course.clone(), johny);
    Course::add_students(course, jane);

    let adam = StudentNew {
        name: "Adam".into(),
    };
    let course2 = CourseNew {
        name: "Intro to rust".into(),
    };

    let mut platform = Platform::new();
    platform.enroll(&adam, &course2);

    for c in adam.courses(platform) {
        println!("Adam is taking {}", c);
    }
}
