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

/// struct for typed errors of method [`get_report_log`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetReportLogError {
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`scan_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScanArtifactError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`stop_scan_artifact`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopScanArtifactError {
    Status400(crate::models::Errors),
    Status401(crate::models::Errors),
    Status403(crate::models::Errors),
    Status404(crate::models::Errors),
    Status500(crate::models::Errors),
    UnknownValue(serde_json::Value),
}

/// Get the log of the scan report
pub async fn get_report_log(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    report_id: &str,
    x_request_id: Option<&str>,
) -> Result<String, Error<GetReportLogError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/scan/{report_id}/log", local_var_configuration.base_path, project_name=crate::apis::urlencode(project_name), repository_name=crate::apis::urlencode(repository_name), reference=crate::apis::urlencode(reference), report_id=crate::apis::urlencode(report_id));
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
        let local_var_entity: Option<GetReportLogError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Scan the specified artifact
pub async fn scan_artifact(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
) -> Result<(), Error<ScanArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/scan",
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ScanArtifactError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancelling a scan job for a particular artifact
pub async fn stop_scan_artifact(
    configuration: &configuration::Configuration,
    project_name: &str,
    repository_name: &str,
    reference: &str,
    x_request_id: Option<&str>,
) -> Result<(), Error<StopScanArtifactError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project_name}/repositories/{repository_name}/artifacts/{reference}/scan/stop",
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StopScanArtifactError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
