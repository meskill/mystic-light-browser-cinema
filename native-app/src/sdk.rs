use mystic_light_sdk::{CommonError, MysticLightSDK};

const LIB_PATH: &str = if cfg!(target_arch = "x86_64") {
    "./sdk/MysticLight_SDK_x64.dll"
} else {
    "./sdk/MysticLight_SDK.dll"
};

pub fn create_sdk() -> Result<MysticLightSDK, CommonError> {
    MysticLightSDK::new(LIB_PATH)
}
