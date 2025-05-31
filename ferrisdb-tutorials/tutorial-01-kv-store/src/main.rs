use tutorial_01_kv_store::KeyValueStore;

fn main() {
    let mut store = KeyValueStore::new();

    // Store some user data
    store.set("user:1".to_string(), "Alice".to_string());
    store.set("user:2".to_string(), "Bob".to_string());

    // Retrieve and display
    println!("Looking up users...");

    match store.get("user:1") {
        Some(name) => println!("User 1: {}", name),
        None => println!("User 1 not found"),
    }

    // Try a missing key
    match store.get("user:999") {
        Some(name) => println!("User 999: {}", name),
        None => println!("User 999 not found"),
    }
}
