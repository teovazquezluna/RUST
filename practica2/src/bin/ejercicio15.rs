/*Definir una función llamada serie_geométrica que recibe como parámetro un número
entero llamado tamaño y devuelva un arreglo con tamaño definido por ese número pasado
por parámetro. Los valores del arreglo resultado son una serie que empieza en 1 en la
primera celda, dos en la siguiente, cuatro en la siguiente, y así sucesivamente, tal que en
cada celda se duplica el valor de la celda anterior */

fn main(){
    let tamaño:u32 = 10;
let resultado:[u32;tamaño]= serie_geométrica(tamaño);

}
fn serie_geométrica(tamaño:u32) -> [u32;tamaño]{
    let mut array_resultado = [0;tamaño];
    array_resultado[0] = 1;
    for i in 1..array_resultado.len(){
        array_resultado[i] = array_resultado[i-1]*2;
    }
    array_resultado
}

// preguntar como hacer esto
#[test]
fn test_serie_geometrica(){
let tamaño = 1;
let expected = [1];
assert_eq!(expected,serie_geométrica(tamaño));
}
