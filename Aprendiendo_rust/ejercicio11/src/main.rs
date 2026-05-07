/*Escribir un programa que defina un arreglo de 5 cadenas, y
 luego permita al usuario ingresar una cadena por teclado.
 El programa debe imprimir un mensaje
si la cadena ingresada por el usuario se encuentra en el arreglo. */
use std::io;
fn main() {
    println!("Lee tu cadena");
    let string_array: [&str; 5] = ["hola", "yo", "tu", "el", "vosotros"];
    let mut entrada = String::new();
    //leemos cadena

    io::stdin().read_line(&mut entrada).expect("Error");
    //limpiamos cadena
    let entrada = entrada.trim();
    //comparamos cadena
    for i in string_array {
        if entrada == i {
            println!("La cadena se repitio!!");
        }
    }

}
