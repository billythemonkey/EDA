/*
Andrei Oproiu
20/04/2021 - Aula 5
*/



fn main() {
    

    let mut memoria_livre:Vec<usize> = Vec::new();
    memoria_livre.push(10);
    memoria_livre.push(11);
    memoria_livre.push(12);

    println!("{:?}", memoria_livre);

    let p = memoria_livre.pop();
    let position = match p{
        None => 0,
        Some(p) => p
    };
    

    println!("{:?}", memoria_livre);
    println!("{}", position);
    
}
