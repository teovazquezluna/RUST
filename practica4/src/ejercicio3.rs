/*3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada*/

struct Subscribtion {
    month_price:f32,
    subscription_lifetime:u32,
    init_date:Date,
}
struct Date{
    day:u8,
    month:u8,
    year:u16,
}
enum payment_types{
    Cash,
    MercadoPago(Virtual_wallet),
    Credit_card(Card),
    Bank_transfer(Bank),
    Crypto(Crypto),
}
trait payment{
    
}
struct Bank implements payment{
client_CBU:u64,
amount:f32,
titular:String,
orden_code:u64,
amount_payed:f32,
}
struct Virtual_wallet(){
    titular:String;
    
}

pub fn ejer3(){
    
}