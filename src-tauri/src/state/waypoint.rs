new_key_type! {
    pub struct WaypointID;
}
use serde_with::serde_as;
#[serde_as]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum WaypointScope { 
    
    uuid(
        #[serde_as(as="WaypointIDFFI")]
        WaypointID
    ),
    first,
    last
}

    use serde::{Deserializer, Serializer, Deserialize, Serialize};
    use slotmap::{Key, KeyData};

serde_with::serde_conv!(
    pub(crate) WaypointIDFFI,
    WaypointID,
    |id: &WaypointID| id.data().as_ffi(),
    |value: u64| -> Result<WaypointID, std::convert::Infallible> {
        Ok(WaypointID::from(KeyData::from_ffi(value)))
    }
);
// impl Serialize for WaypointID{
//     fn serialize<S>(
//         id: &WaypointID,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
        
//         serializer.serialize_u64(id.data().as_ffi())
//     }
// }
// impl Deserialize<'_> for WaypointID{
//     fn deserialize<'de, D>(
//         deserializer: D,
//     ) -> Result<WaypointID, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         let dt = s.parse::<u64>().unwrap();
//         Ok(WaypointID::from(KeyData::from_ffi(dt)))
//     }
// }

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub isInitialGuess: bool,
    pub translationConstrained: bool,
    pub headingConstrained: bool,
    pub controlIntervalCount: usize
}

use std::{sync::atomic::{AtomicU64, Ordering}, collections::HashMap, f64::consts::PI};

use slotmap::{new_key_type};
impl Waypoint {
    pub fn new() -> Self {

        Waypoint {
            x: 0.0,
            y: 0.0,
            heading: 0.0,
            isInitialGuess: false,
            translationConstrained: true,
            headingConstrained: true,
            controlIntervalCount: 40
        }
    }
    pub fn set_translation(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y=y;
        self
    }
}


pub fn scopeToWaypointID<'a>(waypoints: &'a Vec<WaypointID>, scope: & Option<WaypointScope>) -> Option<&'a WaypointID> {
    if let Some(scope) = scope.as_ref() {
    match scope {
        WaypointScope::uuid(id) => waypoints.iter().find(|&r| *r == *id),
        WaypointScope::first => waypoints.first(),
        WaypointScope::last => waypoints.last()
    }
} else {None}
}

pub fn scopeToPosition<'a>(waypoints: &'a Vec<&WaypointID>, scope: & Option<WaypointScope>) -> Option<usize> {
    if let Some(scope) = scope.as_ref() {
    match scope {
        WaypointScope::uuid(id) => waypoints.iter().position(|&r| *r == *id),
        WaypointScope::first => if waypoints.len() > 0 {Some(0)} else {None},
        WaypointScope::last => if waypoints.len() > 0 {Some(waypoints.len()-1)} else {None}
    }
} else {None}
}