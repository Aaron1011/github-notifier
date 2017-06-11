#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub id: String,
    #[serde(rename = "subject")]
    pub body: Body,
    pub repository: Repository
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    pub title: String,
    pub url: String,

    #[serde(rename = "latest_comment_url")]
    pub latest_comment: Option<String>,

    #[serde(rename = "type")]
    pub event_type: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub url: String
}

