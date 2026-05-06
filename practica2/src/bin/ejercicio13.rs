/*Definir una función llamada ordenar_nombres
que recibe un arreglo de String y los ordena en orden alfabético. */
fn main() {
    let mut array_string = [
        "hola".to_string(),
        "jejeje".to_string(),
        "mundo".to_string(),
        "abracadabra".to_string(),
    ];
    println!("{:?}", array_string);
    ordenar_nombres(&mut array_string);
    println!("{:?}", array_string);
}
fn ordenar_nombres(array_string: &mut [String]) {
    if array_string.len() <= 1 {
        return;  
    }
    for i in 0..array_string.len()-1 {
        let mut pos: usize = i;
        for j in i + 1..array_string.len() {
            if array_string[j] < array_string[pos] {
                pos = j;
            }
        }
        array_string.swap(pos, i);
    }
}

#[test]
fn test_ordenar_nombres(){
    let mut array_desordenado = ["d".to_string(),"c".to_string(),"b".to_string(),"l".to_string(),"a".to_string()];
    ordenar_nombres(&mut array_desordenado);
    let expected = ["a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"l".to_string()];
    assert_eq!(expected,array_desordenado);
}
#[test]
fn test_ordenar_nombres_array_vacio(){
    let mut array_desordenado:[String;0] = [];
    ordenar_nombres(&mut array_desordenado);
    let expected :[String;0]= [];
    assert_eq!(expected,array_desordenado);
}
