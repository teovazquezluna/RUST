/* 4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
isósceles o escaleno.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna. */

#[derive(Debug)]
pub enum TipoTriangulo {
    Equilatero,
    Isosceles,
    Escaleno,
}

#[derive(Debug)]
pub struct Triangulo {
    pub longitud_lado1: f32,
    pub longitud_lado2: f32,
    pub longitud_lado3: f32,
}

impl Triangulo {
    pub fn new(longitud_lado1: f32, longitud_lado2: f32, longitud_lado3: f32) -> Triangulo {
        if longitud_lado1 == 0.0 || longitud_lado2 == 0.0 || longitud_lado3 == 0.0 {
            panic!("Lado invalido no puede ser 0");
        }
        Triangulo {
            longitud_lado1,
            longitud_lado2,
            longitud_lado3,
        }
    }

    pub fn calcular_area(&self) -> f32 {
        let s = (self.longitud_lado1 + self.longitud_lado2 + self.longitud_lado3) / 2.0;
        (
            s * (s - self.longitud_lado1) * (s - self.longitud_lado2) * (s - self.longitud_lado3)
        )
        .sqrt()
    }

    pub fn calcular_perimetro(&self) -> f32 {
        self.longitud_lado1 + self.longitud_lado2 + self.longitud_lado3
    }

    pub fn determinar_tipo(&self) -> TipoTriangulo {
        if self.longitud_lado1 == self.longitud_lado2 && self.longitud_lado2 == self.longitud_lado3
        {
            TipoTriangulo::Equilatero
        } else if self.longitud_lado1 == self.longitud_lado2
            || self.longitud_lado2 == self.longitud_lado3
            || self.longitud_lado3 == self.longitud_lado1
        {
            TipoTriangulo::Isosceles
        } else {
            TipoTriangulo::Escaleno
        }
    }
    fn comparar_tipos(tipo1:&TipoTriangulo,tipo2:&TipoTriangulo)->bool{
        match(tipo1,tipo2){
            (TipoTriangulo::Equilatero , TipoTriangulo::Equilatero)=> true,
            (TipoTriangulo::Isosceles , TipoTriangulo::Isosceles) => true,
            (TipoTriangulo::Escaleno , TipoTriangulo::Escaleno) =>true,
            _=>false
        }
    }
}

pub fn resolver() {
    let triangulo1 = Triangulo::new(5.0, 7.0, 9.0);
    println!("Crearemos un triangulo de lados {:?}", triangulo1);
    println!("Este triangulo es de tipo:{:?}", triangulo1.determinar_tipo());

    let triangulo2 = Triangulo::new(5.5, 5.5, 5.5);
    println!("Crearemos un triangulo de lados {:?}", triangulo2);
    println!("Este triangulo es de tipo:{:?}", triangulo2.determinar_tipo());

    let triangulo3 = Triangulo::new(5.0, 9.0, 9.0);
    println!("Crearemos un triangulo de lados {:?}", triangulo3);
    println!("Este triangulo es de tipo:{:?}", triangulo3.determinar_tipo());
    println!("Su area es {}", triangulo3.calcular_area());
    println!("Su perimetro es de {}", triangulo3.calcular_perimetro());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparar_tipos(){ 
        let tipo1=TipoTriangulo::Equilatero;
        let tipo2=TipoTriangulo::Equilatero;
        let tipo3=TipoTriangulo::Isosceles;

        assert!(Triangulo::comparar_tipos(&tipo1, &tipo2));
        assert!(!Triangulo::comparar_tipos(&tipo1, &tipo3));
    }
    #[test]
    fn test_new() {
        let t = Triangulo::new(4.0, 5.0, 6.0);
        assert_eq!(4.0, t.longitud_lado1);
        assert_eq!(5.0, t.longitud_lado2);
        assert_eq!(6.0, t.longitud_lado3);
    }

    #[test]
    #[should_panic(expected = "Lado invalido no puede ser 0" )]
    fn test_new_0(){
        let _triangulo = Triangulo::new(1.0, 2.0, 0.0);
    }

    #[test]
    #[should_panic(expected = "Lado invalido no puede ser 0")]
    fn test_new_todos0() {
    Triangulo::new(0.0, 0.0, 0.0);
    }

    #[test]
    fn test_determinar_tipo_equilatero() {
        let t = Triangulo::new(6.0, 6.0, 6.0);
        let tipo1=t.determinar_tipo();
        let tipo2 = TipoTriangulo::Equilatero;
        assert!(Triangulo::comparar_tipos(&tipo1,&tipo2));
    }

    #[test]
    fn test_determinar_tipo_isosceles() {
        let t = Triangulo::new(5.0, 5.0, 7.0);
        let tipo1=t.determinar_tipo();
        let tipo2 = TipoTriangulo:: Isosceles;
        assert!(Triangulo::comparar_tipos(&tipo1,&tipo2));
    }

    #[test]
    fn test_determinar_tipo_escaleno() {
        let t = Triangulo::new(3.0, 4.0, 6.0);
        let tipo1=t.determinar_tipo();
        let tipo2 = TipoTriangulo::Escaleno;
        assert!(Triangulo::comparar_tipos(&tipo1,&tipo2));
    }

    #[test]
    fn test_calcular_area() {
        let t = Triangulo::new(3.0, 4.0, 5.0);
        let expected = 6.0;
        let area = t.calcular_area();
        assert_eq!(expected,area);
        assert!(area>0.0);
    }

    #[test]
    fn test_calcular_perimetro() {
        let t = Triangulo::new(5.0, 4.0, 5.0);
        assert_eq!(14.0, t.calcular_perimetro());
    }
}
