use rand::Rng;

fn main() {
    let mut random_numbers = [(0, 0); 10];

    let mut rng = rand::thread_rng();

    for i in 0..10 {
        random_numbers[i].0 = rng.gen_range(1..=10);

        random_numbers[i].1 = rng.gen_range(1..=10);
    }

    println!("{:?}", random_numbers);
}