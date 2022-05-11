/// App Permissions
/// 
/// The permissions granted to the user-to-server access token.
/// 
/// # Example
/// 
/// ```json
/// {
///   "contents": "read",
///   "deployments": "write",
///   "issues": "read",
///   "single_file": "read"
/// }
/// ```
#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct AppPermissions<'a> {
    /// The level of permission to grant the access token for GitHub Actions workflows, workflow runs, and artifacts. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub actions: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for repository creation, deletion, settings, teams, and collaborators creation. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub administration: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for checks on code. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checks: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for repository contents, commits, branches, downloads, releases, and merges. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub contents: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for deployments and deployment statuses. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub deployments: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for managing repository environments. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub environments: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for issues and related comments, assignees, labels, and milestones. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub issues: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to search repositories, list collaborators, and access repository metadata. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub metadata: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for packages published to GitHub Packages. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub packages: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to retrieve Pages statuses, configuration, and builds, as well as create new builds. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pages: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for pull requests and related comments, assignees, labels, milestones, and merges. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pull_requests: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage the post-receive hooks for a repository. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub repository_hooks: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage repository projects, columns, and cards. Can be one of: `read`, `write`, or `admin`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub repository_projects: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to view and manage secret scanning alerts. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub secret_scanning_alerts: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage repository secrets. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub secrets: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to view and manage security events like code scanning alerts. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub security_events: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage just a single file. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub single_file: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for commit statuses. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statuses: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage Dependabot alerts. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vulnerability_alerts: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to update GitHub Actions workflow files. Can be one of: `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub workflows: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for organization teams and members. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub members: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage access to an organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_administration: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage the post-receive hooks for an organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_hooks: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for viewing an organization's plan. Can be one of: `read`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_plan: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage organization projects and projects beta (where available). Can be one of: `read`, `write`, or `admin`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_projects: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token for organization packages published to GitHub Packages. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_packages: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage organization secrets. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_secrets: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to view and manage GitHub Actions self-hosted runners available to an organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_self_hosted_runners: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to view and manage users blocked by the organization. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organization_user_blocking: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    /// The level of permission to grant the access token to manage team discussions and related comments. Can be one of: `read` or `write`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub team_discussions: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    #[serde(flatten)]
    pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
}
