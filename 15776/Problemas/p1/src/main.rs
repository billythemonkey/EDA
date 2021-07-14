use rand::distributions::{Distribution, Uniform};
use statistical;

fn random_numbers() -> Vec<f64>{

    //Ex 1.1.1.b)
    //Function that creates an table woth 1000 entries with random uniform numbers between 0 and 99, including.

    let mut rng = rand::thread_rng();
    let die = Uniform::from(0.0..100.0);

    let mut a1 : Vec<f64> = Vec::new();

    for elem in 0..1000 {
        a1.push(die.sample(&mut rng));
    }

    a1
}


fn index_counter(a: &Vec<f64>) -> Vec<f64>{

    //Ex 1.1.1.c)
    // Function that return number of times an entry repeats in an table

    let mut a2: Vec<f64> = vec![0.0;100];

    for &i in a {
        a2[i as usize] += 1.0;
    }

    a2
}

// fn vec_average(a: &Vec<i32>) -> f64{
    
//     //Ex 1.1.1.d)
//     // Function that return the average of the table

//     let mut sum = 0;
    
//     for i in a {
//         sum += i;
//     }

//     let avg = sum as f64 / a.len() as f64;

//     avg
// }

fn mean(a: &Vec<f64>) -> Option<f64>{

    //Ex 1.1.1.d)
    // Function that return the average of the table

    let sum = a.iter().sum::<f64>() as f64;
    let count = a.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn standard_deviation(a: &Vec<f64>) -> Option<f64>{

    //Ex 1.1.1.e)
    // Function that return the average of the table


    match (mean(a),a.len()){
        (Some(a_mean), count) if count > 0 => {
            let var = a.iter().map(|value| {
                let diff = a_mean - (*value as f64);
                diff * diff
            }).sum::<f64>() / count as f64;

            Some(var.sqrt())
        },
        _ => None
    }

}

// fn std_deviation(a: &Vec<i32>) -> Option<f64> {
//     match (mean(a), a.len()) {
//         (Some(a_mean), count) if count > 0 => {
//             let variance = a.iter().map(|value| {
//                 let diff = a_mean - (*value as f64);

//                 diff * diff
//             });

//             Some(variance.sqrt())
//         },
//         _ => None
//     }
// }



fn main() {

    let random_array = random_numbers();
    //println!("Array size is {} -> {:?}", array.len(), array);
    let index_array = index_counter(&random_array);
    //println!("Inde Array size is {} -> {:?}", index_array.len(), index_array);
    
    // let average = vec_average(&random_array);
    // println!("Average -> {}", average);

    let mean = mean(&random_array);
    println!("Average -> {:?}", mean);

    let standard_deviation = standard_deviation(&random_array);
    println!("Standard Deviation -> {:?}", standard_deviation);

    let variance = statistical::variance(&random_array, Some(0.0));
    println!("Variance -> {:?}", variance);
}
