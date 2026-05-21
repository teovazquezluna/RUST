/* 10- Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el isbn, el título, autor, número de páginas, género (novela, infantil,
técnico, otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de
vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en
préstamo. Del cliente se conoce el nombre, teléfono y dirección de correo electrónico.
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

use crate::tp3::ej3::Fecha;

fn fecha_es_anterior(fecha: &Fecha, otra_fecha: &Fecha) -> bool {
    otra_fecha.es_mayor(fecha)
}

#[derive(Clone, PartialEq, Debug)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Libro {
    pub isbn: String,
    pub titulo: String,
    pub autor: String,
    pub numero_de_paginas: u32,
    pub genero: Genero,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: String,
    pub direccion_de_correo_electronico: String,
}

#[derive(Clone, PartialEq)]
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
    pub fn new(
        libro: Libro,
        cliente: Cliente,
        fecha_de_vencimiento_del_prestamo: Fecha,
    ) -> Prestamo {
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

    pub fn obtener_cantidad_de_copias(&self, libro: &Libro) -> u32 {
        self.copias_de_los_libros_a_disposicion_para_prestar
            .get(&libro.titulo)
            .copied()
            .unwrap_or(0)
    }

    pub fn decrementar_cantidad_de_copias_a_disposicion(&mut self, libro: &Libro) {
        if let Some(copias) = self
            .copias_de_los_libros_a_disposicion_para_prestar
            .get_mut(&libro.titulo)
        {
            if *copias > 0 {
                *copias -= 1;
            }
        }
    }

    pub fn incrementar_cantidad_de_copias_a_disposicion(&mut self, libro: &Libro) {
        if let Some(copias) = self
            .copias_de_los_libros_a_disposicion_para_prestar
            .get_mut(&libro.titulo)
        {
            *copias += 1;
        }
    }

    pub fn contar_prestamos_de_un_cliente(&self, cliente: &Cliente) -> usize {
        self.prestamos_efectuados
            .iter()
            .filter(|prestamo| {
                prestamo.estado == Estado::EnPrestamo && prestamo.cliente == *cliente
            })
            .count()
    }

    pub fn realizar_un_prestamo_de_un_libro_para_un_cliente(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
        fecha_de_vencimiento_del_prestamo: Fecha,
    ) -> bool {
        let copias = self.obtener_cantidad_de_copias(libro);
        if copias > 0 && self.contar_prestamos_de_un_cliente(cliente) < 5 {
            self.prestamos_efectuados.push(Prestamo::new(
                libro.clone(),
                cliente.clone(),
                fecha_de_vencimiento_del_prestamo,
            ));
            self.decrementar_cantidad_de_copias_a_disposicion(libro);
            true
        } else {
            false
        }
    }

    pub fn ver_prestamos_a_vencer_en_los_proximos_dias(
        &self,
        fecha_actual: Fecha,
        dias: u32,
    ) -> Vec<&Prestamo> {
        let mut total = fecha_actual.dia + dias;
        let mut fecha_limite = fecha_actual.clone();
        let dias_del_mes = fecha_actual.calculardiasdelmes();
        if total > dias_del_mes {
            while total > dias_del_mes {
                total -= dias_del_mes;
                if fecha_limite.mes == 12 {
                    fecha_limite.año += 1;
                    fecha_limite.mes = 1;
                } else {
                    fecha_limite.mes += 1;
                }
                fecha_limite.dia = 1;
            }
        }
        fecha_limite.dia = total;
        self.prestamos_efectuados
            .iter()
            .filter(|prestamo| {
                fecha_es_anterior(
                    &prestamo.fecha_de_vencimiento_del_prestamo,
                    &fecha_limite,
                )
            })
            .collect()
    }

    pub fn ver_los_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<&Prestamo> {
        self.prestamos_efectuados
            .iter()
            .filter(|prestamo| {
                prestamo.estado == Estado::EnPrestamo
                    && fecha_es_anterior(
                        &prestamo.fecha_de_vencimiento_del_prestamo,
                        &fecha_actual,
                    )
            })
            .collect()
    }

    pub fn buscar_prestamo(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
    ) -> Option<&mut Prestamo> {
        self.prestamos_efectuados
            .iter_mut()
            .find(|prestamo| prestamo.libro == *libro && prestamo.cliente == *cliente)
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
    let mut biblioteca = Biblioteca::new(
        String::from("Biblioteca Central"),
        String::from("Calle Principal 123"),
    );

    let libro1 = Libro {
        isbn: String::from("978-123"),
        titulo: String::from("Rust Programming"),
        autor: String::from("Steve"),
        numero_de_paginas: 500,
        genero: Genero::Tecnico,
    };

    let libro2 = Libro {
        isbn: String::from("978-456"),
        titulo: String::from("Harry Potter"),
        autor: String::from("Rowling"),
        numero_de_paginas: 350,
        genero: Genero::Novela,
    };

    let cliente1 = Cliente {
        nombre: String::from("Alice"),
        telefono: String::from("123456"),
        direccion_de_correo_electronico: String::from("alice@mail.com"),
    };

    biblioteca
        .copias_de_los_libros_a_disposicion_para_prestar
        .insert(libro1.titulo.clone(), 3);
    biblioteca
        .copias_de_los_libros_a_disposicion_para_prestar
        .insert(libro2.titulo.clone(), 2);

    println!(
        "{} -> {} copias",
        libro1.titulo,
        biblioteca.obtener_cantidad_de_copias(&libro1)
    );
    println!(
        "{} -> {} copias",
        libro2.titulo,
        biblioteca.obtener_cantidad_de_copias(&libro2)
    );

    let fecha_actual = Fecha::new(5, 3, 2026);
    let fecha_de_vencimiento_del_prestamo = Fecha::new(10, 3, 2026);

    let prestamo_creado = biblioteca.realizar_un_prestamo_de_un_libro_para_un_cliente(
        &libro1,
        &cliente1,
        fecha_de_vencimiento_del_prestamo,
    );
    println!("¿Préstamo creado? {}", prestamo_creado);
    println!(
        "Préstamos del cliente: {}",
        biblioteca.contar_prestamos_de_un_cliente(&cliente1)
    );

    let prestamos_a_vencer =
        biblioteca.ver_prestamos_a_vencer_en_los_proximos_dias(fecha_actual.clone(), 7);
    println!("Préstamos a vencer en 7 días: {}", prestamos_a_vencer.len());

    let prestamos_vencidos = biblioteca.ver_los_prestamos_vencidos(fecha_actual.clone());
    println!("Préstamos vencidos: {}", prestamos_vencidos.len());

    biblioteca.devolver_libro(&libro1, &cliente1, fecha_actual);
    println!(
        "Copias de {} después de devolver: {}",
        libro1.titulo,
        biblioteca.obtener_cantidad_de_copias(&libro1)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realizar_un_prestamo_de_un_libro_para_un_cliente_y_devolver_libro() {
        let mut biblioteca = Biblioteca::new("B".to_string(), "D".to_string());
        let libro = Libro {
            isbn: "1".to_string(),
            titulo: "L".to_string(),
            autor: "A".to_string(),
            numero_de_paginas: 1,
            genero: Genero::Otros,
        };
        let cliente = Cliente {
            nombre: "C".to_string(),
            telefono: "1".to_string(),
            direccion_de_correo_electronico: "e@e.com".to_string(),
        };
        biblioteca
            .copias_de_los_libros_a_disposicion_para_prestar
            .insert(libro.titulo.clone(), 1);
        assert!(biblioteca.realizar_un_prestamo_de_un_libro_para_un_cliente(
            &libro,
            &cliente,
            Fecha::new(1, 1, 2027)
        ));
        assert_eq!(0, biblioteca.obtener_cantidad_de_copias(&libro));
        biblioteca.devolver_libro(&libro, &cliente, Fecha::new(2, 1, 2027));
        assert_eq!(1, biblioteca.obtener_cantidad_de_copias(&libro));
    }
}
