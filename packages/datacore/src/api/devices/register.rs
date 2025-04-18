use axum::{Json, extract::State};
use oasgen::{OaSchema, oasgen};
use powerpals_entities::devices::Device;
use powerpals_macaddr::HardwareAddress;
use powerpals_tsid::IDClientUser;
use prefixed_tsid::tsid::TSIDDatabaseID;
use serde::Deserialize;

use crate::{controllers::devices::DevicesController, db::Pool, error::error::APIError};

#[derive(Deserialize, OaSchema)]
pub struct RegisterDeviceRequest {
    pub user_id: TSIDDatabaseID<IDClientUser>,
    pub hardware_address: HardwareAddress,
}

/// Registers a new device for a user. This should only ever be called once for a given device.
#[oasgen]
pub async fn api_devices_register(
    State(pool): State<Pool>,
    Json(data): Json<RegisterDeviceRequest>,
) -> Result<Json<Device>, APIError> {
    let mut db = pool.get().await?;

    println!("Attempting device registration for {}", data.user_id);
    let new_device =
        DevicesController::register_device(&mut db, data.user_id, data.hardware_address).await?;

    Ok(Json(new_device))
}
