#[derive(Debug)]
pub struct Course {
    name: String,
}



impl Course {
    pub fn new(name: String) -> Course {
        Course {
            name,
        }
    }
}


#[derive(Debug)]
pub struct Class {
    name: String,

}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
        }
    }
}

#[derive(Debug)]
pub struct Student {
    name: String,
    class: Option<Class>,
    courses: Vec<Course>,
}


impl Student {
    pub fn new(name: String) -> Student {
        Student {
            name,
            class: None,
            courses: Vec::new(),
        }
    }

    pub fn join_class(self: Self, class_name: String) -> Student {
        let class = Class::new(class_name);
        Student {
            class: Some(class),
            ..self
        }
    }


    pub fn change_class(self: Self, class_name: String) -> Student {
        let class = Class::new(class_name);
        Student {
            class: Some(class),
            courses: Vec::new(),
            ..self
        }
    }

    pub fn quit_class(self: Self) -> Student {
        Student {
            class: None,
            courses: Vec::new(),
            ..self
        }
    }

    pub fn join_course(self: &mut Self, course_name: String) {
        let course = Course::new(course_name);
        self.courses.push(course);
    }

    pub fn quit_course(self: &mut Self, course_name: String)  {
        let course = Course::new(course_name);

        if let Some(index) = self.courses.iter().position(|x| x.name==course.name ) {
            self.courses.remove(index);
        }
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_class() {
        let student = Student::new(String::from("stu"));
        let student = student.join_class(String::from("class01"));
        println!("join_class: {:?}", student);
    }

    #[test]
    fn change_class() {
        let student = Student::new(String::from("stu"));
        let student = student.change_class(String::from("class02"));
        println!("change_class: {:?}", student);
    }

    #[test]
    fn quit_class() {
        let student = Student::new(String::from("stu"));
        let student = student.quit_class();
        println!("quit_class: {:?}", student);
    }


    #[test]
    fn join_course() {
        let student = Student::new(String::from("stu"));
        let mut student = student.join_class(String::from("class01"));
        student.join_course(String::from("english"));
        student.join_course(String::from("math"));
        student.join_course(String::from("physics"));
        println!("join_course: {:?}", student);
    }


    #[test]
    fn quit_course() {
        let student = Student::new(String::from("stu"));
        let mut student = student.join_class(String::from("class01"));
        student.join_course(String::from("english"));
        student.join_course(String::from("math"));
        student.join_course(String::from("physics"));

        student.quit_course(String::from("math"));
        println!("quit_course: {:?}", student);
    }



}
