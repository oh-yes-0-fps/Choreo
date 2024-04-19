// place files you want to import through the `$lib` alias in this folder.
import {invoke} from "@tauri-apps/api";
import { listen, TauriEvent} from "@tauri-apps/api/event";
import { writable, get as getStore, derived } from "svelte/store";
import type {Writable, Subscriber} from "svelte/store";

export let activePath = writable<number>(1);

export function PathOrder(id: number) {

    const internal = writable<Waypoint[]>([], (set: Subscriber<Waypoint[]>)=> {
        let unlisten = listen("update_path_waypoints", (e: TauriEvent)=>{
            if (e.payload.id == id) {
                set(e.payload.order);
            }
        })
    });
    console.log("create path", id)
    invoke("get_path_waypoints", {id}).then(w=>internal.set(w));
   

    const subscribe = internal.subscribe;
    
    const set = (v) => {
        // invoke("update_waypoint", {
        //     id,
        //     update: {[key]:v}
        // })
        // internal.set(v);
    };

    const get = ()=>getStore(internal);
    
    //const update = (fn) => set(fn(_val));

    // We create our store as a function so that it can be passed as a callback where the value to set is the first parameter
    function store(val) {set(val)}
    store.subscribe = subscribe;
    store.set = set;
    store.get = get;
    //store.update = update;
    // if (WaypointSubscribers[id] === undefined) {
    //     WaypointSubscribers[id] = {};
    // }
    // WaypointSubscribers[id][key] = store;
    return store;
}


let WaypointSubscribers : Record<number, Record<string, RemoteValue<any>>> = {}

type RemoteValue<T> = Writable<T> & {get: ()=>T}
export function WaypointValue<T>(id: number, key: string, init: T): RemoteValue<T> {
    if (WaypointSubscribers[id]?.[key] !== undefined) {
        return WaypointSubscribers[id][key];
    }

    const internal = writable<T>(init, (set: Subscriber<T>)=> {
        let unlisten = listen("update_waypoint", (e: TauriEvent)=>{
            if (e.payload.id == id) {
                set(e.payload.update[key]);
            }
        })
    });
    let _val = init;


   

    const subscribe = internal.subscribe;
    
    const set = (v:T) => {
        let payload = {
            id,
            update: {[key]:v}
        }
        invoke("update_waypoint", payload).catch(e=>console.error(id, key, e))
        internal.set(v);
    };

    const get = ()=>getStore(internal);
    
    const update = (fn) => set(fn(_val));

    // We create our store as a function so that it can be passed as a callback where the value to set is the first parameter
    function store(val) {set(val)}
    store.subscribe = subscribe;
    store.set = set;
    store.get = get;
    store.update = update;
    if (WaypointSubscribers[id] === undefined) {
        WaypointSubscribers[id] = {};
    }
    WaypointSubscribers[id][key] = store;
    return store;
}

export function WaypointStore(wpt:Waypoint): WaypointStore {
        let id = wpt.id
        return {
            x: WaypointValue<number>(id, "x", wpt.x),
            y: WaypointValue<number>(id, "y", wpt.y),
            heading: WaypointValue<number>(id, "heading", wpt.heading),
            is_initial_guess: WaypointValue<boolean>(id, "is_initial_guess", wpt.is_initial_guess),
            translation_constrained: WaypointValue<boolean>(id, "translation_constrained", wpt.translation_constrained),
            heading_constrained: WaypointValue<boolean>(id, "heading_constrained", wpt.heading_constrained),
            control_interval_count: WaypointValue<number>(id, "control_interval_count", wpt.control_interval_count)
        }

}
export function waypointType(pt: WaypointStore) {
    return derived([pt.is_initial_guess, pt.heading_constrained, pt.translation_constrained],
        ([guess, heading, trans])=>{
            if (guess) return 3;
            if (!heading && !trans) return 2;
            if (trans && !heading) return 1;
            return 0;
        })
}

export type WaypointStore = {
    x: RemoteValue<number>,
    y: RemoteValue<number>,
    heading: RemoteValue<number>,
    is_initial_guess: RemoteValue<boolean>,
    translation_constrained: RemoteValue<boolean>
    heading_constrained: RemoteValue<boolean>
    control_interval_count: RemoteValue<number>

}
export type Waypoint = {
    id: number,
    x: number,
    y: number,
    heading: number,
    is_initial_guess: boolean,
    translation_constrained:boolean,
    heading_constrained:boolean,
    control_interval_count: number
}

export async function add_path_waypoint(path_id: number, update: Partial<Waypoint>) {
    let newWpt = await invoke("add_path_waypoint", {id: path_id, update});
    // start the observers instead of waiting for more queries
    WaypointStore(newWpt);
    return newWpt.id;
}

export async function get_path_waypoints(path_id:number) {
    return await invoke("get_path_waypoints", {id: path_id});
}
export async function update_waypoint(id: number, update: Partial<Waypoint>) {
    invoke("update_waypoint", {id, update});
}

