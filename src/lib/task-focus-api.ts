import { hasClickUpSettings } from "./clickup-settings";
import type { ClickUpSettings } from "../types/clickup-settings";
import type { TaskFocusData } from "../types/task-focus";

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

function isDesktopRuntime() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function fetchTaskFocus(
  settings: ClickUpSettings,
  taskId: string
): Promise<TaskFocusData> {
  if (!hasClickUpSettings(settings)) {
    throw new Error("Configure ClickUp token and workspace in Settings first.");
  }

  const parsedWorkspaceId = Number(settings.workspaceId.trim());
  if (!Number.isInteger(parsedWorkspaceId) || parsedWorkspaceId <= 0) {
    throw new Error("Workspace ID must be a positive number.");
  }

  const normalizedTaskId = taskId.trim();
  if (!normalizedTaskId) {
    throw new Error("Task ID is required.");
  }

  if (!isDesktopRuntime()) {
    throw new Error("Live task details are available in the Tauri desktop app.");
  }

  const { invoke } = await import("@tauri-apps/api/core");

  return invoke<TaskFocusData>("get_task_focus", {
    workspaceId: parsedWorkspaceId,
    taskId: normalizedTaskId,
    authToken: settings.authToken
  });
}
