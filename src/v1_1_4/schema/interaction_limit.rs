
/// Interaction Restrictions
/// 
/// Limit interactions to a specific type of user for a specified duration
#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
pub struct InteractionLimit<'a> {
    /// The type of GitHub user that can comment, open issues, or create pull requests while the interaction limit is in effect. Can be one of: `existing_users`, `contributors_only`, `collaborators_only`.
    /// 
    /// # Example
    /// 
    /// ```json
    /// "collaborators_only"
    /// ```
    pub limit: ::std::borrow::Cow<'a, str>,

    /// The duration of the interaction restriction. Can be one of: `one_day`, `three_days`, `one_week`, `one_month`, `six_months`. Default: `one_day`.
    /// 
    /// # Example
    /// 
    /// ```json
    /// "one_month"
    /// ```
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expiry: ::std::option::Option<::std::borrow::Cow<'a, str>>,

    #[serde(flatten)]
    pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
}
