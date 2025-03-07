fn main() {
    let mut rng = rand::thread_rng();
    let num1: u32 = rng.gen_range(0..=100);
    let num2: u32 = rng.gen_range(0..=100);
    println!("{} + {}", num1, num2);
}
