// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use octocrab::Octocrab;
use nipper::Document;
use base64::{Engine as _, engine::general_purpose};
use dotenvy::dotenv;
use std::env;

#[tauri::command]
async fn action_managger(
    new_id: &str,
    leetcode_url: &str,
    leetcode_name: &str,
    leetcode_icon: &str,
    language_url: &str,
    language_name: &str,
    language_icon: &str,
    solution_url: &str,
    description: &str,
) ->  Result<String, String>{
    
    dotenv().ok(); // Fontos: beolvassa a .env fájlt
    let token = env::var("VITE_GITHUB_TOKEN").unwrap_or_else(|_| "There is no token".to_string());

    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build().map_err(|e| format!("Octocrab exception: {}", e))?;

    let owner = "6gDav";
    let repo = "LeetCodeSulotions";
    let path = "index.html";

    // --- 1. ASZINKRON MŰVELET (GitHub lekérés) ---
    let content_items = octocrab
        .repos(owner, repo)
        .get_content()
        .path(path)
        .r#ref("main")
        .send()
        .await.map_err(|e| format!("Octocrab exception: {}", e))?;

    let file_content = &content_items.items[0];
    let file_sha = file_content.sha.clone();
    let nyers_content = file_content.content.as_ref().unwrap().replace("\n", "");
    let decoded_bytes = general_purpose::STANDARD.decode(nyers_content).map_err(|e| format!("Octocrab exception: {}", e))?;
    let regi_html = String::from_utf8(decoded_bytes).map_err(|e| format!("Octocrab exception: {}", e))?;

    // --- 2. SZINKRON MŰVELET (Nipper - egy külön blokkban) ---
    // A blokk végén a 'document' megsemmisül, így nem kell átvinni az await-en.
    let new_html_string = {
        let document = Document::from(&regi_html);
        
        //let new_id = "test";
        let new_option = format!("<option value=\"leetcode{}\">LeetCode {}. </option>\n", new_id, new_id);

        let mut select = document.select("select");
        if select.length() > 0 {
            select.append_html(new_option);
        }

        let new_section_html = format!(
        r#"
            <section id="leetcode{id}" class="leetcode-solutions-container">
                <h2>LeetCode {id}.</h2>
                <h3><a href="{leetcodeurl}" target="_blank" title="LeetCode Link">{leetcodename}</a> {leetcodeicon}</h3>
                <h3>Programming language: <a href="{languageurl}" target="_blank" title="Rust lang Link">{languagename}</a> {languageicon}</h3>
                <h3><a href="{solutionurl}" target="_blank">My solutions</a> ☺️</h3>
                <hr>
                <h3>🧠 How I solved the problem</h3>
                <p>
                    {description}
                </p>
            </section>
            "#,
        id = new_id,
        leetcodeurl = leetcode_url,
        leetcodename = leetcode_name,
        leetcodeicon = leetcode_icon,
        languageurl = language_url,
        languagename = language_name,
        languageicon = language_icon,
        solutionurl = solution_url,
        description = description
        );

        let containers = document.select(".description-section > div");
        if containers.length() > 0 {
            containers.last().append_html(new_section_html.as_str());
        } else {
            let mut main = document.select("main");
            if main.length() > 0 {
                main.append_html(new_section_html.as_str());
            }
        }
        
        // Visszaadjuk a stringet, a 'document' pedig itt felszabadul
        document.html().to_string()
    };

    // --- 3. ASZINKRON MŰVELET (GitHub feltöltés) ---
    // Itt már csak a 'new_html_string'-et használjuk, ami egy sima String (és Send!)
    octocrab
        .repos(owner, repo)
        .update_file(path, "Automatic update with octocrab", new_html_string, file_sha)
        .branch("main")
        .send()
        .await.map_err(|e| format!("Octocrab exception: {}", e))?;

    Ok("Ok".to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() { // <--- Fontos, hogy itt legyen a 'pub'!
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![action_managger])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}