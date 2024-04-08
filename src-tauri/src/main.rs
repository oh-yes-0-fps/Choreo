use std::sync::Mutex;

use serde::Deserialize;
use slotmap::{SlotMap, new_key_type, DefaultKey, Key, KeyData};
mod state;

use state::{waypoint::{WaypointScope, Waypoint, WaypointID}, constraint::{ConstraintScope, Constraint, ConstraintData, Constraints}, robotconfig::ChoreoRobotConfig};
use tauri::State;
use crate::state::{waypoint::scopeToWaypointID, path::Path};



#[derive(Deserialize)]
struct Managed {
    paths: Mutex<SlotMap<DefaultKey, Mutex<Path>>>
}
impl Managed {
    pub fn add_path(&self){
        self.paths.lock().unwrap().insert(Mutex::from(Path::new()));
    }

    pub fn getPathIDs(&self) -> Vec<u64>{
        self.paths.lock().unwrap().keys().map(|k| k.data().as_ffi()).collect::<Vec<u64>>()
    }
}





fn main() {
    let mut pt1 = Waypoint::new();
    let mut pt2 = Waypoint::new();
    pt2.x = 1.0;
    pt2.y = 1.0;
    let mut path = Path::new();
    let id1  = path.add_waypoint(pt1);
    path.add_waypoint(pt2);
    //let constraint = path.get_constraint(path.add_constraint(&Constraints.WptZeroVelocity));
    
    
    //path.get_constraint(constraint).scope = ConstraintScope::wpt(WaypointScope::uuid(id1));
    path.delete_waypoint(id1);
    //  {
    //     config: ChoreoRobotConfig::default(),
    //     waypoints: vec![
    //         pt2,
    //         pt1
    //     ],
    //     constraints: vec![
    //         Constraint { 
    //             scope: ConstraintScope::sgmt(
    //                 WaypointScope::first,
    //                 WaypointScope::uuid(pt1)),
    //             data: ConstraintData::WptZeroVelocity{} }
    //     ]
    // };
    
    path.generate_trajectory();

    let ser = serde_json::to_string_pretty(&path);
    
    if ser.is_ok() {
        let ser = ser.unwrap();
        println!("{}", ser);
        //println!("{:?}", scopeToWaypointID(&path.waypoints, &path.constraints.first().unwrap().scope.0));
        let newPath = serde_json::from_str::<Path>(ser.as_str());
        if newPath.is_ok() {
            //println!("{:?}", scopeToWaypointID(&path.waypoints, &path.constraints.first().unwrap().scope.1));
        }
    }

    tauri::Builder::default()
    .manage(Managed{paths: Mutex::from(SlotMap::with_key())})
    
     .invoke_handler(tauri::generate_handler![
        get_paths,
        add_path
     ])
    //     generate_trajectory,
    //     cancel,
    //     open_file_dialog,
    //     file_event_payload_from_dir,
    //     save_file,
    //     contains_build_gradle,
    //     delete_file,
    //     delete_dir,
    //     delete_traj_segments,
    //     open_file_app
    // ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]

fn add_path (state: State<Managed>) {
    state.add_path()
}

#[tauri::command]

fn get_paths (state: State<Managed>) -> Vec<u64>{
    state.getPathIDs()
}




