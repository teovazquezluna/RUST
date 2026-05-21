/* 7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser: rojo,
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖ ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna.
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
no lo agrega y retorna false.
➢ eliminar_auto(auto): elimina un auto de la lista de autos.
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖ Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna.
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■ Si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%.
■ Si la marca es BMW le aplica un recargo del 15%.
■ Si el año es menor a 2000 se le aplica un descuento del 5%. */



#[derive(Debug)]
pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub autos: Vec<Auto>,
    pub capacidad_maxima:usize,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String,capacidad_maxima:usize) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_maxima,
            autos: Vec::with_capacity(capacidad_maxima),
        }
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad_maxima {
            self.autos.push(auto);
            true
        } else {
            false
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) {
        if self.autos.is_empty(){
            panic!("Error , vec de autos vacio");
        }
        for i in 0..self.autos.len(){
            if self.autos[i].modelo == auto.modelo && ConcesionarioAuto::comparar_colores(&self.autos[i].color, &auto.color)&& self.autos[i].año ==  auto.año {
                self.autos.remove(i);
                return;
            }
        }

    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for i_auto in &self.autos{
            if i_auto.modelo == auto.modelo && ConcesionarioAuto::comparar_colores(&i_auto.color, &auto.color) && i_auto.año ==  auto.año {
                return Some(i_auto);
            }
        }
        None
    }
    pub fn comparar_colores(color1:&Color,color2:&Color)->bool{
        match(color1,color2){
            (Color::Amarillo,Color::Amarillo)=>true,
            (Color::Azul,Color::Azul)=>true,
            (Color::Blanco,Color::Blanco)=>true,
            (Color::Negro,Color::Negro)=>true,
            (Color::Rojo,Color::Rojo)=>true,
            (Color::Verde,Color::Verde)=>true,
            _=>false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Debug, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub año: u32,
    pub precio_bruto: f32,
    pub color: Color,
}

impl Auto {
    pub fn new(
        marca: String,
        modelo: String,
        año: u32,
        precio_bruto: f32,
        color: Color,
    ) -> Auto {
        Auto {
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }

    pub fn calcular_precio(&self) -> f32 { 
        let mut ajuste: f32 = 1.0;
        ajuste += match self.color {
            Color::Rojo | Color::Amarillo | Color::Azul => 0.25,
            _ => -0.10,
        };
        if self.marca == "BMW" {
            ajuste += 0.15;
        }
        if self.año < 2000 {
            ajuste -= 0.05;
        }
        self.precio_bruto * ajuste 
    }
}

pub fn resolver() {

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_comparar_color(){
        let color1 = Color::Blanco;
        let color2 = Color::Blanco;
        let color3 = Color::Amarillo;
        assert!(ConcesionarioAuto::comparar_colores(&color1, &color2));
        assert!(!ConcesionarioAuto::comparar_colores(&color1, &color3));
    }
    #[test]
    fn test_new_auto() {
        let marca= "renault".to_string();
        let modelo = "clio".to_string();
        let año = 2006;
        let precio_bruto = 7000.0;
        let color= Color::Blanco;
        let auto = Auto::new(marca.clone(), modelo.clone(), año, precio_bruto, color.clone());

        assert_eq!(auto.marca, marca);
        assert_eq!(auto.modelo, modelo);
        assert_eq!(auto.año, año);
        assert_eq!(auto.precio_bruto, precio_bruto);
        assert!(ConcesionarioAuto::comparar_colores(&auto.color, &color));
    }

    #[test]
    fn test_new_concesionario() {
        let nombre = "toyota".to_string();
        let direccion = "calle 1y40".to_string();
        let capacidad_maxima=3;
        let concesionaria = ConcesionarioAuto::new(nombre,direccion,capacidad_maxima);

        assert_eq!(concesionaria.nombre, "toyota");
        assert_eq!(concesionaria.direccion, "calle 1y40");
        assert_eq!(concesionaria.capacidad_maxima, 3);
        assert!(concesionaria.autos.is_empty());
    }

    #[test]
    fn test_agregar_auto() {
        let mut concesionaria = ConcesionarioAuto::new("fiat".to_string(),"av carlitos".to_string(),2,);
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);

        let se_agrego = concesionaria.agregar_auto(auto);

        assert!(se_agrego);
        assert_eq!(concesionaria.autos[0].marca, "fiat".to_string());
        assert_eq!(concesionaria.autos[0].modelo, "600".to_string());
    }

    #[test]
    fn test_agregar_auto_supera_limite() {
        let mut concesionaria = ConcesionarioAuto::new("fiat".to_string(),"av carlitos".to_string(),1,);
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);
        assert!(concesionaria.agregar_auto(auto));
        let auto2 = Auto::new("fiat".to_string(),"600 Electrico".to_string(), 2010, 30000.0, Color::Blanco);
        let expected = false;
        assert_eq!(expected,concesionaria.agregar_auto(auto2)) 

    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionaria = ConcesionarioAuto::new("fiat".to_string(),"av san pablo".to_string(),2,);
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);
        concesionaria.agregar_auto(auto.clone());
        assert!(!concesionaria.autos.is_empty());
        concesionaria.eliminar_auto(&auto);
        assert!(concesionaria.autos.is_empty());
    }

    #[test]
    #[should_panic(expected="Error , vec de autos vacio")]
    fn test_eliminar_auto_vec_vacio() {
        let mut concesionaria = ConcesionarioAuto::new("fiat".to_string(),"av san pablo".to_string(),2,);
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);
        concesionaria.eliminar_auto(&auto);
    }

    #[test]
    fn test_buscar_auto() {
        let mut concesionaria = ConcesionarioAuto::new("fiat".to_string(),"av san pablo".to_string(),2,);
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);
        concesionaria.agregar_auto(auto.clone());
        let auto2 = Auto::new("fiat".to_string(),"600 Electrico".to_string(), 2010, 30000.0, Color::Blanco);
        concesionaria.agregar_auto(auto2.clone());
        if let Some(encontrado) = concesionaria.buscar_auto(&auto2){
            assert_eq!(auto2.año,encontrado.año);
            assert_eq!(auto2.precio_bruto,encontrado.precio_bruto);
            assert_eq!(auto2.modelo,encontrado.modelo);
            assert_eq!(auto2.marca,encontrado.marca);
            assert!(ConcesionarioAuto::comparar_colores(&auto2.color, &encontrado.color));
        }
    }

    #[test]
    fn test_buscar_auto_vec_vacio() {
        let concesionaria = ConcesionarioAuto::new("fiat".to_string(),"av san pablo".to_string(),2,);
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);
        assert!(concesionaria.buscar_auto(&auto).is_none());
    
    }

    #[test]
    fn test_calcular_precio() {
        let auto = Auto::new("fiat".to_string(),"600".to_string(), 1955, 3000.0, Color::Azul);
        let precio = auto.calcular_precio();
        let expected = 3600.0002;
        assert_eq!(precio, expected);
    }
}
