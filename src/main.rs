extern crate reqwest;
extern crate hyper;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate notify_rust;

mod client;
mod types;

use client::*;
use types::*;
use std::fs::File;
use reqwest::Method;
use std::io::Read;
use std::collections::HashMap;

use notify_rust::Notification as OSD;

fn main() {
    let client = GHClient::new(get_token());
    let resp = client.request(Method::Get, "https://api.github.com/notifications?all=true")
        .send();

    let mut data = String::new();
    resp.unwrap().read_to_string(&mut data).unwrap();

    let notes: Vec<Notification> = serde_json::from_str(&data).unwrap();
    let map = group_by_repos(notes.into_iter());

    let mut msg = String::with_capacity(100);

    for repo in map.keys() {
        msg += &format!("Repo: {}\n", repo.full_name);
        for note in map.get(repo).unwrap() {
            msg += &format!("\t{} {}\n", note.body.title, note.body.url);
        }
    }

    OSD::new()
    .summary("Github Notification")
    .body(&msg)
    .show().unwrap();

    
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
