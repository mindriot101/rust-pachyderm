extern crate pachyderm;

fn main() {
    let client = pachyderm::Client::new("localhost", 30650);
    client.clear_repos().expect("clearing repos");

    client.create_repo("images").expect("creating images repo");
    let repos = client.list_repo().expect("fetching repos");
    assert_eq!(
        repos
            .iter()
            .filter(|r| r.repo.get_ref().name == "images")
            .count(),
        1
    );

    let repo = client
        .inspect_repo("images")
        .expect("inspecting images repo");
    println!("{:?}", repo);
}
