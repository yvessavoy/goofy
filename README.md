# Goofy
**Goofy provides rust bindings to the mobile instagram API.**

## Goofy by example
### Get a profile by username
```rust
use goofy::Client;
fn main() {
    let username = "foo";
    let password = "bar";
    let client = Client::new(&foo, &bar).expect("Could not create client");
    let profile = client.get_profile_by_username("mozilla");
    assert_eq!(profile.username, "mozilla");
}
```

### Store a client to disk
This is especially useful when you repeatetly start your program. Storing the client to disk avoids repeated login and thus lowers the chance of getting blocked by instagram.
```rust
use goofy::Client;
fn main() {
    let username = "foo";
    let password = "bar";
    let client = Client::new(&foo, &bar).expect("Could not create client");
    client.export("goofy-session.txt").expect("Could not persist client to disk");
}
```