import { useMemo } from "react";

import { useForYouStore } from "../../stores/for-you-store";
import { Button } from "../ui/button";
import { Card } from "../ui/card";
import { Skeleton } from "../ui/skeleton";
import { TaskTabs } from "../ui/task-tabs";
import { TaskRow } from "./task-row";

export function ForYouView() {
  const {
    tasks,
    summary,
    insights,
    activeTab,
    isLoading,
    errorMessage,
    setActiveTab,
    generateForYou
  } = useForYouStore();

  const todayCount = tasks.filter((task) => task.state === "today").length;
  const upcomingCount = tasks.filter((task) => task.state === "upcoming").length;
  const blockedCount = tasks.filter((task) => task.state === "blocked").length;

  const visibleTasks = useMemo(
    () => tasks.filter((task) => task.state === activeTab),
    [activeTab, tasks]
  );

  return (
    <section className="space-y-6">
      <header className="flex flex-col gap-4 rounded-2xl border bg-[var(--card)] p-6 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <p className="text-sm font-medium text-[var(--muted-foreground)]">
            Clicklane
          </p>
          <h1 className="mt-2 text-3xl font-semibold text-balance">For you</h1>
          <p className="mt-1 max-w-xl text-sm text-[var(--muted-foreground)] text-pretty">
            Prioritize what matters now. Generate a concise execution brief from
            your current task set.
          </p>
        </div>
        <div className="space-y-2">
          <Button disabled={isLoading} onClick={() => void generateForYou()}>
            {isLoading ? "Building brief..." : "Generate For you brief"}
          </Button>
          {errorMessage ? (
            <p className="text-sm text-[var(--danger)]">{errorMessage}</p>
          ) : null}
        </div>
      </header>

      <div className="grid gap-6 lg:grid-cols-[1.5fr_1fr]">
        <Card className="space-y-5">
          <div>
            <h2 className="text-lg font-semibold text-balance">Today strategy</h2>
            {isLoading ? (
              <div className="mt-3 space-y-2">
                <Skeleton className="h-4 w-11/12" />
                <Skeleton className="h-4 w-10/12" />
                <Skeleton className="h-4 w-7/12" />
              </div>
            ) : (
              <p className="mt-3 text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
                {summary}
              </p>
            )}
          </div>

          <div className="space-y-3">
            <h3 className="text-sm font-semibold uppercase text-[var(--muted-foreground)]">
              Suggested moves
            </h3>
            {isLoading ? (
              <div className="space-y-3">
                <Skeleton className="h-20 w-full" />
                <Skeleton className="h-20 w-full" />
                <Skeleton className="h-20 w-full" />
              </div>
            ) : (
              insights.map((insight) => (
                <article
                  key={insight.title}
                  className="rounded-xl border bg-[var(--card)] p-4"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div>
                      <h4 className="text-sm font-semibold text-balance">
                        {insight.title}
                      </h4>
                      <p className="mt-2 text-sm text-[var(--muted-foreground)] text-pretty">
                        {insight.reason}
                      </p>
                    </div>
                    <span className="shrink-0 rounded-lg bg-[var(--muted)] px-2 py-1 text-xs font-medium">
                      {insight.actionLabel}
                    </span>
                  </div>
                </article>
              ))
            )}
          </div>
        </Card>

        <Card className="space-y-4">
          <h2 className="text-lg font-semibold text-balance">Work queue</h2>
          <TaskTabs
            value={activeTab}
            onValueChange={setActiveTab}
            todayCount={todayCount}
            upcomingCount={upcomingCount}
            blockedCount={blockedCount}
          />

          <div className="space-y-3">
            {visibleTasks.length > 0 ? (
              visibleTasks.map((task) => <TaskRow key={task.id} task={task} />)
            ) : (
              <div className="rounded-xl border bg-[var(--card)] p-4">
                <p className="text-sm text-[var(--muted-foreground)] text-pretty">
                  No tasks in this bucket yet.
                </p>
                <Button className="mt-3" variant="ghost">
                  Create first task
                </Button>
              </div>
            )}
          </div>
        </Card>
      </div>
    </section>
  );
}
