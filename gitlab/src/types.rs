// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! API entities
//!
//! These entities are exposed by Gitlab via its API.
//!
//! There are some places where Gitlab does not completely specify its types. This causes
//! problems when the types and names change inside of those. If found, issues should be filed
//! upstream.

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use derive_builder::Builder;
use log::error;
use serde::de::{DeserializeOwned, Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{self, Value};
use url::Url;

// This is only used in internal API calls.
//#[derive(Serialize, Deserialize, Debug, Clone)]
//pub struct UserSafe {
//    pub username: String,
//    pub name: String,
//}

/// Type alias for slice of string two-tuples
pub type QueryParamSlice<'a> = &'a [(&'a str, &'a str)];

/// Type alias for Vec of string two-tuples
pub type QueryParamVec<'a> = Vec<(&'a str, &'a str)>;

/// Type-safe user ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct UserId(u64);
impl_id!(UserId);

/// The states a user account can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserState {
    /// The user is active and may perform actions.
    Active,
    /// Blocked from logging in.
    Blocked,
    /// Blocked from logging in via LDAP.
    LdapBlocked,
}
enum_serialize!(UserState -> "user state",
    Active => "active",
    Blocked => "blocked",
    LdapBlocked => "ldap_blocked",
);

/// Basic user information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBasic {
    /// The username.
    pub username: String,
    /// The display name.
    pub name: String,
    /// The user's ID.
    pub id: UserId,
    /// The state of the user account.
    pub state: UserState,
    /// The URL of the user's avatar.
    pub avatar_url: Option<String>,
    /// The URL of the user's profile page.
    pub web_url: String,
}

/// A unifying trait for all user types.
///
/// This is used to allow (direct) user queries to return the right information because
/// administrator users receive additional information for all user queries versus
/// non-administrator users.
pub trait UserResult: DeserializeOwned {}
impl<T: DeserializeOwned + Into<UserBasic>> UserResult for T {}

/// More detailed information only accessible to administrators.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// The username.
    pub username: String,
    /// The display name.
    pub name: String,
    /// The user's ID.
    pub id: UserId,
    /// The state of the user account.
    pub state: UserState,
    /// The URL of the user's avatar.
    pub avatar_url: Option<String>,
    /// The URL of the user's profile page.
    pub web_url: String,
    /// When the account was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Whether the user is an administrator or not.
    ///
    /// Only available when talking to GitLab as an admin.
    pub is_admin: Option<bool>,
    /// The highest access level available to the user.
    ///
    /// Only available when talking to GitLab as an admin.
    pub highest_role: Option<AccessLevel>,
    /// Self-described biography of the user.
    pub bio: Option<String>,
    /// Whether the account has a private profile.
    pub private_profile: Option<bool>,
    /// Geographic location of the user.
    pub location: Option<String>,
    /// User public email address, if any.
    pub public_email: Option<String>,

    /// Skype contact information.
    pub skype: String,
    /// LinkedIn contact information.
    pub linkedin: String,
    /// Twitter contact information.
    pub twitter: String,
    /// Custom URL for the user's website.
    pub website_url: String,
    /// Organization the user belongs to.
    pub organization: Option<String>,
}

impl From<User> for UserBasic {
    fn from(user: User) -> Self {
        UserBasic {
            username: user.username,
            name: user.name,
            id: user.id,
            state: user.state,
            avatar_url: user.avatar_url,
            web_url: user.web_url,
        }
    }
}

/// External authentication tokens.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    /// The provider of the token.
    pub provider: String,
    /// The UID for the provider.
    pub extern_uid: String,
}

/// Type-safe theme ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ThemeId(u64);
impl_id!(ThemeId);

/// Type-safe color scheme ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ColorSchemeId(u64);
impl_id!(ColorSchemeId);

/// Full user structure information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPublic {
    /// The username.
    pub username: String,
    /// The display name.
    pub name: String,
    /// The user's ID.
    pub id: UserId,
    /// The state of the user account.
    pub state: UserState,
    /// The URL of the user's avatar.
    pub avatar_url: Option<String>,
    /// The URL of the user's profile page.
    pub web_url: String,
    /// When the account was created.
    pub created_at: Option<DateTime<Utc>>,
    /// Whether the user is an administrator or not.
    ///
    /// Only available when talking to GitLab as an admin.
    pub is_admin: Option<bool>,
    /// The highest access level available to the user.
    ///
    /// Only available when talking to GitLab as an admin.
    pub highest_role: Option<AccessLevel>,
    /// Self-described biography of the user.
    pub bio: Option<String>,
    /// Whether the account has a private profile.
    pub private_profile: Option<bool>,
    /// Geographic location of the user.
    pub location: Option<String>,
    /// User public email address, if any.
    pub public_email: Option<String>,

    /// Skype contact information.
    pub skype: String,
    /// LinkedIn contact information.
    pub linkedin: String,
    /// Twitter contact information.
    pub twitter: String,
    /// Custom URL for the user's website.
    pub website_url: String,
    /// Organization the user belongs to.
    pub organization: Option<String>,

    /// When the user last logged in.
    pub last_sign_in_at: Option<DateTime<Utc>>,
    /// When the user last made an action.
    pub last_activity_on: Option<NaiveDate>,
    /// When the user's account was confirmed.
    pub confirmed_at: Option<DateTime<Utc>>,
    /// The primary email address for the user.
    pub email: String,

    /// The theme used by the user, if configured.
    pub theme_id: Option<ThemeId>,
    /// The color scheme used by the user.
    pub color_scheme_id: ColorSchemeId,
    /// The number of projects the user may create.
    pub projects_limit: u64,
    /// When the user's current session started.
    pub current_sign_in_at: Option<DateTime<Utc>>,

    /// List of identities associated with the user.
    pub identities: Vec<Identity>,

    /// Whether the user can create groups.
    pub can_create_group: bool,
    /// Whether the user can create a new project.
    pub can_create_project: bool,
    /// Whether the user has two-factor authentication enabled.
    pub two_factor_enabled: bool,
    /// Whether the account is externally controlled.
    pub external: bool,
}

impl From<UserPublic> for UserBasic {
    fn from(user: UserPublic) -> Self {
        UserBasic {
            username: user.username,
            name: user.name,
            id: user.id,
            state: user.state,
            avatar_url: user.avatar_url,
            web_url: user.web_url,
        }
    }
}

impl From<UserPublic> for User {
    fn from(user: UserPublic) -> Self {
        User {
            username: user.username,
            name: user.name,
            id: user.id,
            state: user.state,
            avatar_url: user.avatar_url,
            web_url: user.web_url,
            created_at: user.created_at,
            is_admin: user.is_admin,
            highest_role: user.highest_role,
            bio: user.bio,
            private_profile: user.private_profile,
            location: user.location,
            public_email: user.public_email,
            skype: user.skype,
            linkedin: user.linkedin,
            twitter: user.twitter,
            website_url: user.website_url,
            organization: user.organization,
        }
    }
}

/// Type-safe email ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EmailId(u64);
impl_id!(EmailId);

/// Email address.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Email {
    /// ID of the email.
    pub id: EmailId,
    /// The email address.
    pub email: String,
}

/// Type-safe hook ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HookId(u64);
impl_id!(HookId);

/// A web hook to notify of events.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hook {
    /// The ID of the hook.
    pub id: HookId,
    /// The URL to contact.
    pub url: String,
    /// When the hook was created.
    pub created_at: DateTime<Utc>,
    /// Whether the hook is contacted for push events.
    pub push_events: bool,
    /// Whether the hook is contacted for tag push events.
    pub tag_push_events: bool,
    /// Whether the communication with the hook is verified using TLS certificates.
    pub enable_ssl_verification: bool,
}

/// A web hook to notify of project events.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectHook {
    /// The ID of the hook.
    pub id: HookId,
    /// The URL to contact.
    pub url: String,
    /// When the hook was created.
    pub created_at: DateTime<Utc>,
    /// The project associated with the hook.
    pub project_id: ProjectId,
    /// Whether the hook is contacted for push events.
    pub push_events: bool,
    /// Filter branches for which the hook is contacted for push events.
    pub push_events_branch_filter: Option<String>,
    /// Whether the hook is contacted for tag push events.
    pub tag_push_events: bool,
    /// Whether the hook is contacted for issue events.
    pub issues_events: bool,
    /// Whether the hook is contacted for confidential issue events.
    pub confidential_issues_events: Option<bool>,
    /// Whether the hook is contacted for merge request events.
    pub merge_requests_events: bool,
    /// Whether the hook is contacted for note events.
    pub note_events: bool,
    /// Whether the hook is contacted for confidential note events.
    pub confidential_note_events: Option<bool>,
    /// Whether the hook is contacted for repository update events.
    pub repository_update_events: bool,
    /// Whether the communication with the hook is verified using TLS certificates.
    pub enable_ssl_verification: bool,
    /// Whether the hook is contacted for job events.
    pub job_events: bool,
    /// Whether the hook is contacted for pipeline events.
    pub pipeline_events: bool,
    /// Whether the hook is contacted for wiki page events.
    pub wiki_page_events: bool,
    /// Secret token to validate received payloads
    pub token: Option<String>,
}

impl From<ProjectHook> for Hook {
    fn from(hook: ProjectHook) -> Self {
        Hook {
            id: hook.id,
            url: hook.url,
            created_at: hook.created_at,
            push_events: hook.push_events,
            tag_push_events: hook.tag_push_events,
            enable_ssl_verification: hook.enable_ssl_verification,
        }
    }
}

/// The events a webhook listener may receive.
#[derive(Debug, Default, Clone, Copy)]
pub struct WebhookEvents {
    /// Whether to receive job events of not.
    job: bool,
    /// Whether to receive issue events of not.
    issues: bool,
    /// Whether to receive confidential issue events of not.
    confidential_issues: bool,
    /// Whether to receive merge request events of not.
    merge_requests: bool,
    /// Whether to receive note (comment) events of not.
    note: bool,
    /// Whether to receive pipeline events of not.
    pipeline: bool,
    /// Whether to receive push events of not.
    push: bool,
    /// Whether to receive wiki events of not.
    wiki_page: bool,
}

impl WebhookEvents {
    /// Create a new, empty webhook event set.
    pub fn new() -> Self {
        WebhookEvents {
            job: false,
            issues: false,
            confidential_issues: false,
            merge_requests: false,
            note: false,
            pipeline: false,
            push: false,
            wiki_page: false,
        }
    }

    with_event! {with_job, job}
    with_event! {with_issues, issues}
    with_event! {with_confidential_issues, issues}
    with_event! {with_merge_requests, merge_requests}
    with_event! {with_note, note}
    with_event! {with_pipeline, pipeline}
    with_event! {with_push, push}
    with_event! {with_wiki_page, wiki_page}

    get_event! {job}
    get_event! {issues}
    get_event! {confidential_issues}
    get_event! {merge_requests}
    get_event! {note}
    get_event! {pipeline}
    get_event! {push}
    get_event! {wiki_page}
}

