
use nipper::Document;
use crate::repository_data::RepositoryData;

pub fn html_maipulation(old_html: String, data: &RepositoryData) -> Result<String, String>
{
    let document = Document::from(&old_html);

    //let new_id = "test";
    let new_option = format!(
        "<option value=\"leetcode{}\">LeetCode {}. </option>\n",
        &data.new_id, &data.new_id
    );

    let mut select = document.select("select");
    if select.length() > 0 {
        select.append_html(new_option);
    }

    let new_section_html = format!(
        r#"
        <section id="leetcode{id}" class="leetcode-sulotions-container">
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
        id = &data.new_id,
        leetcodeurl = &data.leetcode_url,
        leetcodename = &data.leetcode_name,
        leetcodeicon = &data.leetcode_icon,
        languageurl = &data.language_url,
        languagename = &data.language_name,
        languageicon = &data.language_icon,
        solutionurl = &data.solution_url,
        description = &data.description
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
    
    Ok(document.html().to_string())
}