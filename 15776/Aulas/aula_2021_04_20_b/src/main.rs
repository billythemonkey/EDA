/**
 * Andrei Razvan Oproiu
 * main.rs
 * 12/07/2021
 *
 * Lista Ligada
 */

struct ListaLigada {
    vec_next: Vec<usize>,
    vec_prev: Vec<usize>,
    key: Vec<i32>,
    head: usize,
}

impl ListaLigada {
    fn new(size: usize) -> Self {
        Self {
            vec_next: vec![0; size],
            vec_prev: vec![0; size],
            key: vec![0; size],
            head: 0,
        }
    }

    fn insert(&mut self, pos: usize, key: i32) {
        self.key[pos] = key;
        self.vec_next[pos] = self.head;
        if self.head != 0 {
            self.vec_prev[self.head] = pos;
        }

        self.head = pos;
        self.vec_prev[pos] = 0;
    }

    fn search(&self, value: i32) -> usize {
        let mut x = self.head;
        while x != 0 && self.key[x] != value {
            x = self.vec_next[x];
        }
        x
    }

    fn delete(&mut self, value: i32){
        let position = self.search(value);

        if self.vec_prev[position] != 0 {
            self.vec_next[self.vec_prev[position]] = self.vec_next[position];
        }else{
            self.head = self.vec_next[position];
        }
        if self.vec_next[position] != 0 {
            self.vec_prev[self.vec_next[position]] = self.vec_prev[position];
        }
    }

    fn print(&self) {
        println!("Head -> {}", self.head);
        println!("Key  -> {:?}", self.key);
        println!("Next -> {:?}", self.vec_next);
        println!("Prev -> {:?}", self.vec_prev);
    }
}

fn main() {
    const N: usize = 10;
    let mut memoria_livre = vec![0; N];

    for i in 0..N {
        memoria_livre[i] = i;
    }

    let mut position = match memoria_livre.pop() {
        Some(pos) => pos,
        None => 0,
    };

    let mut lista: ListaLigada = ListaLigada::new(N);
    lista.print();
    lista.insert(position, 25);
    lista.print();
    position = match memoria_livre.pop() {
        Some(pos) => pos,
        None => 0,
    };

    lista.insert(position, 37);
    lista.print();

    position = match memoria_livre.pop() {
        Some(pos) => pos,
        None => 0,
    };

    lista.insert(position, 52);
    lista.print();

    position = match memoria_livre.pop() {
        Some(pos) => pos,
        None => 0,
    };

    lista.insert(position, 87);
    lista.print();

    let pointer = lista.search(52);
    println!("Pointer -> {}", pointer);

    lista.delete(52);
    lista.print();
}
