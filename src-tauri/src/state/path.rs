use slotmap::SlotMap;
use sqlx::{Pool, Sqlite};
use trajoptlib::{
    HolonomicTrajectory, InitialGuessPoint, SwervePathBuilder,
};

use super::{constraint::{ConstraintData, Constraint, ConstraintDefinition, ConstraintID}, robotconfig::ChoreoRobotConfig, waypoint::{Waypoint, WaypointID, scope_to_position,get_waypoint_impl}, utils::sqlxStringify};
use serde_with::serde_as;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Path {
    pub config: ChoreoRobotConfig,
    pub waypoints: Vec<WaypointID>,
    constraint_pool: SlotMap<ConstraintID, Constraint>,
    pub constraints: Vec<ConstraintID>
}

impl Path {
    pub fn new() -> Self {
        Path {
            config: ChoreoRobotConfig::default(),
            waypoints : vec![],
            constraint_pool : SlotMap::with_key(),
            constraints: vec![]
        }
    }
    pub async fn add_waypoint(&mut self, pool: &Pool<Sqlite>, pt: Waypoint) -> Result<WaypointID, sqlx::Error> {
        let id = crate::waypoint::add_waypoint_impl(pool, &pt).await?;
        self.waypoints.push(id);
        Ok(id)
    }

    // pub fn delete_waypoint (&mut self, pt: WaypointID) {
    //     if let Some(index) = self.index_of(pt) {
    //         self.waypoints.remove(index);
    //         if self.index_of(pt).is_none() {
    //             self.point_pool.remove(pt);
    //         }
    //     }
    // }

    pub fn index_of(&self, pt: WaypointID) -> Option<usize>{
        self.waypoints.iter().position(|&r| r == pt)
    }

    pub fn add_constraint(&mut self, definition: &ConstraintDefinition) -> ConstraintID {
        let con = Constraint::of(definition);
        let key = self.constraint_pool.insert(con);
        self.constraints.push(key);
        key
    }

    pub fn get_constraint(&mut self, id: ConstraintID) -> Option<&mut Constraint>{
        return self.constraint_pool.get_mut(id);
    }




