use base64::{engine::general_purpose, Engine as _};

mod repository_data;
mod html_maipulation;
mod octocrab_instance;
mod git_mannaging;
use repository_data::RepositoryData;

const OWNER: &str = "6gDav";
const REPO: &str = "LeetCodeSulotions";
const PATH: &str = "index.html";

#[tauri::command]
async fn action_managger(data: RepositoryData) -> Result<String, String> {
    let octocrab = octocrab_instance::create_octocrab_instance(&data.token)?;

    let content_items = octocrab
        .repos(OWNER, REPO)
        .get_content()
        .path(PATH)
        .r#ref("main")
        .send()
        .await
        .map_err(|e| format!("Octocrab exception: {}", e))?;

    let file_content = &content_items.items[0];
    let file_sha = file_content.sha.clone();
    let nyers_content = file_content.content.as_ref().unwrap().replace("\n", "");
    let decoded_bytes = general_purpose::STANDARD
        .decode(nyers_content)
        .map_err(|e| format!("Octocrab exception: {}", e))?;
    let old_html =
        String::from_utf8(decoded_bytes).map_err(|e| format!("Octocrab exception: {}", e))?;

    let new_html_string = html_maipulation::html_maipulation(old_html, &data)?;

    let commit_message = format!("Automatic update ... problem: {} added", &data.new_id);

    git_mannaging::git_repo_upade(&octocrab, OWNER, REPO, PATH, &commit_message, &new_html_string, &file_sha).await?;

    Ok("Done".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![action_managger])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
