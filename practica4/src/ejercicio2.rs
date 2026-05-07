/*Dado el siguiente struct:
struct Persona<'a>{
nombre:&'a str,
apellido:&'a str,
direccion:&'a str,
ciudad:&'a str,
salario:f64,
edad:u8,
}
a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.

b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad.
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario.
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario.

e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario
-Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.
Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure.*/
fn min_and_max_salary<'a>(people: &[Person<'a>]) -> Option<(&'a Person<'a>, &'a Person<'a>)> {
    people.iter().min_by(compare)
} // preguntar a un profe o ayudante como usar esto en iterators
fn get_age_array<'a>(people: &[Person<'a>]) -> Vec<u8> {
    people.iter().map(|person| person.edad).collect()
}
fn is_exist<'a>(people: &[Person<'a>], person: &Person<'a>) -> bool {
    people.iter().any(|p| {
        p.nombre == person.nombre
            && p.apellido == person.apellido
            && p.direccion == person.direccion
            && p.ciudad == person.ciudad
            && p.salario == person.salario
            && p.edad == person.edad
    })
}

fn is_any_living_in<'a>(people: &'a Vec<Person<'a>>, city: String) -> bool {
    people.iter().any(|person| person.ciudad == city)
}
fn are_people_living_in<'a>(people: &'a Vec<Person<'a>>, city: String) -> bool {
    people.iter().all(|person| person.ciudad == city)
}
fn major_than_age_and_live_in_<'a>(
    people: &'a Vec<Person<'a>>,
    age: u8,
    city: String,
) -> Vec<&'a Person<'a>> {
    people
        .iter()
        .filter(|person| person.edad > age && person.ciudad == city)
        .collect()
}
fn list_major_salary<'a>(people_vec: &'a Vec<Person<'a>>, salary: f64) -> Vec<&Person<'a>> {
    people_vec
        .iter()
        .filter(|person| person.salario > salary)
        .collect()
}
struct Person<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

pub fn ej2() {}
