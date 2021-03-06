use core::clock::{SecondsValue, SpeedFactor};
use frontend::input::AxisValue;
use std::f32::consts;

pub const DEFAULT_WINDOW_WIDTH: u32 = 1280;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 720;
pub const VIEW_SCALE_BASE: f32 = 100.0;
pub const VIEW_ZOOM_MAX: f32 = 8.0;
pub const VIEW_ZOOM_MIN: f32 = 1. / 4.0;
pub const VIEW_ZOOM_MULTIPLIER: f32 = consts::SQRT_2;
pub const VIEW_ZOOM_DURATION: f32 = 0.25;
pub const CAMERA_IMPULSE: f32 = 5.0;
pub const CAMERA_INERTIA: f32 = 4.0;
pub const CAMERA_LIMIT: f32 = 0.5;
pub const FRAME_SMOOTH_COUNT: usize = 120;
pub const FRAME_TIME_TARGET: SecondsValue = 1. / 60.;
pub const LOG_INTERVAL: SecondsValue = 5.0;
pub const SAVE_INTERVAL: SecondsValue = 300.0;
pub const DEAD_ZONE: AxisValue = 0.3f32;
pub const TURN_SPEED: f32 = consts::PI * 200.;
pub const DEBUG_DRAW_BRAKE_SCALE: f32 = 0.05;
pub const DEBUG_DRAW_MOVE_SCALE: f32 = 0.05;
pub const MIN_FRAME_LENGTH: SecondsValue = (1.0 / 1000.0) as SecondsValue;
pub const MAX_FRAME_LENGTH: SecondsValue = (1.0 / 30.0) as SecondsValue;
pub const THRUST_POWER: f32 = 5000.;
pub const POWER_BOOST: f32 = 100.;
pub const DRAG_COEFFICIENT: f32 = 0.000_001;
#[allow(unused)]
pub const COMPASS_SPRING_POWER: f32 = 1000.0;
pub const JOINT_UPPER_ANGLE: f32 = consts::PI / 6.;
pub const JOINT_LOWER_ANGLE: f32 = -consts::PI / 6.;
pub const JOINT_FREQUENCY: f32 = 5.0;
pub const JOINT_DAMPING_RATIO: f32 = 0.9;
pub const LINEAR_DAMPING_DEFAULT: f32 = 0.8;
pub const LINEAR_DAMPING_PLAYER: f32 = 2.0;
pub const ANGULAR_DAMPING: f32 = 0.9;
pub const PICK_EPS: f32 = 0.001f32;
pub const DEFAULT_RESOURCE_CHARGE: f32 = 0.8;
pub const DEFAULT_SPORE_CHARGE: f32 = 0.8;
pub const DEFAULT_MINION_CHARGE: f32 = 0.3;
pub const INITIAL_SPAWN_RADIUS_RATIO: f32 = 0.1;
pub const INITIAL_SPAWN_RADIUS_SLICES: f32 = 19.;
pub const INITIAL_SPAWN_RADIUS_INCREMENT: f32 = 0.5;
pub const MATURITY_MINION_DEFAULT: f32 = 0.5;
pub const MATURITY_DEFAULT: f32 = 1.0;
pub const GROWTH_COST_RATIO: f32 = 0.1;
pub const SPAWN_COST_THRESHOLD: f32 = 0.95;
pub const SPAWN_COST_RATIO: f32 = 0.75;
pub const COLLISION_BASE_COST: f32 = 0.5;
pub const WORLD_RADIUS: f32 = 80.;
pub const DEFAULT_CHARGE_DECAY_TIME: SecondsValue = 0.5;
pub const MINION_CHARGE_DECAY_TIME: SecondsValue = 0.25;
pub const PLAYER_CHARGE_DECAY_TIME: SecondsValue = 0.1;
pub const PLAYER_CHARGE_INITIAL_VALUE: f32 = 25.0;
pub const PLAYER_CHARGE_REST_VALUE: f32 = 0.05;
pub const EMITTER_DISTANCE: f32 = 40.;
pub const EMITTER_PERIOD: SecondsValue = 0.2;
#[allow(unused)]
pub const EMITTER_SPREAD_ANGLE: f32 = consts::PI / 12.;
pub const EMITTER_SPREAD_JITTER: f32 = 0.1;
pub const EMITTER_INTENSITY_DECAY: f32 = 1.0;
pub const BULLET_SPEED_SCALE: f32 = 100.;
pub const BULLET_FIRE_RATE_SCALE: SecondsValue = 0.5;
pub const BULLET_FULL_CHARGE: SecondsValue = 1.0;
pub const BULLET_FIRE_RATE: SecondsValue = 45.0;
pub const DENSITY_DEFAULT: f32 = 1.0;
pub const DENSITY_RESOURCE: f32 = DENSITY_DEFAULT;
pub const DENSITY_PLAYER: f32 = 1.0;
pub const DENSITY_MINION: f32 = 0.2;
pub const DENSITY_SPORE: f32 = 0.5;
pub const RESTITUTION_DEFAULT: f32 = 0.6;
pub const RESTITUTION_PLAYER: f32 = 0.1;
pub const FRICTION_DEFAULT: f32 = 0.7;
pub const FRICTION_PLAYER: f32 = 0.6;
pub const B2_LINEAR_SLOP: f32 = 0.005;
pub const DEFAULT_MINION_GENE_POOL_FILE: &str = "minion_gene_pool.csv";
pub const DEFAULT_MINION_GENE_POOL: &[&str] = &[
	"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
	"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
	"GzB2lQVwM00tTAm5gwajjf4wc0a5GzB2lQVwM00tTAm5gwajjf4wc0a5",
	"GzB2lQdwM10vQEu5zwaPgDhfq2v8GzB2lQdwM10vQEu5zwaPgDhfq2v8",
];

