// 7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre,
// la dirección y tiene una capacidad máxima para albergar X cantidad de autos.
// De los autos se conocen los campos de:
// - marca
// - modelo
// - año
// - precio_bruto
// - color (pueden ser: rojo, verde, azul, amarillo, blanco o negro)
//
// Para dichas estructuras implemente los siguientes métodos:
//
// ConcesionarioAuto:
// - new: que pasando los parámetros correspondientes, crea un
//   ConcesionarioAuto y lo retorna.
// - agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
//   la máxima cantidad para albergarlos y retorna true; en caso de que lo supere
//   no lo agrega y retorna false.
// - eliminar_auto(auto): elimina un auto de la lista de autos.
// - buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
//
// Auto:
// - new: que pasando los parámetros correspondientes, crea un Auto y lo retorna.
// - calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
//   - si es de color primario le aplica un recargo del 25%, sino le aplica un
//     descuento del 10%.
//   - si la marca es BMW le aplica un recargo del 15%.
//   - si el año es menor a 2000 le aplica un descuento del 5%
const X: usize = 3;
#[derive(PartialEq, Debug)]
enum Colores {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Black,
}
#[derive(PartialEq, Debug)]
struct Auto {
    brand: String,
    model: String,
    year: u32,
    base_price: f32,
    color: Colores,
}
impl Auto {
    fn new(brand: String, model: String, year: u32, base_price: f32, color: Colores) -> Auto {
        Auto {
            brand,
            model,
            year,
            base_price,
            color,
        }
    }
    fn calcular_precio(&self) -> f32 {
        let mut adjustment: f32 = 1.0;
        adjustment = adjustment
            + match self.color {
                Colores::Red | Colores::Yellow | Colores::Blue => 0.25,
                _ => -0.10,
            };
        if self.brand == "BMW" {
            adjustment += 0.15;
        }
        if self.year < 2000 {
            adjustment += -0.05;
        }
        let final_price = self.base_price * adjustment;
        final_price
    }
}
#[derive(Debug)]
struct ConcesionarioAuto {
    dealership: String,
    address: String,
    cars: Vec<Auto>,
}
impl ConcesionarioAuto {
    fn new(n: String, adr: String) -> ConcesionarioAuto {
        ConcesionarioAuto {
            dealership: n,
            address: adr,
            cars: Vec::with_capacity(X),
        }
    }
    fn agregar_auto(&mut self, car: Auto) -> bool {
        if self.cars.len() + 1 <= X {
            self.cars.push(car);
            true
        } else {
            false
        }
    }
    fn eliminar_auto(&mut self) {
        self.cars.pop();
    }
    fn buscar_auto(&self, car: &Auto) -> Option<&Auto> {
        for act_car in &self.cars {
            if car == act_car {
                return Some(act_car);
            }
        }
        None
    }
}
fn main() {
    let mut concesionaria =
        ConcesionarioAuto::new("Rust Motors".to_string(), "123 Memory Lane".to_string());

    let auto1 = Auto::new(
        "BMW".to_string(),
        "M3".to_string(),
        1998,
        10000.0,
        Colores::Red,
    );

    let auto2 = Auto::new(
        "Toyota".to_string(),
        "Corolla".to_string(),
        2010,
        8000.0,
        Colores::White,
    );

    let auto3 = Auto::new(
        "Ford".to_string(),
        "Fiesta".to_string(),
        1995,
        5000.0,
        Colores::Blue,
    );

    println!("Agregar auto1: {}", concesionaria.agregar_auto(auto1));
    println!("Agregar auto2: {}", concesionaria.agregar_auto(auto2));
    println!("Agregar auto3: {}", concesionaria.agregar_auto(auto3));

    println!("Inventario actual: {:#?}", concesionaria.cars);

    // Buscar auto
    let auto_busqueda = Auto::new(
        "Toyota".to_string(),
        "Corolla".to_string(),
        2010,
        8000.0,
        Colores::White,
    );

    match concesionaria.buscar_auto(&auto_busqueda) {
        Some(auto) => {
            println!("Auto encontrado: {:#?}", auto);
            println!("Precio final: {}", auto.calcular_precio());
        }
        None => println!("Auto no encontrado"),
    }

    concesionaria.eliminar_auto();
    println!(
        "Inventario luego de eliminar uno: {:#?}",
        concesionaria.cars
    );
}
