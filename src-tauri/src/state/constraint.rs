
use std::sync::atomic::AtomicI64;

use sqlx::{Pool, Sqlite, Error, FromRow, sqlite::SqliteQueryResult};
use sqlxinsert::SqliteInsert;

use super::{waypoint::WaypointScope, utils::sqlx_stringify};

pub static const_id: AtomicI64 = AtomicI64::new(0);

#[derive(serde::Serialize, serde::Deserialize, Clone)]

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

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Constraint {
    pub scope: ConstraintScope,
    pub data: ConstraintData,
}


impl Constraint {
    pub fn of(definition: &ConstraintDefinition) ->Self{
        Constraint {
            scope: ConstraintScope::none(),
            data: definition.default_data
        }
    }

    pub fn definition(data: ConstraintData) -> ConstraintDefinition {
        match data {
            ConstraintData::WptVelocityDirection { direction: _ } => Constraints.wpt_velocity_direction,
            ConstraintData::WptZeroVelocity {  } => Constraints.wpt_zero_velocity,
            ConstraintData::StopPoint {  } => Constraints.stop_point,
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
    WptZeroVelocity,
    StopPoint,
    MaxVelocity {
               velocity: f64,
    },
    ZeroAngularVelocity,
    StraightLine,
    PointAt {
        x: f64,
        y: f64,
        tolerance: f64,
    },
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct ConstraintDefinition {
    id: i64,
    default_data: ConstraintData,
    waypoint: bool,
    segment: bool
    
}

pub struct ConstraintDefs {
    pub wpt_zero_velocity: ConstraintDefinition,
    pub wpt_velocity_direction: ConstraintDefinition,
    pub stop_point: ConstraintDefinition

}

pub static Constraints: ConstraintDefs = ConstraintDefs {
    wpt_zero_velocity: ConstraintDefinition {
        id: 0,
        default_data: ConstraintData::WptZeroVelocity {  },
        waypoint: true,
        segment: false,
    },
    wpt_velocity_direction: ConstraintDefinition { 
        id: 1,
        default_data: ConstraintData::WptVelocityDirection { direction: 0.0 }, 
        waypoint: true,
        segment: false },
    stop_point: ConstraintDefinition {
        id: 2,
        default_data: ConstraintData::StopPoint {  },
        waypoint: true,
        segment: false }
};

pub async fn create_constraint_tables(pool: &Pool<Sqlite>) -> Result<(), Error>  {
    // sqlx::query(
    //     "CREATE TABLE constraint_types (
    //         id INT PRIMARY KEY,
    //         name VARCHAR(30),
    //         has_table BOOL
    //     )
    //     "
    // ).execute(pool).await?;
    sqlx::query(
        "CREATE TABLE constraints (
                constraint_id serial INT PRIMARY KEY,
                path INT NOT NULL,
                start INT REFERENCES waypoints(wpt_id),
                end INT REFERENCES waypoints(wpt_id),
                kind INT NOT NULL,
                arg0 REAL,
                arg1 REAL,
                arg2 REAL,
                arg3 REAL
        )
        "
    ).execute(pool).await?;
    // sqlx::query(
    //     "CREATE TABLE wpt_velocity_direction (
    //         constraint_id INT PRIMARY KEY REFERENCES Constraints(constraint_id),
    //         type INT as (0) STORED,
    //         direction REAL
    //     )
    //     "
    // ).bind(0).execute(pool).await?;
    // sqlx::query(
    //     "INSERT INTO constraint_types (id, name, has_table) VALUES(?, , true)"
    // ).bind(0)
    // .execute(pool).await?;
    // sqlx::query(
    //     "INSERT INTO constraint_types (id, name, has_table) VALUES(1, \"WptZeroVelocity\", false)"
    // ).execute(pool).await?;
    Ok(())
}

#[derive(FromRow, SqliteInsert, Debug)]
struct ConstraintRow {
    constraint_id: i64,
    path: i64,
    start: Option<i64>,
    end: Option<i64>,
    kind: i64,
    arg0: Option<f64>,
    arg1: Option<f64>,
    arg2: Option<f64>,
    arg3: Option<f64>
}

impl ConstraintRow {
    pub fn to_constraint(&self) -> Result<Constraint, String> {
        println!("{:?}", self);
        let start = self.start.map(deser_wpt_scope);
        let end = self.end.map(deser_wpt_scope);
        let scope = ConstraintScope (start, end);
        let data = match self.kind {
            0 => {
                if let Some(direction) = self.arg0 {
                    Ok(ConstraintData::WptVelocityDirection { direction })
                } else {Err("No Direction")}
            },
            1=>Ok(ConstraintData::WptZeroVelocity{}),
            2=>Ok(ConstraintData::StopPoint{}),
            _=>Err("unknown type")
        }?;
        Ok(Constraint {scope, data})
    }

    pub fn from_constraint(path_id: &i64, con: &Constraint) -> ConstraintRow {
        let constraint_id = const_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let scope = con.scope.clone();
        let start = scope.0.map(ser_wpt_scope);
        let end = scope.1.map(ser_wpt_scope);
        let kind = Constraint::definition(con.data).id;
        let mut arg0 = None;
        let mut arg1 = None;
        let mut arg2 = None;
        let mut arg3 = None;
        match con.data {
            ConstraintData::WptVelocityDirection { direction } => {arg0 = Some(direction);},
            ConstraintData::MaxVelocity { velocity } => arg0 = Some(velocity),
            ConstraintData::PointAt { x, y, tolerance } => {
                arg0 = Some(x);
                arg1 = Some(y);
                arg2 = Some(tolerance);
            },
            // nothing needed for constraints with no extra data
            _ => {}
            
        };
        ConstraintRow {
            constraint_id,
            path: path_id.clone(),
            start, end, kind, arg0, arg1, arg2, arg3 }
    }

}
fn deser_wpt_scope(wpt_id: i64) -> WaypointScope {
    match wpt_id {
        0=>WaypointScope::First,
        1=>WaypointScope::Last,
        _=>WaypointScope::Uuid(wpt_id)
    }
}
fn ser_wpt_scope(scope: WaypointScope) -> i64 {
    match scope {
        WaypointScope::First => 0,
        WaypointScope::Last => 1,
        WaypointScope::Uuid(wpt_id)=>wpt_id
    }
}

pub async fn get_constraint(pool: &Pool<Sqlite>, id: &i64) -> Result<Constraint, String> {
    let row: ConstraintRow = sqlx::query_as(
        "SELECT constraint_id, path, start, end, kind, arg0, arg1, arg2, arg3
        FROM constraints WHERE constraint_id == ?
        "
    ).bind(id).fetch_one(pool).await.map_err(sqlx_stringify)?;
    row.to_constraint()
}

pub async fn get_path_constraints(pool: &Pool<Sqlite>, path_id: &i64) -> Result<Vec<Constraint>, String> {
    let rows: Vec<ConstraintRow> = sqlx::query_as(
        "SELECT constraint_id, path, start, end, kind, arg0, arg1, arg2, arg3
        FROM constraints WHERE path == ?
        "
    ).bind(path_id).fetch_all(pool).await.map_err(sqlx_stringify)?;
    let mut result:Vec<Constraint> = vec![];
    for row in rows {
        result.push(row.to_constraint()?);
    }
    Ok(result)
}

pub async fn add_constraint(pool: &Pool<Sqlite>, path_id:&i64, con: &Constraint) -> Result<i64, Error> {
    let row = ConstraintRow::from_constraint(path_id, con);
    let res = row.insert_raw(pool, "constraints").await.map(|_|row.constraint_id);
    res
}

