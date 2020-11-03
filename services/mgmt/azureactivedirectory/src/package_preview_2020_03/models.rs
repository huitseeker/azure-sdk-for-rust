#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkPolicyListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkPolicy>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkPolicy {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerTenantId", skip_serializing_if = "Option::is_none")]
    pub owner_tenant_id: Option<String>,
    #[serde(rename = "allTenants", skip_serializing_if = "Option::is_none")]
    pub all_tenants: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tenants: Vec<String>,
    #[serde(rename = "resourceName", skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceBase {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkPolicyUpdateParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmProxyResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}