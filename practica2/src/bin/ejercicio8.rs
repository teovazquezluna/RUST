/*Definir la función llamada sumar_arreglos
que recibe 2 arreglos del mismo tamaño de números
 flotantes y retorna un nuevo arreglo que contiene la
  suma de los elementos de los arreglos pasados por parámetro,
  correspondiéndose el resultado con
cada posición de los arreglos pasados por parámetro. */
fn main() {
    let array1 = [1.1, 4.5, 3.7];
    let array2 = [6.6, 3.1, 2.0];
    println!("{:?}", sumar_arreglos(array1, array2));
}
fn sumar_arreglos(array1: [f32;3], array2: [f32;3]) -> [f32;3] {
    let mut array:[f32;3]= [0.0;3];
    for i in 0..array1.len() {
        array[i] = array1[i]+array2[i];
    }
    array
}

#[test]
fn test_sumar_arreglos(){
let a1 = [1.2,1.4,0.4];
let a2 = [1.6,5.7,2.6];
let expected_array = [2.8000002,7.1,3.0];
assert_eq!(expected_array,sumar_arreglos(a1, a2));
}
#[test]
fn test_sumar_arreglos_wrong(){
    let a1 = [1.0,0.0,0.0];
    let a2 = [1.0,0.0,0.0];
    let expected_array = [6.0,0.0,0.0];
    assert_ne!(expected_array,sumar_arreglos(a1, a2));
}