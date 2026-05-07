/*2. Estructura Rectángulo
Escribir un programa que defina la estructura
Rectángulo que tenga campos para la longitud y el ancho.
Para dicha estructura implemente los siguientes métodos:

new: que pasando los parámetros correspondientes, crea un
 Rectángulo y lo retorna.

calcular_area: calcular el área y la retorna.

calcular_perimetro: calcula el perímetro y lo retorna. */

struct Rectangulo {
    longitud: f32,
    ancho: f32,
}
impl Rectangulo {
    fn new(largo: f32, ancho: f32) -> Rectangulo {
        Rectangulo {
            longitud: largo,
            ancho: ancho,
        }
    }
    fn calcular_area(&self) -> f32 {
        self.ancho * self.longitud
    }
    fn calcular_perimetro(&self) -> f32 {
        self.ancho * 2.0 + self.longitud * 2.0
    }
    fn is_square(&self) -> bool {
        self.ancho == self.longitud
    }
}
fn main() {
    let rectangulin: Rectangulo = Rectangulo::new(30.2, 30.2);
    let perimetro: f32 = rectangulin.calcular_perimetro();
    let area: f32 = rectangulin.calcular_area();
    println!("Area: {}", area);
    println!("Perimetro: {}", perimetro);
    if rectangulin.is_square() {
        println!("es cuadrado!");
    } else {
        println!("Es un rectangulo");
    }
}
