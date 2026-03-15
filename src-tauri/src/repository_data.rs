use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RepositoryData {
    pub token: String,
    pub new_id: String,
    pub leetcode_url: String,
    pub leetcode_name: String,
    pub leetcode_icon: String,
    pub language_url: String,
    pub language_name: String,
    pub language_icon: String,
    pub solution_url: String,
    pub description: String,
}