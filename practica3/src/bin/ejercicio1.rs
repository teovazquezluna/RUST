/*1. Estructura Persona
Escribir un programa que defina una estructura
Persona que tenga campos para el nombre, la edad y la dirección
 (que puede ser nulo al momento de la creación de una persona).
  Para dicha estructura implemente los siguientes métodos:

new: que pasando los parámetros correspondientes, crea una Persona
y la retorna.

imprimir: que imprime los datos de la persona sobre el mensaje
 ejecutado por ej: person.imprimir(), donde person es una variable
 del tipo Persona.

obtener_edad: retorna la edad de la persona.

actualizar_direccion(nueva_direccion): permite cambiar la dirección
actual. */
struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    fn new(nombre: String, edad: u32, direccion: Option<String>) -> Persona {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    fn imprimir(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Edad: {}", self.edad);
        if let Some(d) = &self.direccion {
            println!("Dirección: {}", d);
        } else {
            println!("no tengo jaja");
        }
    }
    fn get_edad(&self) -> u32 {
        self.edad
    }
    fn set_direccion(&mut self, nuevadir: String) {
        self.direccion = Some(nuevadir);
    }
}

fn main() {
    let mut person = Persona::new(
        "Pablo".to_string(),
        30,
        Some("Calle loco avenida libertador".to_string()),
    );
    person.imprimir();
    let direccion_nueva = "GOD AND GOODNESS".to_string();
    person.set_direccion(direccion_nueva);
    person.imprimir();
    let edad = person.get_edad();
    println!("epico la edad es {}", edad);
}
