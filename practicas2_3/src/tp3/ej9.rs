/* 9- Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal (perro, gato, caballo, otros) y su dueño. Del
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:
➔ Crear una veterinaria.
➔ Agregar una nueva mascota a la cola de atención de la veterinaria.
➔ Agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ Atender la próxima mascota de la cola.
➔ Eliminar una mascota específica de la cola de atención dado que se retira.
➔ Registrar una atención.
➔ Buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono.
➔ Modificar el diagnóstico de una determinada atención.
➔ Modificar la fecha de la próxima visita de una determinada atención.
➔ Eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3. */

use core::panic;
use std::collections::VecDeque;

use crate::tp3::ej3::Fecha;

#[derive(Clone, Debug)]
pub struct Dueño {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
}
impl Dueño{
    pub fn new(nombre:String,direccion:String,telefono:String)->Dueño{
        Dueño{
            nombre,
            direccion,
            telefono,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TipoDeAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Clone, Debug)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u32,
    pub tipo_de_animal: TipoDeAnimal,
    pub dueño: Dueño,
}
impl Mascota{
    pub fn new(nombre:String,edad:u32,tipo_de_animal:TipoDeAnimal,dueño:Dueño)->Mascota{
        Mascota{
            nombre,
            edad,
            tipo_de_animal,
            dueño,
        }
    }
}

pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico_final: String,
    pub tratamiento: String,
    pub fecha_de_la_proxima_visita: Option<Fecha>,
}
impl Atencion{
    pub fn new(mascota:Mascota,diagnostico_final:String,tratamiento:String,fecha_de_la_proxima_visita:Option<Fecha>)->Atencion{
        Atencion{
            mascota,
            diagnostico_final,
            tratamiento,
            fecha_de_la_proxima_visita,
        }
    }
}

pub struct Veterinaria {
    pub nombre: String,
    pub direccion: String,
    pub id: u32,
    pub cola_de_atencion: VecDeque<Mascota>,
    pub registro_de_atenciones: Vec<Atencion>,
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32) -> Veterinaria {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_de_atencion: VecDeque::new(),
            registro_de_atenciones: Vec::new(),
        }
    }  
    pub fn mismo_dueño(d1:&Dueño,d2:&Dueño)->bool{
        d1.nombre==d2.nombre &&
        d1.direccion==d2.direccion &&
        d1.telefono==d2.telefono
    }

    pub fn mismo_tipo_de_animal(tipo1:&TipoDeAnimal,tipo2:&TipoDeAnimal)->bool{
        match(tipo1,tipo2){
            (TipoDeAnimal::Perro,TipoDeAnimal::Perro)=>true,
            (TipoDeAnimal::Gato,TipoDeAnimal::Gato)=>true,
            (TipoDeAnimal::Caballo,TipoDeAnimal::Caballo)=>true,
            (TipoDeAnimal::Otros,TipoDeAnimal::Otros)=>true,
            _=>false,
        }
    }

    fn misma_mascota(m1:&Mascota,m2:&Mascota)->bool{
        m1.nombre==m2.nombre &&
        m1.edad==m2.edad &&
        Veterinaria::mismo_tipo_de_animal(&m1.tipo_de_animal,&m2.tipo_de_animal) &&
        Veterinaria::mismo_dueño(&m1.dueño,&m2.dueño)
    }

    pub fn agregar_mascota_cola(&mut self,mascota: Mascota) {
        self.cola_de_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_cola_prioridad(&mut self,mascota: Mascota) {
        self.cola_de_atencion.push_front(mascota);
    }

    pub fn atender_cola(&mut self) -> Option<Mascota> {
        self.cola_de_atencion.pop_front()
    }

    pub fn eliminar_de_cola(&mut self,mascota: &Mascota) {
        if self.cola_de_atencion.is_empty(){
            panic!("Ya no hay mascotas para eliminar");
        }
        for i in 0..self.cola_de_atencion.len(){
            if Veterinaria::misma_mascota(&self.cola_de_atencion[i], mascota){
                self.cola_de_atencion.remove(i);
                return;
            }
        }

    }

    pub fn registrar_una_atencion(&mut self, atencion: Atencion) {
        self.registro_de_atenciones.push(atencion);
    }

    pub fn buscar_una_atencion(&self,nombre_de_la_mascota: &str,nombre_del_dueño: &str,telefono: &str) -> Option<usize> {
        for i in 0..self.registro_de_atenciones.len(){
            if self.registro_de_atenciones[i].mascota.nombre == nombre_de_la_mascota && self.registro_de_atenciones[i].mascota.dueño.nombre == nombre_del_dueño && self.registro_de_atenciones[i].mascota.dueño.telefono == telefono {
                return Some(i);
            }
            }
        
        None
    }
    
    

    pub fn modificar_diagnostico_atencion(&mut self,indice: usize,diagnostico_final: String) {
        if indice<self.registro_de_atenciones.len(){
        self.registro_de_atenciones[indice].diagnostico_final=diagnostico_final;

    }}

    pub fn modificar_fecha_atencion(&mut self,indice: usize,fecha_de_la_proxima_visita: Fecha) {
        if indice < self.registro_de_atenciones.len() {
            self.registro_de_atenciones[indice].fecha_de_la_proxima_visita = Some(fecha_de_la_proxima_visita);
        }
    }

    pub fn eliminar_una_determinada_atencion(&mut self, indice: usize) {
        if indice < self.registro_de_atenciones.len() {
            self.registro_de_atenciones.remove(indice);
        }
    }
}

