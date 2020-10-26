use crate::Client;

#[test]
fn test_fetch_public_profile() {
    let client = if std::path::Path::new("session.txt").exists() {
        Client::import("session.txt").expect("Error importing client")
    } else {
        let username = std::env::var("GOOFY_USER").expect("GOOFY_USER not set");
        let password = std::env::var("GOOFY_PASS").expect("GOOFY_PASS not set");
        Client::new(&username, &password).expect("Could not create client")
    };

    let profile = client
        .get_profile_by_username("thesecretyves")
        .expect("Error fetching profile");
    assert_eq!(profile.username, "thesecretyves");
}
