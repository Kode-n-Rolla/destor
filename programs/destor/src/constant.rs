pub const PROTOCOL_CONFIG_SEED: &[u8] = b"protocol_config";
pub const ORGANIZATION_SEED: &[u8] = b"organization";
pub const MEMBER_SEED: &[u8] = b"member";
pub const VEHICLE_SEED: &[u8] = b"vehicle";
pub const NOTE_SEED: &[u8] = b"note";

pub const MAX_COLOR_LENGTH: usize = 30;
pub const MAX_MODEL_LENGTH: usize = 30;
pub const MAX_DESCRIPTION_LENGTH: usize = 200;
pub const MAX_REPORT_URI_LENGTH: usize = 120;
pub const MAX_REPORT_HASH_LENGTH: usize = 64;
pub const MAX_NOTE_SIGNERS: usize = 3;
pub const MANUFACTURER_NOTE_SIGNERS: usize = 2;
pub const ROAD_INSPECTION_NOTE_SIGNERS: usize = 2;
pub const SERVICE_HUB_NOTE_SIGNERS: usize = 2;
pub const INSURANCE_NOTE_SIGNERS: usize = 2;
pub const OWNER_NOTE_SIGNERS: usize = 2;