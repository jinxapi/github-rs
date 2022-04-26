
pub enum Sort<'sort> {
    Default,
    Ascending(&'sort str),
    Descending(&'sort str),
}

impl<'sort> Default for Sort<'sort> {
    fn default() -> Self {
        Sort::Default
    }
}

impl<'sort> Sort<'sort> {
    pub fn extract(&self) -> (Option<&str>, Option<&str>) {
        match self {
            Sort::Default => (None, None),
            Sort::Ascending(sort) => (Some(sort), Some("asc")),
            Sort::Descending(sort) => (Some(sort), Some("desc"))
        }
    }
}

#[derive(Default)]
pub struct IssueFilter<'filter> {
    /// Indicates which sorts of issues to return. Can be one of:
    /// * `assigned`: Issues assigned to you
    /// * `created`: Issues created by you
    /// * `mentioned`: Issues mentioning you
    /// * `subscribed`: Issues you're subscribed to updates for
    /// * `all` or `repos`: All issues the authenticated user can see, regardless of participation or creation
    pub filter: ::std::option::Option<&'filter str>,
    /// Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
    pub state: ::std::option::Option<&'filter str>,
    /// A list of comma separated label names. Example: `bug,ui,@high`
    pub labels: ::std::option::Option<&'filter str>,
    /// Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    pub since: ::std::option::Option<&'filter str>,
}
