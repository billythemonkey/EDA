use std::time::{SystemTime, UNIX_EPOCH};

fn hash_multiplicativa(k:i32, m:usize) -> usize{
    const A:f64 = 0.6180339887;
    let h = (m as f64 * ((k as f64 * A) % 1.0)).floor();

    h as usize
}

fn hash_teste_linear(k:i32, i:usize, m:usize) -> usize{
    let hh = (hash_multiplicativa(k, m) + i) % m;

    hh
}

fn hash_insert( t:&mut Vec<i32>, k:i32, m:usize){
    let mut i:usize = 0;

    // loop{
    //     let h = hash_teste_linear(k, i, m);
    //     if t[h] == 0 && i < m{
    //         t[h] = k;
    //         break;
    //     }else{
    //         i += 1;
    //     }
    // };
    
    let mut j = hash_teste_linear(k, i, m);

    while t[j] != 0{
        i += 1;
        j = hash_teste_linear(k, i, m);
    }
    
    t[j] = k;

}


fn main() {
    const N:usize = 30;
    const M:usize = 15;
    let mut t = vec![0; N];

    hash_insert(&mut t, 4, M);
    hash_insert(&mut t, 16, M);
    hash_insert(&mut t, 19, M);
    hash_insert(&mut t, 7, M);

    println!("{:?}", t);

    for k in 0..10 {
        let h = hash_multiplicativa(k, M);
        for i in 0..3{
            let hh = hash_teste_linear(k, i, M);
            println!("hh({}, {}) -> {}", k, i, hh)
        }
        
        //println!("h ({}) -> {}",k, h);
    }

}
