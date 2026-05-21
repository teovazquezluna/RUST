/* Definir la función llamada cantidad_de_cadenas_mayor_a que recibe un arreglo de
String y un límite, y retorna la cantidad de cadenas con longitud mayor al límite. */
pub fn cantidad_de_cadenas_mayor_a(array: &[String; 3], limite: i32) -> u32 {
    let mut mayor_longitud: u32 = 0;
    for i in array {
        if (*i).len() as i32 > limite {
            mayor_longitud += 1;
        }
    }
    mayor_longitud
}

pub fn resolver() {
    let array_cadena = [
        "hola".to_string(),
        "mundito".to_string(),
        "rust".to_string(),
    ];
    let limite = 8;
    let resultado: u32 = cantidad_de_cadenas_mayor_a(&array_cadena, limite);
    println!("{}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_de_cadenas_mayor_a() {
        let limite = 0;
        let array_string = [
            "hola".to_string(),
            "a".to_string(),
            "jajaja".to_string(),
        ];
        let expected = 3;
        assert_eq!(expected, cantidad_de_cadenas_mayor_a(&array_string, limite));
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_0() {
        let limite = 1;
        let array_string = ["h".to_string(), "a".to_string(), "j".to_string()];
        let expected = 0;
        assert_eq!(expected, cantidad_de_cadenas_mayor_a(&array_string, limite));
    }
}
