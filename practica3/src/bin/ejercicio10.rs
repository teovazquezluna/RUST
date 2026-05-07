use std::collections::HashMap;

struct Library {
    name: String,
    address: String,
    available_copies: Vec<Book>,
    loans: Vec<Loan>,
    inventory: HashMap<String, u32>,
}
impl Library {
    fn new(name: String, address: String) -> Library {
        Library {
            name,
            address,
            available_copies: Vec::new(),
            loans: Vec::new(),
            inventory: HashMap::new(),
        }
    }
    fn count_copies(&self, book: &Book) -> usize {
        self.available_copies
            .iter()
            .filter(|copies| copies.title == book.title && copies.pages == book.pages)
            .count()
    }
    fn get_inventory(&self, book: &Book) -> Option<&u32> {
        self.inventory.get(&book.title)
    }
    fn decrement_inventory(&mut self, book: &Book) {
        if let Some(copies) = self.inventory.get_mut(&book.title) {
            if *copies > 0 {
                *copies -= 1;
            }
        }
    }
    fn increment_inventory(&mut self, book: &Book) {
        if let Some(copy) = self.inventory.get_mut(&book.title) {
            *copy += 1;
        }
    }
    fn count_client_loans(&self, client: &Client) -> usize {
        self.loans
            .iter()
            .filter(|loan| {
                (match loan.status {
                    Status::Borrowing => true,
                    Status::Returned => false,
                }) && loan.client.name == client.name
                    && loan.client.email_address == client.email_address
                    && loan.client.phone_number == client.phone_number
            })
            .count()
    }
    fn create_loan(
        &mut self,
        book: &Book,
        client: &Client,
        due: Date,
        return_: Option<Date>,
    ) -> bool {
        if let Some(copies) = self.get_inventory(book) {
            if *copies > 0 && self.count_client_loans(client) < 5 {
                let loan = Loan::new(book.clone(), client.clone(), due, None);
                self.loans.push(loan);
                self.decrement_inventory(book);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn upcoming_due_date(&self, today: Date, days: usize) -> Vec<&Loan> {
        let mut total = today.day as usize + days;
        let mut due_date2: Date = today.clone();
        let amount_month_days: usize = Date::calculardiasdelmes(&today) as usize;
        if total > amount_month_days {
            while total <= amount_month_days {
                total -= amount_month_days;
                if due_date2.month == 12 {
                    due_date2.set_year(due_date2.year + 1);
                }
                due_date2.set_month(due_date2.month + 1);
            }
        }
        due_date2.set_day(total as u8);

        self.loans
            .iter()
            .filter(|loan| loan.due_date.is_less_date(&due_date2))
            .collect()
    }
    fn due_loans(&self, today: Date) -> Vec<&Loan> {
        self.loans
            .iter()
            .filter(|loan| {
                (match loan.status {
                    Status::Borrowing => true,
                    _ => false,
                }) && loan.due_date.is_less_date(&today)
            })
            .collect()
    }
    fn find_loan(&mut self, book: &Book, client: &Client) -> Option<&mut Loan> {
        self.loans
            .iter_mut()
            .find(|loan| loan.client == *client && loan.book == *book)
    }
    /* devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
    estado “devuelto”, se registra la fecha de devolución y se incrementa la
    cantidad de libros en 1 del libro devuelto en el registro de copias a
    disposición. */
    fn return_book(&mut self, book: &Book, client: &Client, today: Date) {
        if let Some(loan) = self.find_loan(book, client) {
            loan.status = Status::Returned;
            loan.return_date = Some(today);
            self.increment_inventory(book);
        }
    }
}
#[derive(Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    genre: Genre,
}
#[derive(Clone, PartialEq)]
enum Genre {
    Novel,
    Children,
    Technical,
    Other,
}
struct Loan {
    book: Book,
    client: Client,
    due_date: Date,
    return_date: Option<Date>,
    status: Status,
}
impl Loan {
    fn new(book: Book, client: Client, due_date: Date, return_date: Option<Date>) -> Loan {
        Loan {
            book,
            client,
            due_date,
            return_date,
            status: Status::Borrowing,
        }
    }
}
enum Status {
    Returned,
    Borrowing,
}
#[derive(Clone)]
struct Date {
    year: u16,

    month: u8,
    day: u8,
}

impl Date {
    fn is_less_date(&self, date: &Date) -> bool {
        (self.year, self.month, self.day) < (date.year, date.month, date.day)
    }
    fn es_biciesto(&self) -> bool {
        if ((self.year % 400) == 0) || (((self.year % 4) == 0) && (self.year % 100 != 0)) {
            true
        } else {
            false
        }
    }
    fn calculardiasdelmes(&self) -> u32 {
        let day_month_cant = match self.month {
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
    fn set_month(&mut self, month: u8) {
        self.month = month;
    }
    fn set_day(&mut self, day: u8) {
        self.day = day;
    }
    fn set_year(&mut self, year: u16) {
        self.year = year;
    }
}
#[derive(Clone, PartialEq)]
struct Client {
    name: String,
    phone_number: String,
    email_address: String,
}

/*


➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
existe.
➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
estado “devuelto”, se registra la fecha de devolución y se incrementa la
cantidad de libros en 1 del libro devuelto en el registro de copias a
disposición.

Nota: para la fecha utilice lo implementado en el punto 3.➔
Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el título, autor, número de páginas, género(novela, infantil, técnico,
otros).Para registrar un préstamo se requiere el libro, el cliente, la fecha de vencimiento del
préstamo, la fecha de devolución y el estado que puede ser devuelto o en préstamo.
Del
cliente se conoce el nombre, teléfono y dirección de correo electrónico.

Implemente los métodos necesarios para realizar las siguientes acciones: btener cantidad de copias: dado un determinado libro retorna el retorna la
cantidad de copias a disposición que hay para prestar de dicho libro.
➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
la cantidad de copias de libros a disposición para prestar.
➔ incrementar cantidad de copias a disposición: dado un libro incremente en 1
la cantidad de copias del libro a disposición para ser prestado.
➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
“en préstamo” de un determinado cliente.
➔ ver la cantidad disponible de un determinado libro: retorna la cantidad de
libros disponibles del registro de “copias a disposición” de un determinado
libro.➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
para un determinado cliente cumpliendo con lo siguiente
◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
◆ haya al menos una copia disponible en el registro de copias a
disposición.
De ser así descuenta 1 en el registro de “copias a disposición” y
retorna true, si no cumple con alguna de las condiciones retorna false.
➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
vencer el los próximos días, el valor de días es pasado por parámetro.

➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
préstamos” donde la fecha de vencimiento es menor a la fecha actual.
 */

fn main() {
    // Crear biblioteca
    let mut library = Library::new(
        String::from("Central Library"),
        String::from("123 Main Street"),
    );

    // Crear libros
    let book1 = Book {
        title: String::from("Rust Programming"),
        author: String::from("Steve"),
        pages: 500,
        genre: Genre::Technical,
    };

    let book2 = Book {
        title: String::from("Harry Potter"),
        author: String::from("Rowling"),
        pages: 350,
        genre: Genre::Novel,
    };

    // Crear cliente
    let client1 = Client {
        name: String::from("Alice"),
        phone_number: String::from("123456"),
        email_address: String::from("alice@mail.com"),
    };

    // Agregar inventario
    library.inventory.insert(book1.title.clone(), 3);
    library.inventory.insert(book2.title.clone(), 2);

    println!("Inventario inicial:");

    if let Some(copies) = library.get_inventory(&book1) {
        println!("{} -> {} copies", book1.title, copies);
    }

    if let Some(copies) = library.get_inventory(&book2) {
        println!("{} -> {} copies", book2.title, copies);
    }

    // Crear fecha
    let today = Date {
        year: 2026,
        month: 3,
        day: 5,
    };

    let due = Date {
        year: 2026,
        month: 3,
        day: 10,
    };

    // Crear préstamo
    let created = library.create_loan(&book1, &client1, due.clone(), None);

    println!("Loan created? {}", created);

    // Ver cantidad de préstamos del cliente
    let count = library.count_client_loans(&client1);

    println!("Client loans: {}", count);

    // Ver préstamos a vencer en próximos días
    let upcoming = library.upcoming_due_date(today.clone(), 7);

    println!("Loans due in next 7 days: {}", upcoming.len());

    for loan in upcoming {
        println!(
            "Book due soon: {} for client {}",
            loan.book.title, loan.client.name
        );
    }

    // Ver préstamos vencidos
    let overdue = library.due_loans(today.clone());

    println!("Overdue loans: {}", overdue.len());

    // Devolver libro
    library.return_book(&book1, &client1, today.clone());

    println!("Book returned.");

    // Ver inventario después de devolver
    if let Some(copies) = library.get_inventory(&book1) {
        println!("{} inventory after return: {}", book1.title, copies);
    }
}
