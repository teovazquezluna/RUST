/* Definir una función llamada serie_geométrica que recibe como parámetro un número
entero llamado tamaño y devuelva un arreglo con tamaño definido por ese número pasado
por parámetro. Los valores del arreglo resultado son una serie que empieza en 1 en la
primera celda, dos en la siguiente, cuatro en la siguiente, y así sucesivamente. */
pub fn serie_geometrica<const N: usize>(tamaño: u32) -> [u32; N] {
    if tamaño as usize != N {
        panic!("Tamaño no coincide");
    }
    let mut array_resultado = [0; N];
    array_resultado[0] = 1;
    for i in 1..array_resultado.len() {
        array_resultado[i] = array_resultado[i - 1] * 2;
    }
    array_resultado
}

pub fn resolver() {
    let tamaño: u32 = 10;
    let resultado = serie_geometrica::<10>(tamaño);
    let _ = resultado;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie_geometrica() {
        let tamaño = 1;
        let expected = [1];
        assert_eq!(expected, serie_geometrica::<1>(tamaño));
    }
    #[test]
    #[should_panic(expected = "Tamaño no coincide")]
    fn test_tamaños_distintos(){
        let tamaño = 3;
        serie_geometrica::<2>(tamaño);
    }
}
