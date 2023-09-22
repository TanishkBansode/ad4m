use juniper::{
    FieldError, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, GraphQLScalar,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::js_core::JsCoreHandle;

#[derive(Clone)]
pub struct RequestContext {
    pub capability: String,
    pub js_handle: JsCoreHandle,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub did: String,
    #[graphql(name = "directMessageLanguage")]
    pub direct_message_language: Option<String>,
    pub perspective: Option<Perspective>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentSignature {
    pub public_key: String,
    pub signature: String,
}

#[derive(GraphQLObject, Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AgentStatus {
    pub did: Option<String>,
    pub did_document: Option<String>,
    pub error: Option<String>,
    pub is_initialized: bool,
    pub is_unlocked: bool,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Apps {
    pub auth: AuthInfo,
    pub request_id: String,
    pub revoked: Option<bool>,
    pub token: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthInfo {
    pub app_desc: String,
    pub app_icon_path: Option<String>,
    pub app_name: String,
    pub app_url: String,
    pub capabilities: Vec<Capability>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthInfoInput {
    #[graphql(name = "appDesc")]
    pub app_desc: String,
    #[graphql(name = "appDomain")]
    pub app_domain: String,
    #[graphql(name = "appIconPath")]
    pub app_icon_path: Option<String>,
    #[graphql(name = "appName")]
    pub app_name: String,
    #[graphql(name = "appUrl")]
    pub app_url: Option<String>,
    #[graphql(name = "capabilities")]
    pub capabilities: Option<Vec<CapabilityInput>>,
}

#[derive(GraphQLObject, Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub can: Vec<String>,
    pub with: Resource,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityInput {
    pub can: Vec<String>,
    pub with: ResourceInput,
}

#[derive(GraphQLScalar, Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[graphql(transparent)]
// The javascript `Date` as string. pub struct represents date and time as the ISO Date string.
pub struct DateTime(chrono::DateTime<chrono::Utc>);

#[derive(GraphQLObject, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntanglementProof {
    #[graphql(name = "deviceKey")]
    pub device_key: String,
    #[graphql(name = "deviceKeySignedByDid")]
    pub device_key_signed_by_did: String,
    #[graphql(name = "deviceKeyType")]
    pub device_key_type: String,
    #[graphql(name = "did")]
    pub did: String,
    #[graphql(name = "didSignedByDeviceKey")]
    pub did_signed_by_device_key: Option<String>,
    #[graphql(name = "didSigningKeyId")]
    pub did_signing_key_id: String,
}

#[derive(GraphQLInputObject, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntanglementProofInput {
    #[graphql(name = "deviceKey")]
    pub device_key: String,
    #[graphql(name = "deviceKeySignedByDid")]
    pub device_key_signed_by_did: String,
    #[graphql(name = "deviceKeyType")]
    pub device_key_type: String,
    #[graphql(name = "did")]
    pub did: String,
    #[graphql(name = "didSignedByDeviceKey")]
    pub did_signed_by_device_key: String,
    #[graphql(name = "didSigningKeyId")]
    pub did_signing_key_id: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfo {
    pub addon: Option<String>,
    pub message: String,
    pub title: String,
    pub r#type: f64,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionProof {
    pub invalid: Option<bool>,
    pub key: Option<String>,
    pub signature: Option<String>,
    pub valid: Option<bool>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionProofInput {
    pub invalid: Option<bool>,
    pub key: Option<String>,
    pub signature: Option<String>,
    pub valid: Option<bool>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionRendered {
    pub author: String,
    pub data: String,
    pub icon: Icon,
    pub language: LanguageRef,
    pub proof: ExpressionProof,
    pub timestamp: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub code: Option<String>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionCall {
    pub name: String,
    pub parameters_stringified: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InteractionMeta {
    pub label: String,
    pub name: String,
    pub parameters: Vec<InteractionParameter>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InteractionParameter {
    pub name: String,
    pub type_: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageHandle {
    pub address: String,
    pub constructor_icon: Option<Icon>,
    pub icon: Option<Icon>,
    pub name: String,
    pub settings: Option<String>,
    pub settings_icon: Option<Icon>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageMeta {
    pub address: String,
    pub author: String,
    pub description: Option<String>,
    pub name: String,
    pub possible_template_params: Option<Vec<String>>,
    pub source_code_link: Option<String>,
    pub template_applied_params: Option<String>,
    pub template_source_language_address: Option<String>,
    pub templated: Option<bool>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageMetaInput {
    pub description: String,
    pub name: String,
    pub possible_template_params: Option<Vec<String>>,
    pub source_code_link: Option<String>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageRef {
    pub address: String,
    pub name: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub predicate: Option<String>,
    pub source: String,
    pub target: String,
}

#[derive(GraphQLEnum, Debug, Deserialize, Serialize, Clone)]
pub enum LinkStatus {
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "local")]
    Local,
}

//Impl display for LinkStatus
impl std::fmt::Display for LinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LinkStatus::Shared => write!(f, "shared"),
            LinkStatus::Local => write!(f, "local"),
        }
    }
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkExpression {
    pub author: String,
    pub data: Link,
    pub proof: ExpressionProof,
    pub timestamp: String,
    pub status: Option<LinkStatus>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkExpressionInput {
    pub author: String,
    pub data: LinkInput,
    pub proof: ExpressionProofInput,
    pub timestamp: String,
    pub status: Option<LinkStatus>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkExpressionMutations {
    pub additions: Vec<LinkExpression>,
    pub removals: Vec<LinkExpression>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkExpressionUpdated {
    pub new_link: LinkExpression,
    pub old_link: LinkExpression,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkInput {
    pub predicate: Option<String>,
    pub source: String,
    pub target: String,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkMutations {
    pub additions: Vec<LinkInput>,
    pub removals: Vec<LinkExpressionInput>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkQuery {
    pub from_date: Option<DateTime>,
    pub limit: Option<f64>,
    pub predicate: Option<String>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub until_date: Option<DateTime>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Neighbourhood {
    pub link_language: String,
    pub meta: Perspective,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OnlineAgent {
    pub did: String,
    pub status: PerspectiveExpression,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Perspective {
    pub links: Vec<LinkExpression>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveExpression {
    pub author: String,
    pub data: Perspective,
    pub proof: ExpressionProof,
    pub timestamp: String,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveHandle {
    pub name: Option<String>,
    pub neighbourhood: Option<Neighbourhood>,
    pub shared_url: Option<String>,
    pub state: String,
    pub uuid: String,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveInput {
    pub links: Vec<LinkExpressionInput>,
}

#[derive(GraphQLInputObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveUnsignedInput {
    pub links: Vec<LinkInput>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub domain: String,
    pub pointers: Vec<String>,
}

#[derive(GraphQLInputObject, Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceInput {
    pub domain: String,
    pub pointers: Vec<String>,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    pub ad4m_executor_version: String,
    pub is_initialized: bool,
    pub is_unlocked: bool,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SentMessage {
    pub message: PerspectiveExpression,
    pub recipient: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct NeighbourhoodSignalFilter {
    pub perspective: PerspectiveHandle,
    pub signal: PerspectiveExpression,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct PerspectiveLinkFilter {
    pub perspective: PerspectiveHandle,
    pub link: LinkExpression,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveLinkUpdatedFilter {
    pub new_link: LinkExpression,
    pub old_link: LinkExpression,
    pub perspective: PerspectiveHandle,
}

#[derive(GraphQLObject, Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkUpdated {
    pub new_link: LinkExpression,
    pub old_link: LinkExpression,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveStateFilter {
    pub state: String,
    pub perspective: PerspectiveHandle,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum JsResultType<T>
where
    T: std::fmt::Debug + Serialize + 'static,
{
    Ok(T),
    Error(String),
}

impl<T> JsResultType<T>
where
    T: std::fmt::Debug + Serialize + 'static,
{
    pub fn get_graphql_result(self) -> FieldResult<T> {
        match self {
            JsResultType::Ok(result) => Ok(result),
            JsResultType::Error(error) => Err(FieldError::from(error.clone())),
        }
    }
}

// Define the trait with a generic associated type `Value`
pub trait GetValue {
    type Value: Clone + DeserializeOwned + Send + 'static + std::fmt::Debug;
    fn get_value(&self) -> Self::Value;
}

pub trait GetFilter {
    fn get_filter(&self) -> Option<String>;
}

impl GetValue for Option<Apps> {
    type Value = Option<Apps>;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

impl GetFilter for Option<Apps> {
    fn get_filter(&self) -> Option<String> {
        None
    }
}

// Implement the trait for the `NeighbourhoodSignalFilter` struct
impl GetValue for NeighbourhoodSignalFilter {
    type Value = PerspectiveExpression;

    fn get_value(&self) -> Self::Value {
        self.signal.clone()
    }
}

// Implement the trait for the `NeighbourhoodSignalFilter` struct
impl GetFilter for NeighbourhoodSignalFilter {
    fn get_filter(&self) -> Option<String> {
        Some(self.perspective.uuid.clone())
    }
}

// Implement the trait for the `PerspectiveLinkFilter` struct
impl GetValue for PerspectiveLinkFilter {
    type Value = LinkExpression;

    fn get_value(&self) -> Self::Value {
        self.link.clone()
    }
}

// Implement the trait for the `PerspectiveLinkFilter` struct
impl GetFilter for PerspectiveLinkFilter {
    fn get_filter(&self) -> Option<String> {
        Some(self.perspective.uuid.clone())
    }
}

// Implement the trait for the `PerspectiveLinkUpdatedFilter` struct
impl GetValue for PerspectiveLinkUpdatedFilter {
    type Value = LinkUpdated;

    fn get_value(&self) -> Self::Value {
        LinkUpdated {
            new_link: self.new_link.clone(),
            old_link: self.old_link.clone(),
        }
    }
}

// Implement the trait for the `PerspectiveLinkUpdatedFilter` struct
impl GetFilter for PerspectiveLinkUpdatedFilter {
    fn get_filter(&self) -> Option<String> {
        Some(self.perspective.uuid.clone())
    }
}

// Implement the trait for the `PerspectiveStateFilter` struct
impl GetValue for PerspectiveStateFilter {
    type Value = String;

    fn get_value(&self) -> Self::Value {
        self.state.clone()
    }
}

// Implement the trait for the `PerspectiveStateFilter` struct
impl GetFilter for PerspectiveStateFilter {
    fn get_filter(&self) -> Option<String> {
        Some(self.perspective.uuid.clone())
    }
}

// Implement the trait for the `AgentStatus` struct
impl GetValue for AgentStatus {
    type Value = AgentStatus;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

// Implement the trait for the `AgentStatus` struct
impl GetFilter for AgentStatus {
    fn get_filter(&self) -> Option<String> {
        None
    }
}

// Implement the trait for `Agent` struct
impl GetValue for Agent {
    type Value = Agent;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

// Implement the trait for `Agent` struct
impl GetFilter for Agent {
    fn get_filter(&self) -> Option<String> {
        None
    }
}

//Implement the trait for `ExceptionInfo` struct
impl GetValue for ExceptionInfo {
    type Value = ExceptionInfo;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

//Implement the trait for `ExceptionInfo` struct
impl GetFilter for ExceptionInfo {
    fn get_filter(&self) -> Option<String> {
        None
    }
}

//Implement the trait for `PerspectiveHandle` struct
impl GetValue for PerspectiveHandle {
    type Value = PerspectiveHandle;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

//Implement the trait for `PerspectiveHandle` struct
impl GetFilter for PerspectiveHandle {
    fn get_filter(&self) -> Option<String> {
        None
    }
}

//Implement the trait for `String`
impl GetValue for String {
    type Value = String;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

//Implement the trait for `String`
impl GetFilter for String {
    fn get_filter(&self) -> Option<String> {
        None
    }
}

//Implement the trait for `PerspectiveExpression`
impl GetValue for PerspectiveExpression {
    type Value = PerspectiveExpression;

    fn get_value(&self) -> Self::Value {
        self.clone()
    }
}

//Implement the trait for `PerspectiveExpression`
impl GetFilter for PerspectiveExpression {
    fn get_filter(&self) -> Option<String> {
        None
    }
}
