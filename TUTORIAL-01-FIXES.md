# Tutorial 01: Immediate Fixes to Apply

## 1. Fix "What You'll Learn" Section

Replace lines 49-60 with:

```mdx
<CardGrid>
  <Card title="🦀 New Rust Concepts" icon="code">
    - **Variables**: How Rust handles data
    - **Mutability**: When data can change
    - **Structs**: Creating custom types
    - **HashMap**: Rust's key-value collection
    - **Option**: Handling missing values
  </Card>

  <Card title="📚 Database Knowledge" icon="database">
    - **Key-Value Model**: Simplest database design
    - **In-Memory Storage**: Trading durability for speed
    - **Hash Tables**: O(1) performance magic
  </Card>
</CardGrid>
```

## 2. Add Step 7 (Before "Comparing with Real FerrisDB")

Insert after line 450:

````mdx
### Step 7: Making It Clippy-Compliant

Rust has a helpful tool called `clippy` that suggests improvements. Let's follow one of its recommendations:

<Tabs>
  <TabItem label="Add Default Implementation">

    ```rust
    use std::collections::HashMap;

    #[derive(Default)]  // Add this line
    pub struct KeyValueStore {
        data: HashMap<String, String>,
    }
    ```

    This automatically implements the `Default` trait, which provides a standard way to create an empty store.

  </TabItem>
  
  <TabItem label="Why This Matters">

    ```rust
    // With Default, users can now do:
    let store = KeyValueStore::default();

    // In addition to:
    let store = KeyValueStore::new();
    ```

    It's a Rust convention: if your type has an obvious "empty" or "default" state, implement Default!

    **Bonus**: This satisfies Rust's linter (clippy) and makes your code more idiomatic.

  </TabItem>
</Tabs>

<Aside type="tip" title="Running Clippy">
  You can check your code with clippy anytime:
  
  ```bash
  cargo clippy
````

It's like having a Rust expert review your code!

</Aside>
```

## 3. Fix Style Inconsistency

Replace lines 540-544 with:

```mdx
- 🦀 **Structs**: Creating custom types
- 🦀 **Mutability**: Understanding `&mut self` vs `&self`
- 🦀 **HashMap**: Using Rust's built-in collections
- 🦀 **Option<T>**: Safe handling of nullable values
- 🦀 **Methods**: Adding behavior with `impl` blocks
```

## 4. Add Practical Usage Example

Insert after Step 6 (before line 458):

````mdx
### Trying It Out

Let's see our key-value store in action outside of tests:

```rust
// Create a new file: src/main.rs
use ferrisdb_tutorial_01::KeyValueStore;

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
```
````

Run it:

```bash
cargo run
```

You should see:

```
Looking up users...
User 1: Alice
User 999 not found
```

````

## 5. Tab Reduction Proposal

For future consideration - replace heavy tab usage with this pattern:

Instead of:
```mdx
<Tabs>
  <TabItem label="Write This Code">
    ```rust
    // code here
    ```
  </TabItem>
  <TabItem label="Understanding">
    // explanation
  </TabItem>
</Tabs>
````

Use:

````mdx
```rust
// code here
```
````

<Aside type="tip" title="Understanding This Code" collapsible>
  // explanation here
</Aside>
```

This creates a smoother reading flow while keeping explanations available.

## Summary

These fixes address:

- ✅ Readability of bullet points
- ✅ Missing explanation for `#[derive(Default)]`
- ✅ Style consistency
- ✅ Practical usage example
- ✅ Better learning flow

The tutorial will be more cohesive and less surprising for learners!
