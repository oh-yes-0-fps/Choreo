// place files you want to import through the `$lib` alias in this folder.
import { invoke } from "@tauri-apps/api";
import { listen, TauriEvent } from "@tauri-apps/api/event";
import type { Event } from "@tauri-apps/api/event";
import { writable, get as getStore, derived } from "svelte/store";
import type { Writable, Subscriber } from "svelte/store";
import { NavbarItemData } from "./uistate.js";

export function deletePathWaypoint(pathId: number, wptId: number) {
    invoke("delete_path_waypoint", {pathId, wptId});
}

export let WaypointSubscribers: Record<number, WaypointStore> = {}

export type RemoteValue<T> = Writable<T> & {
    get: () => T,
    setNoPush: (arg: T) => void,
    push: () => void
}

type UpdateWaypointPayload = {
    id: number;
    update: Partial<Waypoint>
}

function handleUpdate<K extends keyof WaypointNoID>(id: number, key: K, val: WaypointNoID[K]) {
    const pt = WaypointSubscribers[id];
    // this one is typed according to the key
    if (pt === undefined) {
        invoke("get_waypoint", { id }).then((pt: Waypoint) => {
            WaypointSubscribers[id] = WaypointStore(pt)
        })
    } else {

        let store = pt[key];
        store.setNoPush(val);
    }
}
// Set up a listener to update waypoint value stores, and create new ones if necessary
listen<UpdateWaypointPayload>("update_waypoint", (e: Event<UpdateWaypointPayload>) => {
    const id = e.payload.id;
    for (let key in Object.keys(e.payload.update)) {
        if (key != "id") {
            let k = key as keyof WaypointNoID;
            let val = e.payload.update[k] as WaypointNoID[typeof k];
            handleUpdate(id, k, val);
        }
    }
});
type WaypointNoID = Omit<Waypoint, "id">
export function WaypointValue<K extends keyof WaypointNoID>(id: number, key: K, init: WaypointNoID[K]): RemoteValue<WaypointNoID[K]> {
    type T = WaypointNoID[K];
    const preExisting = WaypointSubscribers[id]?.[key]
    if (preExisting !== undefined) {
        return preExisting;
    }

    const internal = writable<T>(init);
    let _val = init;


    const setNoPush = (v: T) => {
        _val = v;
        internal.set(v);
    }
    const push = () => {
        let payload = {
            id,
            update: { [key]: _val }
        }
        invoke("update_waypoint", payload).catch(e => console.error(id, key, e))
    }

    const subscribe = internal.subscribe;

    const set = (v: T) => {
        setNoPush(v);
        push();
    };

    const get = () => _val;

    const update = (fn: (arg: T) => T) => set(fn(_val));

    let store: RemoteValue<WaypointNoID[K]> = {
        subscribe,
        set,
        get,
        update,
        setNoPush,
        push
    }
    // The relationship between the value of `key` and the generic type of the remote value
    return store;
}

export function WaypointStore(wpt: Waypoint): WaypointStore {
    let id = wpt.id
    if (WaypointSubscribers[id] !== undefined) {
        return WaypointSubscribers[id];
    }
    console.log(wpt);
    
    let store = {
        x: WaypointValue<"x">(id, "x", wpt.x),
        y: WaypointValue<"y">(id, "y", wpt.y),
        heading: WaypointValue<"heading">(id, "heading", wpt.heading),
        is_initial_guess: WaypointValue<"is_initial_guess">(id, "is_initial_guess", wpt.is_initial_guess),
        translation_constrained: WaypointValue<"translation_constrained">(id, "translation_constrained", wpt.translation_constrained),
        heading_constrained: WaypointValue<"heading_constrained">(id, "heading_constrained", wpt.heading_constrained),
        control_interval_count: WaypointValue<"control_interval_count">(id, "control_interval_count", wpt.control_interval_count)
    }
    WaypointSubscribers[id] = store;
    return store;

}
export function waypointType(pt: WaypointStore) {
    return derived([pt.is_initial_guess, pt.heading_constrained, pt.translation_constrained],
        ([guess, heading, trans]) => {
            if (guess) return 3;
            if (!heading && !trans) return 2;
            if (trans && !heading) return 1;
            return 0;
        })
}

export type WaypointStore = { [K in keyof WaypointNoID]: RemoteValue<WaypointNoID[K]> }
export type Waypoint = {
    id: number,
    x: number,
    y: number,
    heading: number,
    is_initial_guess: boolean,
    translation_constrained: boolean,
    heading_constrained: boolean,
    control_interval_count: number
}

export function getWaypoint(id: number) : Waypoint | undefined {
    let store = WaypointSubscribers[id];
    if (store === undefined) {return undefined;}
    return {
        id,
        x: store.x.get(),
        y: store.y.get(),
        heading: store.heading.get(),
        is_initial_guess: store.is_initial_guess.get(),
        translation_constrained: store.translation_constrained.get(),
        heading_constrained: store.heading_constrained.get(),
        control_interval_count: store.control_interval_count.get()
    }
}


export function type(point: Waypoint) {
    if (point.is_initial_guess) { return 3 }
    if ((!point.heading_constrained) && (!point.translation_constrained)) { return 2; }
    if (!point.heading_constrained) { return 1; }
    return 0;
}
export function typeName(point: Waypoint) {
    return NavbarItemData[type(point)].name;
}

export async function add_path_waypoint(path_id: number, update: Partial<Waypoint>) {
    let newWpt = await invoke("add_path_waypoint", { id: path_id, update });
    // start the observers instead of waiting for more queries
    WaypointStore(newWpt);
    return newWpt.id;
}

export async function get_path_waypoints(path_id: number) {
    return await invoke("get_path_waypoints", { id: path_id });
}
export async function update_waypoint(id: number, update: Partial<Waypoint>) {
    invoke("update_waypoint", { id, update });
}

