use atlas_rs::client::Client;

mod config;

fn main() {
    let c = Client::new("foo").verbose(true).default_probe(14037);

    let v = atlas_rs::version();

    println!("Running {}\n{:#?}", v, c);
}
