#![allow(non_snake_case)]

#[path="actions_set_default_workflow_permissions.rs"]
mod actions_set_default_workflow_permissions_Module;
pub use actions_set_default_workflow_permissions_Module::*;

#[path="actions_workflow_access_to_repository.rs"]
mod actions_workflow_access_to_repository_Module;
pub use actions_workflow_access_to_repository_Module::*;

#[path="app_permissions.rs"]
mod app_permissions_Module;
pub use app_permissions_Module::*;

#[path="deployment_branch_policy.rs"]
mod deployment_branch_policy_Module;
pub use deployment_branch_policy_Module::*;

#[path="interaction_limit.rs"]
mod interaction_limit_Module;
pub use interaction_limit_Module::*;

#[path="selected_actions.rs"]
mod selected_actions_Module;
pub use selected_actions_Module::*;
