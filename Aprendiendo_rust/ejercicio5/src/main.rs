/*Escribir un programa que defina una variable de tipo cadena, y
luego permita al usuario ingresar una cadena por teclado para
concatenar su valor.
 El programa debe imprimir la cadena en mayúsculas. */

use std::io;
fn main() {
    let mut string = String::from("Hola Mundo");
    println!("ingrese su texto");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error");
    string = string + entrada.trim();
    string = string.to_uppercase();
    print!("{}", string);
}
