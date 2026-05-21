/* 2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario */
pub struct Rectangulo {
    pub longitud: f32,
    pub ancho: f32,
}

impl Rectangulo {
    pub fn new(longitud: f32, ancho: f32) -> Rectangulo {
        if longitud == 0.0 || ancho == 0.0 {
            panic!("Ancho o largo no validos");
        }
        Rectangulo { longitud, ancho }
    }

    pub fn calcular_area(&self) -> f32 {
        self.ancho * self.longitud
    }

    pub fn calcular_perimetro(&self) -> f32 {
        self.ancho * 2.0 + self.longitud * 2.0
    }

    pub fn es_cuadrado(&self) -> bool {
        self.ancho == self.longitud
    }
}

pub fn resolver() {
    let rectangulo: Rectangulo = Rectangulo::new(30.2, 30.2);
    let perimetro: f32 = rectangulo.calcular_perimetro();
    let area: f32 = rectangulo.calcular_area();
    println!("Area: {}", area);
    println!("Perimetro: {}", perimetro);
    if rectangulo.es_cuadrado() {
        println!("es cuadrado!");
    } else {
        println!("Es un rectangulo");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_area() {
        let longitud = 2.89;
        let ancho = 3.1;
        let rectangulo: Rectangulo = Rectangulo::new(longitud, ancho);
        assert_eq!(rectangulo.calcular_area(), longitud * ancho);
    }

    #[test]
    fn test_calcular_perimetro() {
        let longitud = 3.0;
        let ancho = 9.0;
        let rectangulo = Rectangulo::new(longitud, ancho);
        let perimetro_esperado = (longitud + ancho) * 2.0;
        assert_eq!(perimetro_esperado, rectangulo.calcular_perimetro());
    }

    #[test]
    fn test_es_cuadrado() {
        let longitud = 3.0;
        let ancho = 3.0;
        let rectangulo = Rectangulo::new(longitud, ancho);
        assert!(rectangulo.es_cuadrado());
    }

    #[test]
    fn test_no_es_cuadrado() {
        let longitud = 4.0;
        let ancho = 3.0;
        let rectangulo = Rectangulo::new(longitud, ancho);
        assert!(!rectangulo.es_cuadrado());
    }

    #[test]
    #[should_panic(expected = "Ancho o largo no validos")]
    fn test_new_0() {
        let ancho = 0.0;
        let longitud = 2.4;
        let _rectangulo = Rectangulo::new(longitud, ancho);
    }
}
