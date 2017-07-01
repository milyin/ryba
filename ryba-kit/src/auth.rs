use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct SessionData<'a> {
    user_name: &'a str,
    extra_data: &'a str,
    password: &'a str,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn hash(user_name: &str, extra_data: &str, password: &str) -> u64 {
    calculate_hash(&SessionData {
                        user_name: user_name,
                        extra_data: extra_data,
                        password: password,
                    })
}
