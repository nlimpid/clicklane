import type { ClickUpTask } from "../../types/for-you";

const priorityBadgeMap: Record<ClickUpTask["priority"], string> = {
  urgent: "bg-[#f7dfdf] text-[#8c2525]",
  high: "bg-[#efe8d5] text-[#765d20]",
  normal: "bg-[#dfebe7] text-[#1f5e51]"
};

export function TaskRow({ task }: { task: ClickUpTask }) {
  return (
    <article className="rounded-xl border bg-[var(--card)] p-4">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <h3 className="truncate text-sm font-semibold">{task.title}</h3>
          <p className="mt-1 text-xs text-[var(--muted-foreground)] text-pretty">
            {task.list}
          </p>
        </div>
        <span
          className={`shrink-0 rounded-full px-2 py-1 text-[11px] font-semibold capitalize ${priorityBadgeMap[task.priority]}`}
        >
          {task.priority}
        </span>
      </div>

      <div className="mt-3 flex items-center justify-between text-xs text-[var(--muted-foreground)]">
        <span>{task.dueLabel ?? "No due date"}</span>
        <span className="tabular-nums">{task.estimateMinutes}m</span>
      </div>
    </article>
  );
}
