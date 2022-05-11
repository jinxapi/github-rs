#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct ActionsWorkflowAccessToRepository<'a> {
    /// Defines the level of access that workflows outside of the repository have to actions and reusable workflows within the
    /// repository. `none` means access is only possible from workflows in this repository. Can be one of `none`, `organization`, `enterprise`.
    pub access_level: ::std::borrow::Cow<'a, str>,

    #[serde(flatten)]
    pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
}
