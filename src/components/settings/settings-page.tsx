import type { FormEvent } from "react";
import { useState } from "react";

import { normalizeClickUpSettings } from "../../lib/clickup-settings";
import type { ClickUpSettings } from "../../types/clickup-settings";
import { Button } from "../ui/button";
import { Card } from "../ui/card";

interface SettingsPageProps {
  settings: ClickUpSettings;
  onSave: (settings: ClickUpSettings) => void;
}

export function SettingsPage({ settings, onSave }: SettingsPageProps) {
  const [authToken, setAuthToken] = useState(settings.authToken);
  const [workspaceId, setWorkspaceId] = useState(settings.workspaceId);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [savedMessage, setSavedMessage] = useState<string | null>(null);

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const nextSettings = normalizeClickUpSettings({ authToken, workspaceId });
    if (!nextSettings.authToken || !nextSettings.workspaceId) {
      setSavedMessage(null);
      setErrorMessage("Both token and workspace ID are required.");
      return;
    }

    onSave(nextSettings);
    setSavedMessage("Settings saved for this desktop app.");
    setErrorMessage(null);
  }

  return (
    <Card className="space-y-5">
      <div>
        <p className="text-xs font-semibold uppercase text-[var(--muted-foreground)]">
          Local setup
        </p>
        <h2 className="mt-2 text-2xl font-semibold text-balance">Settings</h2>
        <p className="mt-2 max-w-2xl text-sm leading-6 text-[var(--muted-foreground)] text-pretty">
          Save the ClickUp token and workspace once, then keep the task page clean.
          Both values stay masked on screen, including the workspace ID.
        </p>
      </div>

      <form
        className="space-y-4 rounded-xl border bg-[var(--background)] p-4"
        onSubmit={handleSubmit}
      >
        <label className="block space-y-2 text-sm font-medium text-[var(--foreground)]">
          <span>ClickUp token</span>
          <input
            autoComplete="off"
            className="h-11 w-full rounded-lg border bg-[var(--card)] px-3 text-sm text-[var(--foreground)] outline-none"
            onChange={(event) => setAuthToken(event.target.value)}
            placeholder="pk_... or Bearer ..."
            type="password"
            value={authToken}
          />
          <p className="text-xs text-[var(--muted-foreground)] text-pretty">
            Personal API keys and OAuth Bearer tokens are both accepted.
          </p>
        </label>

        <label className="block space-y-2 text-sm font-medium text-[var(--foreground)]">
          <span>Workspace ID</span>
          <input
            autoComplete="off"
            className="h-11 w-full rounded-lg border bg-[var(--card)] px-3 text-sm text-[var(--foreground)] outline-none"
            inputMode="numeric"
            onChange={(event) => setWorkspaceId(event.target.value)}
            placeholder="Workspace ID"
            type="password"
            value={workspaceId}
          />
          <p className="text-xs text-[var(--muted-foreground)] text-pretty">
            Stored for task lookup only. The task page does not surface it.
          </p>
        </label>

        {errorMessage ? (
          <p className="text-sm text-[var(--danger)]">{errorMessage}</p>
        ) : null}
        {savedMessage ? (
          <p className="text-sm text-[var(--accent)]">{savedMessage}</p>
        ) : null}

        <div className="flex flex-wrap gap-3">
          <Button type="submit">Save settings</Button>
          <span className="inline-flex items-center rounded-lg bg-[var(--muted)] px-3 text-xs font-medium text-[var(--muted-foreground)]">
            Workspace remains hidden outside this page
          </span>
        </div>
      </form>
    </Card>
  );
}
