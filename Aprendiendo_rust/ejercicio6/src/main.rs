/*Escribir un programa que
defina una variable de tipo entero sin signo, y
 luego permita al usuario ingresar un número entero por
 teclado para sumarse con la variable definida.
El programa debe imprimir el valor del número elevado al cuadrado. */

use std::io;
fn main() {
    let sin_signo: u32 = 2;
    println!("lea entero");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error");

    let num: u32 = entrada.trim().parse().expect("Ingrese un numero positivo");

    let mut resultado = sin_signo + num;
    resultado = alcuadrado(resultado, 2);
    println!("{}", resultado);
}
fn alcuadrado(base: u32, exp: i32) -> u32 {
    if exp == 1 {
        base
    } else {
        base * alcuadrado(base, exp - 1)
    }
}
