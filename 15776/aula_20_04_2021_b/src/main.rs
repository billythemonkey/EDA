/*
Andrei Oproiu
20/04/2021 - Aula 5
*/

struct ListaLigada{

    head : usize,
    vec_key : Vec<i32>,
    vec_next : Vec<usize>,
    vec_prev : Vec<usize>,
    
}

impl ListaLigada {

    fn new (n: usize) -> ListaLigada {
        ListaLigada{
            head : 0,
            vec_key: vec![0; n],
            vec_next: vec![0; n],
            vec_prev: vec![0; n]
        }
    }

    fn search(&mut self, key: i32) -> usize {

        /*
            A variavel p é o nosso iterador e começa como cabeça da lista
            para não perder tempo a procurar onde ainda não ha valores para
            vec_key.
         */
        let mut p = self.head;

        while self.vec_next[p] != 0 && self.vec_key[p] != key{
            //println!("{} {}", p, self.vec_key[p]);
            p = self.vec_next[p]; 
        }

        p
    }

    fn delete(&mut self, p: usize, memoria_livre:&mut Vec<usize>) {

        // let prev = self.vec_prev[p];
        // let next = self.vec_next[p];

        // if self.vec_prev[p] != 0 {
        //     self.vec_next[prev] = next;
        // }else{
        //     self.head = self.vec_next[p];
        // }
        // if self.vec_next[p] != 0{
        //     self.vec_prev[next] = prev;
        // }

        let p1 = self.vec_prev[p];
        println!("P1 -> {}", p1);
        let p2 = self.vec_next[p];
        println!("P2 -> {}", p2);

        self.vec_next[p1] = p2;
        self.vec_prev[p2] = p1;

        //self.head = self.vec_next[p];

        memoria_livre.push(p);

    }

    fn add(&mut self,p:usize,key:i32) {

        /*
           - Quando a nossa lista está vazia o self.head = 0;
                a) vec_previous[p] = 0;
                b) if head == 0 {
                    vec_next[p] = 0;
                }else {
                    vec_next[p] = self.head;
                } 
        */
        // if self.head == 0 {
        //     self.vec_next[p] = self.head;
        //     self.vec_prev[p] = self.head; 
        // }else {
        //     println!("Head -> {}, p ->{}", self.head, p);
        //     self.vec_prev[self.head] = p;
        //     //self.head = p;
        //     self.vec_next[p] = self.head;
        // }
        // self.vec_key[p] = key;
        // // self.vec_prev[p] = 0;
        // // self.vec_next[p] = self.head;

        // self.head = p;

        self.vec_next[p] = self.head;
        if self.head != 0 {
            self.vec_prev[self.head] = p;
        }
        self.vec_key[p] = key;
        self.head = p;
        self.vec_prev[p] = 0;

    }

    fn print(&self){
        println!("Head -> {}", self.head);
        println!("Key  -> {:?}", self.vec_key);
        println!("Next -> {:?}", self.vec_next);
        println!("Prev -> {:?}", self.vec_prev);
    }
}


fn main() {
    const N: usize= 10;

    let mut memoria_livre = Vec::new();//vec![0;N];
    let mut index_array = Vec::new();
    // memoria_livre.push(10);
    // memoria_livre.push(11);
    // memoria_livre.push(12);

    for m in 0..N {
        memoria_livre.push(m);
    }

    for n in 0..N{
        index_array.push(n);
    }

    println!("{:?}", memoria_livre);
    
    let mut p = memoria_livre.pop();
    let mut position = match p{
        None => 0,
        Some(p) => p
    };
    
    // println!("{:?}", memoria_livre);
    // println!("{}", position);
    
    let mut lista_ligada = ListaLigada::new(N);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);
    lista_ligada.add(position, 100);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);


    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 50);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 30);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 20);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 10);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);

    let search: i32 = 30;

    position = lista_ligada.search(search);
    println!("Search -> {}", position);

    lista_ligada.delete(position, &mut memoria_livre);
    println!("{:?}", memoria_livre);

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 1);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);


    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 69);
    lista_ligada.print();
    println!("Index-> {:?}", index_array);
}
