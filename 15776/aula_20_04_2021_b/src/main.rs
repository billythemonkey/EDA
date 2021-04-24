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

    fn search(&self, key: i32) -> usize {

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
        let p1 = self.vec_prev[p];
        let p2 = self.vec_next[p];

        self.vec_next[p1] = p2;
        self.vec_prev[p2] = p1;

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
        if self.head == 0 {
            self.vec_next[p] = self.head;
            self.vec_prev[p] = self.head; 
        }else {
            self.vec_next[p] = self.head;
            self.vec_prev[self.head] = p;
            // self.head = p;
        }
        self.vec_key[p] = key;
        // self.vec_prev[p] = 0;
        // self.vec_next[p] = self.head;

        self.head = p;
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
    // memoria_livre.push(10);
    // memoria_livre.push(11);
    // memoria_livre.push(12);

    for m in 0..N {
        memoria_livre.push(m);
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
    lista_ligada.add(position, 99);
    lista_ligada.print();


    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 34);
    lista_ligada.print();

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 77);
    lista_ligada.print();

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 11);
    lista_ligada.print();

    p = memoria_livre.pop();
    position = match p{
        None => 0,
        Some(p) => p
    };

    lista_ligada.add(position, 54);
    lista_ligada.print();

    let search: i32 = 34;

    position = lista_ligada.search(search);
    println!("{}", position);

    lista_ligada.delete(position, &mut memoria_livre);
    println!("{:?}", memoria_livre);
    lista_ligada.print();
}
