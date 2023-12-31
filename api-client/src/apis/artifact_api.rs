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

/// struct for typed errors of method [`add_label`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddLabelError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status409(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`copy_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyArtifactError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status405(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTagError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status405(crate::models::Errors),
    Status409(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteArtifactError {
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTagError {
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_addition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAdditionError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetArtifactError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_vulnerabilities_addition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVulnerabilitiesAdditionError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_accessories`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAccessoriesError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_artifacts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArtifactsError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTagsError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_label`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveLabelError {
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status409(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// Add label to the specified artiact.
pub async fn add_label(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    label: crate::models::Label,
    x_request_id: Option<&str>,
) -> Result<(), Error<AddLabelError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/labels",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name),
        reference = crate::apis::urlencode(reference)
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
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&label);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AddLabelError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Copy the artifact specified in the \"from\" parameter to the repository.
pub async fn copy_artifact(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    from: &str,
    x_request_id: Option<&str>,
) -> Result<(), Error<CopyArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("from", &from.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
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
        let local_var_entity: Option<CopyArtifactError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a tag for the specified artifact
pub async fn create_tag(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    tag: crate::models::Tag,
    x_request_id: Option<&str>,
) -> Result<(), Error<CreateTagError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/tags",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name),
        reference = crate::apis::urlencode(reference)
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
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    local_var_req_builder = local_var_req_builder.json(&tag);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the artifact specified by the reference under the project and repository. The reference can be digest or tag
pub async fn delete_artifact(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
) -> Result<(), Error<DeleteArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name),
        reference = crate::apis::urlencode(reference)
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
        let local_var_entity: Option<DeleteArtifactError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the tag of the specified artifact
pub async fn delete_tag(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    tag_name: &str,
    x_request_id: Option<&str>,
) -> Result<(), Error<DeleteTagError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/tags/{tag_name}", local_var_configuration.base_path, project_name=crate::apis::urlencode(project_name), repository_name=crate::apis::urlencode(repository_name), reference=crate::apis::urlencode(reference), tag_name=crate::apis::urlencode(tag_name));
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
        let local_var_entity: Option<DeleteTagError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the addition of the artifact specified by the reference under the project and repository.
pub async fn get_addition(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    addition: &str,
    x_request_id: Option<&str>,
) -> Result<String, Error<GetAdditionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/additions/{addition}", local_var_configuration.base_path, project_name=crate::apis::urlencode(project_name), repository_name=crate::apis::urlencode(repository_name), reference=crate::apis::urlencode(reference), addition=crate::apis::urlencode(addition));
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
        let local_var_entity: Option<GetAdditionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the artifact specified by the reference under the project and repository. The reference can be digest or tag.
pub async fn get_artifact(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
    x_accept_vulnerabilities: Option<&str>,
    with_tag: Option<bool>,
    with_label: Option<bool>,
    with_scan_overview: Option<bool>,
    with_accessory: Option<bool>,
    with_signature: Option<bool>,
    with_immutable_status: Option<bool>,
) -> Result<crate::models::Artifact, Error<GetArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name),
        reference = crate::apis::urlencode(reference)
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
    if let Some(ref local_var_str) = with_tag {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_label {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_label", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_scan_overview {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_scan_overview", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_accessory {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_accessory", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_signature {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_signature", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_immutable_status {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_immutable_status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_accept_vulnerabilities {
        local_var_req_builder = local_var_req_builder.header(
            "X-Accept-Vulnerabilities",
            local_var_param_value.to_string(),
        );
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
        let local_var_entity: Option<GetArtifactError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the vulnerabilities addition of the artifact specified by the reference under the project and repository.
pub async fn get_vulnerabilities_addition(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
    x_accept_vulnerabilities: Option<&str>,
) -> Result<String, Error<GetVulnerabilitiesAdditionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/additions/vulnerabilities", local_var_configuration.base_path, project_name=crate::apis::urlencode(project_name), repository_name=crate::apis::urlencode(repository_name), reference=crate::apis::urlencode(reference));
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
    if let Some(local_var_param_value) = x_accept_vulnerabilities {
        local_var_req_builder = local_var_req_builder.header(
            "X-Accept-Vulnerabilities",
            local_var_param_value.to_string(),
        );
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
        let local_var_entity: Option<GetVulnerabilitiesAdditionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List accessories of the specific artifact
pub async fn list_accessories(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
    q: Option<&str>,
    sort: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<crate::models::Accessory>, Error<ListAccessoriesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/accessories", local_var_configuration.base_path, project_name=crate::apis::urlencode(project_name), repository_name=crate::apis::urlencode(repository_name), reference=crate::apis::urlencode(reference));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = q {
        local_var_req_builder = local_var_req_builder.query(&[("q", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
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
        let local_var_entity: Option<ListAccessoriesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List artifacts under the specific project and repository. Except the basic properties, the other supported queries in \"q\" includes \"tags=*\" to list only tagged artifacts, \"tags=nil\" to list only untagged artifacts, \"tags=~v\" to list artifacts whose tag fuzzy matches \"v\", \"tags=v\" to list artifact whose tag exactly matches \"v\", \"labels=(id1, id2)\" to list artifacts that both labels with id1 and id2 are added to
pub async fn list_artifacts(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    x_request_id: Option<&str>,
    q: Option<&str>,
    sort: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
    x_accept_vulnerabilities: Option<&str>,
    with_tag: Option<bool>,
    with_label: Option<bool>,
    with_scan_overview: Option<bool>,
    with_signature: Option<bool>,
    with_immutable_status: Option<bool>,
    with_accessory: Option<bool>,
) -> Result<Vec<crate::models::Artifact>, Error<ListArtifactsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = q {
        local_var_req_builder = local_var_req_builder.query(&[("q", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_tag {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_tag", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_label {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_label", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_scan_overview {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_scan_overview", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_signature {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_signature", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_immutable_status {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_immutable_status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_accessory {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_accessory", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_accept_vulnerabilities {
        local_var_req_builder = local_var_req_builder.header(
            "X-Accept-Vulnerabilities",
            local_var_param_value.to_string(),
        );
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
        let local_var_entity: Option<ListArtifactsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List tags of the specific artifact
pub async fn list_tags(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
    q: Option<&str>,
    sort: Option<&str>,
    page: Option<i64>,
    page_size: Option<i64>,
    with_signature: Option<bool>,
    with_immutable_status: Option<bool>,
) -> Result<Vec<crate::models::Tag>, Error<ListTagsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/tags",
        local_var_configuration.base_path,
        project_name = crate::apis::urlencode(project_name),
        repository_name = crate::apis::urlencode(repository_name),
        reference = crate::apis::urlencode(reference)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = q {
        local_var_req_builder = local_var_req_builder.query(&[("q", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_signature {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_signature", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = with_immutable_status {
        local_var_req_builder =
            local_var_req_builder.query(&[("with_immutable_status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_request_id {
        local_var_req_builder =
            local_var_req_builder.header("X-Request-Id", local_var_param_value.to_string());
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
        let local_var_entity: Option<ListTagsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove the label from the specified artiact.
pub async fn remove_label(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    label_id: i64,
    x_request_id: Option<&str>,
) -> Result<(), Error<RemoveLabelError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/labels/{label_id}", local_var_configuration.base_path, project_name=crate::apis::urlencode(project_name), repository_name=crate::apis::urlencode(repository_name), reference=crate::apis::urlencode(reference), label_id=label_id);
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
        let local_var_entity: Option<RemoveLabelError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
