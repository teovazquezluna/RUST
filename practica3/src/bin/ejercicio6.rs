/*6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
// nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
// conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
// métodos:
//
// * Examen:
//   > new: que pasando los parámetros correspondientes, crea un Examen y lo retorna.
// * Estudiante:
//   > new: que pasando los parámetros correspondientes, crea un Estudiante y lo retorna.
//   > obtener_promedio: retorna el promedio de las notas.
//   > obtener_calificacion_mas_alta: retorna la nota más alta.
//   > obtener_calificacion_mas_baja: retorna la nota más baja.
//
// Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen. */
struct Student {
    name: String,
    idnum: u32,
    exam_grades_list: Vec<Exam>,
}
struct Exam {
    subject_name: String,
    grade: f32,
}
impl Student {
    fn new(n: String, id: u32) -> Student {
        Student {
            name: n,
            idnum: id,
            exam_grades_list: Vec::new(),
        }
    }
    fn get_average(&self) -> Option<f32> {
        let mut total: f32 = 0.0;
        for i in &self.exam_grades_list {
            total += i.grade;
        }
        if self.exam_grades_list.len() != 0 {
            Some(total / self.exam_grades_list.len() as f32)
        } else {
            None
        }
    }
    fn highest_grade(&self) -> Option<f32> {
        let mut max = 0.0;
        for exam in &self.exam_grades_list {
            if exam.grade > max {
                max = exam.grade;
            }
        }
        if self.exam_grades_list.len() != 0 {
            Some(max)
        } else {
            None
        }
    }

    fn lowest_grade(&self) -> Option<f32> {
        let mut min = 10000.0;
        for exam in &self.exam_grades_list {
            if exam.grade < min {
                min = exam.grade;
            }
        }
        if self.exam_grades_list.len() != 0 {
            Some(min)
        } else {
            None
        }
    }
    fn add_exam(&mut self, exam: Exam) {
        self.exam_grades_list.push(exam);
    }
}
impl Exam {
    fn new(name: String, grade: f32) -> Exam {
        Exam {
            subject_name: name,
            grade,
        }
    }
}

fn main() {
    // Crear un estudiante
    let mut student = Student::new(String::from("Teo"), 12345);

    // Crear exámenes
    let exam1 = Exam::new(String::from("Matemática"), 8.5);
    let exam2 = Exam::new(String::from("Historia"), 6.0);
    let exam3 = Exam::new(String::from("Programación"), 9.5);

    // Agregarlos al estudiante
    student.add_exam(exam1);
    student.add_exam(exam2);
    student.add_exam(exam3);

    // Mostrar promedio
    match student.get_average() {
        Some(avg) => println!("Promedio: {}", avg),
        None => println!("No hay exámenes cargados"),
    }

    // Mostrar nota más alta
    match student.highest_grade() {
        Some(max) => println!("Nota más alta: {}", max),
        None => println!("No hay exámenes cargados"),
    }

    // Mostrar nota más baja
    match student.lowest_grade() {
        Some(min) => println!("Nota más baja: {}", min),
        None => println!("No hay exámenes cargados"),
    }
}
