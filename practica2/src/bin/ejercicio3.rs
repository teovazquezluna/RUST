/*Definir la función llamada suma_pares que recibe como
parámetro un arreglo
de números enteros y retorna la suma de los números pares. */
fn main() {
    let arreglo_pares: [i32; 5] = [2, 4, 3, 5, 6];
    let suma: i32 = sumar_pares(arreglo_pares);
    println!("{}", suma);
}
fn sumar_pares(array: [i32;5]) -> i32 {
    let mut suma: i32 = 0;
    for i in array {
        if (i % 2) == 0 {
            suma += i;
        }
    }
    return suma;
}
#[test]
fn test_sumar_pares(){
    let array=[8,132,2,2,8];
    let expected=152;
    assert_eq!(expected,sumar_pares(array));
}
#[test]
fn test_sumar_pares_caso0(){
    let array = [0,0,0,0,0];
    let expected = 0;
    assert_eq!(expected,sumar_pares(array));
}