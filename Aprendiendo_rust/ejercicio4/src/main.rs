/*Escribir un programa que defina una tupla que contenga una cadena,
un número entero con signo y un valor booleano,
y luego imprima cada valor de la tupla.
*/
fn main() {
    let tupla: (String, i32, bool) = ("hola Mundo".to_string(), -42, false);
    println!("{}", tupla.0);
    println!("{}", tupla.1);
    println!("{}", tupla.2);
}
