/**
 * Noções muito básicas do funcionamento da linguagem Rust
 * Andrei Razvan Oproiu
*/


fn esquesita(x: f64) -> (f64, f64, String){

    // (f64, i32,...) = tuplo

    let z: (f64, f64, String) = (x, x, String::from("teste"));
    z
}

fn dobro(x: f64) -> f64{
    let c = 2.0;
    c * x
}


fn main() {

    let a = 88.1;
    let mut b = 76.3;

    let z = esquesita(a);

    b = 232.12;
    println!("Hello, world!");
    println!("a = {} b = {}", dobro(a), b);
    println!("elementos do tuplo: {} {} {}", z.0, z.1, z.2 )
}
