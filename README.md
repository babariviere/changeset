# changeset

Library to generate a changeset.

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
changeset = "0.1"
```

And in your main.rs or lib.rs:

```rust
#[macro_use]
extern crate changeset;
```

## Exemple

```rust
changeset!(UserChangeSet {
    /// User's name
    name: String,
    age: usize
});
```

This will generate:

```rust
struct UserChangeSet {
    /// User's name
    pub name: Option<String>,
    pub age: Option<usize>,
}

impl UserChangeSet {
    #[allow(missing_docs)]
    pub fn new() -> UserChangeSet {
        UserChangeSet {
            name: None,
            age: None,
        }
    }

    /// User's name
    pub fn name(mut self, name: String) -> UserChangeSet {
        self.name = Some(name);
        self
    }

    pub fn age(mut self, age: usize) -> UserChangeSet {
        self.age = Some(age);
        self
    }

    #[allow(missing_docs)]
    pub fn merge(&mut self, rhs: UserChangeSet) {
        if let Some(name) = rhs.name {
            self.name = Some(name);
        }
        if let Some(age) = rhs.age {
            self.age = Some(age);
        }
    }
}
```

You can also generate public struct just by adding `pub` keyword.

License: MIT
