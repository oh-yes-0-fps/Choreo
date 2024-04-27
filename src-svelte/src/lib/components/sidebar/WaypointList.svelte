<script lang="ts">
    import { type, deletePathWaypoint, type Waypoint, getWaypoint} from "$lib/waypoint.js";
import styles from "./Sidebar.module.css";
import SidebarWaypoint from "./SidebarWaypoint.svelte"
export let pathId;
export let pathOrderStore;
$: waypoints = $pathOrderStore;
console.log("wpts", waypoints);
const getListStyle = (isDraggingOver: boolean) => ({
  outline: isDraggingOver ? "2px solid var(--darker-purple)" : "transparent"
});
const onDragEnd = (result: any) => {
    // dropped outside the list
    // if (!result.destination) {
    //   return;
    // }
    reorder(result.source.index, result.destination.index);
  }
  const reorder = (startIndex: number, endIndex: number) => {
    // this.context.model.document.pathlist.activePath.reorder(
    //   startIndex,
    //   endIndex
    // );
  }
  const getIssue = (holonomicWaypoint: Waypoint, index: number) => {
    let issue = "";
    if (holonomicWaypoint.is_initial_guess) {
    if (index == 0) {
        issue = "Cannot start with an initial guess point.";
    } else if (index == waypoints.length - 1) {
        issue = "Cannot end with an initial guess point.";
    }
    }
    if (type(holonomicWaypoint) == 2) {
    if (index == 0) {
        issue = "Cannot start with an empty waypoint.";
    } else if (index == waypoints.length - 1) {
        issue = "Cannot end with an empty waypoint.";
    }
    }
    return issue;
  }

</script>
<div>
    {#each waypoints as id, i}
    {@const point = getWaypoint(id)}
    {#if point !== undefined}
    <SidebarWaypoint
    waypoint={point}
    index={i}
    issue={getIssue(point, i)}
    pathLength={waypoints.length}
    handleDelete={(id)=>deletePathWaypoint(pathId, id)}
  ></SidebarWaypoint>
  {/if}
  {/each}
</div>

