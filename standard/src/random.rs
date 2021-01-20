use rand::Rng;

pub fn random_in_range() {
    let secret_number = rand::thread_rng().gen_range(1, 1001);
    println!("Your number was {}", secret_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_rng() {
        random_in_range();
        assert!(true);
    }
}
