use super::list::*;

struct Car {
    serial_number: usize,
    brand: String,
    model: String,
    year: usize,
    month: String,
}

impl Car {
    fn new(brand: String, model: String, year: usize, month: String, list: SerialNumberList) -> Self {
        let car = Self {
            serial_number: list.get_serial_number(),
            brand: brand,
            model: model,
            year: year,
            month: month,
        };
        car
    }

    pub fn add_car_to_list(&self, car_list: SerialNumberList) -> usize{

    }

    pub fn get_car_from_serial_number(&self, serial_number: usize, list: SerialNumberList) -> Self{

    }


}
