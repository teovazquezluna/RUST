/*
4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:

    - new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
    - determinar_tipo: retorna el tipo del triángulo. Los tipos pueden ser:
      equilátero, isósceles o escaleno.
    - calcular_area: calcular el área y la retorna.
    - calcular_perimetro: calcula el perímetro y lo retorna.
*/
#[derive(Debug)]
struct Triangulo {
    side1: f32,
    side2: f32,
    side3: f32,
}
#[derive(Debug)]
enum Tipos {
    Equilatero,
    Isosceles,
    Escaleno,
}
impl Triangulo {
    fn calcular_area(&self) -> f32 {
        let s = (self.side1 + self.side2 + self.side3) / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }
    fn calcular_perimetro(&self) -> f32 {
        self.side1 + self.side2 + self.side3
    }
    fn determinar_tipo(&self) -> Tipos {
        if self.side1 == self.side2 && self.side2 == self.side3 {
            Tipos::Equilatero
        } else if self.side1 == self.side2 || self.side2 == self.side3 || self.side3 == self.side1 {
            Tipos::Isosceles
        } else {
            Tipos::Escaleno
        }
    }
    fn new(a: f32, b: f32, c: f32) -> Triangulo {
        Triangulo {
            side1: a,
            side2: b,
            side3: c,
        }
    }
}
fn main() {
    let triangulo1 = Triangulo::new(5.0, 7.0, 9.0);
    println!("Crearemos un triangulo de lados {:?}", triangulo1);
    let mi_tipo: Tipos = triangulo1.determinar_tipo();
    println!("Este triangulo es de tipo:{:?} ", mi_tipo);
    let triangulo2 = Triangulo::new(5.5, 5.5, 5.5);
    println!("Crearemos un triangulo de lados {:?}", triangulo2);
    let mi_tipo: Tipos = triangulo2.determinar_tipo();
    println!("Este triangulo es de tipo:{:?} ", mi_tipo);
    let triangulo3 = Triangulo::new(5.0, 9.0, 9.0);
    println!("Crearemos un triangulo de lados {:?}", triangulo3);
    let mi_tipo: Tipos = triangulo3.determinar_tipo();
    println!("Este triangulo es de tipo:{:?} ", mi_tipo);

    println!("Su area es {}", triangulo3.calcular_area());
    println!("Su perimetro es de {}", triangulo3.calcular_perimetro());
}
