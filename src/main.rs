use clap::Parser;
use fake::faker::address::en::{BuildingNumber, CityName, PostCode, StateName, StreetName};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Dummy, Serialize, Deserialize)]
pub struct User {
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "FreeEmail()")]
    email: String,
    address: Address,
}

#[derive(Debug, Dummy, Serialize, Deserialize)]
pub struct Address {
    #[dummy(faker = "BuildingNumber()")]
    building: String,
    #[dummy(faker = "StreetName()")]
    street: String,
    #[dummy(faker = "CityName()")]
    city: String,
    #[dummy(faker = "StateName()")]
    state: String,
    #[dummy(faker = "PostCode()")]
    postcode: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    number: usize,
}

fn main() {
    let cli = Cli::parse();
    for _ in 0..cli.number {
        let user: User = Faker.fake();
        println!("{}", serde_json::to_string(&user).unwrap());
    }
}
