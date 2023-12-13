/*
 * Harbor API
 *
 * These APIs provide services for manipulating Harbor project.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_immu_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateImmuRuleError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_immu_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteImmuRuleError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_immu_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListImmuRulesError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_immu_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateImmuRuleError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// This endpoint add an immutable tag rule to the project
pub async fn create_immu_rule(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    immutable_rule: crate::models::ImmutableRule,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
) -> Result<(), Error<CreateImmuRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/immutabletagrules",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_is_resource_name {
        local_var_req_builder =
            local_var_req_builder.header("X-Is-Resource-Name", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&immutable_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateImmuRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_immu_rule(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    immutable_rule_id: i64,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
) -> Result<(), Error<DeleteImmuRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/immutabletagrules/{immutable_rule_id}",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id),
        immutable_rule_id = immutable_rule_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_is_resource_name {
        local_var_req_builder =
            local_var_req_builder.header("X-Is-Resource-Name", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteImmuRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint returns the immutable tag rules of a project
pub async fn list_immu_rules(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
    page: Option<i64>,
    page_size: Option<i64>,
    q: Option<&str>,
    sort: Option<&str>,
) -> Result<Vec<crate::models::ImmutableRule>, Error<ListImmuRulesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/immutabletagrules",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = q {
        local_var_req_builder = local_var_req_builder.query(&[("q", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_is_resource_name {
        local_var_req_builder =
            local_var_req_builder.header("X-Is-Resource-Name", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListImmuRulesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_immu_rule(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    immutable_rule_id: i64,
    immutable_rule: crate::models::ImmutableRule,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
) -> Result<(), Error<UpdateImmuRuleError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/immutabletagrules/{immutable_rule_id}",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id),
        immutable_rule_id = immutable_rule_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_is_resource_name {
        local_var_req_builder =
            local_var_req_builder.header("X-Is-Resource-Name", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&immutable_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateImmuRuleError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
