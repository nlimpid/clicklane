import { useState } from "react";

import { ForYouView } from "./components/for-you/for-you-view";
import { SettingsPage } from "./components/settings/settings-page";
import { TaskFocusPanel } from "./components/task-focus/task-focus-panel";
import { cn } from "./lib/cn";
import {
  hasClickUpSettings,
  loadClickUpSettings,
  saveClickUpSettings
} from "./lib/clickup-settings";
import type { ClickUpSettings } from "./types/clickup-settings";

type AppPage = "focus" | "for-you" | "settings";

const pageLabels: Record<AppPage, string> = {
  focus: "Task focus",
  "for-you": "For you",
  settings: "Settings"
};

function initialPage(settings: ClickUpSettings): AppPage {
  return hasClickUpSettings(settings) ? "focus" : "settings";
}

export function App() {
  const [settings, setSettings] = useState<ClickUpSettings>(() => loadClickUpSettings());
  const [activePage, setActivePage] = useState<AppPage>(() => initialPage(settings));
  const settingsReady = hasClickUpSettings(settings);

  function handleSave(nextSettings: ClickUpSettings) {
    saveClickUpSettings(nextSettings);
    setSettings(nextSettings);
    setActivePage("focus");
  }

  return (
    <div className="app-root min-h-dvh bg-[var(--background)] text-[var(--foreground)]">
      <main className="relative min-h-dvh overflow-hidden px-5 py-8 sm:px-8 sm:py-10">
        <div className="pointer-events-none absolute inset-0 -z-10">
          <div className="absolute -left-20 top-16 size-56 rounded-full border border-[var(--border)]" />
          <div className="absolute right-8 top-24 size-24 rotate-12 rounded-xl bg-[var(--muted)]" />
          <div className="absolute bottom-8 right-20 size-32 rounded-full border border-[var(--border)]" />
        </div>

        <section className="mx-auto w-full max-w-6xl space-y-6">
          <header className="rounded-2xl border bg-[var(--card)] p-5">
            <div className="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between">
              <div>
                <p className="text-sm font-medium text-[var(--muted-foreground)]">
                  Clicklane
                </p>
                <p className="mt-2 text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
                  Use a focused ClickUp desktop flow: one page for live task
                  detail, one for your planning brief, and one for setup.
                </p>
              </div>

              <nav
                aria-label="Primary navigation"
                className="grid gap-2 rounded-xl bg-[var(--muted)] p-1 sm:grid-cols-3"
              >
                {(Object.keys(pageLabels) as AppPage[]).map((page) => (
                  <button
                    key={page}
                    aria-current={activePage === page ? "page" : undefined}
                    className={cn(
                      "rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                      activePage === page
                        ? "bg-[var(--card)] text-[var(--foreground)]"
                        : "text-[var(--muted-foreground)] hover:bg-[var(--card)]"
                    )}
                    onClick={() => setActivePage(page)}
                    type="button"
                  >
                    {pageLabels[page]}
                  </button>
                ))}
              </nav>
            </div>

            <div className="mt-4 flex flex-wrap gap-2">
              <span className="rounded-full bg-[var(--muted)] px-3 py-2 text-xs font-medium text-[var(--muted-foreground)]">
                {settings.authToken ? "Token saved" : "Token missing"}
              </span>
              <span className="rounded-full bg-[var(--muted)] px-3 py-2 text-xs font-medium text-[var(--muted-foreground)]">
                {settings.workspaceId ? "Workspace saved" : "Workspace missing"}
              </span>
              {!settingsReady ? (
                <span className="rounded-full bg-[#f7dfdf] px-3 py-2 text-xs font-medium text-[#8c2525]">
                  Finish Settings before live task lookup
                </span>
              ) : null}
            </div>
          </header>

          {activePage === "focus" ? (
            <TaskFocusPanel
              onOpenSettings={() => setActivePage("settings")}
              settings={settings}
            />
          ) : null}

          {activePage === "for-you" ? <ForYouView /> : null}

          {activePage === "settings" ? (
            <SettingsPage onSave={handleSave} settings={settings} />
          ) : null}
        </section>
      </main>
    </div>
  );
}
