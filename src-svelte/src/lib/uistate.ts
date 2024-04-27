import { writable } from "svelte/store";
import SquareCircle from "virtual:icons/mdi/square-circle";
import Circle from "virtual:icons/mdi/circle";
import CircleOutline from "virtual:icons/mdi/circle-outline";
import InitialGuess from "virtual:icons/mdi/help-circle-outline";

export let fieldScalingFactor = writable(1);

/* Navbar stuff */
export const WaypointData: {
    [key: string]: {
      index: number;
      name: string;
      icon: any;
    };
  } = {
    FullWaypoint: {
      index: 0,
      name: "Pose Waypoint",
      icon: SquareCircle
    },
    TranslationWaypoint: {
      index: 1,
      name: "Translation Waypoint",
      icon: Circle
    },
    EmptyWaypoint: {
      index: 2,
      name: "Empty Waypoint",
      icon: CircleOutline
    },
    InitialGuessPoint: {
      index: 3,
      name: "Initial Guess Point",
      icon: InitialGuess
    }
  };
  const NavbarData: {
    [key: string]: {
      index: number;
      name: string;
      icon: any;
    };
  } = Object.assign({}, WaypointData);
  const waypointNavbarCount = Object.keys(NavbarData).length;
//   const constraintsIndices: number[] = [];
//   const navbarIndexToConstraint: { [key: number]: typeof ConstraintStore } = {};
//   const navbarIndexToConstraintDefinition: {
//     [key: number]: ConstraintDefinition;
//   } = {};
//   {
//     let constraintsOffset = Object.keys(NavbarData).length;
//     Object.entries(constraints).forEach(([key, data], index) => {
//       NavbarData[key] = {
//         index: constraintsOffset,
//         name: data.name,
//         icon: data.icon
//       };
//       navbarIndexToConstraint[constraintsOffset] = ConstraintStores[key];
//       navbarIndexToConstraintDefinition[constraintsOffset] = data;
//       constraintsIndices.push(constraintsOffset);
//       constraintsOffset++;
//     });
//   }
//   const constraintNavbarCount = Object.keys(constraints).length;
//   export const ObstacleData: {
//     [key: string]: {
//       index: number;
//       name: string;
//       icon: ReactElement;
//     };
//   } = {
//     CircleObstacle: {
//       index: Object.keys(NavbarData).length,
//       name: "Circular Obstacle",
//       icon: <DoNotDisturb />
//     }
//   };
//   let obstacleNavbarCount = 0;
//   obstacleNavbarCount = Object.keys(ObstacleData).length;
//   Object.entries(ObstacleData).forEach(([name, data]) => {
//     const obstaclesOffset = Object.keys(NavbarData).length;
//     NavbarData[name] = {
//       index: obstaclesOffset,
//       name: data.name,
//       icon: data.icon
//     };
//   });
  
//   const eventMarkerCount = 1;
//   NavbarData.EventMarker = {
//     index: Object.keys(NavbarData).length,
//     name: "Event Marker",
//     icon: <Room></Room>
//   };
  
  /** An map of  */
  export const NavbarLabels = (() => {
    const x: { [key: string]: number } = {};
    Object.entries(NavbarData).forEach(([key, data], index) => {
      x[key] = index;
    });
    return x;
  })();
  
  /** An array of name-and-icon objects for the navbar */
  export const NavbarItemData = (() => {
    const x: Array<{ name: string; icon: any }> = [];
    Object.entries(NavbarData).forEach(([key, data], index) => {
      x[data.index] = { name: data.name, icon: data.icon };
    });
    return x;
  })();
  console.log(NavbarItemData);
  
  const NavbarItemSections = [waypointNavbarCount]//, constraintNavbarCount];
//   NavbarItemSections.push(obstacleNavbarCount);
//   NavbarItemSections.push(eventMarkerCount);
  
  export const NavbarItemSectionEnds = NavbarItemSections.map((s, idx) =>
    NavbarItemSections.slice(0, idx + 1).reduce((prev, cur) => prev + cur, -1)
  );
  console.log(NavbarItemSectionEnds);
