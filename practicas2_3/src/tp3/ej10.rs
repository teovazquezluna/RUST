/* 10- Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el isbn, el título, autor, número de páginas, género (novela, infantil,
técnico, otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de
vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en
préstamo. 
Del cliente se conoce el nombre, teléfono y dirección de correo electrónico.

Implemente los métodos necesarios para realizar las siguientes acciones:
➔ Obtener cantidad de copias: dado un determinado libro retorna la cantidad de
copias a disposición que hay para prestar de dicho libro.
➔ Decrementar cantidad de copias a disposición; dado un libro decrementa en
1 la cantidad de copias de libros a disposición para prestar.
➔ Incrementar cantidad de copias a disposición: dado un libro incrementa en 1
la cantidad de copias del libro a disposición para ser prestado.
➔ Contar préstamos de un cliente: devuelve la cantidad de préstamos en
estado “en préstamo” de un determinado cliente.
➔ Realizar un préstamo de un libro para un cliente: crea un préstamo de un
libro para un determinado cliente cumpliendo con lo siguiente:
◆ Que el cliente no tenga más de 5 préstamos en el estado “en
préstamo”.
◆ Que haya al menos una copia disponible en el registro de copias a
disposición. De ser así descuenta 1 en el registro de “copias a
disposición” y retorna true, si no cumple con alguna de las
condiciones retorna false.
➔ Ver préstamos a vencer el los próximos días: retorna una lista de préstamos
a vencer el los próximos días, el valor de días es pasado por parámetro.
➔ Ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
préstamo” donde la fecha de vencimiento es menor a la fecha actual.
➔ Buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
existe.
➔ Devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
estado “devuelto”, se registra la fecha de devolución y se incrementa la
cantidad de libros en 1 del libro devuelto en el registro de copias a
disposición.
Nota: para la fecha utilice lo implementado en el punto 3. */


use std::collections::HashMap;

use crate::tp3::{ej3::Fecha};



#[derive(Clone, Debug)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Clone,Debug)]
pub struct Libro {
    pub isbn: String,
    pub titulo: String,
    pub autor: String,
    pub numero_de_paginas: u32,
    pub genero: Genero,
}

#[derive(Clone,Debug)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: String,
    pub direccion_de_correo_electronico: String,
}

#[derive(Clone)]
pub enum Estado {
    Devuelto,
    EnPrestamo,
}

pub struct Prestamo {
    pub libro: Libro,
    pub cliente: Cliente,
    pub fecha_de_vencimiento_del_prestamo: Fecha,
    pub fecha_de_devolucion: Option<Fecha>,
    pub estado: Estado,
}

impl Prestamo {
    pub fn new(libro: Libro,cliente: Cliente,fecha_de_vencimiento_del_prestamo: Fecha) -> Prestamo {
        Prestamo {
            libro,
            cliente,
            fecha_de_vencimiento_del_prestamo,
            fecha_de_devolucion: None,
            estado: Estado::EnPrestamo,
        }
    }
}

pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub copias_de_los_libros_a_disposicion_para_prestar: HashMap<String, u32>,
    pub prestamos_efectuados: Vec<Prestamo>,
}

impl Biblioteca {
    pub fn new(nombre: String, direccion: String) -> Biblioteca {
        Biblioteca {
            nombre,
            direccion,
            copias_de_los_libros_a_disposicion_para_prestar: HashMap::new(),
            prestamos_efectuados: Vec::new(),
        }
    }
    pub fn comparar_clientes(cliente1: &Cliente, cliente2: &Cliente) -> bool {
        cliente1.nombre == cliente2.nombre
        && cliente1.telefono == cliente2.telefono
        && cliente1.direccion_de_correo_electronico
            == cliente2.direccion_de_correo_electronico
}

    pub fn comparar_libros(libro1: &Libro, libro2: &Libro) -> bool {
        libro1.isbn == libro2.isbn
        && libro1.titulo == libro2.titulo
        && libro1.autor == libro2.autor
        && libro1.numero_de_paginas == libro2.numero_de_paginas
}

    pub fn obtener_cantidad_de_copias(&self, libro: &Libro) -> u32 {
        if let Some(copias) = self.copias_de_los_libros_a_disposicion_para_prestar.get(&libro.isbn)  {
            *copias
        }else{
            panic!("No existen copias");
        }  
        }

    pub fn decrementar_cantidad_de_copias_a_disposicion(&mut self, libro: &Libro) {
        if let Some(copias) = self.copias_de_los_libros_a_disposicion_para_prestar.get_mut(&libro.isbn){
            if *copias > 0 {
                *copias -= 1;
            }
        }else{
            panic!("No existen copias a decrementar")
        }
    }

