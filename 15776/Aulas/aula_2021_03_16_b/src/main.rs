use rand::prelude::*;
/**
 * Aula de 16 de Março de 2021 Turno B
 * Andrei Razvan Oproiu
 * main.rs
 * utilização de vetores como tabelas com utilização dinâmica da memória
 * - tabelas de dimensão definida em tempo de compilação (arrays
 * - tabelas de dimensão definida em tempo de execução (Vec)
 * - Média, Variância, Desvio Padrão
 *
*/
use rand::Rng;
use std::time::Instant;

fn bubble_sort(a: &mut Vec<i32>){

    let mut i: usize = 0;
    

    while i < a.len() {
        let mut j: usize = a.len() - 1; 
        while j <  a.len(){
            j += 1;
        }
        i += 1;
    }
}

fn media(x: &Vec<f64>) -> f64 {
    // inicializar a variável da média
    let mut m: f64 = 0.0;
    // inicializar a variável de paragem do ciclo while
    let mut i: usize = 0;
    // enquanto o 'i' for menor que o tamanho do vetor 'x'
    // o 'i' é incrementado após a soma do valor do vetor 'x' na variável 'm'

    while i < x.len() {
        m += x[i];
        i += 1;
    }

    // dividir o valor somado de 'm' pelo tamanho do vetor
    m = m / x.len() as f64;
    m
}

fn desvio_p(x: &Vec<f64>) -> f64 {
    let mut s: f64 = 0.0;
    let mut i: usize = 0;
    // Média
    let m: f64 = media(&x);

    while i < x.len() {
        s += (x[i] - m) * (x[i] - m);
        i += 1;
    }
    // Variância
    s = s / x.len() as f64;
    // Desvio Padrão
    s = s.sqrt();
    s
}

fn criar_tabela_aleatoria(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut a_vec: Vec<i32> = vec![0; n];

    let mut i = 0;

    while i < n {
        let a: i32 = rng.gen_range(0..100);
        a_vec[i] = a;
        i += 1;
    }
    a_vec
}

fn main() {
    let now = Instant::now();

    const N: usize = 1000;
    //let a_vec: Vec<i32> = criar_tabela_aleatoria(N);
    let mut rng = thread_rng();
    let mut b_vec: Vec<f64> = vec![0.0; N];
    
    let mut i: usize = 0;

    while i < N {
        let y :f64 = rng.gen();
        b_vec[i] = y;
        //println!("{}", a_vec[i]);
        i += 1;
    }

    let t1 = now.elapsed().as_nanos() as f64;
    println!("T1 -> {}", t1);

    let m = media(&b_vec);
    println!("Média -> {}", m);

    let d = desvio_p(&b_vec);
    println!("Desvio Padrão -> {}", d);

    let e = d / m * 100.0;
    println!("Erro Ralativo -> {}", e);




}
