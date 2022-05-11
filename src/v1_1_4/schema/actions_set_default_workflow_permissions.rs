#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct ActionsSetDefaultWorkflowPermissions<'a> {
    /// The default workflow permissions granted to the GITHUB_TOKEN when running workflows.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default_workflow_permissions: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// Whether GitHub Actions can approve pull requests. Enabling this can be a security risk.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub can_approve_pull_request_reviews: ::std::option::Option<bool>,

    #[serde(flatten)]
    pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
}
