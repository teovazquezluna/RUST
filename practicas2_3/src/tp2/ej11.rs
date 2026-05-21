/* Definir la función llamada multiplicar_valores que recibe un arreglo de enteros
y un factor, y multiplica los valores del arreglo por ese factor. */
pub fn multiplicar_valores(arreglo_entero: &mut [u32; 3], factor: u32) {
    for i in 0..arreglo_entero.len() {
        arreglo_entero[i] = arreglo_entero[i] * factor;
    }
}

pub fn resolver() {
    let mut arreglo = [2, 4, 85];
    let factor = 4;
    multiplicar_valores(&mut arreglo, factor);
    println!("{:?}", arreglo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplicar_valores() {
        let mut arreglo_enteros = [1, 2, 3];
        let factor = 3;
        let expected = [3, 6, 9];
        multiplicar_valores(&mut arreglo_enteros, factor);
        assert_eq!(expected, arreglo_enteros);
    }

    #[test]
    fn test_multiplicar_valores_0() {
        let mut arreglo_enteros = [1, 2, 3];
        let factor = 0;
        let expected = [0; 3];
        multiplicar_valores(&mut arreglo_enteros, factor);
        assert_eq!(expected, arreglo_enteros);
    }
}
