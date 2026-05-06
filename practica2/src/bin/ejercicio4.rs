/*Definir la función llamada cantidad_impares que recibe
como parámetro un arreglo de números enteros y retorna la
cantidad de números impares. */
fn main() {
    let array: [i32; 4] = [37, 51, 12, 55];
    let can_num_impar: i32 = cantidad_impares(array);
    println!("{}", can_num_impar);
}
fn cantidad_impares(array: [i32;4]) -> i32 {
    let mut resultado: i32 = 0;
    for i in array {
        if (i % 2) == 1 {
            resultado += 1;
        }
    }
    resultado
}
#[test]
fn test_cantidad_impares(){
    let array = [1,5,7,9];
    let expected = 4;
    assert_eq!(expected,cantidad_impares(array));
}
#[test]
fn test_cantidad_impares_caso0(){
    let array = [2,4,6,8];
    let expected = 0;
    assert_eq!(expected,cantidad_impares(array));
}