pub fn resolver() {
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new_veterinaria(){
        let nombre="veterinaria".to_string();
        let direccion="calle 12".to_string();
        let id=1;

        let veterinaria=Veterinaria::new(nombre,direccion,id);

        assert_eq!(veterinaria.nombre,"veterinaria");
        assert_eq!(veterinaria.direccion,"calle 12");
        assert_eq!(veterinaria.id,id);
        assert!(veterinaria.cola_de_atencion.is_empty());
        assert!(veterinaria.registro_de_atenciones.is_empty());
    }
    #[test]
    fn test_new_dueño(){
        let nombre="juan".to_string();
        let direccion="calle 44".to_string();
        let telefono="221".to_string();
        let dueño=Dueño::new(nombre,direccion,telefono);

        assert_eq!(dueño.nombre,"juan");
        assert_eq!(dueño.direccion,"calle 44");
        assert_eq!(dueño.telefono,"221");
}

    #[test]
    fn test_new_mascota(){
        let dueño=Dueño::new("juan".to_string(),"calle".to_string(),"221".to_string());
        let nombre ="firulais".to_string();
        let edad = 6;
        let tipo_de_animal = TipoDeAnimal::Perro;
        let mascota = Mascota::new(nombre.clone(), edad, tipo_de_animal.clone(), dueño.clone());

        assert_eq!(mascota.nombre , nombre);
        assert_eq!(mascota.edad,edad);
        assert!(Veterinaria::mismo_tipo_de_animal(&mascota.tipo_de_animal,&tipo_de_animal));
        assert!(Veterinaria::mismo_dueño(&mascota.dueño, &dueño));
   
    }

    #[test]
    fn test_new_atencion(){
        let dueño=Dueño::new("juan".to_string(),"calle".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);
        let diagnostico_final="enfermo".to_string();
        let tratamiento="comer".to_string();
        let fecha_de_la_proxima_visita=None;

        let atencion = Atencion::new(mascota.clone(), diagnostico_final.clone(), tratamiento.clone(), fecha_de_la_proxima_visita);
        assert_eq!(atencion.diagnostico_final,diagnostico_final);
        assert_eq!(atencion.tratamiento,tratamiento);
        assert!(Veterinaria::misma_mascota(&atencion.mascota, &mascota));
        assert!(atencion.fecha_de_la_proxima_visita.is_none());
    }
    #[test]
    fn test_agregar_mascota(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);

        veterinaria.agregar_mascota_cola(mascota);
        assert!(!veterinaria.cola_de_atencion.is_empty());
    }

    #[test]
    fn test_agregar_mascota_prioridad(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño.clone());
        let mascota2=Mascota::new("canela".to_string(),1,TipoDeAnimal::Gato,dueño);
        veterinaria.agregar_mascota_cola(mascota);
        veterinaria.agregar_mascota_cola_prioridad(mascota2);

        assert_eq!(veterinaria.cola_de_atencion[0].nombre,"canela");
    }

    #[test]
    fn test_atender_mascota(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);

        veterinaria.agregar_mascota_cola(mascota);

        let mascota_por_atender=veterinaria.atender_cola();

        assert!(mascota_por_atender.is_some());
        assert!(veterinaria.cola_de_atencion.is_empty());
    }

    #[test]
    fn test_eliminar_mascota(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);

        veterinaria.agregar_mascota_cola(mascota.clone());
        veterinaria.eliminar_de_cola(&mascota);
        assert!(veterinaria.cola_de_atencion.is_empty());
    }
    #[test]
    #[should_panic(expected ="Ya no hay mascotas para eliminar")]
    fn test_eliminar_mascota_cola_vacia(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);

        veterinaria.eliminar_de_cola(&mascota);
    }

    #[test]
    fn test_registrar_atencion(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);
        let atencion=Atencion::new(mascota,"gripe".to_string(),"reposo".to_string(),None);

        veterinaria.registrar_una_atencion(atencion);

        assert!(!veterinaria.registro_de_atenciones.is_empty());
    }

    #[test]
    fn test_buscar_atencion(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);
        let atencion=Atencion::new(mascota,"gripe".to_string(),"reposo".to_string(),None);

        veterinaria.registrar_una_atencion(atencion);
        let resultado=veterinaria.buscar_una_atencion("Condor","juan","221");
        let expected=Some(0);

        assert_eq!(resultado,expected);
    }

    #[test]
    fn test_modificar_diagnotico(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);
        let atencion=Atencion::new(mascota,"gripe".to_string(),"reposo".to_string(),None);


        veterinaria.registrar_una_atencion(atencion);
        veterinaria.modificar_diagnostico_atencion(0,"curado".to_string());
        let expected = "curado".to_string();
        assert_eq!(veterinaria.registro_de_atenciones[0].diagnostico_final,expected);
    }

    #[test]
    fn test_eliminar_atencion(){
        let mut veterinaria=Veterinaria::new("veterinaria".to_string(),"calle 12".to_string(),1);
        let dueño=Dueño::new("juan".to_string(),"calle 44".to_string(),"221".to_string());
        let mascota=Mascota::new("Condor".to_string(),12,TipoDeAnimal::Perro,dueño);
        let atencion=Atencion::new(mascota,"gripe".to_string(),"reposo".to_string(),None);
        veterinaria.registrar_una_atencion(atencion);
        veterinaria.eliminar_una_determinada_atencion(0);
        assert!(veterinaria.registro_de_atenciones.is_empty());
    }
}
