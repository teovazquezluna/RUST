/* Definir la función llamada es_primo que recibe un número entero
positivo mayor a 1 y retorna true si es primo, false caso contrario. */
pub fn es_primo(num: u32) -> bool {
    if num <= 1 {
        println!("elige un numero mayor a 1");
        return false;
    }
    for i in 2..num {
        if (num % i) == 0 {
            return false;
        }
    }
    true
}

pub fn resolver() {
    let numero = 4;
    println!("{}", es_primo(numero));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_primo() {
        let numero = 7;
        assert!(es_primo(numero));
    }

    #[test]
    fn test_es_primo_valor1() {
        let numero = 1;
        assert_eq!(false, es_primo(numero));
    }
}
