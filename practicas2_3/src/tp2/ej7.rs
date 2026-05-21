/* Definir la función llamada cantidad_de_mayores que recibe un arreglo de enteros
y un límite, y retorna la cantidad de números mayores al límite. */
pub fn cantidad_de_mayores(array_integer: [i32; 3], limite: i32) -> i32 {
    let mut cantidad_num_mayor: i32 = 0;
    for i in array_integer {
        if i > limite {
            cantidad_num_mayor += 1;
        }
    }
    cantidad_num_mayor
}

pub fn resolver() {
    let array: [i32; 3] = [392, 24, 7];
    let limite = 100;
    println!("{}", cantidad_de_mayores(array, limite));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_de_mayores() {
        let array = [100, 50, 49];
        let limite = 50;
        let expected = 1;
        assert_eq!(expected, cantidad_de_mayores(array, limite));
    }

    #[test]
    fn test_cantidad_de_mayores_limite0() {
        let array = [100, 50, 49];
        let limite = 0;
        let expected = 3;
        assert_eq!(expected, cantidad_de_mayores(array, limite));
    }
}
