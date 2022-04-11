use std::time::Duration;

/// The number of blocks to process between difficulty adjustments.
pub const CYCLE_BLOCK_LIMIT: u32 = 500;

/// The target block time, in seconds.
pub const TARGET_BLOCK_TIME: Duration = Duration::from_millis(2);

/// The minimum factor by which to multiply the target per adjustment.
pub const MIN_FACTOR: f64 = 0.25;

/// The maximum factor by which to multiply the target per adjustment.
pub const MAX_FACTOR: f64 = 4.00;
