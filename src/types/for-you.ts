export type TaskState = "today" | "upcoming" | "blocked";
export type TaskPriority = "urgent" | "high" | "normal";

export interface ClickUpTask {
  id: string;
  title: string;
  list: string;
  priority: TaskPriority;
  estimateMinutes: number;
  state: TaskState;
  dueLabel?: string;
}

export interface ForYouInsight {
  title: string;
  reason: string;
  actionLabel: string;
}

export interface ForYouResult {
  summary: string;
  insights: ForYouInsight[];
}
