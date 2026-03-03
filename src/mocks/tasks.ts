import type { ClickUpTask } from "../types/for-you";

export const seedTasks: ClickUpTask[] = [
  {
    id: "task-101",
    title: "Ship desktop timeline filter to beta users",
    list: "Desktop Experience",
    priority: "urgent",
    estimateMinutes: 45,
    state: "today",
    dueLabel: "Due in 3h"
  },
  {
    id: "task-102",
    title: "Draft migration notes from Electron to Tauri",
    list: "Engineering Notes",
    priority: "high",
    estimateMinutes: 35,
    state: "today",
    dueLabel: "Due today"
  },
  {
    id: "task-103",
    title: "Refine push notification permission copy",
    list: "UX Writing",
    priority: "normal",
    estimateMinutes: 20,
    state: "upcoming",
    dueLabel: "Tomorrow"
  },
  {
    id: "task-104",
    title: "Resolve flaky sync test in CI",
    list: "Infrastructure",
    priority: "high",
    estimateMinutes: 60,
    state: "blocked",
    dueLabel: "Blocked by API key rollout"
  }
];
