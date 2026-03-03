import { Tabs } from "@base-ui/react/tabs";

import { cn } from "../../lib/cn";
import type { TaskState } from "../../types/for-you";

interface TaskTabsProps {
  value: TaskState;
  todayCount: number;
  upcomingCount: number;
  blockedCount: number;
  onValueChange: (value: TaskState) => void;
}

const items: Array<{ value: TaskState; label: string; countKey: keyof Omit<TaskTabsProps, "value" | "onValueChange"> }> = [
  { value: "today", label: "Today", countKey: "todayCount" },
  { value: "upcoming", label: "Upcoming", countKey: "upcomingCount" },
  { value: "blocked", label: "Blocked", countKey: "blockedCount" }
];

export function TaskTabs({
  value,
  todayCount,
  upcomingCount,
  blockedCount,
  onValueChange
}: TaskTabsProps) {
  const counts = { todayCount, upcomingCount, blockedCount };

  return (
    <Tabs.Root value={value} onValueChange={(next) => onValueChange(next as TaskState)}>
      <Tabs.List className="grid grid-cols-3 gap-2 rounded-xl bg-[var(--muted)] p-1">
        {items.map((item) => (
          <Tabs.Tab
            key={item.value}
            value={item.value}
            className={cn(
              "rounded-lg px-3 py-2 text-sm font-medium text-[var(--muted-foreground)] transition-colors",
              "data-[selected]:bg-[var(--card)] data-[selected]:text-[var(--foreground)]"
            )}
          >
            <span>{item.label}</span>
            <span className="ml-2 tabular-nums">{counts[item.countKey]}</span>
          </Tabs.Tab>
        ))}
      </Tabs.List>
    </Tabs.Root>
  );
}