    pub fn incrementar_cantidad_de_copias_a_disposicion(&mut self, libro: &Libro) {
        if let Some(copias) = self.copias_de_los_libros_a_disposicion_para_prestar.get_mut(&libro.isbn){
            *copias += 1;
        }else{
            panic!("No existen copias a incrementar")
        }
    }

    pub fn contar_prestamos_de_un_cliente(&self, cliente: &Cliente) -> usize {
        let mut contador =0;
        for prestamo in &self.prestamos_efectuados{
            if Biblioteca::comparar_clientes(&prestamo.cliente, &cliente) && match prestamo.estado{Estado::EnPrestamo =>true,_=>false,} {
                contador+=1;
            }
        }
        contador
    }

    pub fn realizar_un_prestamo_de_un_libro_para_un_cliente(&mut self,libro: &Libro,cliente: &Cliente,fecha_de_vencimiento_del_prestamo: Fecha) -> bool {
        let copias = self.obtener_cantidad_de_copias(libro);
        if copias > 0 && !self.contar_prestamos_de_un_cliente(cliente) > 5 {
            self.prestamos_efectuados.push(Prestamo::new(libro.clone(),cliente.clone(),fecha_de_vencimiento_del_prestamo,));
            self.decrementar_cantidad_de_copias_a_disposicion(libro);
            true
        } else {
            false
        }
    }

    pub fn ver_prestamos_a_vencer_en_los_proximos_dias(&self,fecha_actual: Fecha,dias: u32,) -> Vec<&Prestamo> {
        let mut prestamos = Vec::new();
        let mut fecha_limite:Fecha = fecha_actual;
        fecha_limite.sumar_dias(dias);
        for prestamo in &self.prestamos_efectuados{
            if !prestamo.fecha_de_vencimiento_del_prestamo.es_mayor(&fecha_limite){
                prestamos.push(prestamo);
            }
        }
        prestamos

    }

    pub fn ver_los_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<&Prestamo> {
        let mut prestamos = Vec::new();
        for prestamo in &self.prestamos_efectuados{
            if !prestamo.fecha_de_vencimiento_del_prestamo.es_mayor(&fecha_actual){
                prestamos.push(prestamo);
            }
        }
        prestamos
    }

    pub fn buscar_prestamo(&mut self,libro: &Libro,cliente: &Cliente,) -> Option<&mut Prestamo> {
        for prestamo in &mut self.prestamos_efectuados{
            if Biblioteca::comparar_clientes(&prestamo.cliente, &cliente) && Biblioteca::comparar_libros(&prestamo.libro, libro){
                return Some(prestamo);
            }
        }
        None

     }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_actual: Fecha) {
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            prestamo.estado = Estado::Devuelto;
            prestamo.fecha_de_devolucion = Some(fecha_actual);
            self.incrementar_cantidad_de_copias_a_disposicion(libro);
        }
    }
}

pub fn resolver() {
   
}

#[cfg(test)]
mod tests {
    use super::*;

    //funciones auxiliares para los tests
    fn new_libro() -> Libro {
        Libro { isbn: "1".to_string(), titulo: "libro".to_string(), autor: "autor".to_string(), numero_de_paginas: 10, genero: Genero::Novela }
    }

    fn new_cliente() -> Cliente {
        Cliente { nombre: "juan".to_string(), telefono: "123".to_string(), direccion_de_correo_electronico: "juan@gmail".to_string() }
    }

    fn new_fecha() -> Fecha {
        Fecha::new(1,1,2026)
    }

    #[test]
    fn test_comparar_clientes_true() {
        let c1 = new_cliente();
        let c2 = new_cliente();

        assert!(Biblioteca::comparar_clientes(&c1,&c2));
    }

    #[test]
    fn test_comparar_clientes_false() {
        let c1 = new_cliente();
        let c2 = Cliente { nombre: "otro".to_string(), telefono: "123".to_string(), direccion_de_correo_electronico: "juan@gmail".to_string() };

        assert!(!Biblioteca::comparar_clientes(&c1,&c2));
    }

