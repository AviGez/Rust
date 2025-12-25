mod numbers;
mod car;

use crate::numbers::Numbers;
use crate::car::Car;

fn main() {
    let numbers1 = Numbers::new(vec![10, 20, 30, 40, 50, 60]);
    let sum = numbers1.sum();
    println!("סכום כל האיברים: {}\n", sum);

    let numbers2 = Numbers::new(vec![45, 12, 89, 23, 56, 78, 34, 91]);
    println!("{:?}", numbers2.as_slice());
    let max = numbers2.max();
    println!("{}", max);

    let mut car = Car::new("red", 80);
    car.accelerate(20);
    println!("Car color: {}, speed: {}", car.color(), car.speed());
}
