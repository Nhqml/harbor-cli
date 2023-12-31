/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ConfigurationsResponseScanAllPolicyParameter : The parameters of the policy, the values are dependent on the type of the policy.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationsResponseScanAllPolicyParameter {
    /// The offset in seconds of UTC 0 o'clock, only valid when the policy type is \"daily\"
    #[serde(rename = "daily_time", skip_serializing_if = "Option::is_none")]
    pub daily_time: Option<i32>,
}

impl ConfigurationsResponseScanAllPolicyParameter {
    /// The parameters of the policy, the values are dependent on the type of the policy.
    pub fn new() -> ConfigurationsResponseScanAllPolicyParameter {
        ConfigurationsResponseScanAllPolicyParameter { daily_time: None }
    }
}
