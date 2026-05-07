use std::collections::VecDeque;

/*.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:
➔ crear una veterinaria.
➔ agregar una nueva mascota a la cola de atención de la veterinaria.
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ atender la próxima mascota de la cola.
➔ eliminar una mascota específica de la cola de atención dado que se retira.
➔ registrar una atención.

➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono.
➔ modificar el diagnóstico de una determinada atención.

➔ modificar la fecha de la próxima visita de una determinada atención.
➔ eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3. */
struct VetClinic {
    name: String,
    address: String,
    id: u32,
    attention_queue: VecDeque<Pet>,
    visit_record: Vec<Attention>,
}
impl VetClinic {
    fn new(na: String, addr: String, id: u32) -> VetClinic {
        let mut vec = VecDeque::new();
        let mut record = Vec::new();
        VetClinic {
            name: na,
            address: addr,
            id,
            attention_queue: vec,
            visit_record: record,
        }
    }
    fn add_pet(&mut self, pet: Pet) {
        self.attention_queue.push_back(pet);
    }
    fn enqueue_priority_pet(&mut self, pet: Pet) {
        self.attention_queue.push_front(pet);
    }
    fn attend_next(&mut self) -> Option<Pet> {
        self.attention_queue.pop_front()
    }
    fn remove_from_queue(&mut self, pet: Pet) {
        let pos = self
            .attention_queue
            .iter()
            .position(|p| p.nickname == pet.nickname && p.owner == pet.owner);
        if let Some(position) = pos {
            self.attention_queue.remove(position);
        }
    }
    fn record_visit(&mut self, at: Attention) {
        self.visit_record.push(at);
    }
    fn search_attention(&self, pet_name: &str, owner_name: &str, phone: &str) -> Option<usize> {
        self.visit_record.iter().position(|record| {
            record.pet.nickname == pet_name
                && record.pet.owner.name == owner_name
                && record.pet.owner.phone == phone
        })
    }
    fn set_diagnosis(&mut self, new_diagnosis: String, index: usize) {
        let attention = self.visit_record.get_mut(index);
        if let Some(attention) = attention {
            attention.diagnosis = new_diagnosis;
        }
    }
    fn set_date(&mut self, new_date: Date, index: usize) {
        self.visit_record[index].next_date = Some(new_date);
    }
    fn remove_attention(&mut self, index: usize) {
        self.visit_record.remove(index);
    }
}
struct Pet {
    nickname: String,
    age: u32,
    ttype: TypeOfAnimal,
    owner: Owner,
}
impl Pet {}
#[derive(PartialEq)]
struct Owner {
    name: String,
    address: String,
    phone: String,
}
impl Owner {}
enum TypeOfAnimal {
    Horse,
    Cat,
    Dog,
    Other,
}
struct Attention {
    pet: Pet,
    diagnosis: String,
    treatment: String,
    next_date: Option<Date>,
}
struct Date {
    day: String,
    month: String,
    year: String,
}
fn main() {
    let owner = Owner {
        name: "Jane Smith".to_string(),
        address: "21 Jump Street".to_string(),
        phone: "555-1234".to_string(),
    };

    let pet = Pet {
        nickname: "Mittens".to_string(),
        age: 3,
        ttype: TypeOfAnimal::Cat,
        owner,
    };

    let mut clinic = VetClinic::new("Happy Pets".to_string(), "Main Street 123".to_string(), 1);

    println!("Clinic created: {}", clinic.name);

    clinic.add_pet(pet);

    println!("Pet added to queue.");
    println!("Queue length: {}", clinic.attention_queue.len());

    if let Some(pet) = clinic.attend_next() {
        println!("Attending pet: {}", pet.nickname);

        let visit = Attention {
            pet,
            diagnosis: "Skin allergy".to_string(),
            treatment: "Antihistamines".to_string(),
            next_date: Some(Date {
                day: "10".to_string(),
                month: "06".to_string(),
                year: "2026".to_string(),
            }),
        };

        clinic.record_visit(visit);

        println!("Visit recorded.");
        println!("Total records: {}", clinic.visit_record.len());
    }

    if let Some(index) = clinic.search_attention("Mittens", "Jane Smith", "555-1234") {
        println!("Record found at index {}", index);

        clinic.set_diagnosis("Mild dermatitis".to_string(), index);
        println!("Diagnosis updated.");

        clinic.remove_attention(index);
        println!("Record removed.");
    }
}
