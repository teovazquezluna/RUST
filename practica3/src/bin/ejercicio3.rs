/* 3- Escribir un programa que defina una estructura Fecha que tenga campos para el día,
el mes y el año. Para dicha estructura implemente los siguientes métodos:

    - new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
    - es_fecha_valida: retorna true si es una fecha válida, false caso contrario.
      (Tenga en cuenta los años bisiestos también).
    - es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
    - sumar_dias(dias): suma la cantidad de días a la fecha, modificándose.
    - restar_dias(dias): resta la cantidad de días a la fecha, modificándose.
    - es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje
      es mayor a la fecha pasada por parámetro.
*/
#[derive(Debug, Clone)]
struct Date {
    dia: u32,
    mes: u32,
    año: u32,
}
impl Date {
    fn es_mayor(&self, fecha: &Date) -> bool {
        if self.año != fecha.año {
            self.año > fecha.año
        } else if self.mes != fecha.mes {
            self.mes > fecha.mes
        } else {
            self.dia > fecha.dia
        }
    }
    fn calculardiasdelmes(&self) -> u32 {
        let day_month_cant = match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.es_biciesto() {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };
        day_month_cant
    }
    fn sumar_dias(&mut self, dias: u32) {
        let mut totaldays = self.dia + dias;
        let mut cant_day_month = self.calculardiasdelmes();
        if totaldays >= cant_day_month {
            while totaldays > cant_day_month {
                totaldays = totaldays - cant_day_month;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.año += 1;
                }
                cant_day_month = self.calculardiasdelmes();
            }
            self.dia = totaldays;
        } else {
            self.dia = totaldays;
        }
    }
    fn restar_dias(&mut self, mut dias: u32) {
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

    fn es_fecha_valida(&self) -> bool {
        if self.año > 2000 && self.año < 2027 && self.mes != 0 && self.mes < 13 {
            let day_month_cant = self.calculardiasdelmes();
            self.dia > 0 && self.dia <= day_month_cant
        } else {
            false
        }
    }

    // 🔹 Esto asegura que la función

    fn es_biciesto(&self) -> bool {
        if ((self.año % 400) == 0) || (((self.año % 4) == 0) && (self.año % 100 != 0)) {
            true
        } else {
            false
        }
    }
    fn new(day: u32, month: u32, year: u32) -> Date {
        Date {
            dia: day,
            mes: month,
            año: year,
        }
    }
}

fn main() {
    let mut fecha = Date::new(5, 3, 2024);

    println!("Fecha inicial: {:?}", fecha);

    fecha.restar_dias(10);
    println!("Restando 10 días: {:?}", fecha);

    fecha.sumar_dias(40);
    println!("Sumando 40 días: {:?}", fecha);

    let otra = Date::new(1, 1, 2025);
    println!("¿Es mayor que 1/1/2025? {}", fecha.es_mayor(&otra));
}
