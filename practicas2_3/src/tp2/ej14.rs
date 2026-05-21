/* Definir una función llamada incrementar que recibe un número flotante
e incrementa en 1 su valor. */
pub fn incrementar(flotante: &mut f32) {
    *flotante += 1.0;
}

pub fn resolver() {
    let mut f: f32 = 3.4;
    incrementar(&mut f);
    println!("{}", f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incrementar() {
        let mut flotante = 3.5;
        let expected = flotante + 1.0;
        incrementar(&mut flotante);
        assert_eq!(expected, flotante);
    }
}
