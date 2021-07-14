/*
EDA
estruturas.rs
José Jasnau Caeiro
2021-06-14

estruturas de dados simples
 */

pub struct MemoriaListasLigadas {
    key: Vec<i32>,
    next: Vec<usize>,
    prev: Vec<usize>,
    mem: Vec<usize>,
    head: usize
}

impl MemoriaListasLigadas {
    pub fn new(n: usize) -> MemoriaListasLigadas {
        let mut memoria = MemoriaListasLigadas {
            key: vec![0; n],
            next: vec![0;n],
            prev: vec![0;n],
            mem: Vec::new(),
            head: 0
        };

        for k in 1..n {
            memoria.mem.push(k);
        }
        memoria
    }

    pub fn allocate_object(&mut self) -> usize {
        let x= match self.mem.pop() {
            Some(o) => o,
            None => 0
        };
        x
    }

    pub fn free_object(&mut self, x: usize) {
        if self.mem.contains(&x) {
            println!("já está na memória disponível");
        } else {
            self.mem.push(x);
        }
    }

    pub fn list_search(&self, k: i32) -> usize {
        let mut x = self.head;
        while x != 0 && self.key[x] != k {
            x = self.next[x];
        }
        x
    }

    pub fn list_insert(&mut self, x: usize, key: i32) {
        self.next[x] = self.head;
        self.prev[self.head] = x;
        self.key[x] = key;
        self.head = x;
        self.prev[x] = 0;
    }


    pub fn imprimir(&self) {
        println!("head => {} |", self.head);
        print!("mem | ");
        for x in &self.mem {
            print!(" {}, ", x);
        }
        println!();
        print!("idx | ");
        for k in 1..self.key.len() {
            print!("{} ", k);
        }
        println!();
        print!("prev | ");
        for x in self.prev.iter() {
            print!("{} ", x);
        }

        println!();
        print!("key  | ");
        for x in self.key.iter() {
            print!("{} ", x);
        }

        println!();
        print!("next  | ");
        for x in self.next.iter() {
            print!("{} ", x);
        }
        println!();
        println!();
    }
}
