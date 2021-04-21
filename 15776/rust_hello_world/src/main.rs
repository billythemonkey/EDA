struct Dog {
    name: String,
    age: u64,
}

impl Dog {
    fn bark(&self) {
        println!("Baf!");
    }
}

fn build_dog(name: String, age: u64) -> Dog {
    Dog {
        name: name,
        age: age,
    }
}

fn main() {
    let dog = build_dog("Daisy".to_string(), 7);

    dog.bark();
    println!("Nome: {} -> Idade: {}", dog.name, dog.age)
}