/// Type-safe project ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProjectId(u64);
impl_id!(ProjectId);

/// Basic project information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicProjectDetails {
    /// The ID of the project.
    pub id: ProjectId,
    /// The display name of the project.
    pub name: String,
    /// The display name of the project with the namespace.
    pub name_with_namespace: String,
    /// The path to the project's repository.
    pub path: String,
    /// The path to the project's repository with its namespace.
    pub path_with_namespace: String,
    /// The URL to the main page of the repository.
    pub http_url_to_repo: String,
    /// The URL to the main page of the repository.
    pub web_url: String,
}

/// Visibility levels of projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisibilityLevel {
    /// The project is visible to anonymous users.
    Public,
    /// The project is visible to logged in users.
    Internal,
    /// The project is visible only to users with explicit access.
    Private,
}
enum_serialize!(VisibilityLevel -> "visibility level",
    Public => "public",
    Internal => "internal",
    Private => "private",
);

/// Visibility levels for project features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatureVisibilityLevel {
    /// Feature is disabled.
    Disabled,
    /// Feature is enabled and accessible privately.
    Private,
    /// Feature is enabled and accessible with project-wide visibility level.
    Enabled,
    /// Feature is enabled and accessible publicly.
    Public,
}
enum_serialize!(FeatureVisibilityLevel -> "feature visibility level",
    Disabled => "disabled",
    Private => "private",
    Enabled => "enabled",
    Public => "public",
);

// TODO: enum for NotificationLevel

/// Structure for a group a project has been shared with.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedGroup {
    /// The ID of the group.
    pub group_id: GroupId,
    /// The name of the group.
    pub group_name: String,
    /// The access level of the group.
    pub group_access_level: u64,
}

/// Access information to a project.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
// Called `MemberAccess` in entities.rb, but it is just a base class for `ProjectAccess` and
// `GroupAccess`. Combine them here.
pub struct MemberAccess {
    /// The access level of the membership (see `VisibilityLevel`).
    pub access_level: u64,
    /// The notification level of the current user.
    pub notification_level: Option<u64>,
}

/// Permissions granted to the current user to a project.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Permissions {
    /// The access granted by the project to the current user.
    pub project_access: Option<MemberAccess>,
    /// The access granted by the group to the current user.
    pub group_access: Option<MemberAccess>,
}

/// The avatar of a project's namespace.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectNamespaceAvatar {
    /// The URL of the namespace avatar.
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProjectLinks {
    #[serde(rename = "self")]
    /// API URL of project itself.
    self_: String,
    /// API URL of project issues, if enabled.
    issues: Option<String>,
    /// API URL of project merge requests, if enabled.
    merge_requests: Option<String>,
    /// API URL of project repository branches.
    repo_branches: String,
    /// API URL of project labels.
    labels: String,
    /// API URL of project events.
    events: String,
    /// API URL of project members.
    members: String,
}

/// Project information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    /// The ID of the project.
    pub id: ProjectId,
    /// The description of the project.
    pub description: Option<String>,
    /// The default branch for the project.
    pub default_branch: Option<String>,
    /// A list of tags for the project.
    pub tag_list: Vec<String>,
    /// Whether the project is archived or not.
    pub archived: bool,
    /// Whether the project has an empty repository or not.
    pub empty_repo: bool,
    /// Whether the project is public, internal, or private.
    pub visibility: VisibilityLevel,
    /// The URL to clone the repository over SSH.
    pub ssh_url_to_repo: String,
    /// The URL to clone the repository over HTTPS.
    pub http_url_to_repo: String,
    /// The URL for the project's homepage.
    pub web_url: String,
    /// The URL for the project's readme.
    pub readme_url: Option<String>,
    /// The owner of the project (`None` for a group-owned project).
    pub owner: Option<UserBasic>,
    /// The display name of the project.
    pub name: String,
    /// The display name of the project with the namespace.
    pub name_with_namespace: String,
    /// The path to the project's repository.
    pub path: String,
    /// The path to the project's repository with its namespace.
    pub path_with_namespace: String,
    /// Whether the continuous integration container registry is enabled.
    ///
    /// This is supposed to be just `bool`, but projects created before the registry was
    /// supported appear to return `null`.
    pub container_registry_enabled: Option<bool>,
    /// When the repository was created.
    pub created_at: DateTime<Utc>,
    /// When the last activity on the project occurred.
    pub last_activity_at: DateTime<Utc>,
    /// Whether continuous integration shared runners are enabled.
    pub shared_runners_enabled: bool,
    /// Whether LFS object storage is enabled.
    pub lfs_enabled: bool,
    /// The user who created the repository.
    pub creator_id: UserId,
    /// The namespace the project lives in.
    pub namespace: Namespace,
    /// If the project is a fork, details about it.
    pub forked_from_project: Option<BasicProjectDetails>,
    /// The URL to the project avatar.
    pub avatar_url: Option<String>,
    /// The path to CI config file.
    pub ci_config_path: Option<String>,
    /// Description of error if project failed to import.
    pub import_error: Option<String>,
    /// The number of stars for the project.
    pub star_count: u64,
    /// The number of forks.
    pub forks_count: u64,
    /// The number of open issues (if issues are enabled).
    pub open_issues_count: Option<u64>,
    /// The continuous integration runner token (if enabled).
    pub runners_token: Option<String>,
    /// Whether jobs are publicly visible.
    pub public_jobs: bool,
    /// Groups the project is shared with.
    pub shared_with_groups: Vec<SharedGroup>,
    /// Whether the project only enables the merge button if all pipelines are passing.
    pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
    /// Whether the project only enables the merge button if all discussions are resolved.
    pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    /// Whether enable 'Delete source branch' option by default for all new merge requests.
    pub remove_source_branch_after_merge: Option<bool>,
    /// Whether to show the link to create/view merge request when pusing from command line.
    pub printing_merge_request_link_enabled: Option<bool>,
    /// Whether access to the project may be requested.
    pub request_access_enabled: bool,
    /// Whether to automatically resolve merge request diff discussions when they become outdated,
    /// if configured.
    pub resolve_outdated_diff_discussions: Option<bool>,

    /// Whether jobs are enabled or not.
    pub jobs_enabled: bool,
    /// Whether issues are enabled or not.
    pub issues_enabled: bool,
    /// Whether merge requests are enabled or not.
    pub merge_requests_enabled: bool,
    /// Whether snippets are enabled or not.
    pub snippets_enabled: bool,
    /// Whether the project wiki is enabled or not.
    pub wiki_enabled: bool,

    /// Visibility of builds.
    pub builds_access_level: FeatureVisibilityLevel,
    /// Visibility of issues.
    pub issues_access_level: FeatureVisibilityLevel,
    /// Visibility of merge requests.
    pub merge_requests_access_level: FeatureVisibilityLevel,
    /// Visibility of repository.
    pub repository_access_level: FeatureVisibilityLevel,
    /// Visibility of snippets.
    pub snippets_access_level: FeatureVisibilityLevel,
    /// Visibility of wiki.
    pub wiki_access_level: FeatureVisibilityLevel,

    /// The merge method used when merging merge request.
    pub merge_method: Option<String>,
    /// Statistics about the project.
    pub statistics: Option<ProjectStatistics>,

    /// If this is present, it is `ProjectWithAccess`, but since it is so similar, just have it be
    /// optional here.
    pub permissions: Option<Permissions>,

    /// Links to related API URLs provided by GitLab in response to
    /// direct project lookup.  We do not expose this because our
    /// clients do not need them.
    _links: Option<ProjectLinks>,
}

#[cfg(test)]
impl Project {
    pub fn has_links(&self) -> bool {
        self._links.is_some()
    }
}

/// Statistics about a project.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProjectStatistics {
    /// The number of commits in the repository.
    pub commit_count: u64,
    /// The size, in bytes, of the total storage required for the project.
    pub storage_size: u64,
    /// The size, in bytes, of the repository.
    pub repository_size: u64,
    /// The size, in bytes, of uploaded LFS files.
    pub lfs_objects_size: u64,
    /// The size, in bytes, of uploaded job artifacts.
    pub job_artifacts_size: u64,
}

/// Access levels for groups and projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessLevel {
    /// Anonymous access.
    Anonymous,
    /// Guest access (can see the project).
    Guest,
    /// Reporter access (can open issues).
    Reporter,
    /// Developer access (can push branches, handle issues and merge requests).
    Developer,
    /// Maintainer access (can push to protected branches).
    Maintainer,
    /// Owner access (full rights).
    Owner,
    /// Admin access (full rights).
    Admin,
}

impl From<AccessLevel> for u64 {
    fn from(access: AccessLevel) -> Self {
        match access {
            AccessLevel::Anonymous => 0,
            AccessLevel::Guest => 10,
            AccessLevel::Reporter => 20,
            AccessLevel::Developer => 30,
            AccessLevel::Maintainer => 40,
            AccessLevel::Owner => 50,
            AccessLevel::Admin => 60,
        }
    }
}

impl From<u64> for AccessLevel {
    fn from(access: u64) -> Self {
        if access >= 60 {
            AccessLevel::Admin
        } else if access >= 50 {
            AccessLevel::Owner
        } else if access >= 40 {
            AccessLevel::Maintainer
        } else if access >= 30 {
            AccessLevel::Developer
        } else if access >= 20 {
            AccessLevel::Reporter
        } else if access >= 10 {
            AccessLevel::Guest
        } else {
            AccessLevel::Anonymous
        }
    }
}

impl AccessLevel {
    pub fn as_str(&self) -> &str {
        match self {
            AccessLevel::Admin => "admin",
            AccessLevel::Owner => "owner",
            AccessLevel::Developer => "developer",
            AccessLevel::Anonymous => "anonymous",
            AccessLevel::Guest => "guest",
            AccessLevel::Maintainer => "maintainer",
            AccessLevel::Reporter => "reporter",
        }
    }
}

impl Display for AccessLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", Into::<u64>::into(*self))
    }
}

impl Serialize for AccessLevel {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        u64::from(*self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AccessLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(<u64 as Deserialize>::deserialize(deserializer)?.into())
    }
}

/// A member with extra permissions on a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Member {
    /// The username.
    pub username: String,
    /// The display name.
    pub name: String,
    /// The user's ID.
    pub id: UserId,
    /// The state of the user account.
    pub state: UserState,
    /// The URL of the user's avatar.
    pub avatar_url: Option<String>,
    /// The URL of the user's profile page.
    pub web_url: String,
    /// The access level of the user.
    pub access_level: u64,
    /// When the membership expires.
    pub expires_at: Option<NaiveDate>,
}

impl From<Member> for UserBasic {
    fn from(member: Member) -> Self {
        UserBasic {
            username: member.username,
            name: member.name,
            id: member.id,
            state: member.state,
            avatar_url: member.avatar_url,
            web_url: member.web_url,
        }
    }
}

