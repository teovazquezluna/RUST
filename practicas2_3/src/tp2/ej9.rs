/* Definir la función llamada cantidad_en_rango que recibe un arreglo de enteros,
un límite inferior y uno superior, y retorna la cantidad en ese rango inclusive. */
pub fn cantidad_en_rango(array_int: [i32; 3], inferior: i32, superior: i32) -> u32 {
    let mut cantidad_numeros_en_rango = 0;
    for i in 0..array_int.len() {
        if (array_int[i] >= inferior) && (array_int[i] <= superior) {
            cantidad_numeros_en_rango += 1;
        }
    }
    cantidad_numeros_en_rango
}

pub fn resolver() {
    let array = [2, 56, 31];
    let inferior = 20;
    let superior = 32;
    let resultado = cantidad_en_rango(array, inferior, superior);
    println!("{}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn  test_valor_inclusive(){
        let array = [20,50,80];
        let inferior = 20;
        let superior = 80;
        let expected = 3;
        assert_eq!(expected,cantidad_en_rango(array, inferior, superior));
    }
    #[test]
    fn test_cantidad_en_rango() {
        let array_prueba = [2, 4, 6];
        let inferior = 1;
        let superior = 1000;
        let expected = 3;
        assert_eq!(expected, cantidad_en_rango(array_prueba, inferior, superior));
    }

    #[test]
    fn test_cantidad_en_rango_cant0() {
        let array_prueba = [500, 400, 600];
        let inferior = 1;
        let superior = 100;
        let expected = 0;
        assert_eq!(expected, cantidad_en_rango(array_prueba, inferior, superior));
    }
}
