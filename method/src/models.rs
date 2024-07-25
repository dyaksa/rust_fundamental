#[derive(Debug)]
pub struct Car {
    brand: String,
    model: String,
    manualfacture_year: i32,
}

impl Car {
    pub fn new(brand: String, model: String) -> Self {
        Self {brand, model, manualfacture_year: 0}
    }

    pub fn info(&self) -> String {
        if self.manualfacture_year == 0 {
            format!("{} model {}", self.brand, self.model)
        }else {
            format!("{} model {} from {}", self.brand, self.model, self.manualfacture_year)
        }
    }

    pub fn congratulate(&self, name: String) {
        println!("Hello, {}!", name);
        println!("congrats with your new car: {}", self.info());
        println!("vroooooom vroooooommmmmmmm!");
    }

    pub fn set_manufacture_year(&mut self, year: i32) {
        self.manualfacture_year = year;
    }
}