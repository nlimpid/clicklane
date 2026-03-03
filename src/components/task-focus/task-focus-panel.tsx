import type { FormEvent } from "react";
import { useState } from "react";

import { cn } from "../../lib/cn";
import { hasClickUpSettings } from "../../lib/clickup-settings";
import { fetchTaskFocus } from "../../lib/task-focus-api";
import type { ClickUpSettings } from "../../types/clickup-settings";
import type { TaskFocusData } from "../../types/task-focus";
import { Button } from "../ui/button";
import { Card } from "../ui/card";
import { Skeleton } from "../ui/skeleton";

const dateFormatter = new Intl.DateTimeFormat(undefined, {
  month: "short",
  day: "numeric",
  hour: "2-digit",
  minute: "2-digit"
});

function fieldTone(required: boolean) {
  return required
    ? "border-[var(--border)] bg-[var(--muted)]"
    : "border-[var(--border)] bg-[var(--card)]";
}

function priorityTone(priority: string | null) {
  switch (priority?.toLowerCase()) {
    case "urgent":
      return "bg-[#f7dfdf] text-[#8c2525]";
    case "high":
      return "bg-[#efe8d5] text-[#765d20]";
    case "normal":
      return "bg-[#dfebe7] text-[#1f5e51]";
    default:
      return "bg-[var(--muted)] text-[var(--muted-foreground)]";
  }
}

function statusTone(status: string | null) {
  const normalized = status?.toLowerCase() ?? "";

  if (normalized.includes("block")) {
    return "bg-[#f7dfdf] text-[#8c2525]";
  }
  if (normalized.includes("done") || normalized.includes("close")) {
    return "bg-[#dfebe7] text-[#1f5e51]";
  }
  if (normalized.includes("progress") || normalized.includes("test")) {
    return "bg-[#efe8d5] text-[#765d20]";
  }

  return "bg-[var(--muted)] text-[var(--muted-foreground)]";
}

function formatClickUpTimestamp(value: string | null) {
  if (!value) {
    return "Not set";
  }

  const numericValue = Number(value);
  const date = Number.isFinite(numericValue)
    ? new Date(numericValue)
    : new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return dateFormatter.format(date);
}

function summaryCopy(task: TaskFocusData) {
  if (task.description) {
    if (task.description.length <= 360) {
      return task.description;
    }

    return `${task.description.slice(0, 357).trimEnd()}...`;
  }

  if (task.keyFields.length > 0) {
    return "Description is empty, so the panel is emphasizing the populated decision fields instead.";
  }

  return "This task has no description yet. Use the signals and subtask chain to drive the next execution step.";
}

interface TaskFocusPanelProps {
  settings: ClickUpSettings;
  onOpenSettings: () => void;
}