/// A member with extra permissions on a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessRequester {
    /// The username.
    pub username: String,
    /// The display name.
    pub name: String,
    /// The user's ID.
    pub id: UserId,
    /// The state of the user account.
    pub state: UserState,
    /// The URL of the user's avatar.
    pub avatar_url: Option<String>,
    /// The URL of the user's profile page.
    pub web_url: String,
    /// When the membership request was created.
    pub requested_at: DateTime<Utc>,
}

impl From<AccessRequester> for UserBasic {
    fn from(member: AccessRequester) -> Self {
        UserBasic {
            username: member.username,
            name: member.name,
            id: member.id,
            state: member.state,
            avatar_url: member.avatar_url,
            web_url: member.web_url,
        }
    }
}

/// Type-safe group ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GroupId(u64);
impl_id!(GroupId);

/// Group information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    /// The ID of the group.
    pub id: GroupId,
    /// The name of the group.
    pub name: String,
    /// The path to the group.
    pub path: String,
    /// The description of the group.
    pub description: Option<String>,
    /// Whether the project is public, internal, or private.
    pub visibility: VisibilityLevel,
    /// Whether LFS is enabled for the group.
    pub lfs_enabled: bool,
    /// The URL to the group avatar.
    pub avatar_url: Option<String>,
    /// The URL to the group's profile page.
    pub web_url: String,
    /// Whether membership requests are allowed for the group.
    pub request_access_enabled: bool,
    pub full_name: String,
    pub full_path: String,
    pub parent_id: Option<GroupId>,
    /// Statistics about the group.
    pub statistics: Option<GroupStatistics>,
}

/// Statistics about a group.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GroupStatistics {
    /// The size, in bytes, of the total storage required for the group.
    pub storage_size: u64,
    /// The size, in bytes, of all repositories in the group.
    pub repository_size: u64,
    /// The size, in bytes, of uploaded LFS files in the group.
    pub lfs_objects_size: u64,
    /// The size, in bytes, of uploaded job artifacts in the group.
    pub job_artifacts_size: u64,
}

/// Group information with a project listing.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupDetail {
    /// The ID of the group.
    pub id: GroupId,
    /// The name of the group.
    pub name: String,
    /// The path to the group.
    pub path: String,
    /// The description of the group.
    pub description: Option<String>,
    /// Whether the project is public, internal, or private.
    pub visibility: VisibilityLevel,
    /// Whether LFS is enabled for the group.
    pub lfs_enabled: bool,
    /// The URL to the group avatar.
    pub avatar_url: Option<String>,
    /// The URL to the group's profile page.
    pub web_url: String,
    /// The projects in a group.
    pub projects: Vec<Project>,
    /// Projects the group shares with other groups or users.
    pub shared_projects: Vec<Project>,
    /// Whether membership requests are allowed for the group.
    pub request_access_enabled: bool,
    pub full_name: String,
    pub full_path: String,
    pub parent_id: Option<GroupId>,
    /// Statistics about the group.
    pub statistics: Option<GroupStatistics>,
}

impl From<GroupDetail> for Group {
    fn from(detail: GroupDetail) -> Self {
        Group {
            id: detail.id,
            name: detail.name,
            path: detail.path,
            description: detail.description,
            visibility: detail.visibility,
            lfs_enabled: detail.lfs_enabled,
            avatar_url: detail.avatar_url,
            web_url: detail.web_url,
            request_access_enabled: detail.request_access_enabled,
            full_name: detail.full_name,
            full_path: detail.full_path,
            parent_id: detail.parent_id,
            statistics: detail.statistics,
        }
    }
}

/// A branch on a repository.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoBranch {
    /// The name of the branch.
    pub name: String,
    /// The commit of the branch.
    pub commit: Option<RepoCommit>,
    /// Whether the branch is merged into the main branch or not.
    pub merged: Option<bool>,
    /// Whether the branch is protected or not.
    pub protected: Option<bool>,
    /// Whether the developers can push directly to the branch or not.
    pub developers_can_push: Option<bool>,
    /// Whether the developers can merge into the branch or not.
    pub developers_can_merge: Option<bool>,
    /// Whether the current user can push to the branch.
    pub can_push: Option<bool>,
    /// Whether the branch is the repository default branch.
    pub default: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PRBAccessLevel {
    access_level: u64,
    access_level_description: String,
}

/// A protected branch on a repository
#[derive(Deserialize, Debug, Clone)]
pub struct ProtectedRepoBranch {
    pub name: String,
    pub push_access_levels: Vec<PRBAccessLevel>,
    pub merge_access_levels: Vec<PRBAccessLevel>,
    pub code_owner_approval_required: bool,
}

/// The ID of a git object.
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ObjectId(String);

impl ObjectId {
    /// Construct a new `ObjectId`
    pub fn new<O: ToString>(oid: O) -> Self {
        ObjectId(oid.to_string())
    }

    /// The value of the id.
    pub fn value(&self) -> &String {
        &self.0
    }
}

/// The kinds of objects Gitlab can return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    /// A `tree` object.
    Tree,
    /// A `blob` object.
    Blob,
}
enum_serialize!(ObjectType -> "object type",
    Tree => "tree",
    Blob => "blob",
);

/// An object inside of a repository.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoTreeObject {
    /// The ID of the object.
    pub id: ObjectId,
    /// The name of the object.
    pub name: String,
    #[serde(rename = "type")]
    /// The type of the object.
    pub type_: ObjectType,
    /// The path to the object inside of the repository.
    pub path: String,
    /// The mode of the object.
    pub mode: String,
}

/// A commit in a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoCommit {
    /// The ID of the commit.
    pub id: ObjectId,
    /// The short ID of the commit.
    pub short_id: ObjectId,
    /// The summary of the commit.
    pub title: String,
    /// The commit ID of the parents of the commit.
    pub parent_ids: Vec<ObjectId>,
    /// The commit author's name.
    pub author_name: String,
    /// The commit author's email address.
    pub author_email: String,
    /// The commit's authorship date.
    pub authored_date: DateTime<Utc>,
    /// The committer's name.
    pub committer_name: String,
    /// The committer's email address.
    pub committer_email: String,
    /// The commit's commit date.
    pub committed_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    /// The full commit message.
    pub message: String,
}

/// Stats about a commit.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct RepoCommitStats {
    /// The number of lines added by the commit.
    pub additions: u64,
    /// The number of lines deleted by the commit.
    pub deletions: u64,
    /// The number of lines changed by the commit.
    pub total: u64,
}

/// A commit in a project with statistics.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoCommitDetail {
    /// The ID of the commit.
    pub id: ObjectId,
    /// The short ID of the commit.
    pub short_id: ObjectId,
    /// The summary of the commit.
    pub title: String,
    /// The commit ID of the parents of the commit.
    pub parent_ids: Vec<ObjectId>,
    /// The commit author's name.
    pub author_name: String,
    /// The commit author's email address.
    pub author_email: String,
    /// The commit's authorship date.
    pub authored_date: DateTime<Utc>,
    /// The committer's name.
    pub committer_name: String,
    /// The committer's email address.
    pub committer_email: String,
    /// The commit's commit date.
    pub committed_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    /// The full commit message.
    pub message: String,
    /// Statistics about the commit.
    pub stats: Option<RepoCommitStats>,
    /// The last pipeline for this commit, if any.
    pub last_pipeline: Option<PipelineBasic>,
    /// The project associated with the commit.
    pub project_id: ProjectId,
    // XXX: Investigate what this is.
    /// This looks to be CI related; ignoring without better docs.
    status: Value,
}

/// Type-safe snippet ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SnippetId(u64);
impl_id!(SnippetId);

/// A project-specific snippet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectSnippet {
    /// The ID of the snippet.
    pub id: SnippetId,
    /// The title of the snippet.
    pub title: String,
    /// The name of the snippet.
    pub file_name: String,
    /// The author of the snippet.
    pub author: UserBasic,
    /// When the snippet was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the snippet was created.
    pub created_at: DateTime<Utc>,
    /// When the snippet was created.
    pub expires_at: Option<DateTime<Utc>>,
    /// The URL of the snippet.
    pub web_url: String,
}

// This is just used as a common "base class" in Ruby.
//#[derive(Serialize, Deserialize, Debug, Clone)]
//pub struct ProjectEntity {
//    pub id: ProjectEntityId,
//    pub iid: ProjectEntityInternalId,
//    pub project_id: ProjectId,
//    pub title: String,
//    pub description: String,
//    pub state: ProjectEntityState,
//    pub created_at: DateTime<Utc>,
//    pub updated_at: DateTime<Utc>,
//}

/// A diff within a repository.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoDiff {
    /// The path on the old side of the diff.
    pub old_path: String,
    /// The path on the new side of the diff.
    pub new_path: String,
    /// The mode on the old side of the diff.
    pub a_mode: String,
    /// The mode on the new side of the diff.
    pub b_mode: String,
    pub diff: String,
    /// Whether the diff indicates the addition of a file.
    pub new_file: bool,
    /// Whether the diff indicates the rename of a file.
    pub renamed_file: bool,
    /// Whether the diff indicates the deletion of a file.
    pub deleted_file: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DiffRefs {
    /// SHA referencing base commit in the source branch
    pub base_sha: Option<ObjectId>,
    /// SHA referencing head commit in the source branch
    pub head_sha: Option<ObjectId>,
    /// SHA referencing commit in target branch
    pub start_sha: Option<ObjectId>,
}

/// Type-safe milestone ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MilestoneId(u64);
impl_id!(MilestoneId);

/// Type-safe milestone internal ID (internal to a project).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MilestoneInternalId(u64);
impl_id!(MilestoneInternalId);

/// The states a milestone may be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilestoneState {
    /// The milestone is active.
    Active,
    /// The milestone has been closed.
    Closed,
}
enum_serialize!(MilestoneState -> "milestone type",
    Active => "active",
    Closed => "closed",
);

/// A milestone in a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Milestone {
    /// The ID of the milestone.
    pub id: MilestoneId,
    /// The user-visible ID of the milestone.
    pub iid: MilestoneInternalId,
    /// The ID of the project if this is a project milestone.
    pub project_id: Option<ProjectId>,
    /// The ID of the group if this is a group milestone.
    pub group_id: Option<GroupId>,
    /// The title of the milestone.
    pub title: String,
    /// The description of the milestone.
    pub description: Option<String>,
    /// The state of the milestone.
    pub state: MilestoneState,
    /// When the milestone was created.
    pub created_at: DateTime<Utc>,
    /// When the milestone was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the milestone is due.
    pub due_date: Option<NaiveDate>,
    /// When the milestone was started.
    pub start_date: Option<NaiveDate>,
}

