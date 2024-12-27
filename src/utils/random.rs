use rand::{
    Rng, 
    thread_rng, 
    distributions::Alphanumeric
};

pub struct Random;

impl Random {

    pub fn generate_random_identifier(&self) -> String {
        let mut rng = thread_rng();
        let random_string: String = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        format!("_{}", random_string)
    }

}