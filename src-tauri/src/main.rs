use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicI64, Ordering},
        Mutex,
    },
};

use serde::Deserialize;
use slotmap::{DefaultKey, Key, SlotMap};
mod state;

use crate::state::{
    path::Path,
    waypoint::{
        add_waypoint, add_waypoint_impl, get_waypoint, get_waypoint_impl, update_waypoint,
        update_waypoint_impl,
    },
};
use state::waypoint::{self, PartialWaypoint, Waypoint};
use tauri::{Manager, State};

#[derive(Deserialize)]
struct Managed {
    wpt_id: AtomicI64,
    paths: Mutex<SlotMap<DefaultKey, Mutex<Path>>>,
}
impl Managed {
    pub fn add_path(&self) {
        self.paths.lock().unwrap().insert(Mutex::from(Path::new()));
    }

    pub fn getPathIDs(&self) -> Vec<u64> {
        self.paths
            .lock()
            .unwrap()
            .keys()
            .map(|k| k.data().as_ffi())
            .collect::<Vec<u64>>()
    }
}

pub async fn create_tables(
    pool: &Pool<Sqlite>,
) -> Result<<Sqlite as sqlx::Database>::QueryResult, Error> {
    waypoint::create_waypoint_table(pool).await
}
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Error, Pool, Sqlite,
};

async fn test_db(handle: tauri::AppHandle) {
    let pool = handle.state::<Pool<Sqlite>>();
    let id = add_waypoint_impl(&pool, &Waypoint::new()).await;
    println!("{:?}", id);
    let second_id = id.unwrap();
    println!("{:?}", add_waypoint_impl(&pool, &Waypoint::new()).await);
    println!("{:?}", get_waypoint_impl(&pool, &second_id).await);

    // let mut update = PartialWaypoint::default();
    // update.x=Some(1.0f64);
    let update = serde_json::from_str::<PartialWaypoint>("{\"y\":1.0}").unwrap();
    println!(
        "{:?}",
        update_waypoint_impl(&pool, &second_id, update).await
    );
    println!("{:?}", get_waypoint_impl(&pool, &second_id).await);
}
fn main() {
    /*
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
    */
    tauri::Builder::default()
        .manage(Managed {
            wpt_id: AtomicI64::new(5),
            paths: Mutex::from(SlotMap::with_key()),
        })
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let sqlite_opts = SqliteConnectOptions::from_str(":memory:").unwrap();

                // min_connections = 3 to prevent the DB from being wiped randomly
                let pool = SqlitePoolOptions::new()
                    .min_connections(3)
                    .max_connections(10) // default is 10
                    .connect_with(sqlite_opts)
                    .await
                    .unwrap();

                println!("{:?}", create_tables(&pool).await);
                handle.manage(pool);
                test_db(handle).await;
            });
            Ok(())
            // define in memory DB connection options
        })
        .invoke_handler(tauri::generate_handler![
            get_paths,
            add_path,
            add_waypoint,
            update_waypoint,
            get_waypoint
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

fn add_path(state: State<Managed>) {
    state.add_path()
}

#[tauri::command]

fn get_paths(state: State<Managed>) -> Vec<u64> {
    state.getPathIDs()
}
