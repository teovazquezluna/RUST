/*Escribir un programa que defina una tupla que contenga una
cadena y un arreglo de enteros, y luego imprima la cadena y
la suma de los valores en el arreglo. */
fn main() {
    let array: [i32; 3] = [6, 7, 8];
    let tupla: (String, [i32; 3]) = ("Hola".to_string(), array);
    let suma = suma_de_valores(array);
println!("{} la suma es {}", tupla.0, suma);

}


fn suma_de_valores(array: [i32; 3]) -> i32 {
    let mut resultado: i32 = 0;
    for i in 0..3 {
        resultado = resultado + array[i];
    }
    resultado
}

#[test]
fn test_tupla(){
    let array =[1,2,3];
    let expected =6;
    let result = suma_de_valores(array);
    assert_eq!(expected,result);
    assert!(suma_de_valores(array) == 6);
}