impl Milestone {
    /// Create a new blank milestone: it needs at least the ProjectId and title
    /// ProjectId and title are mandatory for new milestone API of Gitlab
    pub fn new_for_project(project_id: ProjectId, title: String) -> Milestone {
        Milestone {
            id: MilestoneId::new(0),
            iid: MilestoneInternalId::new(0),
            project_id: Some(project_id),
            group_id: None,
            title,
            description: None,
            state: MilestoneState::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            start_date: None,
        }
    }
    /// Create a new blank group milestone: it needs at least the GroupId and title
    /// GroupId and title are mandatory for new milestone API of Gitlab
    pub fn new_for_group(group_id: GroupId, title: String) -> Milestone {
        Milestone {
            id: MilestoneId::new(0),
            iid: MilestoneInternalId::new(0),
            project_id: None,
            group_id: Some(group_id),
            title,
            description: None,
            state: MilestoneState::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            start_date: None,
        }
    }
    /// Complements the milestone with optional paramater: description
    pub fn with_description(mut self, description: String) -> Milestone {
        self.description = Some(description);
        self
    }
    /// Complements the milestone with optional parameter: due_date
    pub fn with_due_date(mut self, due_date: NaiveDate) -> Milestone {
        self.due_date = Some(due_date);
        self
    }
    /// Complements the milestone with optional parameter: start_date
    pub fn with_start_date(mut self, start_date: NaiveDate) -> Milestone {
        self.start_date = Some(start_date);
        self
    }
}

/// Type-safe label ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LabelId(u64);
impl_id!(LabelId);

/// Type-safe label color.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LabelColor(String);

impl LabelColor {
    /// Creates a LabelColor from RGB values
    pub fn from_rgb(r: u8, g: u8, b: u8) -> LabelColor {
        LabelColor(format!("#{:02X}{:02X}{:02X}", r, g, b))
    }

    /// Get the value from a LabelColor
    pub fn value(self) -> String {
        self.0
    }
}

impl FromStr for LabelColor {
    type Err = ();

    /// Creates a LabelColor from standard HTML values
    fn from_str(stdcolor: &str) -> Result<Self, Self::Err> {
        let hex = match stdcolor {
            "white" => "FFFFFF",
            "silver" => "C0C0C0",
            "gray" => "808080",
            "black" => "000000",
            "red" => "FF0000",
            "maroon" => "800000",
            "yellow" => "FFFF00",
            "olive" => "808000",
            "lime" => "00FF00",
            "green" => "008000",
            "aqua" => "00FFFF",
            "teal" => "008080",
            "blue" => "0000FF",
            "navy" => "000080",
            "fuchsia" => "FF00FF",
            "purple" => "800080",
            _ => "808080",
        };

        Ok(LabelColor(format!("#{}", hex)))
    }
}

/// An label on a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    /// The Id of the label.
    pub id: LabelId,
    /// The name of the label.
    pub name: String,
    /// The color of the label.
    pub color: LabelColor,
    /// The description of the label.
    pub description: Option<String>,
    /// The number of opened issues associated with the label.
    pub open_issues_count: Option<u64>,
    /// the number of closed issues associated with the label.
    pub closed_issues_count: Option<u64>,
    /// The number of open merge request associated with the label.
    pub open_merge_requests_count: Option<u64>,
    /// Whether or not the account connecting has subscribed to the label.
    pub subscribed: bool,
    /// The priority of the label.
    pub priority: Option<u64>,
}

impl Label {
    /// Create a new Label: it needs at least a name and a color.
    /// ProjectId is mandatory for Gitlab API
    pub fn new(name: String, color: LabelColor) -> Label {
        Label {
            id: LabelId::new(0),
            name,
            color,
            description: None,
            open_issues_count: None,
            closed_issues_count: None,
            open_merge_requests_count: None,
            subscribed: false,
            priority: None,
        }
    }
    /// Complements the label with optional parameter: description
    pub fn with_description(mut self, description: String) -> Label {
        self.description = Some(description);
        self
    }

    /// Complements the label with optional parameter: priority
    pub fn with_priority(mut self, priority: u64) -> Label {
        self.priority = Some(priority);
        self
    }
}

/// Type-safe issue ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct IssueId(u64);
impl_id!(IssueId);

/// Type-safe issue internal ID (internal to a project).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct IssueInternalId(u64);
impl_id!(IssueInternalId);

/// The states an issue may be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueState {
    /// The issue is open.
    Opened,
    /// The issue has been closed.
    Closed,
    /// The issue has been opened after being closed.
    Reopened,
}
enum_serialize!(IssueState -> "issue type",
    Opened => "opened",
    Closed => "closed",
    Reopened => "reopened",
);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct IssueLinks {
    #[serde(rename = "self")]
    /// API URL of issue itself.
    self_: String,
    /// API URL of issue notes.
    notes: String,
    /// API URL of issue award emoji.
    award_emoji: String,
    /// API URL of issue project.
    project: String,
}

/// An issue on a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
    /// The ID of the issue.
    pub id: IssueId,
    /// The user-visible ID of the issue.
    pub iid: IssueInternalId,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The title of the issue.
    pub title: String,
    /// The description of the issue.
    pub description: Option<String>,
    /// The state of the issue.
    pub state: IssueState,
    /// When the issue was created.
    pub created_at: DateTime<Utc>,
    /// When the issue was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the issue was closed, if closed.
    pub closed_at: Option<DateTime<Utc>>,
    /// The user that closed the issue.
    pub closed_by: Option<UserBasic>,
    /// The labels attached to the issue.
    pub labels: Vec<String>,
    /// The milestone of the issue.
    pub milestone: Option<Milestone>,
    /// The author of the issue.
    pub author: UserBasic,
    /// The assignee of the issue.
    pub assignee: Option<UserBasic>,
    /// The assignees of the issue.
    pub assignees: Option<Vec<UserBasic>>,
    /// Whether the current user is subscribed or not.
    /// GitLab does not include this in responses with lists of issues but
    /// does on an individual issue.
    pub subscribed: Option<bool>,
    /// Time estimates.
    pub time_stats: IssuableTimeStats,
    /// The number of comments on the issue.
    pub user_notes_count: u64,
    /// The number of merge requests referencing the issue.
    pub merge_requests_count: u64,
    /// The number of upvotes for the issue.
    pub upvotes: u64,
    /// The number of downvotes against the issue.
    pub downvotes: u64,
    /// When the issue is due.
    pub due_date: Option<NaiveDate>,
    /// Whether the issue is has a non-empty task list.
    /// GitLab does not include this in issue references.
    pub has_tasks: Option<bool>,
    /// Whether the issue is confidential or not.
    pub confidential: bool,
    /// Whether the discussion has been locked.
    pub discussion_locked: Option<bool>,
    /// The URL of the issue.
    pub web_url: String,

    /// Links to related API URLs provided by GitLab in response to
    /// direct issue lookup.  We do not expose this because our
    /// clients do not need them.
    _links: Option<IssueLinks>,
}

impl Issue {
    /// Creates a new blank issue: it needs at least the ProjectId, title and author
    /// ProjectId and author are mandatory in the Issue struct itself
    /// title is mandatory for the new issue API of Gitlab
    pub fn new(project_id: ProjectId, title: String, author: UserBasic) -> Issue {
        // initialize with default parameters
        Issue {
            id: IssueId::new(0),
            iid: IssueInternalId::new(0),
            project_id,
            title,
            description: None,
            state: IssueState::Opened,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            closed_by: None,
            labels: Vec::new(),
            milestone: None,
            author,
            assignee: None,
            assignees: None,
            subscribed: None,
            time_stats: IssuableTimeStats {
                time_estimate: 0,
                total_time_spent: 0,
                human_time_estimate: None,
                human_total_time_spent: None,
            },
            user_notes_count: 0,
            merge_requests_count: 0,
            upvotes: 0,
            downvotes: 0,
            due_date: None,
            has_tasks: None,
            confidential: false,
            discussion_locked: None,
            web_url: "".into(),
            _links: None,
        }
    }
    /// Complements the issue with optional parameter: iid
    pub fn with_iid(mut self, iid: IssueInternalId) -> Issue {
        self.iid = iid;
        self
    }
    /// Complements the issue with optional parameter: description
    pub fn with_description(mut self, description: String) -> Issue {
        self.description = Some(description);
        self
    }
    /// Complements the issue with optional parameter: confidential
    pub fn with_confidential(mut self, confidential: bool) -> Issue {
        self.confidential = confidential;
        self
    }
    /// Complements the issue with optional parameter: assignees
    pub fn with_assignees(mut self, assignees: Vec<UserBasic>) -> Issue {
        self.assignees = Some(assignees);
        self
    }
    /// Complements the issue with optional parameter: milestone
    pub fn with_milestone(mut self, milestone: Milestone) -> Issue {
        self.milestone = Some(milestone);
        self
    }
    /// Complements the issue with optional parameter: labels
    pub fn with_labels(mut self, labels: Vec<String>) -> Issue {
        self.labels = labels;
        self
    }
    /// Complements the issue with optional parameter: created_at
    pub fn with_created_at(mut self, created_at: DateTime<Utc>) -> Issue {
        self.created_at = created_at;
        self
    }
    /// Complements the issue with optional parameter: due_date
    pub fn with_due_date(mut self, due_date: NaiveDate) -> Issue {
        self.due_date = Some(due_date);
        self
    }
    pub fn has_links(&self) -> bool {
        self._links.is_some()
    }
}

/// A time estimate on an issue or merge request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuableTimeStats {
    /// The time estimate, in seconds.
    pub time_estimate: u64,
    /// The total time spent, in seconds.
    pub total_time_spent: u64,
    /// The time estimate, as a human-readable string.
    pub human_time_estimate: Option<String>,
    /// The total time spent, as a human-readable string.
    pub human_total_time_spent: Option<String>,
}

/// Type-safe external issue ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalIssueId(u64);

/// An external issue reference.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalIssue {
    /// The ID of the issue.
    pub id: ExternalIssueId,
    /// The title of the issue.
    pub title: String,
}

/// A reference to an issue.
#[derive(Debug, Clone)]
pub enum IssueReference {
    /// A reference to an issue on the same Gitlab host.
    Internal(Box<Issue>),
    /// An external issue reference.
    External(ExternalIssue),
}

impl Serialize for IssueReference {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            IssueReference::Internal(ref issue) => issue.serialize(serializer),
            IssueReference::External(ref issue) => issue.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for IssueReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = <Value as Deserialize>::deserialize(deserializer)?;

        serde_json::from_value::<Issue>(val.clone())
            .map(|issue| IssueReference::Internal(Box::new(issue)))
            .or_else(|_| serde_json::from_value::<ExternalIssue>(val).map(IssueReference::External))
            .map_err(|err| D::Error::custom(format!("invalid issue reference: {:?}", err)))
    }
}

/// Type-safe merge request ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MergeRequestId(u64);
impl_id!(MergeRequestId);

/// Type-safe merge request internal ID (internal to a project).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MergeRequestInternalId(u64);
impl_id!(MergeRequestInternalId);

/// The status of the possible merge for a merge request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStatus {
    /// The merge request has not been checked yet.
    Unchecked,
    /// The merge request may be merged.
    CanBeMerged,
    /// The merge request may not be merged yet.
    CannotBeMerged,
    /// The merge request has not been checked but previously
    /// could not be merged.
    CannotBeMergedRecheck,
}
enum_serialize!(MergeStatus -> "merge status",
    Unchecked => "unchecked",
    CanBeMerged => "can_be_merged",
    CannotBeMerged => "cannot_be_merged",
    CannotBeMergedRecheck => "cannot_be_merged_recheck",
);

