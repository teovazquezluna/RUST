/*Escribir un programa que defina una variable de tipo booleano,
y luego permita al usuario ingresar un valor booleano por teclado
para actualizar su valor haciendo las operaciones and y or.
Se deben imprimir ambos resultados. */
use std::io::stdin;
fn main() {
    let booleano: bool = true;
    let mut valor_de_usuario: String = "".to_string();
    println!("lea valor de entrada booleano");
    stdin().read_line(&mut valor_de_usuario).expect("error");
    let valor_de_usuario = valor_de_usuario.trim();
    println!("entrada del usuario: {valor_de_usuario}");

    let valor_de_usuario = match valor_de_usuario {
        "true" => true,
        "false" => false,
        "True" => true,
        "False" => false,
        _ => {
            println!("entrada no valida");
            false
        }
    };
println!("operaciones and ,or ");
    operaciones(booleano, valor_de_usuario);
}

fn operaciones(booleano: bool, user: bool) {
    println!("{}", booleano && user);
    println!("{}", booleano || user);
}
