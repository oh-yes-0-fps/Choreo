import { invoke } from "@tauri-apps/api";
import { Expr, Project, RobotConfig, Traj } from "./2025/DocumentTypes";
import { OpenFilePayload } from "./DocumentManager";

export const Commands = {
  generate: (project: Project, traj: Traj, handle: number) => invoke<Traj>("generate", { project, traj, handle }),
  guessIntervals: (config: RobotConfig<Expr>, traj: Traj) =>
    invoke<number[]>("guess_control_interval_counts", { config, traj }),
  cancel: () => invoke<void>("cancel"),

  /**
   * Deletes the specified file from the specified directory.
   * 
   * @param dir The directory that the file is in.
   * @param name The name of the file to delete.
   * @returns `void`
   */
  deleteFile: (dir: string, name: string) =>
    invoke<void>("delete_file", { dir, name }),
  /**
   * Deletes the specified directory and all of its contents.
   * 
   * @param dir The directory to delete.
   * @returns `void`
   */
  deleteDir: (dir: string) => invoke<void>("delete_dir", { dir }),

  /**
   * Opens the specified directory in the system's file explorer.
   * 
   * @param path The path of the directory to open.
   * @returns `void`
   */
  openInExplorer: (path: string) => invoke<void>("open_in_explorer", { path }),
  /**
   * Opens a file dialog for the user to select a file to open, only permits `.chor` files.
   * 
   * @returns The path of the file that the user selected, or `null` if the user canceled the dialog.
   */
  openProjectDialog: () => invoke<OpenFilePayload>("open_project_dialog"),

  /**
   * Sets an application-wide directory path that will be used as the root for all file operations.
   * 
   * @param dir The directory path to set as the root.
   * @returns `void`
   */
  setDeployRoot: (dir: string) => invoke<void>("set_deploy_root", { dir }),
  /**
   * Gets the application-wide directory path that is used as the root for all file operations.
   * 
   * @returns The directory path that is set as the root.
   */
  getDeployRoot: () => invoke<string>("get_deploy_root"),

  /**
   * @returns The default `Project` that is loaded when a new `Project` is created.
   */
  defaultProject: () => invoke<Project>("default_project"),
  /**
   * Reads the `Project` with the specified name from the deploy root directory.
   * 
   * @param name The name of the `Project` to read without the `.chor` extension.
   * @returns The `Project` that was read.
   */
  readProject: (name: string) => invoke<Project>("read_project", { name }),
  /**
   * Writes the specified `Project` to the deploy root directory.
   * 
   * @param project The `Project` to write.
   * @returns `void`
   */
  writeProject: (project: Project) => invoke<void>("write_project", { project }),

  /**
   * Reads the `Traj` with the specified name from the deploy root directory.
   * 
   * @param name The name of the `Traj` to read without the `.traj` extension.
   * @returns The `Traj` that was read.
   */
  readTraj: (name: string) => invoke<Traj>("read_traj", { name }),
  /**
   * Scans the deploy root directory for all of the `Traj` files and returns them.
   * 
   * @returns All of the `Traj` files in the deploy root directory.
   */
  readAllTraj: () => invoke<Traj[]>("read_all_traj"),
  /**
   * Writes the specified `Traj` to the deploy root directory.
   * 
   * @param traj The `Traj` to write.
   * @returns `void`
   */
  writeTraj: (traj: Traj) => invoke("write_traj", { traj }),
  /**
   * Renames the specified `Traj` to the specified name.
   * 
   * The old `Traj` should not be used again after this operation.
   * 
   * @param oldTraj The `Traj` to rename.
   * @param newName The new name for the `Traj`.
   * @returns The renamed `Traj`.
   */
  renameTraj: (oldTraj: Traj, newName: string) => invoke<Traj>("rename_traj", { oldTraj, newName }),

  /**
   * If the application was opened via CLI and a file was specified, this will return the path of that file.
   * 
   * @returns The path of the file that was opened via CLI, or `null` if no file was specified.
   */
  requestedProject: () => invoke<OpenFilePayload | null>("requested_file"),
};
