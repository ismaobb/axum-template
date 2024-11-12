use crate::core::ApiOk;

pub async fn logout() -> ApiOk<bool> {
    ApiOk::from(true)
}
