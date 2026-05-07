/*// 5- Escribir un programa que defina una estructura Producto que tenga campos para el
// nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
// siguientes métodos:
//
// > new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
// > calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
//   el precio bruto.
// > aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
//   descuento sobre el precio bruto.
// > calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
//   precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
//   parámetros son opcionales. */
#[derive(Debug)]
struct Producto {
    name: String,
    gross_price: f32,
    idnum: u32,
}

impl Producto {
    fn new(name: String, price: f32, id: u32) -> Producto {
        Producto {
            name,
            gross_price: price,
            idnum: id,
        }
    }
    fn calcular_impuestos(&self, taxes_percentage: f32) -> f32 {
        (self.gross_price * taxes_percentage) / 100.0
    }
    fn aplicar_descuento(&self, discount_percentage: f32) -> f32 {
        (self.gross_price * discount_percentage) / 100.0
    }
    fn calcular_precio_total(
        &self,
        tax_percent: Option<f32>,
        discount_percent: Option<f32>,
    ) -> f32 {
        let mut impuesto: f32;
        let mut descuento: f32;
        if let Some(dato) = tax_percent {
            impuesto = self.calcular_impuestos(dato);
        } else {
            impuesto = 0.0;
        }
        if let Some(dato) = discount_percent {
            descuento = self.aplicar_descuento(dato);
        } else {
            descuento = 0.0;
        }
        self.gross_price + impuesto - descuento
    }
}
fn main() {
    let producto = Producto::new("Coca Cola".to_string(), 2500.50, 1297481);
    let tax = 50.98;
    let discount = 30.0;
    println!("Este es mi producto: {:?}", producto);
    println!("impuestos de: {:?}", producto.calcular_impuestos(tax));
    println!(" descuento de: {:?}", producto.aplicar_descuento(discount));
    println!(
        "Lo que realmente pagare {:?}",
        producto.calcular_precio_total(None, Some(discount))
    );
}
