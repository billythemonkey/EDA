/**
*Tiago Miguel Gameiro da Silva
*16237
*/

use rand::Rng;
use num::Complex;
use statistical;
use time::Instant;

#[derive(Debug)]
enum Algoritmo {
    Bubble,
    Insertion,
    Merge,
    Quick,
    Heap
}

fn soma_assina(nome: &str) { let mut soma = 0;
    for x in nome.chars() {
    let v = match x.to_digit(16) { Some(e) => e,
    None => 0 };
    if x.is_ascii_whitespace() { soma += 1;
    } else if x.is_ascii_lowercase() {
    soma += 2; }
    soma += v; }
    const N: u32 = 5;
    let diva = soma % N;
    let escolha = match diva {
            1 => Algoritmo::Bubble,
            2 => Algoritmo::Heap,
            3 => Algoritmo::Insertion,
            4 => Algoritmo::Merge,
            0 => Algoritmo::Quick,
            _ => Algoritmo::Bubble
    };
        println!("algoritmo a implementar -> {:?} Sort", escolha);
}

// Merge Sort starts

fn merge_i32(a: &mut Vec<num::Complex<f64>>, p: usize, q: usize, r:usize){
    let n1 = q - p + 1; //=2
    let n2 = r - q;        //=5

    let mut vec_l: Vec<num::Complex<f64>> = Vec::new();
    let mut vec_r: Vec<num::Complex<f64>> = Vec::new();

    for i in 1..n1 + 1 {
        let x = a[p + i - 1];
        vec_l.push(x);
    }

    for j in 1..n2 + 1 {
        let x = a[q + j];
        vec_r.push(x);
    }

    vec_l.push(Complex::new(f64::MAX, f64::MAX));
    vec_r.push(Complex::new(f64::MAX, f64::MAX));

    let mut i: usize = 0;
    let mut j: usize = 0;

    for k in p..r + 1 {

        if vec_l[i].norm() <= vec_r[j].norm(){
            a[k] = vec_l[i];
            i = i + 1;
        }else {
            a[k] = vec_r[j];
            j = j + 1;
        }
    }
}

fn merge_sort_part_i32(a: &mut Vec<num::Complex<f64>>, p: usize, r: usize){
    
    if p < r {
        let q = ((p + r - 1) as f64 / 2.0).floor() as usize; 
        merge_sort_part_i32(a, p, q);
        merge_sort_part_i32(a, q+1, r);
        merge_i32(a, p, q, r);
    }
}

fn merge_sort_i32(a: &mut Vec<num::Complex<f64>>){ 
    let p = 1;
    let r = a.len()-1;
    merge_sort_part_i32(a, p, r);
}

fn sort_complex(n: i32) -> time::Duration{

    let mut complex: Vec<num::Complex<f64>> = Vec::new();

    let mut rng = rand::thread_rng();

    for _i in 0..n {
        let re: f64 = rng.gen_range(-10.0..10.0);
        let im: f64 = rng.gen_range(-10.0..10.0);
        let number = Complex::new(re, im);
        complex.push(number);
    }

    println!("Size -> {}", complex.len());
    //println!("Unsorted -> {:?}", complex);
    let start = Instant::now();
    merge_sort_i32(&mut complex);
    let elapsed = start.elapsed();
    //println!("Sorted -> {:?}", complex);

    
    //get_execution_time(elapsed);
    let mut norm: Vec<f64> = Vec::new();

    for elem in 0..complex.len() {
        norm.push(complex[elem].norm())
    }
    //println!("Norm -> {:?}", norm);

    println!("{:?}",elapsed);

    return elapsed
}


fn sort_complex_before_time(n: i32){

    let mut complex: Vec<num::Complex<f64>> = Vec::new();

    let mut rng = rand::thread_rng();

    for _i in 0..n {
        let re: f64 = rng.gen_range(-10.0..10.0);
        let im: f64 = rng.gen_range(-10.0..10.0);
        let number = Complex::new(re, im);
        complex.push(number);
    }

    println!("Size -> {}", complex.len());
    println!("Unsorted -> {:?}", complex);
    
    merge_sort_i32(&mut complex);
    println!("Sorted -> {:?}", complex);
}


// fn calculate_array_size(time: time::Duration, n: i32) -> i32{

//     let expected_duration: time::Duration = time::Duration::seconds(10);
//     let size = ((n * expected_duration) / time).floor() as i32;

//     return size;

// }

// fn erro_relativo(a: Vec<i32>) -> f64{

// }

fn sort_reverse(){

}



fn main() {
    
    let n = 10;
    soma_assina("Andrei Razvan Oproiu");
    //let duration = sort_complex(n);
    // let array_size = calculate_array_size(duration, n);
    // sort_complex(array_size);

    sort_complex_before_time(n);

    
}
