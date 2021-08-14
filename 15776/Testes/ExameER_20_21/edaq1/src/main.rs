

use rand::Rng;

#[derive(Debug)]
enum Algoritmo {
    Bubble,
    Insertion,
    Merge,
    Quick,
    Heap,
}

fn asuaassina(nome: &str) {
    let mut soma = 0;
    for x in nome.chars() {
        let v = match x.to_digit(16) {
            Some(e) => e,
            None => 0,
        };
        if x.is_ascii_whitespace() {
            soma += 1;
        } else if x.is_ascii_lowercase() {
            soma += 2;
        } else if x.is_ascii_uppercase() {
            soma += 3;
        }
        soma += v;
    }
    const N: u32 = 5;
    let diva = soma % N;
    let escolha = match diva {
        1 => Algoritmo::Bubble,
        2 => Algoritmo::Heap,
        3 => Algoritmo::Insertion,
        4 => Algoritmo::Merge,
        0 => Algoritmo::Quick,
        _ => Algoritmo::Bubble,
    };
    println!("algoritmo a implementar -> {:?} Sort", escolha);
}

#[derive(Debug,PartialEq, Clone)]
struct Ponto3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Ponto3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    fn norma(&self) -> f64 {
        let norm = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        norm
    }
}

fn sort_ponto(a: &mut Vec<Ponto3D>) {
    for j in 1..a.len() {
        let key = a[j].clone();
        let keynorm = key.norma();
        let mut i = j - 1;
        while i > 0 && a[i].norma() > keynorm {
            a[i + 1] = a[i].clone();
            i = i - 1;
        }
        a[i + 1] = key;
    }
}



fn main() {
    const N: usize = 10;
    let mut rng = rand::thread_rng();

    let name = "Andrei Razvan Oproiu";
    asuaassina(name);

    
    let mut pontos: Vec<Ponto3D> = Vec::new();

    for n in 0..N {
        let ponto = Ponto3D::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        );
        pontos.push(ponto)
    };


    println!("Unsorted");
    for n in 0..pontos.len() {
        println!("{:?}", pontos[n].norma());
    };

    sort_ponto(&mut pontos);

    println!("Sorted");
    for n in 0..pontos.len() {
        println!("{:?}", pontos[n].norma());
    };
    //println!("Sorted -> {:?}", pontos);
}
