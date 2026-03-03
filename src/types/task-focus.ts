export interface TaskFocusField {
  name: string;
  value: string;
  required: boolean;
}

export interface TaskFocusSubtask {
  id: string;
  title: string;
  status: string;
  isClosed: boolean;
}

export interface TaskFocusData {
  id: string;
  customId: string | null;
  title: string;
  status: string | null;
  priority: string | null;
  description: string | null;
  pathLabel: string | null;
  dueDate: string | null;
  startDate: string | null;
  createdAt: string | null;
  updatedAt: string | null;
  assignees: string[];
  keyFields: TaskFocusField[];
  subtasks: TaskFocusSubtask[];
  completedSubtasks: number;
  totalSubtasks: number;
}
