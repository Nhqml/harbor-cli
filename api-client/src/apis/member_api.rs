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

/// struct for typed errors of method [`create_project_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateProjectMemberError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status409(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_project_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteProjectMemberError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_project_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectMemberError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_project_members`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListProjectMembersError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_project_member`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateProjectMemberError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// Create project member relationship, the member can be one of the user_member and group_member,  The user_member need to specify user_id or username. If the user already exist in harbor DB, specify the user_id,  If does not exist in harbor DB, it will SearchAndOnBoard the user. The group_member need to specify id or ldap_group_dn. If the group already exist in harbor DB. specify the user group's id,  If does not exist, it will SearchAndOnBoard the group.
pub async fn create_project_member(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
    project_member: Option<crate::models::ProjectMember>,
) -> Result<(), Error<CreateProjectMemberError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/members",
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
    local_var_req_builder = local_var_req_builder.json(&project_member);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateProjectMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_project_member(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    mid: i64,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
) -> Result<(), Error<DeleteProjectMemberError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/members/{mid}",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id),
        mid = mid
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
        let local_var_entity: Option<DeleteProjectMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the project member information
pub async fn get_project_member(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    mid: i64,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
) -> Result<crate::models::ProjectMemberEntity, Error<GetProjectMemberError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/members/{mid}",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id),
        mid = mid
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetProjectMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all project member information
pub async fn list_project_members(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
    page: Option<i64>,
    page_size: Option<i64>,
    entityname: Option<&str>,
) -> Result<Vec<crate::models::ProjectMemberEntity>, Error<ListProjectMembersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/members",
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
    if let Some(ref local_var_str) = entityname {
        local_var_req_builder =
            local_var_req_builder.query(&[("entityname", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListProjectMembersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update project member relationship
pub async fn update_project_member(
    configuration: &configuration::Configuration,
    project_name_or_id: &str,
    mid: i64,
    x_request_id: Option<&str>,
    x_is_resource_name: Option<bool>,
    role: Option<crate::models::RoleRequest>,
) -> Result<(), Error<UpdateProjectMemberError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name_or_id}/members/{mid}",
        local_var_configuration.base_path,
        project_name_or_id = crate::apis::urlencode(project_name_or_id),
        mid = mid
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
    local_var_req_builder = local_var_req_builder.json(&role);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateProjectMemberError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}