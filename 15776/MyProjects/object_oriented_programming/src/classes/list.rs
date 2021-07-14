

pub struct SerialNumberList{
    pub available: Vec<usize>,
    pub used: Vec<usize>
}

impl SerialNumberList {
    pub fn new() -> Self{
        let list = Self{
            available: vec![0;usize::MAX],
            used: Vec::new()
        };
        
        for n in 0..usize::MAX {
            list.available.push(n)
        }

        list

    }

    pub fn get_serial_number(&mut self) -> usize{
        let serial = match self.available.pop(){
            Some(n) => {n},
            None => {0}
        };

        self.used.push(serial);
        serial
    }

    

    pub fn return_serial_number(&mut self, serial_number: usize){
        self.used.remove(serial_number);
        self.available.push(serial_number)
    }



}