/*Escribir un programa que defina una variable de tipo arreglo
que contenga seis números enteros, y luego multiplique cada valor
 del arreglo por
 un valor constante definido, modificando el contenido del arreglo. */
fn main() {
    const MULTIPLICADOR:i32 = 10;
    let mut array: [i32; 6] = [34, 42, 56, 85, 23, 54];


    for i in &mut array {
        *i *= MULTIPLICADOR;
    }

    for i in array {
        println!("{}", i);
    }
    // OTRA FORMA DE HACERLO
    for i in 0..6 {
        array[i] = array[i] * MULTIPLICADOR;
    }
    for i in array {
        println!("{}", i);
    }
}
