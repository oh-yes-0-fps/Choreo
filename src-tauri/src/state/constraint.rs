use slotmap::new_key_type;

use super::waypoint::WaypointScope;

new_key_type! {
    pub struct ConstraintID;
}

#[derive(serde::Serialize, serde::Deserialize)]

pub struct ConstraintScope (pub Option<WaypointScope>, pub Option<WaypointScope>);
impl ConstraintScope {
    pub fn wpt(point: WaypointScope) -> Self {
        ConstraintScope(Some(point), None)
    }
    pub fn none() -> Self {
        ConstraintScope(None, None)
    }
    pub fn sgmt(pt1: WaypointScope, pt2: WaypointScope) -> Self {
        ConstraintScope(Some(pt1), Some(pt2))
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Constraint {
    pub scope: ConstraintScope,
    pub data: ConstraintData,
}


impl Constraint {
    pub fn of(definition: &ConstraintDefinition) ->Self{
        Constraint {
            scope: ConstraintScope::none(),
            data: definition.default_data.clone()
        }
    }

    pub fn definition(data: ConstraintData) -> ConstraintDefinition {
        match data {
            ConstraintData::WptVelocityDirection { direction: _ } => Constraints.WptVelocityDirection,
            ConstraintData::WptZeroVelocity {  } => Constraints.WptZeroVelocity,
            ConstraintData::StopPoint {  } => Constraints.StopPoint,
            ConstraintData::MaxVelocity { velocity:_ } => todo!(),
            ConstraintData::ZeroAngularVelocity {  } => todo!(),
            ConstraintData::StraightLine {  } => todo!(),
            ConstraintData::PointAt { x:_, y:_, tolerance:_ } => todo!(),
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(tag = "type")]
// Add constraint type, scope, and properties
pub enum ConstraintData {
    WptVelocityDirection {
        direction: f64,
    },
    WptZeroVelocity {},
    StopPoint {},
    MaxVelocity {
               velocity: f64,
    },
    ZeroAngularVelocity {
    },
    StraightLine {
    },
    PointAt {
        x: f64,
        y: f64,
        tolerance: f64,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct ConstraintDefinition {
    default_data: ConstraintData,
    waypoint: bool,
    segment: bool
}

pub struct ConstraintDefs {
    pub WptZeroVelocity: ConstraintDefinition,
    pub WptVelocityDirection: ConstraintDefinition,
    pub StopPoint: ConstraintDefinition

}

pub static Constraints: ConstraintDefs = ConstraintDefs {
    WptZeroVelocity: ConstraintDefinition {
        default_data: ConstraintData::WptZeroVelocity {  },
        waypoint: true,
        segment: false },
    WptVelocityDirection: ConstraintDefinition { 
        default_data: ConstraintData::WptVelocityDirection { direction: 0.0 }, 
        waypoint: true,
        segment: false },
    StopPoint: ConstraintDefinition {
        default_data: ConstraintData::StopPoint {  },
        waypoint: true,
        segment: false }
};