/// The states a merge request may be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeRequestState {
    /// The merge request is open.
    Opened,
    /// The merge request has been closed before merging.
    Closed,
    /// The merge request has been opened after closing.
    Reopened,
    /// The merge request has been merged.
    Merged,
    /// The merge request is locked from further discussion or updates.
    Locked,
}
enum_serialize!(MergeRequestState -> "merge request state",
    Opened => "opened",
    Closed => "closed",
    Reopened => "reopened",
    Merged => "merged",
    Locked => "locked",
);

/// Information about current user's access to the merge request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestUser {
    /// Whether the current user can merge the MR.
    pub can_merge: bool,
}

/// A merge request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequest {
    /// The ID of the merge request.
    pub id: MergeRequestId,
    /// The user-visible ID of the merge request.
    pub iid: MergeRequestInternalId,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The title of the merge request.
    pub title: String,
    /// The description of the merge request.
    pub description: Option<String>,
    /// The state of the merge request.
    pub state: MergeRequestState,
    /// When the merge request was created.
    pub created_at: DateTime<Utc>,
    /// When the merge request was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the merge request was merged.
    pub merged_at: Option<DateTime<Utc>>,
    /// When the merge request was closed.
    pub closed_at: Option<DateTime<Utc>>,
    /// The user that merged the merge request.
    pub merged_by: Option<UserBasic>,
    /// The user that closed the merge request.
    pub closed_by: Option<UserBasic>,
    /// The target branch of the merge request.
    pub target_branch: String,
    /// The source branch of the merge request.
    pub source_branch: String,
    /// The number of upvotes for the merge request.
    pub upvotes: u64,
    /// The number of downvotes against the merge request.
    pub downvotes: u64,
    /// The author of the merge request.
    pub author: UserBasic,
    /// The assignee of the merge request.
    pub assignee: Option<UserBasic>,
    /// The assignees of the merge request.
    pub assignees: Option<Vec<UserBasic>>,
    /// The ID of the project hosting the source branch.
    pub source_project_id: ProjectId,
    /// The ID of the project hosting the target branch.
    pub target_project_id: ProjectId,
    /// The labels attached to the merge request.
    pub labels: Vec<String>,
    /// Whether the merge request is a work-in-progress or not.
    pub work_in_progress: bool,
    /// Whether the merge request allows a maintainer to collaborate.
    pub allow_collaboration: Option<bool>,
    /// Whether the merge request allows a maintainer to push (deprecated).
    pub allow_maintainer_to_push: Option<bool>,
    /// The milestone of the merge request.
    pub milestone: Option<Milestone>,
    /// Whether to squash commits on merge.
    pub squash: bool,
    /// Whether the merge request will be merged once all pipelines succeed or not.
    pub merge_when_pipeline_succeeds: bool,
    /// The status of the merge request.
    pub merge_status: MergeStatus,
    /// The object ID of the head of the source branch.
    ///
    /// This is `None` if the source branch has been deleted.
    pub sha: Option<ObjectId>,
    /// The commits used to construct the merge request diffs.
    pub diff_refs: Option<DiffRefs>,
    /// Description of error if MR failed to merge.
    pub merge_error: Option<String>,
    /// Whether a rebase is in progress.
    pub rebase_in_progress: Option<bool>,
    /// The object ID of the commit which merged the merge request.
    pub merge_commit_sha: Option<ObjectId>,
    /// The object ID of the merge request squash commit.
    pub squash_commit_sha: Option<ObjectId>,
    /// Whether the current user is subscribed or not.
    /// GitLab does not include this in responses with lists of merge requests but
    /// does on an individual merge request.
    pub subscribed: Option<bool>,
    /// Time estimates.
    pub time_stats: IssuableTimeStats,
    /// Whether or not all blocking discussions are resolved.
    pub blocking_discussions_resolved: bool,
    /// The number of paths changed by the merge request.
    ///
    /// This is an integer suffixed by `+` if there are more files changed than some threshold
    /// (probably determined by a timeout).
    pub changes_count: Option<String>,
    /// The number of comments on the merge request.
    pub user_notes_count: u64,
    /// Whether the discussion has been locked.
    pub discussion_locked: Option<bool>,
    /// Whether the merge request should be deleted or not (set by the merger).
    pub should_remove_source_branch: Option<bool>,
    /// Whether the merge request should be deleted or not (set by the author).
    pub force_remove_source_branch: Option<bool>,
    /// Whether the merge request currently has conflicts with the target branch.
    pub has_conflicts: bool,
    /// Information about current user's access to the merge request.
    pub user: Option<MergeRequestUser>,
    /// The URL of the merge request.
    pub web_url: String,
    /// Basic pipeline information for the MR.
    pub pipeline: Option<PipelineBasic>,
}

/// A merge request with changes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestChanges {
    /// The ID of the merge request.
    pub id: MergeRequestId,
    /// The user-visible ID of the merge request.
    pub iid: MergeRequestInternalId,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The title of the merge request.
    pub title: String,
    /// The description of the merge request.
    pub description: Option<String>,
    /// The state of the merge request.
    pub state: MergeRequestState,
    /// When the merge request was created.
    pub created_at: DateTime<Utc>,
    /// When the merge request was last updated.
    pub updated_at: DateTime<Utc>,
    /// When the merge request was merged.
    pub merged_at: Option<DateTime<Utc>>,
    /// When the merge request was closed.
    pub closed_at: Option<DateTime<Utc>>,
    /// The user that merged the merge request.
    pub merged_by: Option<UserBasic>,
    /// The user that closed the merge request.
    pub closed_by: Option<UserBasic>,
    /// The target branch of the merge request.
    pub target_branch: String,
    /// The source branch of the merge request.
    pub source_branch: String,
    /// The number of upvotes for the merge request.
    pub upvotes: u64,
    /// The number of downvotes against the merge request.
    pub downvotes: u64,
    /// The author of the merge request.
    pub author: UserBasic,
    /// The assignee of the merge request.
    pub assignee: Option<UserBasic>,
    /// The assignees of the merge request.
    pub assignees: Option<Vec<UserBasic>>,
    /// The ID of the project hosting the source branch.
    pub source_project_id: ProjectId,
    /// The ID of the project hosting the target branch.
    pub target_project_id: ProjectId,
    /// The labels attached to the merge request.
    pub labels: Vec<String>,
    /// Whether the merge request is a work-in-progress or not.
    pub work_in_progress: bool,
    /// Whether the merge request allows a maintainer to collaborate.
    pub allow_collaboration: Option<bool>,
    /// Whether the merge request allows a maintainer to push (deprecated).
    pub allow_maintainer_to_push: Option<bool>,
    /// The milestone of the merge request.
    pub milestone: Option<Milestone>,
    /// Whether to squash commits on merge.
    pub squash: bool,
    /// Whether the merge request will be merged once all jobs succeed or not.
    pub merge_when_pipeline_succeeds: bool,
    /// The status of the merge request.
    pub merge_status: MergeStatus,
    /// The object ID of the head of the source branch.
    ///
    /// This is `None` if the source branch has been deleted.
    pub sha: Option<ObjectId>,
    /// The commits used to construct the merge request diffs.
    pub diff_refs: Option<DiffRefs>,
    /// Description of error if MR failed to merge.
    pub merge_error: Option<String>,
    /// Whether a rebase is in progress.
    pub rebase_in_progress: Option<bool>,
    /// The object ID of the commit which merged the merge request.
    pub merge_commit_sha: Option<ObjectId>,
    /// The object ID of the merge request squash commit.
    pub squash_commit_sha: Option<ObjectId>,
    /// GitLab does not include this in responses with lists of merge requests but
    /// does on an individual merge request.
    pub subscribed: Option<bool>,
    /// Time estimates.
    pub time_stats: IssuableTimeStats,
    /// Whether or not all blocking discussions are resolved.
    pub blocking_discussions_resolved: bool,
    /// The number of paths changed by the merge request.
    pub changes_count: Option<String>,
    /// The number of comments on the merge request.
    pub user_notes_count: u64,
    /// Whether the discussion has been locked.
    pub discussion_locked: Option<bool>,
    /// Whether the merge request should be deleted or not (set by the merger).
    pub should_remove_source_branch: Option<bool>,
    /// Whether the merge request should be deleted or not (set by the author).
    pub force_remove_source_branch: Option<bool>,
    /// Whether the merge request currently has conflicts with the target branch.
    pub has_conflicts: bool,
    /// Information about current user's access to the merge request.
    pub user: MergeRequestUser,
    /// The URL of the merge request.
    pub web_url: String,
    /// Basic pipeline information for the MR.
    pub pipeline: Option<PipelineBasic>,
    pub changes: Vec<RepoDiff>,
}

impl From<MergeRequestChanges> for MergeRequest {
    fn from(mr: MergeRequestChanges) -> Self {
        MergeRequest {
            id: mr.id,
            iid: mr.iid,
            project_id: mr.project_id,
            title: mr.title,
            description: mr.description,
            state: mr.state,
            created_at: mr.created_at,
            updated_at: mr.updated_at,
            merged_at: mr.merged_at,
            closed_at: mr.closed_at,
            merged_by: mr.merged_by,
            closed_by: mr.closed_by,
            target_branch: mr.target_branch,
            source_branch: mr.source_branch,
            upvotes: mr.upvotes,
            downvotes: mr.downvotes,
            author: mr.author,
            assignee: mr.assignee,
            assignees: mr.assignees,
            source_project_id: mr.source_project_id,
            target_project_id: mr.target_project_id,
            labels: mr.labels,
            work_in_progress: mr.work_in_progress,
            allow_collaboration: mr.allow_collaboration,
            allow_maintainer_to_push: mr.allow_maintainer_to_push,
            milestone: mr.milestone,
            squash: mr.squash,
            merge_when_pipeline_succeeds: mr.merge_when_pipeline_succeeds,
            merge_status: mr.merge_status,
            sha: mr.sha,
            diff_refs: mr.diff_refs,
            merge_error: mr.merge_error,
            rebase_in_progress: mr.rebase_in_progress,
            merge_commit_sha: mr.merge_commit_sha,
            squash_commit_sha: mr.squash_commit_sha,
            subscribed: mr.subscribed,
            time_stats: mr.time_stats,
            blocking_discussions_resolved: mr.blocking_discussions_resolved,
            changes_count: mr.changes_count,
            user_notes_count: mr.user_notes_count,
            discussion_locked: mr.discussion_locked,
            should_remove_source_branch: mr.should_remove_source_branch,
            force_remove_source_branch: mr.force_remove_source_branch,
            has_conflicts: mr.has_conflicts,
            user: Some(mr.user),
            web_url: mr.web_url,
            pipeline: mr.pipeline,
        }
    }
}

