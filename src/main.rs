fn main() {
    let hash = argonautica::Hasher::default()
        .with_password("qwerty123")
        .with_secret_key("my_secret_that_must_be_at_least_thirty_two_bytes_long")
        .hash()
        .unwrap();
    println!("{}", hash);
}
