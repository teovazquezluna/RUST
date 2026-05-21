/* 6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
retorna.
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
retorna.
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta.
➢ obtener_calificacion_mas_baja: retorna la nota más baja.
Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen. */

pub struct Examen {
    pub nombre_de_la_materia: String,
    pub nota: f32,
}

impl Examen {
    pub fn new(nombre_de_la_materia: String, nota: f32) -> Examen {
        Examen {
            nombre_de_la_materia,
            nota,
        }
    }
}

pub struct Estudiante {
    pub nombre: String,
    pub numero_de_identificacion: u32,
    pub calificaciones_de_examenes: Vec<Examen>,
}

impl Estudiante {
    pub fn new(nombre: String, numero_de_identificacion: u32) -> Estudiante {
        Estudiante {
            nombre,
            numero_de_identificacion,
            calificaciones_de_examenes: Vec::new(),
        }
    }

    pub fn obtener_promedio(&self) -> Option<f32> {
        if self.calificaciones_de_examenes.is_empty(){
            None
        }
        else{
            let mut total:f32=0.0;
            for examen in &self.calificaciones_de_examenes{
                total+= examen.nota;
            }
            Some(total/self.calificaciones_de_examenes.len() as f32)
        }

    }

    pub fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        if self.calificaciones_de_examenes.is_empty(){
            None
        }
        else{
            let mut mas_alta:f32=f32::MIN;
            for examen in &self.calificaciones_de_examenes{
                if examen.nota > mas_alta {
                    mas_alta=examen.nota;
                }
            }
            Some(mas_alta)

        }
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        if self.calificaciones_de_examenes.is_empty(){
            None
        }
        else{
            let mut mas_baja:f32=f32::MAX;
            for examen in &self.calificaciones_de_examenes{
                if examen.nota < mas_baja {
                    mas_baja=examen.nota;
                }
            }
            Some(mas_baja)

        }
    }
}

pub fn resolver() {
    let mut estudiante = Estudiante::new(String::from("Teo"), 12345);
    let mut calificaciones_de_examenes = Vec::new();
    calificaciones_de_examenes.push(Examen::new(String::from("Matemática"), 8.5));
    calificaciones_de_examenes.push(Examen::new(String::from("Historia"), 6.0));
    calificaciones_de_examenes.push(Examen::new(String::from("Programación"), 9.5));
    estudiante.calificaciones_de_examenes = calificaciones_de_examenes;

    match estudiante.obtener_promedio() {
        Some(promedio) => println!("Promedio: {}", promedio),
        None => println!("No hay exámenes cargados"),
    }

    match estudiante.obtener_calificacion_mas_alta() {
        Some(nota) => println!("Nota más alta: {}", nota),
        None => println!("No hay exámenes cargados"),
    }

    match estudiante.obtener_calificacion_mas_baja() {
        Some(nota) => println!("Nota más baja: {}", nota),
        None => println!("No hay exámenes cargados"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_estudiante() {
        let nombre = "teo";
        let numero_id = 1;
        let estudiante = Estudiante::new(nombre.to_string(), 1);

        assert_eq!(estudiante.nombre, nombre.to_string());
        assert_eq!(estudiante.numero_de_identificacion, numero_id);
        assert!(estudiante.calificaciones_de_examenes.is_empty());
    }

    #[test]
    fn test_new_examen(){
        let materia = "Matematica".to_string();
        let nota = 9.0;
        let examen = Examen::new(materia.clone(),nota);

        assert_eq!(examen.nombre_de_la_materia,materia);
        assert_eq!(examen.nota, nota);
    }

    #[test]
    fn test_obtener_promedio() {
        let mut estudiante = Estudiante::new("jorgito".to_string(), 1);

        estudiante.calificaciones_de_examenes.push(Examen::new("sociologia".to_string(), 8.0));
        estudiante.calificaciones_de_examenes.push(Examen::new("quimica".to_string(), 9.0));
        estudiante.calificaciones_de_examenes.push(Examen::new("fisica".to_string(), 10.0));

        let expected = 9.0;
        let resultado = estudiante.obtener_promedio();

        assert_eq!(resultado.unwrap(), expected);
    }

    #[test]
    fn test_obtener_calificacion_mas_alta() {
        let mut estudiante = Estudiante::new("san pedro".to_string(), 1);

        estudiante.calificaciones_de_examenes.push(Examen::new("educacion fisica".to_string(), 5.0));
        estudiante.calificaciones_de_examenes.push(Examen::new("filosofia".to_string(), 2.0));
        estudiante.calificaciones_de_examenes.push(Examen::new("historia".to_string(), 1.0));

        let expected = 5.0;
        let resultado = estudiante.obtener_calificacion_mas_alta();

        assert_eq!(resultado.unwrap(), expected);
    }

    #[test]
    fn test_obtener_calificacion_mas_baja() {
        let mut estudiante = Estudiante::new("pablo".to_string(), 1);

        estudiante.calificaciones_de_examenes.push(Examen::new("matematica".to_string(), 4.0));
        estudiante.calificaciones_de_examenes.push(Examen::new("historia".to_string(), 8.0));
        estudiante.calificaciones_de_examenes.push(Examen::new("arte".to_string(), 3.0));

        let expected = 3.0;
        let resultado = estudiante.obtener_calificacion_mas_baja();

        assert_eq!(resultado.unwrap(), expected);
    }

    #[test]
    fn test_funciones_con_examenes0() {
        let estudiante = Estudiante::new("juan".to_string(), 1);

        assert_eq!(estudiante.obtener_promedio(), None);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(),None);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(),None);
    }
}
