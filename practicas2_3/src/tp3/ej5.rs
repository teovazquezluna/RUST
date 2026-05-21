/* 5- Escribir un programa que defina una estructura Producto que tenga campos para el
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
el precio bruto.
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
descuento sobre el precio bruto.
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
parámetros son opcionales. */

#[derive(Debug)]
pub struct Producto {
    pub nombre: String,
    pub precio_bruto: f32,
    pub numero_identificatorio: u32,
}

impl Producto {
    pub fn new(nombre: String, precio_bruto: f32, numero_identificatorio: u32) -> Producto {
        Producto {
            nombre,
            precio_bruto,
            numero_identificatorio,
        }
    }

    pub fn calcular_impuestos(&self, porcentaje_de_impuestos: f32) -> f32 {
        (self.precio_bruto * porcentaje_de_impuestos) / 100.0
    }

    pub fn aplicar_descuento(&self, porcentaje_de_descuento: f32) -> f32 {
        (self.precio_bruto * porcentaje_de_descuento) / 100.0
    }

    pub fn calcular_precio_total(&self,porcentaje_de_impuestos: Option<f32>,porcentaje_descuento: Option<f32>,) -> f32 {
        let porcentaje = match(porcentaje_de_impuestos,porcentaje_descuento){
            (Some(imp),None)=>{self.calcular_impuestos(imp)},
            (None,Some(desc))=>{-self.aplicar_descuento(desc)},
            (Some(imp),Some(desc))=>{self.calcular_impuestos(imp)-self.aplicar_descuento(desc)},
            _=>0.0,
        };
        self.precio_bruto + porcentaje
    }
}

pub fn resolver() {
    let producto = Producto::new("Coca Cola".to_string(), 2500.50, 1297481);
    let porcentaje_de_impuestos = 50.98;
    let porcentaje_de_descuento = 30.0;
    println!("Este es mi producto: {:?}", producto);
    println!("impuestos de: {:?}",producto.calcular_impuestos(porcentaje_de_impuestos));
    println!(" descuento de: {:?}",producto.aplicar_descuento(porcentaje_de_descuento));
    println!("Lo que realmente pagare {:?}",producto.calcular_precio_total(None, Some(porcentaje_de_descuento)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_producto() {
        let nombre = "jabon".to_string();
        let precio_bruto = 100.0;
        let numero_identificatorio = 1234;
        let producto = Producto::new(nombre.clone(), precio_bruto, numero_identificatorio);

        assert_eq!(producto.nombre, nombre);
        assert_eq!(producto.precio_bruto, precio_bruto);
        assert_eq!(producto.numero_identificatorio, numero_identificatorio);
    }

    #[test]
    fn test_calcular_impuestos() {
        let base = 100.0;
        let porcentaje_impuesto = 20.0;
        let expected = 20.0;
        let producto = Producto::new("cebolla".to_string(), base, 1);
        let resultado = producto.calcular_impuestos(porcentaje_impuesto);

        assert_eq!(resultado, expected);
    }

    #[test]
    fn test_aplicar_descuento() {
        let base = 200.0;
        let porcentaje_descuento = 10.0;
        let expected = 20.0;
        let producto = Producto::new("fideos".to_string(), base, 1);
        let resultado = producto.aplicar_descuento(porcentaje_descuento);

        assert_eq!(resultado, expected);
    }

    #[test]
    fn test_calcular_precio_total_con_ambos() {
        let base = 100.0;
        let imp = Some(20.0);
        let desc = Some(10.0);
        let expected = 110.0;
        let producto = Producto::new("leche".to_string(), base, 1);
        let resultado = producto.calcular_precio_total(imp, desc);

        assert_eq!(resultado, expected);
    }

    #[test]
    fn test_calcular_precio_total_con_none() {
        let base = 100.0;
        let imp = None;
        let desc = None;
        let expected = 100.0;
        let producto = Producto::new("vino".to_string(), base, 1);
        let resultado = producto.calcular_precio_total(imp, desc);

        assert_eq!(resultado, expected);
    }

    #[test]
    fn test_calcular_precio_total_mixto() {
        let base = 100.0;
        let imp = Some(15.0);
        let desc = None;
        let expected = 115.0;
        let producto = Producto::new("queso".to_string(), base, 1);
        let resultado = producto.calcular_precio_total(imp, desc);

        assert_eq!(resultado, expected);

        let base2:f32 = 100.0;
        let imp2:Option<f32>= None;
        let desc2 = Some(25.0);
        let expected2 = 75.0;
        let producto2 = Producto::new("yerba".to_string(), base2, 2);
        let resultado2 = producto2.calcular_precio_total(imp2, desc2);

        assert_eq!(resultado2, expected2);
    }
}
