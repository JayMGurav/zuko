use crate::types::Question;
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::collections::HashMap;
use std::fs;
use tera::{Tera, Context as TeraContext};

pub fn bootstrap_solution(
    project_root: PathBuf,
    preferred_language_list: Vec<String>,
    question: &Question,
) -> Result<()> {
    // 1. Extract topic slug
    let topic_slug = question
        .topic
        .as_ref()
        .and_then(|topics| topics.first())
        .map(|t| t.slug.clone())
        .context("No topic found in the question")?;

    let topic_dir = project_root.join(&topic_slug);

    // 2. Create topic directory
    fs::create_dir_all(&topic_dir).context("Failed to create topic directory")?;

    // 3. Initialize Tera templates
    let tera = Tera::new("templates/**/*").context("Failed to initialize Tera templates")?;

    // 4. Create README.md from template
    let mut readme_ctx = TeraContext::new();
    readme_ctx.insert("title", &question.title);
    readme_ctx.insert("content", &question.content);
    // readme_ctx.insert("difficulty", &question.difficulty);
    // readme_ctx.insert("hints", &question.hints);
    // readme_ctx.insert("example_testcase_list", &question.example_testcase_list);
    // readme_ctx.insert("similar_question_list", &question.similar_question_list);
    // readme_ctx.insert("next_challenges", &question.next_challenges);

    let readme_rendered = tera
        .render("readme.md.tera", &readme_ctx)
        .context("Failed to render README template")?;

    let readme_path = topic_dir.join("README.md");
    fs::write(&readme_path, readme_rendered).context("Failed to write README.md")?;

    // 5. Generate code files for each preferred language
    for language in preferred_language_list {
        let lang_dir = topic_dir.join(&language);
        fs::create_dir_all(&lang_dir)
            .context(format!("Failed to create language dir: {}", language))?;

        if let Some(ext) = get_extension_for_language(&language) {
            let template_name = format!("main.{}.tera", ext);
            let main_file_path = lang_dir.join(format!("main.{}", ext));

            // Create context for language-specific code generation
            let mut code_ctx = TeraContext::new();
            code_ctx.insert("question", &question);
            code_ctx.insert("title_slug", &question.title_slug);
            // code_ctx.insert("difficulty", &question.difficulty);
            code_ctx.insert("language", &language);

            let code_rendered = tera
                .render(&template_name, &code_ctx)
                .with_context(|| format!("Failed to render template: {}", template_name))?;

            fs::write(&main_file_path, code_rendered)
                .with_context(|| format!("Failed to write main file for {}", language))?;
        } else {
            eprintln!("No extension found for language: {}", language);
        }
    }

    Ok(())
}

fn get_extension_for_language(language: &str) -> Option<&'static str> {
    let map: HashMap<&str, &str> = vec![
        ("rust", "rs"),
        ("python", "py"),
        ("cpp", "cpp"),
        ("java", "java"),
        ("javascript", "js"),
        ("typescript", "ts"),
        ("go", "go"),
    ]
    .into_iter()
    .collect();

    map.get(language.to_lowercase().as_str()).copied()
}
