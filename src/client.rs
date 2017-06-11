use reqwest::Method;
use reqwest::Client;
use hyper::header::*;
use reqwest::RequestBuilder;
use reqwest::IntoUrl;

#[derive(Debug)]
struct Token(String);

#[derive(Debug)]
pub struct GHClient {
    token: Token,
    headers: Headers,
    client: Client
}

impl GHClient {
    pub fn new(token: String) -> GHClient {
        let mut headers = Headers::new();
        headers.set(Authorization("token ".to_owned() + &token));

        GHClient {
            token: Token(token),
            headers: headers,
            client: Client::new().unwrap()
        }
    }

    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.client.request(method, url)
            .headers(self.headers.clone())

    }
}
