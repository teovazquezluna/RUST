/*1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
números primos. Cree un trait para la determinación del número primo e impleméntelo
según corresponda. Utilice la función iter sobre el vector y aplique un closure para
resolverlo. */
trait Prime {
    fn is_prime(&self) -> bool;
}
impl Prime for u32 {
    fn is_prime(&self) -> bool {
        let mut flag: bool = false;
        if *self < 2 as u32 {
            return false;
        } else if *self == 2 {
            return true;
        }
        for n in 2..*self {
            if (*self % n) == 0 {
                flag = true;
                break;
            }
        }
        flag
    }
}
fn count_prime_numbers(vec: &Vec<u32>) -> usize {
    vec.iter().filter(|number| number.is_prime()).count()
}
pub fn ej1() {
    let vector = vec![2, 6, 4, 3, 1, 8];
    let count = count_prime_numbers(&vector);
    for i in 0..vector.len() {
        println!("{}", vector[i]);
    }
    println!("number of primes {}", count);
}
