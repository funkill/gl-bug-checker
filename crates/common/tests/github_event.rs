use common::github_event::GithubEvent;

const FILE: &str = "tests/event.json";

#[test]
fn load_event() {
    assert!(GithubEvent::new(FILE).is_ok())
}

#[test]
fn get_pr_number() {
    let event = GithubEvent::new(FILE).unwrap();
    assert_eq!(2, event.pr_number().unwrap());
}

#[test]
fn get_base_sha() {
    let event = GithubEvent::new(FILE).unwrap();
    assert_eq!(
        String::from("f95f852bd8fca8fcc58a9a2d6c842781e32a215e"),
        event.base_sha().unwrap()
    );
}

#[test]
fn get_head_sha() {
    let event = GithubEvent::new(FILE).unwrap();
    assert_eq!(
        String::from("ec26c3e57ca3a959ca5aad62de7213c562f8c821"),
        event.head_sha().unwrap()
    );
}
