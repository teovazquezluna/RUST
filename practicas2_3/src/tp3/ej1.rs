/* 1- Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección (que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢ to_string: que retorna un string con los datos de la persona concatenados sobre el
mensaje ejecutado por ej:
person.to_string(), donde person es una variable del tipo Persona.
➢ obtener_edad: retorna la edad de la persona.
➢ actualizar_direccion(nueva_direccion) */
pub struct Persona {
    pub nombre: String,
    pub edad: u32,
    pub direccion: Option<String>,
}

impl Persona {
    pub fn new(nombre: String, edad: u32, direccion: Option<String>) -> Persona {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    pub fn to_string(&self) -> String {
        let mensaje = format!("Nombre: {} \n Edad: {} \n ", self.nombre, self.edad);
        let direccion_texto;
        if let Some(d) = &self.direccion {
            direccion_texto = format!("Direccion: {} ", d);
        } else {
            direccion_texto = "No existe direccion".to_string();
        }
        mensaje + &direccion_texto
    }

    pub fn obtener_edad(&self) -> u32 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
        self.direccion = nueva_direccion;
    }
}

pub fn resolver() {
    let mut persona = Persona::new(
        "Pablo".to_string(),
        30,
        Some("Calle 90 y 120".to_string()),
    );
    println!("{}", persona.to_string());

    let nueva_direccion = None;
    persona.actualizar_direccion(nueva_direccion);

    println!("{}", persona.to_string());

    let edad = persona.obtener_edad();
    println!("La edad es {}", edad);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_nonedir() {
        let per: Persona = Persona {
            nombre: "jorge".to_string(),
            edad: 20,
            direccion: None,
        };
        assert!(per.to_string().contains("No existe direccion"));
    }

    #[test]
    fn test_to_string_somedir() {
        let per: Persona = Persona {
            nombre: "juan".to_string(),
            edad: 70,
            direccion: Some("vivo en la plata".to_string()),
        };
        assert!(per.to_string().contains("vivo en la plata"));
    }

    #[test]
    fn test_new() {
        let nombre: String = String::from("maria");
        let edad: u32 = 40;
        let direccion: Option<String> = Some("algun lugar".to_string());
        let persona = Persona::new(nombre, edad, direccion);
        assert_eq!(persona.nombre, "maria");
        assert_eq!(persona.edad, edad);
        assert_eq!(persona.direccion, Some("algun lugar".to_string()));
    }

    #[test]
    fn test_actualizar_direccion_none() {
        let nombre: String = String::from("julia");
        let edad: u32 = 30;
        let direccion: Option<String> = Some("barcelona".to_string());
        let mut persona = Persona::new(nombre, edad, direccion);
        persona.actualizar_direccion(None);
        assert_eq!(None, persona.direccion);
    }

    #[test]
    fn test_actualizar_direccion_some() {
        let nombre: String = String::from("julia");
        let edad: u32 = 30;
        let direccion: Option<String> = Some("paris".to_string());
        let mut persona = Persona::new(nombre, edad, direccion);
        persona.actualizar_direccion(Some("Villa Elisa".to_string()));
        assert_eq!(Some("Villa Elisa".to_string()), persona.direccion);
    }
}
