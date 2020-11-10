use crate::Client;
use mockito::mock;
use mockito::Mock;
use std::path::Path;

#[test]
fn get_profile_by_username() {
    let _mock = create_mock(
        "/users/jakebaecher/usernameinfo/",
        "user_name_success.json",
        200,
    );

    let client = get_client();

    let profile = client
        .get_profile_by_username("jakebaecher")
        .expect("Error fetching profile");
    assert_eq!(profile.username, "jakebaecher");
}

#[test]
fn get_profile_by_id() {
    let _mock = create_mock("/users/44487504243/info/", "user_id_success.json", 200);

    let client = get_client();

    let profile = client
        .get_profile_by_id(44487504243)
        .expect("Error fetching profile");
    assert_eq!(profile.username, "jakebaecher");
}

#[test]
fn get_followers() {
    let _m = create_mock(
        "/friendships/44487504243/followers/?max_id=&ig_sig_key_version=4",
        "get_followers.json",
        200,
    );
    let client = get_client();
    let followers = client
        .get_followers(44487504243)
        .expect("Failed to fetch followers");
    assert!(followers.len() == 11);
}

#[test]
fn get_following() {
    let _m = create_mock(
        "/friendships/44487504243/following/?max_id=0&ig_sig_key_version=4",
        "get_following.json",
        200,
    );
    let client = get_client();
    let following = client
        .get_following(44487504243)
        .expect("Failed to fetch following");
    assert!(following.len() == 78);
}

fn create_mock(url: &str, response_path: &str, status: usize) -> Mock {
    let response = std::fs::read_to_string(format!("test_responses/{}", response_path)).unwrap();
    mock("GET", url)
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(response)
        .create()
}

fn get_client() -> Client {
    if Path::new("session.txt").exists() {
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
