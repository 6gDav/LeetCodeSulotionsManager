
use octocrab::Octocrab;

pub async fn git_repo_upade(
    octocrab: &Octocrab, 
    owner: &str, 
    repo: &str, 
    path: &str, 
    commit_message: &str, 
    new_html_string: &str, 
    file_sha: &str) -> Result<(), String> {
        octocrab
        .repos(owner, repo)
        .update_file(
            path,
            commit_message,
            new_html_string,
            file_sha,
        )
        .branch("main")
        .send()
        .await
        .map_err(|e| format!("Octocrab exception: {}", e))?;

    Ok(())
}