pub const COLOR_SUNSHINE: [f32; 4] = [400.0, 90.0, 1.0, 1.0];
pub const COLOR_TRANSPARENT: [f32; 4] = [0.; 4];
pub const COLOR_WHITE: [f32; 4] = [1.; 4];
#[allow(unused)]
pub const COLOR_BLACK: [f32; 4] = [0., 0., 0., 1.];

pub const DEFAULT_RESOURCE_GENE_POOL: &[&str] = &["GyA21QoQ", "M00sWS0M"];

pub const CONFIG_DIR_HOME: &str = ".config/rust-oids";
pub const CONFIG_DIR_SAVED_STATE: &str = "saved_state";
pub const CONFIG_DIR_RESOURCES: &str = "resources";
pub const DUMP_FILE_PATTERN_CSV: &str = "%Y%m%d_%H%M%S.csv";
pub const DUMP_FILE_PATTERN_JSON: &str = "%Y%m%d_%H%M%S.json";

pub const CAPTURE_FOLDER_TIMESTAMP_PATTERN: &str = "%Y%m%d_%H%M%S";
pub const CAPTURE_FOLDER: &str = "capture";
pub const CAPTURE_FILENAME_PREFIX: &str = "capture_";

pub const AMBIENT_LIGHTS: &[[f32; 4]] = &[
	[1.0, 1.0, 1.0, 1.0],
	[3.1, 3.1, 3.1, 1.0],
	[10.0, 10.0, 10.0, 1.0],
	[31.0, 31.0, 31.0, 1.0],
	[100.0, 100.0, 100.0, 1.0],
	[0.001, 0.001, 0.001, 1.0],
	[0.01, 0.01, 0.01, 1.0],
	[0.1, 0.1, 0.1, 1.0],
	[0.31, 0.31, 0.31, 0.5],
];

pub const SPEED_FACTORS: &[SpeedFactor] = &[1.0, 0.5, 0.2, 0.1, 1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0];

pub const BACKGROUNDS: &[[f32; 4]] = &[
	[0.05, 0.07, 0.1, 1.0],
	[0.5, 0.5, 0.5, 0.5],
	[1.0, 1.0, 1.0, 1.0],
	[3.1, 3.1, 3.1, 1.0],
	[10.0, 10.0, 10.0, 1.0],
	[0., 0., 0., 1.0],
	[0.01, 0.01, 0.01, 1.0],
];
