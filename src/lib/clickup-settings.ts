import type { ClickUpSettings } from "../types/clickup-settings";

const STORAGE_KEY = "clicklane.settings.v1";

const defaultSettings: ClickUpSettings = {
  authToken: "",
  workspaceId: cleanValue(import.meta.env.VITE_CLICKUP_TEST_WORKSPACE_ID)
};

function cleanValue(value: unknown) {
  return typeof value === "string" ? value.trim() : "";
}

export function loadClickUpSettings(): ClickUpSettings {
  if (typeof window === "undefined") {
    return defaultSettings;
  }

  const raw = window.localStorage.getItem(STORAGE_KEY);
  if (!raw) {
    return defaultSettings;
  }

  try {
    const parsed = JSON.parse(raw) as Partial<ClickUpSettings>;

    return {
      authToken: cleanValue(parsed.authToken),
      workspaceId:
        cleanValue(parsed.workspaceId) || defaultSettings.workspaceId
    };
  } catch {
    return defaultSettings;
  }
}

export function saveClickUpSettings(settings: ClickUpSettings) {
  if (typeof window === "undefined") {
    return;
  }

  const normalized = normalizeClickUpSettings(settings);
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(normalized));
}

export function normalizeClickUpSettings(
  settings: Partial<ClickUpSettings>
): ClickUpSettings {
  return {
    authToken: cleanValue(settings.authToken),
    workspaceId: cleanValue(settings.workspaceId)
  };
}

export function hasClickUpSettings(settings: ClickUpSettings) {
  return Boolean(settings.authToken && settings.workspaceId);
}
