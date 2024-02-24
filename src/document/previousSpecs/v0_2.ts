/** CHANGES from v0.1.1:
 * Replaced wheelMaxVelocity and wheelMaxTorque with wheelGearing, motorMaxVelocity, motorMaxTorque, currentLimit, efficiency.
 */

export const SAVE_FILE_VERSION = "v0.2";
export interface SavedWaypoint {
  x: number;
  y: number;
  heading: number;
  isInitialGuess: boolean;
  translationConstrained: boolean;
  headingConstrained: boolean;
  controlIntervalCount: number; // positive-integer>
}
export interface SavedTrajectorySample {
  timestamp: number; //positive
  x: number;
  y: number;
  heading: number;
  velocityX: number;
  velocityY: number;
  angularVelocity: number;
}
export interface SavedPath {
  waypoints: Array<SavedWaypoint>;
  trajectory: Array<SavedTrajectorySample> | null;
  constraints: Array<SavedConstraint>;
  usesDefaultFieldObstacles: boolean;
  usesControlIntervalGuessing: boolean;
  defaultControlIntervalCount: number;
  circleObstacles: Array<SavedCircleObstacle>;
}
export interface SavedPathList extends Record<string, SavedPath> {}
export interface SavedRobotConfig {
  mass: number;
  rotationalInertia: number;
  wheelbase: number;
  trackWidth: number;
  wheelRadius: number;
  motorMaxVelocity: number;
  motorMaxTorque: number;
  gearing: number;
  bumperLength: number;
  bumperWidth: number;
}
export interface SavedDocument {
  version: typeof SAVE_FILE_VERSION;
  robotConfiguration: SavedRobotConfig;
  paths: SavedPathList;
}
export type SavedWaypointId = "first" | "last" | number;
export interface SavedConstraint {
  scope: Array<SavedWaypointId>;
  type: string;
  [key: string]: unknown;
}
export interface SavedCircleObstacle {
  x: number;
  y: number;
  radius: number;
}
