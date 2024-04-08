use std::f64::consts::PI;

use trajoptlib::{
    SwerveDrivetrain, SwerveModule
};

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
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
    pub fn wheelMaxVelocity(&self) -> f64 {
        return (self.motor_max_velocity * (PI * 2.0)) / 60.0 / self.gearing;
      }
    pub fn wheelMaxTorque(&self) -> f64 {
    return self.motor_max_torque * self.gearing;
    }
    pub fn as_drivetrain(&self) -> SwerveDrivetrain {
        let wheelMaxVelocity = self.wheelMaxVelocity();
        let wheelMaxTorque = self.wheelMaxTorque();
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
                    wheel_max_angular_velocity: wheelMaxVelocity,
                    wheel_max_torque: wheelMaxTorque,
                },
                SwerveModule {
                    x: half_wheel_base,
                    y: -half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheelMaxVelocity,
                    wheel_max_torque: wheelMaxTorque,
                },
                SwerveModule {
                    x: -half_wheel_base,
                    y: half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheelMaxVelocity,
                    wheel_max_torque: wheelMaxTorque,
                },
                SwerveModule {
                    x: -half_wheel_base,
                    y: -half_track_width,
                    wheel_radius: self.wheel_radius,
                    wheel_max_angular_velocity: wheelMaxVelocity,
                    wheel_max_torque: wheelMaxTorque,
                },
            ]
        }
    }
}