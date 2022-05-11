/// The type of deployment branch policy for this environment. To allow all branches to deploy, set to `null`.
#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct DeploymentBranchPolicy<'a> {
    /// Whether only branches with branch protection rules can deploy to this environment. If `protected_branches` is `true`, `custom_branch_policies` must be `false`; if `protected_branches` is `false`, `custom_branch_policies` must be `true`.
    pub protected_branches: bool,

    /// Whether only branches that match the specified name patterns can deploy to this environment.  If `custom_branch_policies` is `true`, `protected_branches` must be `false`; if `custom_branch_policies` is `false`, `protected_branches` must be `true`.
    pub custom_branch_policies: bool,

    #[serde(flatten)]
    pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
}