    pub async fn generate_trajectory(&self,
        pool: &Pool<Sqlite>
    ) -> Result<HolonomicTrajectory, String> {
        let mut path_builder = SwervePathBuilder::new();
        let mut wpt_cnt: usize = 0;
        let mut control_interval_counts: Vec<usize> = Vec::new();
        let mut guess_points_after_waypoint: Vec<InitialGuessPoint> = Vec::new();
        let mut actual_points : Vec<&WaypointID> = Vec::new();
        for (idx, id) in self.waypoints.iter().enumerate() {
            let wpt: Waypoint = crate::waypoint::get_waypoint_impl(pool, id).await.map_err(sqlxStringify)?;
            if wpt.is_initial_guess {
                let guess_point: InitialGuessPoint = InitialGuessPoint {
                    x: wpt.x,
                    y: wpt.y,
                    heading: wpt.heading,
                };
                guess_points_after_waypoint.push(guess_point);
                if let Some(last) = control_interval_counts.last_mut() {
                    *last +=  (wpt.control_interval_count) as usize;
                }
            } else {
                if wpt_cnt > 0 {
                    path_builder.sgmt_initial_guess_points(wpt_cnt - 1, &guess_points_after_waypoint);
                }
                
                guess_points_after_waypoint.clear();
                actual_points.push(id);
                
                if wpt.heading_constrained && wpt.translation_constrained {
                    path_builder.pose_wpt(wpt_cnt, wpt.x, wpt.y, wpt.heading);
                    wpt_cnt += 1;
                } else if wpt.translation_constrained {
                    path_builder.translation_wpt(wpt_cnt, wpt.x, wpt.y, wpt.heading);
                    wpt_cnt += 1;
                } else {
                    path_builder.empty_wpt(wpt_cnt, wpt.x, wpt.y, wpt.heading);
                    wpt_cnt += 1;
                }
                if idx != self.waypoints.len() - 1 {
                    control_interval_counts.push((wpt.control_interval_count) as usize);
                }
            }
        }
    
        path_builder.set_control_interval_counts(control_interval_counts);
    
        for (_, constraint) in &self.constraint_pool {
            let scope = &constraint.scope;
            let positionOpt = (
                scope_to_position(&actual_points, &scope.0),
                scope_to_position(&actual_points, &scope.1)
            );
            let mut isWaypoint = false;

            let position: Option<(usize, usize)> = match positionOpt {
                (None, None) => None,
                (Some(idx1), None) => {isWaypoint = true; Some((idx1, 0))},
                (None, Some(idx2)) => {isWaypoint = true; Some((idx2, 0))},
                (Some(idx1), Some(idx2)) => 
                    if idx1 < idx2 {Some((idx1, idx2))} else {Some((idx2, idx1))}
            };
            if position.is_none() {
                continue;
            }
            let position = position.unwrap();

            match constraint.data {
                ConstraintData::WptVelocityDirection { direction } => {
                    if isWaypoint {
                        path_builder.wpt_linear_velocity_direction(position.0, direction);
                    }
                }
                ConstraintData::WptZeroVelocity {} => {
                    if isWaypoint {
                        path_builder.wpt_linear_velocity_max_magnitude(position.0, 0.0f64);
                    }
                }
                ConstraintData::StopPoint { } => {
                    if isWaypoint {
                        path_builder.wpt_linear_velocity_max_magnitude(position.0, 0.0f64);
                        path_builder.wpt_angular_velocity(position.0, 0.0);
                    }
                }
                ConstraintData::MaxVelocity {velocity } => {
                    if isWaypoint {
                        path_builder
                        .wpt_linear_velocity_max_magnitude(position.0, velocity);
                    } else {
                        path_builder
                        .sgmt_linear_velocity_max_magnitude(
                            position.0,
                            position.1,
                            velocity,
                        );
                    }
                },
                ConstraintData::ZeroAngularVelocity {} => {
                    if isWaypoint {
                        path_builder.wpt_angular_velocity(position.0, 0.0);
                    } else {
                        path_builder.sgmt_angular_velocity(
                        position.0,
                        position.1,
                        0.0,
                    );
                    }
                },
                ConstraintData::StraightLine {} => {
                    if !isWaypoint {
                        for point in position.0..position.1 {
                            let this_pt = point;
                            let next_pt = point + 1;
                            if this_pt != position.0 {
                                // points in between straight-line segments are automatically zero-velocity points
                                path_builder.wpt_linear_velocity_max_magnitude(this_pt, 0.0f64);
                            }
                            let pt1 = &get_waypoint_impl(pool,actual_points[this_pt]).await.map_err(sqlxStringify)?;

                            let pt2 = &get_waypoint_impl(pool,actual_points[next_pt]).await.map_err(sqlxStringify)?;
                            let x1 = pt1.x;
                            let x2 = pt2.x;
                            let y1 = pt1.y;
                            let y2 = pt2.y;
                            path_builder.sgmt_linear_velocity_direction(
                                this_pt,
                                next_pt,
                                (y2 - y1).atan2(x2 - x1),
                            )
                        }
                    }
                }
                ConstraintData::PointAt {
                    x,
                    y,
                    tolerance,
                } => 
                    if isWaypoint {
                        path_builder.wpt_point_at(position.0, x, y, tolerance)
                    } else 
                    {
                         path_builder.sgmt_point_at(
                        position.0,
                        position.1,
                        x,
                        y,
                        tolerance,
                    )
                }, // add more cases here to impl each constraint.
            }
        }
       
    
        path_builder.set_bumpers(self.config.bumper_length, self.config.bumper_width);
    
        // // Skip obstacles for now while we figure out whats wrong with them
        // for o in circleObstacles {
        //     path_builder.sgmt_circle_obstacle(0, wpt_cnt - 1, o.x, o.y, o.radius);
        // }
    
        // // Skip obstacles for now while we figure out whats wrong with them
        // for o in polygonObstacles {
        //     path_builder.sgmt_polygon_obstacle(0, wpt_cnt - 1, o.x, o.y, o.radius);
        // }
        path_builder.set_drivetrain(&self.config.as_drivetrain());
        path_builder.generate(true)
    }
}


