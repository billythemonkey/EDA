struct ListaLigada {
    i_next: Vec<usize>,
    i_prev: Vec<usize>,
    key: Vec<i32>,
    head: usize,
}

impl ListaLigada {
    fn new(n: usize) -> Self {
        Self {
            i_next: vec![0; n],
            i_prev: vec![0; n],
            key: vec![0; n],
            head: 0,
        }
    }

    fn add(&mut self, pos: usize, key: i32) {
        self.key[pos] = key;

        let mut first: usize = 0;

        if self.head == 0 {
            first = pos;
        }else {
            self.i_prev[pos] = self.head;
            self.i_next[self.head] = pos;
            self.i_prev[self.head] = pos;
            self.i_next[pos] = self.head;
        }
        
        // self.i_next[pos] = self.head;
        // if self.head != 0 {
        //     self.i_prev[self.head] = pos;
        // }
        self.head = pos;
        // self.i_prev[pos] = 0;
    }

    fn print(&self) {
        println!("Keys -> {:?}", self.key);
        println!("Next -> {:?}", self.i_next);
        println!("Prev -> {:?}", self.i_prev);
        println!("Head -> {:?}", self.head);
    }
}

fn main() {
    const N: usize = 10;

    let mut memoria_livre: Vec<usize> = Vec::new(); //vec![0; N];

    for n in 0..N {
        memoria_livre.push(n);
    }

    //println!("{:?}", memoria_livre);

    let mut p = memoria_livre.pop();
    let mut position = match p {
        Some(p) => p,
        None => 0,
    };
    

    let mut lista_ligada: ListaLigada = ListaLigada::new(N);
    lista_ligada.print();

    // println!("{}", position);
    // lista_ligada.add(position, 23);
    // lista_ligada.print();
    
    p = memoria_livre.pop();
    position = match p {
        Some(p) => p,
        None => 0,
    };

    // println!("{}", position);
    // lista_ligada.add(position, 25);
    // lista_ligada.print();

    lista_ligada.add(2, 30);
    lista_ligada.print();

    lista_ligada.add(5, 170);
    lista_ligada.print();

    lista_ligada.add(1, 90);
    lista_ligada.print();
}
