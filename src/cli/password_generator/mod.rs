use rand::Rng;

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789()-!@_#$%^+=ABCDEFGHIJKLMNOPQRST";

pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..15).map(|_| { let index = rng.gen_range(0..=CHARSET.len()); CHARSET[index] as char}).collect();

    password
}