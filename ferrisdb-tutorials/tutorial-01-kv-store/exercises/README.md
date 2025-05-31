# Tutorial 01 Exercises

Practice what you've learned with these challenges!

## Challenge 1: Add a delete() method

Implement a method to remove key-value pairs from the store.

**Requirements:**

- Method signature: `pub fn delete(&mut self, key: &str) -> Option<String>`
- Should return the old value if the key existed
- Should return `None` if the key didn't exist

**Starter code:** See `challenge_01_delete.rs`

## Challenge 2: Add TTL (Time To Live) support

Implement basic expiration for entries.

**Requirements:**

- Store expiration time with each entry
- Implement `set_with_ttl(key, value, duration)`
- Expired entries should not be returned by `get()`
- Add `cleanup_expired()` method
- Regular `set()` should store entries without expiration

**Hint:** You'll need to change the internal data structure!

## Challenge 3: Make keys case-insensitive

Modify the store to treat keys case-insensitively while preserving the original key.

**Requirements:**

- `get("KEY")` should return the value for `"key"`
- Store should remember the original casing
- `keys()` method should return original casings
- Add `delete()` method that works case-insensitively
- Add `len()` method to count entries

## Challenge 4: Add prefix scanning

Implement a method to find all keys with a given prefix.

**Requirements:**

- Method: `pub fn scan_prefix(&self, prefix: &str) -> Vec<String>`
- Should return all keys that start with the prefix
- Results should be sorted
- Add `keys()` and `len()` methods for completeness
- Empty prefix should return all keys

## Running the Exercises

```bash
# Try to implement the solution
cargo test --test exercises

# Check the solution
cargo test --test solutions
```

## Tips

- Start with the simplest challenge (#1)
- Write tests first!
- Think about edge cases
- Check the solutions only after attempting

Good luck! 🚀
