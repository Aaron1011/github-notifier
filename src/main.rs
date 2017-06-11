extern crate reqwest;
extern crate hyper;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
//extern crate notify_rust;

mod client;
mod types;

use client::*;
use types::*;
use std::fs::File;
use reqwest::Method;
use std::io::Read;
use std::collections::HashMap;

//use notify_rust::Notification;

fn main() {

    let client = GHClient::new(get_token());
    let resp = client.request(Method::Get, "https://api.github.com/notifications?all=true")
        .send();

    let mut data = String::new();
    resp.unwrap().read_to_string(&mut data).unwrap();

    let notes: Vec<Notification> = serde_json::from_str(&data).unwrap();
    let map = group_by_repos(notes.into_iter());

    for repo in map.keys() {
        println!("Repo: {}", repo.full_name);
        for note in map.get(repo).unwrap() {
            println!("\t{}", note.body.title);
        }
    }

}

type Map = HashMap<Repository, Vec<Notification>>;

fn group_by_repos<I: Iterator<Item = Notification>>(notes: I) -> Map {
    let mut map = Map::new();
    for note in notes {
        map.entry(note.repository.clone()).or_insert_with(|| Vec::new()).push(note);
    }
    map
}

fn get_token() -> String {
    let mut f = File::open("/home/aaron/.github-token").unwrap();
    let mut token = String::new();
    f.read_to_string(&mut token).unwrap();
    token
}
