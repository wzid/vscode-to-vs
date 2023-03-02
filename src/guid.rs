use random_string::generate;

static CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";

pub fn generate_guid() -> String {
    format!("{}-{}-{}-{}", generate_random_string(8), generate_random_string(4), generate_random_string(4), generate_random_string(12))
}

fn generate_random_string(length: usize) -> String {
    generate(length, CHARSET)
}
