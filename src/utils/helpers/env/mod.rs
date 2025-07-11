use crate::utils::errors::AppError;

pub fn env_required(var_name: &str) -> Result<String, AppError> {
    std::env::var(var_name).map_err(|_| {
        AppError::Internal(anyhow::anyhow!(format!("Environment variable {var_name} not found")))
    })
}