    #[test]
    fn test_comparar_libros_true() {
        let l1 = new_libro();
        let l2 = new_libro();

        assert!(Biblioteca::comparar_libros(&l1,&l2));
    }
        #[test]
    fn test_comparar_libros_false() {
        let l1 = new_libro();
        let l2 = Libro{isbn:123.to_string(),titulo:"El señor de los anillos".to_string(),autor:"alguien".to_string(),numero_de_paginas:8000,genero:Genero::Novela};

        assert!(!Biblioteca::comparar_libros(&l1,&l2));
    }
    #[test]
    fn test_obtener_cantidad_de_copias() {
        let libro = new_libro();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),5);
        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro),5);
    }

    #[test]
    #[should_panic(expected="No existen copias")]
    fn test_obtener_cantidad_de_copias_panic() {
        let libro = new_libro();
        let biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.obtener_cantidad_de_copias(&libro);
    }

    #[test]
    fn test_decrementar_cantidad() {
        let libro = new_libro();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),5);

        biblioteca.decrementar_cantidad_de_copias_a_disposicion(&libro);

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro),4);
    }

    #[test]
    #[should_panic(expected ="No existen copias a decrementar")]
    fn test_decrementar_cantidad_panic() {
        let libro = new_libro();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.decrementar_cantidad_de_copias_a_disposicion(&libro);
    }

    #[test]
    fn test_incrementar_cantidad() {
        let libro = new_libro();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),1);

        biblioteca.incrementar_cantidad_de_copias_a_disposicion(&libro);

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro),2);
    }

    #[test]
    #[should_panic(expected ="No existen copias a incrementar")]
    fn test_incrementar_cantidad_panic() {
        let libro = new_libro();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.incrementar_cantidad_de_copias_a_disposicion(&libro);
    }

    #[test]
    fn test_contar_prestamos() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.prestamos_efectuados.push(Prestamo::new(libro,cliente.clone(),new_fecha()));

        assert_eq!(biblioteca.contar_prestamos_de_un_cliente(&cliente),1);
    }

    #[test]
    fn test_realizar_prestamo_true() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),3);

        let resultado = biblioteca.realizar_un_prestamo_de_un_libro_para_un_cliente(&libro,&cliente,new_fecha());

        assert!(resultado);
    }

    #[test]
    fn test_realizar_prestamo_false() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),0);

        let resultado = biblioteca.realizar_un_prestamo_de_un_libro_para_un_cliente(&libro,&cliente,new_fecha());

        assert!(!resultado);

    }
    #[test]
    fn test_realizar_prestamo_false_mayor5() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),0);
        for _i in 0..5 {
            biblioteca.prestamos_efectuados.push(Prestamo::new(libro.clone(), cliente.clone(), Fecha { dia:1, mes: 1, año: 2027 }));
        }
        let resultado = biblioteca.realizar_un_prestamo_de_un_libro_para_un_cliente(&libro,&cliente,new_fecha());

        assert!(!resultado);
        
    }
    #[test]
    fn test_ver_prestamos_a_vencer_en_los_proximos_dias() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.prestamos_efectuados.push(Prestamo::new(libro,cliente,Fecha::new(5,1,2025)));
        let prestamos = biblioteca.ver_prestamos_a_vencer_en_los_proximos_dias(Fecha::new(1,1,2025),10);
        assert_eq!(prestamos.len(),1);
    }
    #[test]
    fn test_ver_prestamos_vencidos() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.prestamos_efectuados.push(Prestamo::new(libro,cliente,Fecha::new(1,1,2020)));

        let prestamos = biblioteca.ver_los_prestamos_vencidos(Fecha::new(1,1,2025));
        let expected = 1;
        assert_eq!(prestamos.len(),expected);
    }

    #[test]
    fn test_buscar_prestamo_some() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        biblioteca.prestamos_efectuados.push(Prestamo::new(libro.clone(),cliente.clone(),new_fecha()));

        let prestamo = biblioteca.buscar_prestamo(&libro,&cliente);

        assert!(prestamo.is_some());
    }

    #[test]
    fn test_buscar_prestamo_none() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());

        let prestamo = biblioteca.buscar_prestamo(&libro,&cliente);

        assert!(prestamo.is_none());
    }

    #[test]
    fn test_devolver_libro() {
        let libro = new_libro();
        let cliente = new_cliente();
        let mut biblioteca = Biblioteca::new("biblio".to_string(),"calle".to_string());
        let fecha =Fecha{dia:1,mes:2,año:2025};
        biblioteca.copias_de_los_libros_a_disposicion_para_prestar.insert(libro.isbn.clone(),1);

        biblioteca.realizar_un_prestamo_de_un_libro_para_un_cliente(&libro, &cliente, fecha);
        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro),0);
        biblioteca.devolver_libro(&libro,&cliente,Fecha::new(1,1,2026));

        assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro),1);
    }
}