/// param to create a merge request.
#[derive(Serialize, Deserialize, Builder, Debug, Clone, Default)]
#[builder(default)]
#[builder(field(private))]
#[builder(setter(into, strip_option))]
pub struct CreateMergeRequestParams {
    /// The source branch on source project
    pub source_branch: String,
    /// The target branch
    pub target_branch: String,
    /// Title of MR
    pub title: String,
    /// assignee user ID
    pub assignee_id: Option<UserId>,
    /// The ID of the user(s) to assign the MR to.
    /// Set to 0 or provide an empty value to unassign all assignees.
    pub assignee_ids: Option<Vec<UserId>>,
    /// Description of MR
    pub description: Option<String>,
    /// The target project (numeric id) if different from source project
    pub target_project_id: Option<ProjectId>,
    /// Labels for MR as a comma-separated list
    pub labels: Option<String>,
    /// The global ID of a milestone
    pub milestone_id: Option<MilestoneId>,
    /// Flag indicating if a merge request should remove the source branch when merging
    pub remove_source_branch: Option<bool>,
    /// Allow commits from members who can merge to the target branch
    pub allow_collaboration: Option<bool>,
    /// Squash commits into a single commit when merging
    pub squash: Option<bool>,
}

impl CreateMergeRequestParams {
    pub fn builder() -> CreateMergeRequestParamsBuilder {
        CreateMergeRequestParamsBuilder::default()
    }
}

/// Type-safe SSH key ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SshKeyId(u64);
impl_id!(SshKeyId);

/// An uploaded SSH key.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SshKey {
    /// The ID of the SSH key.
    pub id: SshKeyId,
    /// The title of the key.
    pub title: String,
    /// The public half of the SSH key.
    pub key: String,
    /// When the key was created.
    pub created_at: DateTime<Utc>,
    /// Whether the key may push to repositories or not.
    pub can_push: bool,
}

/// An uploaded SSH key with its owner.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SshKeyWithUser {
    /// The ID of the SSH key.
    pub id: SshKeyId,
    /// The title of the key.
    pub title: String,
    /// The public half of the SSH key.
    pub key: String,
    /// When the key was created.
    pub created_at: DateTime<Utc>,
    /// The user associated with the SSH key.
    pub user: UserPublic,
}

/// The entities a note may be added to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteType {
    /// A note on a commit.
    Commit,
    /// A note on an issue.
    Issue,
    /// A note on a merge request.
    MergeRequest,
    /// A note on a snippet.
    Snippet,
}
enum_serialize!(NoteType -> "note type",
    Commit => "Commit",
    Issue => "Issue",
    MergeRequest => "MergeRequest",
    Snippet => "Snippet",
);

/// The various types a note can have
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscussionNoteType {
    /// A note in a standard discussion
    DiscussionNote,
    /// A note attached to a diff
    DiffNote,
}

enum_serialize!(DiscussionNoteType -> "discussion note type",
    DiscussionNote => "DiscussionNote",
    DiffNote => "DiffNote",
);

/// The ID of an entity a note is attached to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteableId {
    /// The ID of the commit for a commit note.
    Commit(ObjectId),
    /// The ID of the issue for an issue note.
    Issue(IssueId),
    /// The ID of the merge request for a merge request note.
    MergeRequest(MergeRequestId),
    /// The ID of the snippet for a snippet note.
    Snippet(SnippetId),
}

/// The internal ID of an entity a note is attached to (internal to a project).
/// GitLab only has this for notes attached to issues and merge requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteableInternalId {
    /// The internal ID of the issue for an issue note.
    Issue(IssueInternalId),
    /// The internal ID of the merge request for a merge request note.
    MergeRequest(MergeRequestInternalId),
}

/// Type-safe note (comment) ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NoteId(u64);
impl_id!(NoteId);

/// A note can be attached to text or an image
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotePositionType {
    Text,
    Image,
}

enum_serialize!(NotePositionType -> "note position type",
    Text => "text",
    Image => "image",
);

/// When a note is against a diff, the position of the note
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotePosition {
    /// Base commit in the source branch
    pub base_sha: ObjectId,
    /// SHA referencing the commit in the target branch
    pub start_sha: ObjectId,
    /// The HEAD of the merge request
    pub head_sha: ObjectId,
    /// Whether this note is against text or image
    /// FIXME: image not supported yet.
    pub position_type: NotePositionType,
    /// File path before change
    pub old_path: String,
    /// File path after change
    pub new_path: String,
    /// Line number before the change
    pub old_line: Option<u64>,
    /// Line number after the change
    pub new_line: Option<u64>,
}

/// A comment on an entity.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    /// The ID of the note.
    pub id: NoteId,
    /// The type of the note.
    #[serde(rename = "type")]
    pub note_type: Option<DiscussionNoteType>,
    /// The content of the note.
    pub body: String,
    /// The URL of an attachment to the note.
    pub attachment: Option<String>,
    /// The author of the note.
    pub author: UserBasic,
    /// When the note was created.
    pub created_at: DateTime<Utc>,
    /// When the note was last updated.
    pub updated_at: DateTime<Utc>,
    /// Whether the note can be resolved.
    pub resolvable: bool,
    /// Whether the note has been resolved.
    pub resolved: Option<bool>,
    /// The user that resolved the note.
    pub resolved_by: Option<UserBasic>,
    /// Whether the note was created by a user or in response to an external action.
    ///
    /// System notes include indications that the commit, issue, etc. was referenced elsewhere, a
    /// milestone, assignee, or label change, status chages, and so on.
    pub system: bool,
    // Keep as JSON because its type depends on what `noteable_type` is.
    noteable_id: Value,
    // Keep as JSON because its type depends on what `noteable_type` is.
    noteable_iid: Option<Value>,
    /// The type of entity the note is attached to.
    pub noteable_type: NoteType,
    /// If applicable, the diff data to which the note is attached
    pub position: Option<NotePosition>,
}

impl Note {
    /// The ID of the entity the note is attached to.
    pub fn noteable_id(&self) -> Option<NoteableId> {
        match self.noteable_type {
            NoteType::Commit => {
                self.noteable_id
                    .as_str()
                    .map(|id| NoteableId::Commit(ObjectId::new(id)))
            },
            NoteType::Issue => {
                self.noteable_id
                    .as_u64()
                    .map(|id| NoteableId::Issue(IssueId::new(id)))
            },
            NoteType::MergeRequest => {
                self.noteable_id
                    .as_u64()
                    .map(|id| NoteableId::MergeRequest(MergeRequestId::new(id)))
            },
            NoteType::Snippet => {
                self.noteable_id
                    .as_u64()
                    .map(|id| NoteableId::Snippet(SnippetId::new(id)))
            },
        }
    }

    /// The internal ID of the entity the note is attached to (internal to a project).
    /// This is available only for notes attached to issues and merge requests.
    pub fn noteable_iid(&self) -> Option<NoteableInternalId> {
        match self.noteable_type {
            NoteType::Commit => None,
            NoteType::Issue => {
                self.noteable_iid
                    .as_ref()
                    .and_then(|value| value.as_u64())
                    .map(|id| NoteableInternalId::Issue(IssueInternalId::new(id)))
            },
            NoteType::MergeRequest => {
                self.noteable_iid
                    .as_ref()
                    .and_then(|value| value.as_u64())
                    .map(|id| NoteableInternalId::MergeRequest(MergeRequestInternalId::new(id)))
            },
            NoteType::Snippet => None,
        }
    }
}

/// A threaded discussion
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Discussion {
    /// The discussion ID, a SHA hash
    pub id: ObjectId,
    /// True if the discussion only holds one note.
    pub individual_note: bool,
    /// The discussion notes
    pub notes: Vec<Note>,
}

/// Type-safe award ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct AwardId(u64);
impl_id!(AwardId);

/// An ID of an entity which may receive an award.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwardableId {
    /// The ID of an awarded issue.
    Issue(IssueId),
    /// The ID of an awarded merge request.
    MergeRequest(MergeRequestId),
    /// The ID of an awarded snippet.
    Snippet(SnippetId),
    /// The ID of an awarded note.
    Note(NoteId),
}

/// The entities which may be awarded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwardableType {
    /// An award on an issue.
    Issue,
    /// An award on a merge request.
    MergeRequest,
    /// An award on a snippet.
    Snippet,
    /// An award on a note.
    Note,
}
enum_serialize!(AwardableType -> "awardable type",
    Issue => "Issue",
    MergeRequest => "MergeRequest",
    Snippet => "Snippet",
    Note => "Note",
);

/// An awarded emoji on an entity.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AwardEmoji {
    /// The ID of the award.
    pub id: AwardId,
    /// The name of the awarded emoji.
    pub name: String,
    /// The user which created the award.
    pub user: UserBasic,
    /// When the award was created.
    pub created_at: DateTime<Utc>,
    /// When the award was last updated.
    pub updated_at: DateTime<Utc>,
    awardable_id: u64,
    /// The type of entity that is awarded.
    pub awardable_type: AwardableType,
}

impl AwardEmoji {
    /// The ID of the entity the award is attached to.
    pub fn awardable_id(&self) -> AwardableId {
        match self.awardable_type {
            AwardableType::Issue => AwardableId::Issue(IssueId::new(self.awardable_id)),
            AwardableType::MergeRequest => {
                AwardableId::MergeRequest(MergeRequestId::new(self.awardable_id))
            },
            AwardableType::Snippet => AwardableId::Snippet(SnippetId::new(self.awardable_id)),
            AwardableType::Note => AwardableId::Note(NoteId::new(self.awardable_id)),
        }
    }
}

/// The type of line commented on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    /// An added line was commented on.
    New,
    /// An deleted line was commented on.
    Old,
}
enum_serialize!(LineType -> "line type",
    New => "new",
    Old => "old",
);

/// A note on a commit diff.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitNote {
    /// The content of the note.
    pub note: String,
    /// The path of the file commented on.
    pub path: Option<String>,
    /// The line of the file commented on.
    pub line: Option<u64>,
    /// The type of the line commented on.
    pub line_type: Option<LineType>,
    /// The author of the note.
    pub author: UserBasic,
    /// When the note was created.
    pub created_at: DateTime<Utc>,
}

/// Type-safe commit status ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CommitStatusId(u64);
impl_id!(CommitStatusId);

/// States for commit statuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusState {
    /// The check was created.
    Created,
    /// The check is queued.
    Pending,
    /// The check is currently running.
    Running,
    /// The check succeeded.
    Success,
    /// The check failed.
    Failed,
    /// The check was canceled.
    Canceled,
    /// The check was skipped.
    Skipped,
    /// The check is waiting for manual action.
    Manual,
}

enum_serialize!(StatusState -> "status state",
    Created => "created",
    Pending => "pending",
    Running => "running",
    Success => "success",
    Failed => "failed",
    Canceled => "canceled",
    Skipped => "skipped",
    Manual => "manual",
);

