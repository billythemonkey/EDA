struct ListaLigada {
    head: usize,
    vec_key: Vec<i32>,
    vec_next: Vec<usize>,
    vec_prev: Vec<usize>,
}

impl ListaLigada {
    fn new(n: usize) -> ListaLigada {
        ListaLigada {
            head: 0,
            vec_key: vec![0; n],
            vec_next: vec![0; n],
            vec_prev: vec![0; n],
        }
    }

    fn adicionar(&mut self, p: usize, key: i32) {
        // operações com o elemento a adicionar
        self.vec_key[p] = key;
        self.vec_next[p] = self.head;
        self.vec_prev[p] = 0;

        // operações com o elemento anterior na cabeça de lista
        self.vec_prev[self.head] = p;
        self.head = p;
    }

    fn imprimir(&self) {

        println!("Head -> {}", self.head);
        println!("Key -> {:?}", self.vec_key);
        println!("Next -> {:?}", self.vec_next);
        println!("Prev -> {:?}", self.vec_prev);
    }

    fn reservar(&self, memoria: &mut Vec<usize>) -> usize {
        let pos = memoria.pop();
        let p = match pos {
            Some(pos) => pos,
            None => 0,
        };
        p
    }
}

fn criar_gestor_memoria(n: usize) -> Vec<usize> {
    let mut gestor_memoria: Vec<usize> = Vec::new();
    for j in 1..n {
        gestor_memoria.push(j as usize)
    }
    gestor_memoria
}

fn main() {
    const N: usize = 10;
    let mut memoria = criar_gestor_memoria(N);

    let mut list = ListaLigada::new(N);
    let p = list.reservar(&mut memoria);
    list.adicionar(p, 15);
    list.adicionar(p, 16);
    //list.adicionar(p, 74);
    list.imprimir();
}
