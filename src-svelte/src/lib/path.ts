import { derived, writable, type Subscriber, get as getStore} from "svelte/store";
import type {RemoteValue, Waypoint} from "./waypoint.js"
import { WaypointSubscribers, WaypointStore } from "./waypoint.js";
import { TauriEvent, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";


export let activePath = writable<number>(1);

export let activePathOrder = derived([activePath], ([active]) => { return PathOrder(active) });

let Paths: Record<number, RemoteValue<number[]>> = {};

export function PathOrder(id: number): RemoteValue<number[]> {
    if (Paths[id] !== undefined) {
        console.log("use path", id);
        return Paths[id];
    }
    const idsFromPoints = (points: Waypoint[]) => {
        console.log(points);
        const ids = points.map((pt) => {
            
            // If the point doesn't exist, populate it given the data from the backend
            if (WaypointSubscribers[pt.id] === undefined) {
                WaypointStore(pt);
            }
            return pt.id;
        })
        return ids;
    }
    const internal = writable<number[]>([], (set: Subscriber<number[]>) => {
        let unlisten = listen("update_path_waypoints", (e: TauriEvent) => {
            if (e.payload.id == id) {
                const points = e.payload.order as Waypoint[];

                set(idsFromPoints(points));
            }
        });
        return () => unlisten;
    });
    console.log("create path", id)

    invoke("get_path_waypoints", { id }).then(w => internal.set(idsFromPoints(w)));


    const subscribe = internal.subscribe;

    const set = (v: number[]) => {
        // invoke("update_waypoint", {
        //     id,
        //     update: {[key]:v}
        // })
        // internal.set(v);
    };

    const setNoPush = (v: number[]) => { }
    const push = () => { };

    const update = (fn: (arg0: number[]) => number[]) => set(fn(get()));

    const get = () => getStore(internal);

    //const update = (fn) => set(fn(_val));

    // We create our store as a function so that it can be passed as a callback where the value to set is the first parameter
    const store: RemoteValue<number[]> = {
        subscribe,
        set,
        get,
        update,
        setNoPush,
        push
    }
    // if (WaypointSubscribers[id] === undefined) {
    //     WaypointSubscribers[id] = {};
    // }
    // WaypointSubscribers[id][key] = store;
    Paths[id] = store;
    return store;
}