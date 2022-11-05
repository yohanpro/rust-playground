use std::io;

fn main() {
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input)
    .expect()
    println!("Weight on Mars, {}kg", calculate_weight_on_mars(result));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    const GRAVITY_ON_MARS: f32 = 3.711;
    const GRAVITY_ON_EARTH: f32 = 9.81;
    return weight * GRAVITY_ON_MARS / GRAVITY_ON_EARTH;
}

// 러스트에는 ownership이란 개념이 있음.
// 1. 각 value는 variable에 의해 소유됨.
// 2. owner라는게 스코프 내에서 유효하지 않게 되면, 해당 변수는 자동으로 deallocate됨.
// 3. 소유자는 오직 하나만 존재한다. 
