/*
EDA
ordenacao.rs
José Jasnau Caeiro
2021-06-13

algoritmos de ordenação
 */

pub fn print_a_i32(a: &Vec<i32>) {
    let mut toggle = true;
    let mut c = 1;
    for v in a {
        if toggle {
            toggle = false;
            continue;
        }
        print!("A[{}]= {}, ", c, v);
        c += 1;
    }
}

pub fn print_a_f64(a: &Vec<f64>) {
    let mut toggle = true;
    let mut c = 1;
    for v in a {
        if toggle {
            toggle = false;
            continue;
        }
        println!("A[{}]= {}", c, v);
        c += 1;
    }
}

fn left(i: usize)  -> usize {
    2 * i
}

fn right(i: usize) -> usize {
    2 * i + 1
}

pub fn max_heapify(a: &mut Vec<i32>, i: usize, heap_size: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = usize::MIN;
    if l <= heap_size && a[l] > a[i] {
        largest = l;
    } else {
        largest = i;
    }
    if r <= heap_size && a[r] > a[largest] {
        largest = r;
    }
    if largest != i {
        let tmp = a[i].clone();
        a[i] = a[largest];
        a[largest] = tmp;
        max_heapify(a, largest, heap_size);
    }
}

fn build_max_heap(a: &mut Vec<i32>, heap_size: usize) {
    // max_heapify(a, 1);

    let top = (a.len() as f64 / 2.0).floor() as usize;
    for i in (1..top+1).rev() {
        max_heapify(a, i, heap_size);
    }
}
pub fn heap_sort(a: &mut Vec<i32>) {
    let mut heap_size = a.len()-1;
    build_max_heap(a, heap_size);
    for i in (2..a.len()).rev() {
        let tmp = a[1].clone();
        a[1] = a[i];
        a[i] = tmp;
        heap_size = heap_size - 1;
        max_heapify(a, 1, heap_size);
    }
}

fn partition(a: &mut Vec<i32>, p: usize, r: usize) -> usize {
    let x = a[r];
    let mut i = p - 1;
    for j in p..r {
        if a[j] <= x {
            i = i + 1;
            let tmp = a[i].clone();
            a[i] = a[j];
            a[j] = tmp;
        }
    }
    let tmp = a[i+1].clone();
    a[i+1] = a[r];
    a[r] = tmp;
    i + 1
}

pub fn quick_sort(a: &mut Vec<i32>, p: usize, r: usize) {
    if p < r {
        let q = partition(a, p, r);
        quick_sort(a, p, q-1);
        quick_sort(a, q+1, r);
    }
}

pub fn insertion_sort<T: Clone + std::cmp::PartialOrd>(a: &mut Vec<T>) {
    for j in  2..a.len() {
        // com clone() o valor de a[j] é copiado para key
        let mut key = a[j].clone();
        let mut i = j - 1;
        while i > 0 && a[i] > key {
            a[i+1] = a[i].clone();
            i = i - 1;
        }
        a[i+1] = key;
    }
}

pub fn bubble_sort<T: Clone + std::cmp::PartialOrd>(a: &mut Vec<T>) {
    for i in 1..a.len()-1 {
        for j in (i+1..a.len()).rev() {
            if a[j] < a[j-1] {
                let temp = a[j].clone();
                a[j] = a[j-1].clone();
                a[j-1] = temp;
            }
        }
    }
}

fn merge(a: &mut Vec<f64>,
         p: usize,
         q: usize,
         r: usize) {
    let n1 = q - p + 1;
    let n2 = r - q;

    let mut vec_l: Vec<f64> = Vec::new();
    let mut vec_r: Vec<f64> = Vec::new();

    for i in 1..n1+1 {
        let x = a[p+i-1];
        vec_l.push(x);
    }
    for j in 1..n2+1 {
        let x = a[q+j];
        vec_r.push( x );
    }
    vec_l.push(f64::MAX);
    vec_r.push(f64::MAX);

    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in p..r+1 {
        if vec_l[i] <= vec_r[j] {
            a[k] = vec_l[i];
            i = i + 1;
        } else {
            a[k] = vec_r[j];
            j = j + 1;
        }
    }
}

pub fn merge_sort(a: &mut Vec<f64>, p:usize, r: usize) {
    if p < r {
        let q = ((p + r - 1) as f64 / 2.0).floor() as usize;
        merge_sort(a, p, q);
        merge_sort(a, q+1, r);
        merge(a, p, q, r);
    }
}

