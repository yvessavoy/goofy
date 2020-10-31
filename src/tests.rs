use crate::Client;

#[test]
fn test_get_profile_by_username() {
    let client = get_client();

    let profile = client
        .get_profile_by_username("thesecretyves")
        .expect("Error fetching profile");
    assert_eq!(profile.username, "thesecretyves");
}

#[test]
fn test_get_profile_by_id() {
    let client = get_client();

    let profile = client
        .get_profile_by_id(21882022536)
        .expect("Error fetching profile");
    assert_eq!(profile.username, "thesecretyves");
}

#[test]
fn test_get_followers() {
    let client = get_client();
    let followers = client
        .get_followers(21882022536)
        .expect("Failed to fetch followers");
    assert!(followers.len() > 0);
}

#[test]
fn test_get_following() {
    let client = get_client();
    let following = client
        .get_following(21882022536)
        .expect("Failed to fetch following");
    assert!(following.len() > 0);
}

fn get_client() -> Client {
    if std::path::Path::new("session.txt").exists() {
        Client::import("session.txt").expect("Error importing client")
    } else {
        let username = std::env::var("GOOFY_USER").expect("GOOFY_USER not set");
        let password = std::env::var("GOOFY_PASS").expect("GOOFY_PASS not set");
        let client = Client::new(&username, &password).expect("Could not create client");
        client
            .export("session.txt")
            .expect("Failed to store session to disk");
        client
    }
}
