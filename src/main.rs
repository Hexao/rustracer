pub mod vector;

fn main() {
    let vec1 = vector::Vector::default();
    let vec2 = vector::Vector::new(1.0, 1.0, 1.0);

    let vec3 = &vec1 + &vec2;

    println!("Hello, world!");
    println!("{:?}", vec3);
}
