/*Escribir un programa que defina dos arreglos de 5 números
 enteros cada uno, y luego cree un tercer arreglo que
contenga la suma de los elementos de los dos arreglos originales. */
fn main() {
    let array1: [i32; 5] = [26, 57, 17, 05, 70];
    let array2: [i32; 5] = [82, 10, 38, 92, 38];
    let mut array3: [i32; 5] = [0; 5];
    for i in 0..5 {
        array3[i] = array2[i] + array1[i];
    }
    for i in 0..5 {
        println!("{}", array3[i]);
    }
}
