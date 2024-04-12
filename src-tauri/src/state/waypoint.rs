pub type WaypointID = i64;

use std::sync::atomic::{Ordering, AtomicI64};

use partially::Partial;
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WaypointScope { 
    
    Uuid(
        WaypointID
    ),
    First,
    Last
}

    use slotmap::{Key, KeyData};
    use tauri::Manager;

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, Debug, Partial)]
#[partially(derive(Default, serde::Serialize, serde::Deserialize, Debug))]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub is_initial_guess: bool,
    pub translation_constrained: bool,
    pub heading_constrained: bool,
    pub control_interval_count: i32
}

pub static wpt_id : AtomicI64  = AtomicI64::new(0);



impl Waypoint {
    pub fn new() -> Self {

        Waypoint {
            x: 0.0,
            y: 0.0,
            heading: 0.0,
            is_initial_guess: false,
            translation_constrained: true,
            heading_constrained: true,
            control_interval_count: 40
        }
    }
}


pub fn scope_to_waypoint_id<'a>(waypoints: &'a Vec<WaypointID>, scope: & Option<WaypointScope>) -> Option<&'a WaypointID> {
    if let Some(scope) = scope.as_ref() {
    match scope {
        WaypointScope::Uuid(id) => waypoints.iter().find(|&r| *r == *id),
        WaypointScope::First => waypoints.first(),
        WaypointScope::Last => waypoints.last()
    }
} else {None}
}

pub fn scope_to_position(waypoints: &Vec<&WaypointID>, scope: & Option<WaypointScope>) -> Option<usize> {
    if let Some(scope) = scope.as_ref() {
    match scope {
        WaypointScope::Uuid(id) => waypoints.iter().position(|&r| *r == *id),
        WaypointScope::First => if !waypoints.is_empty() {Some(0)} else {None},
        WaypointScope::Last => if !waypoints.is_empty() {Some(waypoints.len()-1)} else {None}
    }
} else {None}
}

pub async fn create_waypoint_table(
    pool: &Pool<Sqlite>,
) -> Result<<Sqlite as sqlx::Database>::QueryResult, Error> {
    sqlx::query(
        "Create table waypoints (
            wpt_id INT PRIMARY KEY,
            x REAL NOT NULL,
            y REAL NOT NULL,
            heading REAL NOT NULL,
            is_initial_guess BOOL NOT NULL,
            translation_constrained BOOL NOT NULL,
            heading_constrained BOOL NOT NULL,
            control_interval_count INT NOT NULL
        )
    ",
    )
    .execute(pool)
    .await
}
use sqlx::{
    Error, Pool, Sqlite,
};

use crate::Managed;

#[tauri::command] 
pub async fn add_waypoint(
    handle: tauri::AppHandle,
    waypoint: Option<PartialWaypoint>
) -> Result<i64, String> {
    let _pool = handle.state::<Pool<Sqlite>>();
    let mut wpt = Waypoint::new();
    if (waypoint.is_some()) {
        wpt.apply_some(waypoint.unwrap());
    }
    let res = add_waypoint_impl(
        &_pool,
        &wpt
    )
    .await;
    match res {
        Err(sqlxErr)=> Err(format!("{}", sqlxErr)),
        Ok(new_id)=>Ok(new_id)
    }
}

#[tauri::command] 
pub async fn update_waypoint(
    handle: tauri::AppHandle,
    id: i64,
    update: PartialWaypoint
) -> Result<(), String> {
    let _pool = handle.state::<Pool<Sqlite>>();
    let res =  update_waypoint_impl(
        &_pool,
        &id,
        update
    )
    .await;
    match res {
        Err(sqlxErr)=> Err(format!("{}", sqlxErr)),
        Ok(_)=>Ok(())
    }
}

#[tauri::command] 
pub async fn get_waypoint(
    handle: tauri::AppHandle,
    id: i64
) -> Result<Waypoint, String> {
    let _pool = handle.state::<Pool<Sqlite>>();
    let res = get_waypoint_impl(
        &_pool,
        &id
    )
    .await;

    match res {
        Err(sqlxErr)=> Err(format!("{}", sqlxErr)),
        Ok(wpt)=>Ok(wpt)
    }
}


pub async fn add_waypoint_impl(
    pool: &Pool<Sqlite>,
    waypoint: &Waypoint,
) -> Result<i64, Error> {
    let new_id = wpt_id.fetch_add(1, Ordering::Relaxed);
    sqlx::query(
        "INSERT INTO waypoints
        (wpt_id, x, y, heading, is_initial_guess, translation_constrained, heading_constrained, control_interval_count) VALUES(
        ?,       ?, ?, ?,       ?,                ?,                       ?,                   ?)",
    )
    .bind(new_id)
    .bind(waypoint.x)
    .bind(waypoint.y)
    .bind(waypoint.heading)
    .bind(waypoint.is_initial_guess)
    .bind(waypoint.translation_constrained)
    .bind(waypoint.heading_constrained)
    .bind(waypoint.control_interval_count)
    .execute(pool)
    .await?;
    Ok(new_id)
}
pub async fn update_waypoint_impl(
    pool: &Pool<Sqlite>,
    id: &i64,
    waypoint: PartialWaypoint) -> Result<<Sqlite as sqlx::Database>::QueryResult, Error> {
        let mut wpt = get_waypoint_impl(pool, id).await?;
        wpt.apply_some(waypoint);
        sqlx::query(
            "UPDATE waypoints
            SET x = ?, y = ?, heading = ?,
            is_initial_guess = ?, translation_constrained = ?, heading_constrained = ?, control_interval_count = ?
            WHERE wpt_id == ?",
        )
        .bind(wpt.x)
        .bind(wpt.y)
        .bind(wpt.heading)
        .bind(wpt.is_initial_guess)
        .bind(wpt.translation_constrained)
        .bind(wpt.heading_constrained)
        .bind(wpt.control_interval_count)
        .bind(id)
        .execute(pool)
        .await
}

pub async fn get_waypoint_impl(
    pool: &Pool<Sqlite>,
    id: &i64,
) -> Result<Waypoint, Error> {
    sqlx::query_as::<Sqlite, Waypoint>(
        "SELECT x, y, heading, is_initial_guess, translation_constrained, heading_constrained, control_interval_count
        FROM waypoints WHERE wpt_id == ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}