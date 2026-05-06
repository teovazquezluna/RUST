/*Definir la función llamada cantidad_en_rango
 que recibe 3 parámetros: 1 arreglo de enteros, un número entero
 llamado inferior y otro número entero llamado superior. Esta función
  retorna la cantidad de números del arreglo que están entre el
rango de los parámetros inferior y superior inclusive. */
fn main() {
    let array = [2, 56, 31];
    let inferior = 20;
    let superior = 32;
    let resultado = cantidad_en_rango(array, inferior, superior);
    println!("{}", resultado);
}
fn cantidad_en_rango(array_int: [i32;3], inferior: i32, superior: i32) -> u32 {
    let mut cantidad_numeros_en_rango = 0;
    for i in 0..array_int.len() {
        if (array_int[i] > inferior) && (array_int[i]<= superior) {
            cantidad_numeros_en_rango += 1;
        }
    }
    cantidad_numeros_en_rango
}

#[test]
fn test_cantidad_en_rango(){
let array_prueba= [2,4,6];
let inferior =1;
let superior =1000;
let expected  = 3;
assert_eq!(expected,cantidad_en_rango(array_prueba, inferior, superior));
}
#[test]
fn test_cantidad_en_rango_cant0(){
let array_prueba= [500,400,600];
let inferior =1;
let superior =100;
let expected  = 0;
assert_eq!(expected,cantidad_en_rango(array_prueba, inferior, superior));

}