/// A status of a commit.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitStatus {
    /// The ID of the commit status.
    pub id: CommitStatusId,
    /// The object ID of the commit this status is for.
    pub sha: ObjectId,
    #[serde(rename = "ref")]
    /// The name of the reference the status was created for.
    pub ref_: Option<String>,
    /// The state of the commit status.
    pub status: StatusState,
    /// The name of the commit status.
    pub name: String,
    /// The URL associated with the commit status.
    pub target_url: Option<String>,
    /// The description of the commit status.
    pub description: Option<String>,
    /// When the commit status was created.
    pub created_at: DateTime<Utc>,
    /// When the commit status started.
    pub started_at: Option<DateTime<Utc>>,
    /// When the commit status completed.
    pub finished_at: Option<DateTime<Utc>>,
    /// Whether the commit status is allowed to fail.
    pub allow_failure: bool,
    pub coverage: Option<f64>,
    /// The author of the commit status.
    pub author: UserBasic,
}

/// Type-safe environment ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvironmentId(u64);
impl_id!(EnvironmentId);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Environment {
    pub id: EnvironmentId,
    pub name: String,
    pub slug: String,
    pub external_url: Option<String>,
    pub state: Option<String>,
    pub last_deployment: Option<Deployment>,
}

/// Type-safe deployment ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeploymentId(u64);
impl_id!(DeploymentId);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployment {
    pub id: DeploymentId,
    pub iid: u64,
    pub r#ref: String,
    pub sha: String,
    pub created_at: String,
    pub status: Option<String>,
    pub user: UserBasic,
    pub deployable: Deployable,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployable {
    pub commit: Commit,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub id: Option<String>,
    pub short_id: Option<String>,
    pub created_at: Option<String>,
    pub title: Option<String>,
}

/// The target of an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTargetType {
    /// An event targeted a commit.
    Commit,
    /// An event targeted an issue.
    Issue,
    /// An event targeted a merge request.
    MergeRequest,
    /// An event targeted a snippet.
    Snippet,
    /// An event targeted a project snippet.
    ProjectSnippet,
}
enum_serialize!(EventTargetType -> "event target type",
    Commit => "commit",
    Issue => "issue",
    MergeRequest => "merge_request",
    Snippet => "snippet",
    ProjectSnippet => "project_snippet",
);

/// The ID of an event target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventTargetId {
    /// The object ID of a commit event target.
    Commit(ObjectId),
    /// The ID of an issue event target.
    Issue(IssueId),
    /// The ID of a merge request event target.
    MergeRequest(MergeRequestId),
    /// The ID of a snippet event target.
    Snippet(SnippetId),
}

/// An event on a project.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    /// The title of the event.
    pub title: Option<String>,
    /// The ID of the project.
    pub project_id: ProjectId,
    /// The action which triggered the event.
    // FIXME: This should be an enumeration.
    pub action_name: String,
    target_id: Value,
    /// The type of the event target.
    pub target_type: EventTargetType,
    /// The ID of the author of the event.
    pub author_id: UserId,
    pub data: Option<Value>,
    /// The title of the target.
    pub target_title: String,
    /// When the event was created.
    pub created_at: DateTime<Utc>,
    pub note: Option<Note>,
    /// The author of the event.
    pub author: Option<UserBasic>,
    /// The handle of the author.
    pub author_username: Option<String>,
}

impl Event {
    /// The ID of an event's target.
    pub fn target_id(&self) -> Option<EventTargetId> {
        match self.target_type {
            EventTargetType::Commit => {
                self.target_id
                    .as_str()
                    .map(|id| EventTargetId::Commit(ObjectId(id.into())))
            },
            EventTargetType::Issue => {
                self.target_id
                    .as_u64()
                    .map(|id| EventTargetId::Issue(IssueId(id)))
            },
            EventTargetType::MergeRequest => {
                self.target_id
                    .as_u64()
                    .map(|id| EventTargetId::MergeRequest(MergeRequestId(id)))
            },
            EventTargetType::Snippet => {
                self.target_id
                    .as_u64()
                    .map(|id| EventTargetId::Snippet(SnippetId(id)))
            },
            EventTargetType::ProjectSnippet => {
                self.target_id
                    .as_u64()
                    .map(|id| EventTargetId::Snippet(SnippetId(id)))
            },
        }
    }
}

/// The kinds of namespaces supported by Gitlab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceKind {
    /// A user namespace.
    User,
    /// A group namespace.
    Group,
}
enum_serialize!(NamespaceKind -> "namespace kind",
    User => "user",
    Group => "group",
);

/// The ID of a namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceId {
    /// A user namespace ID.
    User(UserId),
    /// A group namespace ID.
    Group(GroupId),
}

/// An entity which can own projects.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Namespace {
    id: u64,
    /// The URL of the namespace.
    pub path: String,
    /// The name of the namespace.
    pub name: String,
    /// The kind of the namespace.
    pub kind: NamespaceKind,
    pub full_path: String,
    /// Number of members in the namespace and its descendants.
    ///
    /// Only available when talking to GitLab as a user that can admin the namespace.
    pub members_count_with_descendants: Option<u64>,
    /// The URL of the user's avatar if namespace is a user.
    pub avatar_url: Option<String>,
    /// The URL to the namespace page (user or group).
    pub web_url: String,
}

impl Namespace {
    /// The ID of the namespace.
    pub fn id(&self) -> NamespaceId {
        match self.kind {
            NamespaceKind::User => NamespaceId::User(UserId(self.id)),
            NamespaceKind::Group => NamespaceId::Group(GroupId(self.id)),
        }
    }
}

/// Type-safe runner ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RunnerId(u64);
impl_id!(RunnerId);

/// A Gitlab CI runner.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runner {
    /// The ID of the runner.
    pub id: RunnerId,
    /// The description of the runner.
    pub description: Option<String>,
    /// Whether the runner is active or not.
    pub active: bool,
    /// Whether the runner is shared or not.
    pub is_shared: bool,
    /// The name of the runner.
    pub name: Option<String>,
}

/// An uploaded artifact from a job.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobArtifactFile {
    /// The name of the artifact.
    pub filename: String,
    /// The size (in bytes) of the artifact.
    pub size: usize,
}

/// Type-safe job ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct JobId(u64);
impl_id!(JobId);

/// Information about a job in Gitlab CI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    /// The ID of the job.
    pub id: JobId,
    /// The status of the job.
    pub status: StatusState,
    pub stage: String,
    /// The name of the job.
    pub name: String,
    #[serde(rename = "ref")]
    /// The name of the reference that was tested.
    pub ref_: Option<String>,
    pub tag: bool,
    pub coverage: Option<f32>,
    /// When the job was created or marked as pending.
    pub created_at: DateTime<Utc>,
    /// When the job was started.
    pub started_at: Option<DateTime<Utc>>,
    /// When the job completed.
    pub finished_at: Option<DateTime<Utc>>,
    /// The user which ran the job.
    pub user: Option<User>,
    /// The artifact file uploaded from the job.
    pub artifacts_file: Option<JobArtifactFile>,
    /// The commit the job tested.
    pub commit: RepoCommit,
    /// The runner which ran the job.
    pub runner: Option<Runner>,
    /// The pipeline the job belongs to.
    pub pipeline: PipelineBasic,
}

/// Type-safe pipeline ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineId(u64);
impl_id!(PipelineId);

/// Information about a pipeline in Gitlab CI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineBasic {
    /// The ID of the pipeline.
    pub id: PipelineId,
    #[serde(rename = "ref")]
    /// The name of the reference that was tested.
    pub ref_: Option<String>,
    /// The object ID that was tested.
    pub sha: ObjectId,
    /// The status of the pipeline.
    pub status: StatusState,
    /// When the pipeline was created.
    pub created_at: Option<DateTime<Utc>>,
    /// When the pipeline was last updated.
    pub updated_at: Option<DateTime<Utc>>,
    /// The URL to the pipeline page.
    pub web_url: String,
}

/// More information about a pipeline in Gitlab CI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipeline {
    /// The ID of the pipeline.
    pub id: PipelineId,
    /// The object ID that was tested.
    pub sha: ObjectId,
    #[serde(rename = "ref")]
    /// The name of the reference that was tested.
    pub ref_: Option<String>,
    /// The status of the pipeline.
    pub status: StatusState,
    /// The URL to the pipeline page.
    pub web_url: String,
    /// FIXME What are the semantics of this field?
    pub before_sha: Option<ObjectId>,
    /// Was this pipeline triggered by a tag.
    pub tag: bool,
    /// Error returned by the parser of `gitlab-ci.yml`, if any.
    pub yaml_errors: Option<String>,
    /// When the pipeline was created.
    pub created_at: Option<DateTime<Utc>>,
    /// When the pipeline was last updated.
    pub updated_at: Option<DateTime<Utc>>,
    /// When the pipeline began running.
    pub started_at: Option<DateTime<Utc>>,
    /// When the pipeline completed.
    pub finished_at: Option<DateTime<Utc>>,
    /// FIXME What are the semantics of this field?
    pub committed_at: Option<DateTime<Utc>>,
    /// Duration of pipeline in seconds.
    pub duration: Option<u64>,
    /// FIXME What are the semantics of this field?
    pub coverage: Option<String>,
    /// The user who triggered this pipeline.
    pub user: UserBasic,
    /// FIXME: What are the semantics of this field?
    /// See <https://gitlab.com/gitlab-org/gitlab-foss/blob/master/app/serializers/detailed_status_entity.rb>.
    pub detailed_status: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipelineVariableType {
    EnvVar,
    File,
}
enum_serialize!(PipelineVariableType -> "variable type",
    EnvVar => "env_var",
    File => "file",
);

impl Default for PipelineVariableType {
    fn default() -> Self {
        PipelineVariableType::EnvVar
    }
}

/// A pipeline variable.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipelineVariable {
    /// Name of the variable.
    pub key: String,
    /// Value of the variable.
    pub value: String,

    /// Type of the variable (eg. `env_var`).
    #[serde(default)]
    pub variable_type: PipelineVariableType,
}

/// Type-safe label event ID.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LabelEventId(u64);
impl_id!(LabelEventId);

/// A resource label event
///
/// Note that resource events were added in Gitlab 11.2.  Any labels added or
/// removed before then will not be returned by the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceLabelEvent {
    /// The ID for the label event
    pub id: LabelEventId,
    pub user: UserBasic,
    pub created_at: DateTime<Utc>,
    /// The merge request id, or issue id (depending on the value of resource_type)
    resource_id: u64,
    /// Either "MergeRequest" or "Issue"
    resource_type: String,
    /// The label may be None if the label has been deleted.
    pub label: Option<EventLabel>,
    pub action: String,
}

impl ResourceLabelEvent {
    /// Returns the id of the merge request or issue that this event is from
    pub fn event_target(&self) -> Option<ResourceLabelEventTarget> {
        match self.resource_type.as_ref() {
            "MergeRequest" => {
                Some(ResourceLabelEventTarget::MergeRequest(MergeRequestId::new(
                    self.resource_id,
                )))
            },
            "Issue" => {
                Some(ResourceLabelEventTarget::Issue(IssueId::new(
                    self.resource_id,
                )))
            },
            _ => None,
        }
    }
}

