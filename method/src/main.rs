mod models;
fn main() {
    let mut car = models::Car::new(
        String::from("Marcedez Benz"),
        String::from("S500")
    );

    println!("car : {:?}", car);

    let info = car.info();
    print!("info : {}", info);

    car.set_manufacture_year(1992);
    let detail_info = car.info();
    println!("detail_info : {:?}", detail_info);

    car.congratulate(String::from("Dyaksa Jauharuddin Nour"));
}
