mod classes;

use crate::classes::list::*;
use crate::classes::car::*;


fn main() {
    
    let serial_numbers = SerialNumberList::new();
    println!("{}", serial_numbers.buffer.len())
}
