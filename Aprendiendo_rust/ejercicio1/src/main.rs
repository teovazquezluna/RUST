use std::io;
/*Escribir un programa que defina una variable de tipo flotante con algún valor,
y luego permita al usuario ingresar un número decimal por teclado para multiplicar,
 dividir, sumar y restar su valor. Se deben imprimir los resultados. */

fn main() {
    let float: f32 = 2.3;
    println!("Ingrese su numero");
    let mut numusuario = String::new();
    io::stdin()
        .read_line(&mut numusuario)
        .expect("Se rompio el teclado");
    let numusuario: f32 = numusuario
        .trim()
        .parse()
        .expect("Entrada erronea escribe un numero");

    operar(float, numusuario);

    fn operar(flotante: f32, numero: f32) {
        println!("{}", (flotante * numero));
        println!("{}", (flotante / numero));
        println!("{}", (flotante + numero));
        println!("{}", (flotante - numero));
    }


}
