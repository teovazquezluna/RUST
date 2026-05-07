/*  cribir un programa que defina un arreglo de 5
números enteros, y luego imprima la suma de los valores del arreglo. */
fn main() {
    let array_5: [i32; 5] = [1, 2, 3, 4, 5];
    let mut resultado: i32 = 0;
    //imprimos la suma
    for i in &array_5 {
        resultado += *i;
    }
    println!("{}", resultado);
}
