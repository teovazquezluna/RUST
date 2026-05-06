/*Definir una función llamada incrementar que recibe
como parámetro un número flotante e incrementa en 1 su valor. */
fn main() {
    let mut f: f32 = 3.4;
    incrementar(&mut f);
    println!("{}", f);
}
fn incrementar(flotante: &mut f32) {
    *flotante += 1.0;

}
#[test]
fn test_incrementar(){
    let mut flotante = 3.5;
    let expected = flotante +1.0;
    incrementar (&mut flotante);
    assert_eq!(expected,flotante);
}