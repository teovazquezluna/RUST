/* Definir la función llamada longitud_de_cadenas que recibe un arreglo de String
y retorna un arreglo con la longitud de las cadenas del parámetro. */
pub fn longitud_de_cadenas(array: &[String; 3]) -> [u32; 3] {
    let mut array_string: [u32; 3] = [0; 3];
    for i in 0..array.len() {
        array_string[i] = array[i].len() as u32;
    }
    array_string
}

pub fn resolver() {
    let multiples_cadenas: [String; 3] = [
        "hola".to_string(),
        "de locos".to_string(),
        "rust🦀".to_string(),
    ];
    println!("{:?}", multiples_cadenas);
    println!("{:?}", longitud_de_cadenas(&multiples_cadenas));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longitud_de_cadenas() {
        let array_prueba = [String::from("a"), "bb".to_string(), "ccc".to_string()];
        let expected_array: [u32; 3] = [1, 2, 3];
        assert_eq!(expected_array, longitud_de_cadenas(&array_prueba));
    }
}
