use octocrab::Octocrab;

pub fn create_octocrab_instance(token: &str) -> Result<Octocrab, String>
{
        let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .map_err(|e| format!("Octocrab exception: {}", e))?;

    Ok(octocrab)
}