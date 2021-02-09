# Goofy
**Goofy provides rust bindings to the mobile instagram API.**

## Goofy by example
### Get a profile by username
```rust
use goofy::Client;

fn main() {
    let client = Client::new("foo", "bar").expect("Could not create client");
    let profile = client.get_profile_by_username("mozilla");
    assert_eq!(profile.username, "mozilla");
}
```

### Get a profile by user id
```rust
use goofy::Client;

fn main() {
    let client = Client::new("foo", "bar").expect("Could not create client");
    let profile = client.get_profile_by_id(7107542290);
    assert_eq!(profile.username, "mozilla");
}
```

### Get followers for a specific profile
```rust
use goofy::Client;

fn main() {
    let client = Client::new("foo", "bar").expect("Could not create client");
    let followers = client.get_followers(7107542290);
    assert!(followers.len() > 0);
}
```

### Store a client to disk
This is especially useful when you repeatedly start your program. Storing the client to disk avoids repeated login and thus lowers the chance of getting blocked by instagram.
```rust
use goofy::Client;

fn main() {
    let client = Client::new("foo", "bar").expect("Could not create client");
    client.export("goofy-session.txt").expect("Could not persist client to disk");
}
```