/// The type of object that on which the resource label event was created
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceLabelEventTarget {
    /// The ID of an issue event target.
    Issue(IssueId),
    /// The ID of a merge request event target.
    MergeRequest(MergeRequestId),
}

/// An label on a project.
///
/// This is like [Label], except that it doesn't have all the same fields
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventLabel {
    /// The Id of the label.
    pub id: LabelId,
    /// The name of the label.
    pub name: String,
    /// The color of the label.
    pub color: LabelColor,
    /// The description of the label.
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RepoFile {
    pub file_path: String,
    pub branch: String,
}

#[derive(Debug, Clone)]
pub enum ProjectFeatures {
    Issues(FeatureVisibilityLevel),
    Repository(FeatureVisibilityLevel),
    MergeRequests(FeatureVisibilityLevel),
    Builds(FeatureVisibilityLevel),
    Wiki(FeatureVisibilityLevel),
    Snippets(FeatureVisibilityLevel),
}

impl ProjectFeatures {
    pub fn name(&self) -> &'static str {
        match self {
            ProjectFeatures::Issues(_) => "issues_access_level",
            ProjectFeatures::Repository(_) => "repository_access_level",
            ProjectFeatures::MergeRequests(_) => "merge_requests_access_level",
            ProjectFeatures::Builds(_) => "builds_access_level",
            ProjectFeatures::Wiki(_) => "wiki_access_level",
            ProjectFeatures::Snippets(_) => "snippets_access_level",
        }
    }

    pub fn access_level(&self) -> &FeatureVisibilityLevel {
        match self {
            ProjectFeatures::Issues(a)
            | ProjectFeatures::Repository(a)
            | ProjectFeatures::MergeRequests(a)
            | ProjectFeatures::Builds(a)
            | ProjectFeatures::Wiki(a)
            | ProjectFeatures::Snippets(a) => a,
        }
    }
}

/// Params for group creation.
///
/// Used with `create_group` method (see [doc here](../gitlab/struct.Gitlab.html#method.create_group))
#[derive(Debug, Clone, Builder, Serialize, Default)]
#[builder(default)]
#[builder(field(private))]
#[builder(setter(into, strip_option))]
pub struct CreateGroupParams {
    /// The group name
    #[builder(setter(skip))]
    pub(crate) name: Option<String>,
    /// The path of the group
    #[builder(setter(skip))]
    pub(crate) path: Option<String>,
    /// The group description
    description: Option<String>,
    /// The group visibility level, can be `private`, `internal` or `public`
    visibility: Option<VisibilityLevel>,
    /// Prevent sharing a project with another group within this group
    share_with_group_lock: Option<bool>,
    /// Require all users in this group to setup two-factor authentication
    require_two_factor_authentication: Option<bool>,
    /// Time before two-factor authentication is enforced
    two_factor_grace_period: Option<u64>,
    /// Determine if developers can create projects in the group
    #[builder(setter(name = "_project_creation_level"))]
    project_creation_level: Option<String>,
    /// Default to Auto Devops pipeline for all projects within this group
    auto_devops_enabled: Option<bool>,
    /// Role allowed to create subgroups
    #[builder(setter(name = "_subgroup_creation_level"))]
    subgroup_creation_level: Option<String>,
    /// Disable email notification
    emails_disabled: Option<bool>,
    /// Disable the capability of a group from getting mentioned
    mentions_disabled: Option<bool>,
    /// Enable/disable Large File Storage (LFS) for the projects in this group
    lfs_enabled: Option<bool>,
    /// Allow users to request membership
    request_access_enabled: Option<bool>,
    /// The parent group ID for creating a nesting group
    parent_id: Option<GroupId>,
    /// [Gitlab Starter and higher] pipeline minutes quota for this group
    shared_runners_minutes_limit: Option<u64>,
    /// [Gitlab Starter and higher] extra pipeline minutes quota for this group
    extra_shared_runners_minutes_limit: Option<u64>,
}

impl CreateGroupParams {
    pub fn builder() -> CreateGroupParamsBuilder {
        CreateGroupParamsBuilder::default()
    }
}

impl CreateGroupParamsBuilder {
    pub fn project_creation_level(&mut self, level: AccessLevel) -> &mut Self {
        self.project_creation_level = Some(Some(level.as_str().to_string()));
        self
    }

    pub fn subgroup_creation_level(&mut self, level: AccessLevel) -> &mut Self {
        self.subgroup_creation_level = Some(Some(level.as_str().to_string()));
        self
    }
}

/// Params for project creation.
///
/// Used with `create_project` method (see [doc here](../gitlab/struct.Gitlab.html#method.create_project))
#[derive(Debug, Clone, Builder, Serialize, Default)]
#[builder(default)]
#[builder(field(private))]
#[builder(setter(into, strip_option))]
pub struct CreateProjectParams {
    /// The name of the project
    #[builder(setter(skip))]
    pub(crate) name: Option<String>,
    /// The path of the project
    #[builder(setter(skip))]
    pub(crate) path: Option<String>,
    /// Namespace for the new projects (defaults to current user namespaces)
    namespace_id: Option<u64>,
    /// `master` by default
    default_branch: Option<String>,
    /// Short project description
    description: Option<String>,
    /// One of `disabled`, `private`, `enabled`
    issues_access_level: Option<FeatureVisibilityLevel>,
    /// One of `disabled`, `private`, `enabled`
    repository_access_level: Option<FeatureVisibilityLevel>,
    /// One of `disabled`, `private`, `enabled`
    merge_requests_access_level: Option<FeatureVisibilityLevel>,
    /// One of `disabled`, `private`, `enabled`
    builds_access_level: Option<FeatureVisibilityLevel>,
    /// One of `disabled`, `private`, `enabled`
    wiki_access_level: Option<FeatureVisibilityLevel>,
    /// One of `disabled`, `private`, `enabled`
    snippets_access_level: Option<FeatureVisibilityLevel>,
    /// One of `disabled`, `private`, `enabled`, `public`
    pages_access_level: Option<FeatureVisibilityLevel>,
    /// Automatically resolve merge requests diffs discussions on lines changed with a push
    resolve_outdated_diff_discussions: Option<bool>,
    /// Enable container registry for this project
    container_registry_enabled: Option<bool>,
    /// Update the container expiration for this project.
    container_expiration_policy_attributes: Option<Vec<String>>,
    /// Enable shared runners for this project
    shared_runners_enabled: Option<bool>,
    /// Project visibility level
    visibility: Option<VisibilityLevel>,
    /// URL to import repository from
    #[builder(setter(name = "_import_url"))]
    import_url: Option<String>,
    /// If `true` jobs can be viewed by non-project members
    public_builds: Option<bool>,
    /// Set wether merge requests can only be merged with successful jobs
    only_allow_merge_if_pipeline_succeeds: Option<bool>,
    /// Set wether merge requests can only be merged when all the discussions are resolved
    only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    /// Set the merge method used
    merge_method: Option<MergeMethod>,
    /// Set wether auto-closing referenced issues on default branch
    autoclose_referenced_issues: Option<bool>,
    /// Enable LFS
    lfs_enabled: Option<bool>,
    /// Allow user to request member access
    request_access_enabled: Option<bool>,
    /// The list of tags for a project
    tag_list: Option<Vec<String>>,
    /// Image file for avatar of the project
    // TODO: Handle the mixed type for avatar data
    #[builder(setter(skip))]
    avatar: Option<Vec<u8>>,
    /// Show link to create/view merge request wehen pushing from the command line
    printing_merge_request_link_enabled: Option<bool>,
    /// The git strategy. Defaults to fetch
    build_git_strategy: Option<BuildGitStrategy>,
    /// The maximum amount of time in minutes a job is allowed to run
    build_timeout: Option<u64>,
    /// Auto-cancel pending pipeline
    auto_cancel_pending_pipelines: Option<bool>,
    /// Test coverage parsing
    build_coverage_regex: Option<String>,
    /// The path to CI config file
    ci_config_path: Option<String>,
    /// Enable Auto DevOps for this project
    auto_devops_enabled: Option<bool>,
    /// Auto Deploy strategy (`continuous`, `manual` or `timed_incremental`)
    auto_devops_deploy_strategy: Option<String>,
    /// [Gitlab starter and higher]
    /// Which storage shard the repository is on. Available only to admins
    repository_storage: Option<String>,
    /// [Gitlab starter and higher]
    /// How many approvers should approve merge requests by default
    approvals_before_merge: Option<u64>,
    /// [Gitlab starter and higher]
    /// The classification label for the project
    external_authorization_classification_label: Option<String>,
    /// [Gitlab starter and higher] Enables pull mirroring in a project
    mirror: Option<bool>,
    /// [Gitlab starter and higher]
    /// Pull mirroring triggers builds
    mirror_trigger_builds: Option<String>,
    /// `false` by default
    initialize_with_readme: Option<bool>,
    /// When used without `use_custom_template`, name of a built-in project template.
    /// When used with `use_custom_template`, name of a custom project template
    template_name: Option<String>,
    /// [Gitlab silver and higher]
    /// When used with `use_custom_template`, project ID of a custom project template.
    /// This is preferable to using `template_name` since `template_name` may be ambiguous
    template_project_id: Option<u64>,
    /// [Gitlab silver and higher]
    /// When used with `use_custom_template`, project ID of a custom project template.
    /// This is preferable to using `template_name` since `template_name` may be ambiguous
    use_custom_template: Option<bool>,
    /// [Gitlab silver and higher]
    /// For group-level custom templates, specifies ID of group from which all the custom project templates are sourced.
    /// Leave empty for instance-level templates. Requires `use_custom_template` to be true
    group_with_project_templates_id: Option<u64>,
    /// [Gitlab silver and higher]
    /// Enable or disable packages repository feature
    packages_enabled: Option<bool>,
}

impl CreateProjectParams {
    pub fn builder() -> CreateProjectParamsBuilder {
        CreateProjectParamsBuilder::default()
    }
}

impl CreateProjectParamsBuilder {
    pub fn import_url(&mut self, url: Url) -> &mut Self {
        self.import_url = Some(Some(url.as_str().to_string()));
        self
    }
}

/// Merge methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeMethod {
    /// A merge commit is created for every merge,
    /// and merging is allowed as long as there are no conflicts
    Merge,
    /// A merge commit is created for every merge, but merging is possible only if
    /// fast-forward merge is possible
    RebaseMerge,
    /// No merge commit create, all merges are fast-forwarded
    FastForward,
}

enum_serialize!(MergeMethod -> "merge_method",
    Merge => "merge",
    RebaseMerge => "rebase_merge",
    FastForward => "ff",
);

/// Build git strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildGitStrategy {
    Fetch,
    Clone,
}

enum_serialize!(BuildGitStrategy -> "build_git_strategy",
    Fetch => "fetch",
    Clone => "clone",
);

/// Auto devops deply strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoDeployStrategy {
    Continuous,
    Manual,
    TimedIncremental,
}

enum_serialize!(AutoDeployStrategy -> "auto_devops_deploy_strategy",
    Continuous => "continuous",
    Manual => "manual",
    TimedIncremental => "timed_incremental",
);