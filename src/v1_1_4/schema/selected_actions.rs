#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct SelectedActions<'a> {
    /// Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub github_owned_allowed: ::std::option::Option<bool>,

    /// Whether actions from GitHub Marketplace verified creators are allowed. Set to `true` to allow all actions by GitHub Marketplace verified creators.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verified_allowed: ::std::option::Option<bool>,

    /// Specifies a list of string-matching patterns to allow specific action(s) and reusable workflow(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/*`."
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub patterns_allowed: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

    #[serde(flatten)]
    pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
}
