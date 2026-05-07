/*Escribir un programa que defina una constante de tipo cadena, y luego
imprima el número de veces que un carácter específico ingresado
 por el usuario aparece en la cadena. Se debe imprimir el resultado */
use std::io;

const CADENA: &str = "ejemploooooo";
fn main() {
    println!("La frase es: '{}'", CADENA);

    let mut entrada = String::new();

    let mut contador: i32 = 0;
    let resultado: i32;
    // leemos el string ingresado por usuario
    println!("Ingresa el carácter que quieres buscar:");
    io::stdin()
        .read_line(&mut entrada)
        .expect("error al leer teclado");
    //sacamos un caracter del string
    let caracter = entrada.trim().chars().next().expect("hubo problemas");
    //validar caracter
    if !caracter.is_alphabetic() {
        println!("Error {} no es una letra del abecedario", caracter);
    } else {
        //descomponer la cadena y comparar con entrada
        for i in CADENA.chars() {
            if i == caracter {
                contador += 1;
            }
        }
        // imprimir numero de veces que aparece
        resultado = contador;
        println!("aparece {} veces", resultado);
    }
}