export function TaskFocusPanel({
  settings,
  onOpenSettings
}: TaskFocusPanelProps) {
  const [taskId, setTaskId] = useState(() =>
    (import.meta.env.VITE_CLICKUP_TEST_TASK_ID ?? "").trim()
  );
  const [task, setTask] = useState<TaskFocusData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const settingsReady = hasClickUpSettings(settings);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (isLoading) {
      return;
    }

    setIsLoading(true);
    setErrorMessage(null);

    try {
      const nextTask = await fetchTaskFocus(settings, taskId);
      setTask(nextTask);
    } catch (error) {
      setErrorMessage(
        error instanceof Error ? error.message : "Task lookup failed. Please try again."
      );
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <Card className="space-y-5">
      <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
        <div>
          <p className="text-xs font-semibold uppercase text-[var(--muted-foreground)]">
            Live task
          </p>
          <h2 className="mt-2 text-2xl font-semibold text-balance">Task focus</h2>
          <p className="mt-2 max-w-2xl text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
            Pull only the execution-critical details from ClickUp: decision fields,
            real owners, and the subtask chain you need to unblock next.
          </p>
        </div>

        <div className="rounded-xl border bg-[var(--muted)] px-3 py-2 text-xs font-medium text-[var(--muted-foreground)]">
          {task ? "Synced from ClickUp" : "Ready for a live lookup"}
        </div>
      </div>

      <form
        className="grid gap-3 rounded-xl border bg-[var(--background)] p-4 md:grid-cols-[minmax(0,1fr)_auto]"
        onSubmit={(event) => void handleSubmit(event)}
      >
        <label className="space-y-2 text-sm font-medium text-[var(--foreground)]">
          <span>Task ID</span>
          <input
            className="h-11 w-full rounded-lg border bg-[var(--card)] px-3 text-sm text-[var(--foreground)] outline-none"
            onChange={(event) => setTaskId(event.target.value)}
            placeholder="Task ID or custom ID"
            value={taskId}
          />
        </label>

        <div className="flex items-end">
          <Button
            className="w-full md:w-auto"
            disabled={isLoading || !settingsReady}
            type="submit"
          >
            {isLoading ? "Loading task..." : "Load live task"}
          </Button>
        </div>
      </form>

      {!settingsReady ? (
        <div className="rounded-xl border bg-[var(--background)] p-4">
          <p className="text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
            Configure your ClickUp token and workspace in Settings before loading a
            live task. The workspace ID stays hidden outside that page.
          </p>
          <Button
            className="mt-3"
            onClick={onOpenSettings}
            type="button"
            variant="ghost"
          >
            Open settings
          </Button>
        </div>
      ) : null}

      {errorMessage ? (
        <p className="text-sm text-[var(--danger)]">{errorMessage}</p>
      ) : null}

      {isLoading ? (
        <div className="grid gap-6 xl:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]">
          <div className="space-y-4">
            <Skeleton className="h-36 w-full" />
            <Skeleton className="h-44 w-full" />
          </div>
          <div className="space-y-4">
            <Skeleton className="h-52 w-full" />
            <Skeleton className="h-52 w-full" />
          </div>
        </div>
      ) : task ? (
        <div className="grid gap-6 xl:grid-cols-[minmax(0,1.15fr)_minmax(0,0.85fr)]">
          <div className="space-y-5">
            <section className="rounded-2xl border bg-[var(--background)] p-5">
              <div className="flex flex-wrap items-center gap-2">
                {task.customId ? (
                  <span className="rounded-full bg-[var(--accent)] px-2 py-1 text-xs font-semibold text-[var(--accent-foreground)]">
                    {task.customId}
                  </span>
                ) : null}
                <span className="rounded-full bg-[var(--muted)] px-2 py-1 text-xs font-medium text-[var(--muted-foreground)]">
                  Internal ID {task.id}
                </span>
                {task.pathLabel ? (
                  <span className="rounded-full bg-[var(--card)] px-2 py-1 text-xs font-medium text-[var(--muted-foreground)]">
                    {task.pathLabel}
                  </span>
                ) : null}
              </div>

              <h3 className="mt-4 text-2xl font-semibold leading-tight text-balance">
                {task.title}
              </h3>
              <p className="mt-3 text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
                {summaryCopy(task)}
              </p>
            </section>

            <section className="rounded-2xl border p-5">
              <div className="flex items-center justify-between gap-3">
                <div>
                  <h3 className="text-lg font-semibold text-balance">Execution chain</h3>
                  <p className="mt-1 text-sm text-[var(--muted-foreground)] text-pretty">
                    Keep the real subtask path visible without dragging the whole
                    ClickUp table into the desktop app.
                  </p>
                </div>
                <span className="rounded-lg bg-[var(--muted)] px-2 py-1 text-xs font-medium text-[var(--muted-foreground)] tabular-nums">
                  {task.completedSubtasks}/{task.totalSubtasks} closed
                </span>
              </div>

              <div className="mt-4 space-y-2">
                {task.subtasks.length > 0 ? (
                  task.subtasks.slice(0, 6).map((subtask) => (
                    <div
                      key={subtask.id}
                      className="flex items-start justify-between gap-3 rounded-xl border bg-[var(--card)] px-3 py-3"
                    >
                      <div className="min-w-0">
                        <p className="truncate text-sm font-medium">{subtask.title}</p>
                        <p className="mt-1 text-xs text-[var(--muted-foreground)]">
                          {subtask.id}
                        </p>
                      </div>
                      <span
                        className={cn(
                          "shrink-0 rounded-full px-2 py-1 text-[11px] font-semibold",
                          subtask.isClosed
                            ? "bg-[#dfebe7] text-[#1f5e51]"
                            : "bg-[var(--muted)] text-[var(--muted-foreground)]"
                        )}
                      >
                        {subtask.status}
                      </span>
                    </div>
                  ))
                ) : (
                  <div className="rounded-xl border bg-[var(--card)] p-4">
                    <p className="text-sm text-[var(--muted-foreground)] text-pretty">
                      No subtasks yet. The parent task is the only execution unit right now.
                    </p>
                  </div>
                )}

                {task.subtasks.length > 6 ? (
                  <p className="text-xs text-[var(--muted-foreground)] tabular-nums">
                    {task.subtasks.length - 6} more subtasks hidden to keep the view focused.
                  </p>
                ) : null}
              </div>
            </section>
          </div>

          <div className="space-y-5">
            <section className="rounded-2xl border p-5">
              <h3 className="text-lg font-semibold text-balance">Signals</h3>
              <div className="mt-4 grid gap-3 sm:grid-cols-2">
                <div className="rounded-xl border bg-[var(--card)] p-3">
                  <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                    Status
                  </p>
                  <span
                    className={cn(
                      "mt-2 inline-flex rounded-full px-2 py-1 text-[11px] font-semibold",
                      statusTone(task.status)
                    )}
                  >
                    {task.status ?? "Not set"}
                  </span>
                </div>

                <div className="rounded-xl border bg-[var(--card)] p-3">
                  <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                    Priority
                  </p>
                  <span
                    className={cn(
                      "mt-2 inline-flex rounded-full px-2 py-1 text-[11px] font-semibold capitalize",
                      priorityTone(task.priority)
                    )}
                  >
                    {task.priority ?? "Unspecified"}
                  </span>
                </div>

                <div className="rounded-xl border bg-[var(--card)] p-3">
                  <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                    Assignees
                  </p>
                  <p className="mt-2 text-sm font-medium text-pretty">
                    {task.assignees.length > 0
                      ? task.assignees.join(", ")
                      : "No assignee"}
                  </p>
                </div>

                <div className="rounded-xl border bg-[var(--card)] p-3">
                  <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                    Due
                  </p>
                  <p className="mt-2 text-sm font-medium tabular-nums">
                    {formatClickUpTimestamp(task.dueDate)}
                  </p>
                </div>

                <div className="rounded-xl border bg-[var(--card)] p-3">
                  <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                    Start
                  </p>
                  <p className="mt-2 text-sm font-medium tabular-nums">
                    {formatClickUpTimestamp(task.startDate)}
                  </p>
                </div>

                <div className="rounded-xl border bg-[var(--card)] p-3">
                  <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                    Last updated
                  </p>
                  <p className="mt-2 text-sm font-medium tabular-nums">
                    {formatClickUpTimestamp(task.updatedAt ?? task.createdAt)}
                  </p>
                </div>
              </div>
            </section>

            <section className="rounded-2xl border p-5">
              <div className="flex items-center justify-between gap-3">
                <div>
                  <h3 className="text-lg font-semibold text-balance">Decision data</h3>
                  <p className="mt-1 text-sm text-[var(--muted-foreground)] text-pretty">
                    Populated custom fields only, so you can scan the high-signal data first.
                  </p>
                </div>
                <span className="rounded-lg bg-[var(--muted)] px-2 py-1 text-xs font-medium text-[var(--muted-foreground)] tabular-nums">
                  {task.keyFields.length} shown
                </span>
              </div>

              <div className="mt-4 grid gap-3">
                {task.keyFields.length > 0 ? (
                  task.keyFields.map((field) => (
                    <div
                      key={`${field.name}-${field.value}`}
                      className={cn("rounded-xl border p-3", fieldTone(field.required))}
                    >
                      <div className="flex items-start justify-between gap-3">
                        <p className="text-xs font-medium uppercase text-[var(--muted-foreground)]">
                          {field.name}
                        </p>
                        {field.required ? (
                          <span className="rounded-full bg-[var(--card)] px-2 py-1 text-[10px] font-semibold text-[var(--muted-foreground)]">
                            Required
                          </span>
                        ) : null}
                      </div>
                      <p className="mt-2 text-sm font-medium text-pretty">{field.value}</p>
                    </div>
                  ))
                ) : (
                  <div className="rounded-xl border bg-[var(--card)] p-4">
                    <p className="text-sm text-[var(--muted-foreground)] text-pretty">
                      No populated custom fields came back for this task.
                    </p>
                  </div>
                )}
              </div>
            </section>
          </div>
        </div>
      ) : (
        <div className="rounded-2xl border bg-[var(--background)] p-5">
          <p className="text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
            Enter a task ID, then load a live task. The panel keeps the view narrow
            on purpose: key signals, decision data, and the execution chain,
            without mirroring ClickUp’s entire form.
          </p>
        </div>
      )}
    </Card>
  );
}
