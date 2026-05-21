/* Definir una función llamada reemplazar_pares que recibe un arreglo de enteros
y reemplaza todos los números pares por -1. */
pub fn reemplazar_pares(array_integer: &mut [i32; 5]) {
    for i in 0..array_integer.len() {
        if (array_integer[i] % 2) == 0 {
            array_integer[i] = -1;
        }
    }
}

pub fn resolver() {
    let mut array = [1, 2, 3, 4, 5];
    reemplazar_pares(&mut array);
    println!("{:?}", array);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reemplazar_pares_ningun_impar() {
        let mut arreglo_remplazar: [i32; 5] = [3, 7, 9, 7, 3];
        let expected: [i32; 5] = arreglo_remplazar;
        reemplazar_pares(&mut arreglo_remplazar);
        assert_eq!(expected, arreglo_remplazar);
    }

    #[test]
    fn test_reemplazar_pares() {
        let mut arreglo_remplazar: [i32; 5] = [0, 2, 4, 8, 10];
        let expected: [i32; 5] = [-1; 5];
        reemplazar_pares(&mut arreglo_remplazar);
        assert_eq!(expected, arreglo_remplazar);
    }
}
