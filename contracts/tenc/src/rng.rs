use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub fn get_random(random_seed: &str, from: u32, to: u32) -> u32 {
    let mut rng: Pcg64 = Seeder::from(random_seed).make_rng();
    rng.gen_range(from..to)
}
