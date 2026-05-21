/* 3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario. Tenga en
cuenta los años bisiestos también.
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose.
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose.
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
la fecha pasada por parámetro. */
#[derive(Debug, Clone)]
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub año: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, año: u32) -> Fecha {
        Fecha { dia, mes, año }
    }

    pub fn es_mayor(&self, fecha: &Fecha) -> bool {
        if !self.es_fecha_valida() || !fecha.es_fecha_valida() {
            panic!("Ingrese fecha valida");
        }

        if self.año != fecha.año {
            self.año > fecha.año
        } else if self.mes != fecha.mes {
            self.mes > fecha.mes
        } else {
            self.dia > fecha.dia
        }
    }

    pub fn calculardiasdelmes(&self) -> u32 {
        match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("mes invalido"),
        }
    }

    pub fn sumar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            let dia_del_mes = self.calculardiasdelmes();
            let dias_restante = (dia_del_mes - self.dia) + 1;
            if dias < dias_restante {
                self.dia += dias;
                dias = 0;
            } else {
                dias -= dias_restante;
                self.mes += 1;
                self.dia = 1;
                if self.mes > 12 {
                    self.año += 1;
                    self.mes = 1;
                }
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            if dias < self.dia {
                self.dia -= dias;
                dias = 0;
            } else {
                dias -= self.dia;
                if self.mes == 1 {
                    self.mes = 12;
                    self.año -= 1;
                } else {
                    self.mes -= 1;
                }
                self.dia = self.calculardiasdelmes();
            }
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes > 0 && self.mes < 13 {
            let dia_del_mes = self.calculardiasdelmes();
            self.dia > 0 && self.dia <= dia_del_mes
        } else {
            false
        }
    }

    pub fn es_bisiesto(&self) -> bool {
        ((self.año % 400) == 0) || (((self.año % 4) == 0) && (self.año % 100 != 0))
    }
}

pub fn resolver() {
    let mut fecha = Fecha::new(5, 3, 2024);

    println!("Fecha inicial: {:?}", fecha);

    fecha.restar_dias(10);
    println!("Resto 10 dias: {:?}", fecha);

    fecha.sumar_dias(40);
    println!("Sumo 40 dias: {:?}", fecha);

    let otra = Fecha::new(1, 1, 2025);
    println!("es mayor que 1/1/2025? {}", fecha.es_mayor(&otra));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar_dias_num_cambio_mes() {
        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_sumar = 31;
        let esperado = Fecha::new(1, 2, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_sumar = 32;
        let esperado = Fecha::new(2, 2, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 3, 2000);
        let dias_a_sumar = 61;
        let esperado = Fecha::new(1, 5, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);
    }

    #[test]
    fn test_sumar_dias_num_bajo() {
        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_sumar = 20;
        let esperado = Fecha::new(21, 1, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_sumar = 30;
        let esperado = Fecha::new(31, 1, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_sumar = 0;
        let esperado = Fecha::new(1, 1, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(5, 1, 2000);
        let dias_a_sumar = 32;
        let esperado = Fecha::new(6, 2, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);
    }

    #[test]
    fn test_sumar_dias_num_cambio_año() {
        let mut fecha = Fecha::new(1, 12, 2000);
        let dias_a_sumar = 31;
        let esperado = Fecha::new(1, 1, 2001);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_sumar = 200;
        let esperado = Fecha::new(19, 7, 2000);
        fecha.sumar_dias(dias_a_sumar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);
    }

    #[test]
    fn test_restar_dias_cambio_mes() {
        let mut fecha = Fecha::new(1, 2, 2000);
        let dias_a_restar = 29;
        let esperado = Fecha::new(3, 1, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(2, 5, 2000);
        let dias_a_restar = 31;
        let esperado = Fecha::new(1, 4, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 5, 2000);
        let dias_a_restar = 61;
        let esperado = Fecha::new(1, 3, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);
    }

    #[test]
    fn test_restar_dias_num_bajo() {
        let mut fecha = Fecha::new(21, 1, 2000);
        let dias_a_restar = 20;
        let esperado = Fecha::new(1, 1, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(31, 1, 2000);
        let dias_a_restar = 30;
        let esperado = Fecha::new(1, 1, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);

        let mut fecha = Fecha::new(1, 1, 2000);
        let dias_a_restar = 0;
        let esperado = Fecha::new(1, 1, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);
    }

    #[test]
    fn test_restar_dias_cambio_año() {
        let mut fecha = Fecha::new(1, 1, 2001);
        let dias_a_restar = 31;
        let esperado = Fecha::new(1, 12, 2000);
        fecha.restar_dias(dias_a_restar);
        assert_eq!(fecha.dia, esperado.dia);
        assert_eq!(fecha.mes, esperado.mes);
        assert_eq!(fecha.año, esperado.año);
    }

    #[test]
    fn test_es_bisiesto() {
        let bisiesto = Fecha::new(3, 5, 2024);
        let bisiesto2 = Fecha::new(1, 1, 400);
        assert!(bisiesto.es_bisiesto());
        assert!(bisiesto2.es_bisiesto());
    }

    #[test]
    fn test_no_es_bisiesto() {
        let no_bisiesto = Fecha::new(2, 2, 2099);
        let no_bisiesto2 = Fecha::new(2, 2, 2100);
        assert!(!no_bisiesto.es_bisiesto());
        assert!(!no_bisiesto2.es_bisiesto());
    }

    #[test]
    fn test_es_fecha_valida() {
        let dia_incorrecto = Fecha::new(50, 1, 2000);
        let mes_incorrecto = Fecha::new(1, 20, 2000);
        assert!(!dia_incorrecto.es_fecha_valida());
        assert!(!mes_incorrecto.es_fecha_valida());
    }

    #[test]
    fn test_es_mayor_true() {
        let fecha_mayor = Fecha::new(1, 1, 2026);
        let fecha = Fecha::new(1, 1, 2000);

        let fecha_mayor2 = Fecha::new(1, 2, 2026);
        let fecha_mayor3 = Fecha::new(2, 2, 2026);
        assert!(fecha_mayor.es_mayor(&fecha));
        assert!(fecha_mayor2.es_mayor(&fecha_mayor));
        assert!(fecha_mayor3.es_mayor(&fecha_mayor2));
    }

    #[test]
    fn test_es_mayor_false() {
        let fecha1 = Fecha::new(1, 2, 2000);
        let fecha2 = Fecha::new(10, 2, 2000);
        assert!(!fecha1.es_mayor(&fecha2));
    }

    #[test]
    #[should_panic(expected = "mes invalido")]
    fn test_calcular_dia_del_mes_invalido() {
        let fecha = Fecha::new(1, 14, 2026);
        fecha.calculardiasdelmes();
    }
}
