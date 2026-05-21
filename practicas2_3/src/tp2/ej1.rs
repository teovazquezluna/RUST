/* Definir la función llamada es_par que recibe como parámetro
un número entero y retorna true si el número es par, false caso contrario. */
const NUM: i32 = 0;

pub fn es_par(num: i32) -> bool {
    if (num % 2) == 0 {
        true
    } else {
        false
    }
}

pub fn resolver() {
    let num: i32 = NUM;
    println!("{}", es_par(num));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn es_par_test() {
        let numero_par: i32 = 10;
        let valor_esperado: bool = true;
        assert_eq!(valor_esperado, es_par(numero_par));
    }
}
