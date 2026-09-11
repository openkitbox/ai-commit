use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub commit: CommitConfig,
    #[serde(default)]
    pub hooks: HookConfig,
    #[serde(default)]
    pub prompts: PromptConfig,
    #[serde(default = "default_active_profile")]
    pub(crate) active_profile: String,
    #[serde(default)]
    pub(crate) profiles: BTreeMap<String, ModelProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ModelProfile {
    #[serde(default = "default_endpoint")]
    pub(crate) endpoint: String,
    #[serde(default)]
    pub(crate) model: String,
    #[serde(default)]
    pub(crate) api_key: String,
    #[serde(default = "default_max_tokens")]
    pub(crate) max_tokens: usize,
    #[serde(default = "default_temperature")]
    pub(crate) temperature: f32,
}

fn default_active_profile() -> String {
    "default".into()
}

fn default_endpoint() -> String {
    "https://ark.cn-beijing.volces.com/api/v3/chat/completions".into()
}

fn default_max_tokens() -> usize {
    4096
}

fn default_temperature() -> f32 {
    0.7
}

pub struct ApiConfig {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

impl ModelProfile {
    fn default_profile() -> Self {
        Self {
            endpoint: default_endpoint(),
            model: String::new(),
            api_key: String::new(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitConfig {
    #[serde(default = "default_false")]
    pub auto_confirm: bool,
    #[serde(default = "default_false")]
    pub dry_run_by_default: bool,
    pub gpg_sign: Option<bool>,
    #[serde(default = "default_true")]
    pub ignore_lock_files: bool,
    #[serde(default)]
    pub custom_ignore_patterns: Vec<String>,
    #[serde(default = "default_context_limit")]
    pub context_limit: usize,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_false() -> bool {
    false
}
fn default_true() -> bool {
    true
}
fn default_context_limit() -> usize {
    800000
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            auto_confirm: false,
            dry_run_by_default: false,
            gpg_sign: None,
            ignore_lock_files: true,
            custom_ignore_patterns: Vec::new(),
            context_limit: 800000,
            language: default_language(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HookConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
    #[serde(default)]
    pub hook_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptConfig {
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_system_prompt_zh")]
    pub system_prompt_zh: String,
    #[serde(default = "default_user_prompt_template")]
    pub user_prompt_template: String,
    #[serde(default = "default_user_prompt_template_zh")]
    pub user_prompt_template_zh: String,
}

fn default_system_prompt() -> String {
    r#"You are an expert software developer and git commit message writer.

Generate concise, clear commit messages following the Conventional Commits specification:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change that improves performance
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools

Format: type(scope): description

IMPORTANT: ALWAYS write the commit message in English, regardless of the language used in the diff content.

PREFERRED FORMAT: Single line under 72 characters
Use bullet points ONLY when there are truly MULTIPLE UNRELATED functional changes.

Default to single line. Only use bullets for truly unrelated changes."#
        .to_string()
}

fn default_system_prompt_zh() -> String {
    r#"你是一位专业的软件开发工程师和 Git 提交信息编写专家。

生成简洁、清晰的提交信息，遵循约定式提交规范。
重要：提交信息必须使用中文编写，类型（type）部分可以使用英文。

格式规范：
- feat: 新功能
- fix: 修复 bug
- docs: 仅文档变更
- style: 不影响代码含义的格式变更
- refactor: 代码重构，既不修复 bug 也不添加新功能
- perf: 性能优化
- test: 添加或修正测试
- chore: 构建过程或辅助工具的变更

格式：类型(范围): 描述
示例：
- feat: 添加用户认证功能
- fix: 修复登录超时问题
- refactor: 优化错误处理逻辑

首选格式：单行，不超过 72 个字符
仅当存在多个完全不相关的功能变更时才使用多行（列表）格式。

默认使用单行格式。只有在真正不相关的变更时才使用列表。

严格要求：输出必须是中文提交信息（类型部分除外）。"#
        .to_string()
}

fn default_user_prompt_template() -> String {
    r#"Analyze the following git diff and generate a commit message.

Git diff:
```diff
{diff}
```

REQUIREMENTS (follow strictly):
1. Write the commit message in English ONLY. Never use Chinese or any other language, even if the diff contains non-English text.
2. Default to a single descriptive line under 72 characters.
3. Only use bullet points if there are multiple COMPLETELY UNRELATED functional changes.

Output ONLY the commit message, nothing else."#
        .to_string()
}

fn default_user_prompt_template_zh() -> String {
    r#"分析以下 git diff 并生成中文提交信息。

重要要求：
1. 提交信息描述部分必须使用中文
2. 类型（type）可以使用英文，如 feat、fix 等
3. 默认使用单行描述，不超过 72 个字符
4. 仅当存在多个完全不相关的功能变更时才使用列表格式

示例输出格式：
- feat: 添加多语言支持
- fix: 修复登录超时问题
- refactor: 重构数据处理逻辑

Git diff:
```diff
{diff}
```

请只输出中文提交信息，不要包含其他内容。"#
        .to_string()
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            system_prompt: default_system_prompt(),
            system_prompt_zh: default_system_prompt_zh(),
            user_prompt_template: default_user_prompt_template(),
            user_prompt_template_zh: default_user_prompt_template_zh(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut profiles = BTreeMap::new();
        profiles.insert("default".into(), ModelProfile::default_profile());

        Self {
            commit: CommitConfig::default(),
            hooks: HookConfig::default(),
            prompts: PromptConfig::default(),
            active_profile: default_active_profile(),
            profiles,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        debug!("Loading configuration from: {}", config_path.display());
        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "Config file not found: {}. Run 'ai-commit config init' to create it.",
                config_path.display()
            ));
        }
        let config_content = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn load_or_create() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            Self::load()
        } else {
            let default = AppConfig::default();
            default.save()?;
            Ok(default)
        }
    }

    pub fn resolve_api_config(&self) -> Result<ApiConfig> {
        let profile = self.profiles.get(&self.active_profile).ok_or_else(|| {
            let available = self.profiles.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
            anyhow::anyhow!(
                "Active profile '{}' not found in configuration. Available profiles: {}",
                self.active_profile,
                if available.is_empty() { "(none)" } else { &available }
            )
        })?;
        resolve_profile(&self.active_profile, profile)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_content = toml::to_string_pretty(self)?;
        fs::write(&config_path, config_content)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(home.join("ai-commit").join("config.toml"))
    }
}

fn resolve_profile(profile_name: &str, profile: &ModelProfile) -> Result<ApiConfig> {
    if profile.model.trim().is_empty() {
        return Err(anyhow::anyhow!(
            "Model is not configured for profile '{}'. Set profiles.{}.model in the config file.",
            profile_name,
            profile_name
        ));
    }

    if profile.api_key.trim().is_empty() {
        return Err(anyhow::anyhow!(
            "API key is not configured for profile '{}'. Set profiles.{}.api_key in the config file.",
            profile_name,
            profile_name
        ));
    }

    Ok(ApiConfig {
        endpoint: profile.endpoint.clone(),
        api_key: profile.api_key.clone(),
        model: profile.model.clone(),
        max_tokens: Some(profile.max_tokens),
        temperature: Some(profile.temperature),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default_contains_unconfigured_profile() {
        let config = AppConfig::default();
        assert!(config.commit.ignore_lock_files);
        assert!(!config.commit.auto_confirm);
        assert_eq!(config.commit.context_limit, 800000);
        assert_eq!(config.active_profile, "default");
        assert!(config.profiles.contains_key("default"));
        assert!(config.profiles["default"].model.is_empty());
        assert!(config.profiles["default"].api_key.is_empty());
    }

    #[test]
    fn test_commit_config_default() {
        let config = CommitConfig::default();
        assert!(!config.auto_confirm);
        assert!(config.ignore_lock_files);
        assert!(config.custom_ignore_patterns.is_empty());
        assert_eq!(config.context_limit, 800000);
    }

    #[test]
    fn test_prompt_config_default() {
        let config = PromptConfig::default();
        assert!(!config.system_prompt.is_empty());
        assert!(config.user_prompt_template.contains("{diff}"));
    }

    #[test]
    fn test_parse_profiles() {
        let toml_str = r#"
active_profile = "deepseek"

[profiles.doubao]
endpoint = "https://ark.example/v1"
model = "doubao"
api_key = "doubao-secret"
max_tokens = 2048
temperature = 0.4

[profiles.deepseek]
endpoint = "https://deepseek.example/v1"
model = "deepseek-chat"
api_key = "deepseek-secret"
max_tokens = 4096
temperature = 0.7
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        let profile = config.profiles.get("deepseek").unwrap();
        assert_eq!(config.active_profile, "deepseek");
        assert_eq!(profile.endpoint, "https://deepseek.example/v1");
        assert_eq!(profile.model, "deepseek-chat");
        assert_eq!(profile.api_key, "deepseek-secret");
        assert_eq!(profile.max_tokens, 4096);
        assert_eq!(profile.temperature, 0.7);
    }

    #[test]
    fn test_resolve_active_profile_uses_direct_api_key() {
        let config: AppConfig = toml::from_str(
            r#"
active_profile = "deepseek"

[profiles.deepseek]
endpoint = "https://deepseek.example/v1"
model = "deepseek-chat"
api_key = "deepseek-secret"
max_tokens = 2048
temperature = 0.4
"#,
        )
        .unwrap();

        let api_config = config.resolve_api_config().unwrap();
        assert_eq!(api_config.endpoint, "https://deepseek.example/v1");
        assert_eq!(api_config.model, "deepseek-chat");
        assert_eq!(api_config.api_key, "deepseek-secret");
        assert_eq!(api_config.max_tokens, Some(2048));
        assert_eq!(api_config.temperature, Some(0.4));
    }

    #[test]
    fn test_missing_active_profile_is_an_error() {
        let config: AppConfig = toml::from_str(
            r#"
active_profile = "missing"

[profiles.default]
model = "test-model"
api_key = "test-key"
"#,
        )
        .unwrap();

        let error = config.resolve_api_config().err().unwrap().to_string();
        assert!(error.contains("missing"));
        assert!(error.contains("default"));
    }

    #[test]
    fn test_empty_model_is_an_error() {
        let config = AppConfig::default();
        let error = config.resolve_api_config().err().unwrap().to_string();
        assert!(error.contains("Model is not configured"));
        assert!(error.contains("profiles.default.model"));
    }

    #[test]
    fn test_empty_api_key_is_an_error() {
        let config: AppConfig = toml::from_str(
            r#"
[profiles.default]
model = "test-model"
"#,
        )
        .unwrap();

        let error = config.resolve_api_config().err().unwrap().to_string();
        assert!(error.contains("API key is not configured"));
        assert!(error.contains("profiles.default.api_key"));
    }

    #[test]
    fn test_parse_empty_config_uses_defaults() {
        let config: AppConfig = toml::from_str("").unwrap();
        assert!(config.commit.ignore_lock_files);
        assert_eq!(config.active_profile, "default");
        assert!(config.profiles.is_empty());
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let config = AppConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.commit.ignore_lock_files, config.commit.ignore_lock_files);
        assert_eq!(parsed.active_profile, config.active_profile);
        assert_eq!(parsed.profiles["default"].model, config.profiles["default"].model);
        assert_eq!(parsed.profiles["default"].api_key, config.profiles["default"].api_key);
    }
}
