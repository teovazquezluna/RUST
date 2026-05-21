/* Definir la función llamada duplicar_valores que recibe un arreglo de números
flotantes y retorna un arreglo nuevo con los valores duplicados del parámetro. */
pub fn duplicar_valores(array: [f32; 5]) -> [f32; 5] {
    let mut array_nuevo: [f32; 5] = [0.0; 5];
    for i in 0..array.len() {
        array_nuevo[i] = array[i] * 2.0;
    }
    array_nuevo
}

pub fn resolver() {
    let array_flotante: [f32; 5] = [2.4, 6.5, 1.1, 3.2, 8.9];
    let vector = duplicar_valores(array_flotante);
    for i in vector {
        println!("{}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicar_valores() {
        let array = [1.0, 2.0, 3.0, 4.0, 5.0];
        let array_esperado = [2.0, 4.0, 6.0, 8.0, 10.0];
        assert_eq!(array_esperado, duplicar_valores(array));
    }
}
