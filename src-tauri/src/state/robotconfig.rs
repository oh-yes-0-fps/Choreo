use std::f64::consts::PI;

use partially::Partial;
use sqlx::{FromRow, Pool, Sqlite, Error};
use trajoptlib::{
    SwerveDrivetrain, SwerveModule
};

pub static keys: &str = "mass, rotational_inertia, motor_max_velocity, motor_max_torque, gearing,
    wheel_radius, bumper_width, bumper_width, bumper_length, wheelbase, trackwidth";
#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, Partial, FromRow, sqlxinsert::SqliteInsert, Debug)]
#[partially(derive(serde::Serialize, serde::Deserialize, Debug))]
pub struct ChoreoRobotConfig {
    pub mass: f64,
    pub rotational_inertia: f64,
    pub motor_max_velocity: f64,
    pub motor_max_torque: f64,
    pub gearing: f64,
    pub wheel_radius: f64,
    pub bumper_width: f64,
    pub bumper_length: f64,
    pub wheelbase: f64,
    pub trackwidth: f64,
}
impl Default for ChoreoRobotConfig {
    fn default() -> Self {
        ChoreoRobotConfig {
            mass: 74.088,
            rotational_inertia: 6.0,
            bumper_width: 0.876,
            bumper_length: 0.876,
            wheelbase: 0.578,
            trackwidth: 0.578,
            motor_max_torque:1.162,
            motor_max_velocity: 4800.0,
            gearing:6.75,
            wheel_radius:0.050799972568014815
        }
    }
}

impl  ChoreoRobotConfig {
    pub fn wheel_max_velocity(&self) -> f64 {
        (self.motor_max_velocity * (PI * 2.0)) / 60.0 / self.gearing
      }
    pub fn wheel_max_torque(&self) -> f64 {
    self.motor_max_torque * self.gearing
    }
    pub fn as_drivetrain(&self) -> SwerveDrivetrain {
        let wheel_max_velocity = self.wheel_max_velocity();
        let wheel_max_torque = self.wheel_max_torque();
        let half_wheel_base = self.wheelbase / 2.0;
        let half_track_width = self.trackwidth / 2.0;
        SwerveDrivetrain {
            mass: self.mass,
            moi: self.rotational_inertia,
            modules: vec![
                SwerveModule {
                    x: half_wheel_base,
                    y: half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheel_max_velocity,
                    wheel_max_torque: wheel_max_torque,
                },
                SwerveModule {
                    x: half_wheel_base,
                    y: -half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheel_max_velocity,
                    wheel_max_torque: wheel_max_torque,
                },
                SwerveModule {
                    x: -half_wheel_base,
                    y: half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheel_max_velocity,
                    wheel_max_torque: wheel_max_torque,
                },
                SwerveModule {
                    x: -half_wheel_base,
                    y: -half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheel_max_velocity,
                    wheel_max_torque: wheel_max_torque,
                },
            ]
        }
    }
}

    pub async fn create_robot_config_table(
        pool: &Pool<Sqlite>,
    ) -> Result<<Sqlite as sqlx::Database>::QueryResult, Error> {
        sqlx::query(
            "Create table robot_config (
                config_id INT PRIMARY KEY,
                mass                REAL NOT NULL,
                rotational_inertia  REAL NOT NULL,
                motor_max_velocity  REAL NOT NULL,
                motor_max_torque    REAL NOT NULL,
                gearing             REAL NOT NULL,
                wheel_radius        REAL NOT NULL,
                bumper_width        REAL NOT NULL,
                bumper_length       REAL NOT NULL,
                wheelbase           REAL NOT NULL,
                trackwidth          REAL NOT NULL
            )
        ",
        )
        .execute(pool)
        .await?;
        ChoreoRobotConfig::default().insert_raw(pool, "robot_config").await
    }

    pub async fn update_robot_config_impl(
        pool: &Pool<Sqlite>,
        update: PartialChoreoRobotConfig
    ) -> Result<(), Error> {

        sqlx::query("
        UPDATE robot_config
        SET
            mass                = COALESCE(?, mass),
            rotational_inertia  = COALESCE(?, rotational_inertia),
            motor_max_velocity  = COALESCE(?, motor_max_velocity),
            motor_max_torque    = COALESCE(?, motor_max_torque),
            gearing             = COALESCE(?, gearing),
            wheel_radius        = COALESCE(?, wheel_radius),
            bumper_width        = COALESCE(?, bumper_width),
            bumper_length       = COALESCE(?, bumper_length),
            wheelbase           = COALESCE(?, wheelbase),
            trackwidth          = COALESCE(?, trackwidth)
        "
        )
        .bind(update.mass)
        .bind(update.rotational_inertia)
        .bind(update.motor_max_velocity)
        .bind(update.motor_max_torque)
        .bind(update.gearing)
        .bind(update.wheel_radius)
        .bind(update.bumper_width)
        .bind(update.bumper_length)
        .bind(update.wheelbase)
        .bind(update.trackwidth)
        .execute(pool).await.map(|_|())
    }

    pub async fn get_robot_config_impl(
        pool: &Pool<Sqlite>
    ) -> Result<ChoreoRobotConfig, Error>{
        sqlx::query_as::<Sqlite, ChoreoRobotConfig>(
            format!(
            "SELECT {} FROM robot_config", keys).as_str()
        ).fetch_one(pool).await
    }
