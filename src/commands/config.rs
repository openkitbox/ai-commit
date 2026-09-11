use crate::config::AppConfig;
use anyhow::Result;
use colored::*;
use std::io::Write;

pub fn init_config() -> Result<()> {
    println!("{}", "🔧 Initializing AI Commit configuration...".cyan());

    let config_path = AppConfig::config_path()?;

    if config_path.exists() {
        println!("{}", "⚠️  Configuration file already exists.".yellow());
        println!("Location: {}", config_path.display().to_string().bright_blue());

        print!("Do you want to overwrite it? (y/N): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("{}", "❌ Configuration initialization cancelled.".red());
            return Ok(());
        }
    }

    let config = AppConfig::default();
    config.save()?;

    println!("{}", "✅ Configuration file created successfully!".green());
    println!("Location: {}", config_path.display().to_string().bright_blue());
    println!();
    println!("{}", "📝 Next steps:".bright_cyan().bold());
    println!("1. Edit {} and set {}", config_path.display(), "profiles.default.model".yellow());
    println!("2. Set the API key in {}", "profiles.default.api_key".yellow());
    println!("3. Add more [profiles.<name>] sections and change active_profile to switch models");
    println!("4. View configuration: {}", "ai-commit config show".yellow());
    println!("5. Start using: {}", "ai-commit".yellow());

    Ok(())
}

pub fn show_config() -> Result<()> {
    println!("{}", "📋 Current AI Commit Configuration".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════".bright_blue());

    let config_path = AppConfig::config_path()?;

    if !config_path.exists() {
        println!("{}", "❌ Configuration file not found.".red());
        println!("Run {} to create it.", "ai-commit config init".yellow());
        return Ok(());
    }
    let config = AppConfig::load()?;

    println!("File location: {}", config_path.display().to_string().bright_blue());
    println!();

    if !config.profiles.is_empty() {
        println!("[profiles]");
        println!("  active_profile = \"{}\"", config.active_profile.bright_green());
        println!();

        for (name, profile) in &config.profiles {
            println!("[profiles.{name}]");
            println!("  endpoint = \"{}\"", profile.endpoint.bright_green());
            println!("  model = \"{}\"", profile.model.bright_green());
            let api_key_status = if profile.api_key.is_empty() { "<not set>" } else { "<set>" };
            println!("  api_key = \"{}\"", api_key_status.bright_green());
            println!("  max_tokens = {}", profile.max_tokens);
            println!("  temperature = {}", profile.temperature);
            println!();
        }
    } else {
        println!("{}", "No model profiles configured.".yellow());
        println!();
    }

    println!("[commit]");
    println!("  auto_confirm = {}", config.commit.auto_confirm);
    println!("  dry_run_by_default = {}", config.commit.dry_run_by_default);
    println!("  ignore_lock_files = {}", config.commit.ignore_lock_files);
    println!("  context_limit = {}", config.commit.context_limit);
    println!("  custom_ignore_patterns = {:?}", config.commit.custom_ignore_patterns);
    println!();
    println!("[prompts]");
    let sys_len = config.prompts.system_prompt.len().min(50);
    println!("  system_prompt = \"{}...\"", &config.prompts.system_prompt[..sys_len]);
    let user_len = config.prompts.user_prompt_template.len().min(50);
    println!("  user_prompt_template = \"{}...\"", &config.prompts.user_prompt_template[..user_len]);

    Ok(())
}

pub fn edit_prompts_help() -> Result<()> {
    let config_path = AppConfig::config_path()?;

    println!("{}", "✏️  How to Edit AI Prompts".bright_cyan().bold());
    println!("{}", "═══════════════════════════".bright_blue());
    println!();
    println!("Configuration file location:");
    println!("{}", config_path.display().to_string().bright_blue());
    println!();
    println!("{}", "📝 Editable prompt sections:".bright_green().bold());
    println!();
    println!("{}", "[prompts.system_prompt]".yellow());
    println!("  • Defines AI behavior and commit format preferences");
    println!("  • Sets the overall style and rules for commit messages");
    println!();
    println!("{}", "[prompts.user_prompt_template]".yellow());
    println!("  • Template for analyzing git diffs");
    println!("  • Use {{diff}} as placeholder for the git diff content");
    println!("  • Controls how AI analyzes changes");
    println!();
    println!("{}", "[profiles]".yellow());
    println!("  • Set active_profile to choose the model used on the next run");
    println!("  • Add model settings under [profiles.<name>]");
    println!("  • Store the API key in the profile's api_key field");
    println!();

    println!("{}", "💡 Tips:".bright_green().bold());
    println!("  • Test changes with: ai-commit --dry-run");
    println!("  • Keep {{diff}} placeholder in templates");
    println!("  • Configuration reloads automatically on the next run");
    println!("  • API keys are masked by config show");

    if !config_path.exists() {
        println!();
        println!("{}", "⚠️  Configuration file not found.".yellow());
        println!("Run {} to create it first.", "ai-commit config init".cyan());
    }

    Ok(())
}
