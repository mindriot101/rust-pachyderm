extern crate pachyderm;

fn main() {
    let client = pachyderm::Client::new("localhost", 30650);
    let repos = client.list_repo().expect("fetching repos");
    println!("{:?}", repos);
}
