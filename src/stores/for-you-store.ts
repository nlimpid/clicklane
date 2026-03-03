import { create } from "zustand";

import { buildForYouResult } from "../lib/for-you-ai";
import { seedTasks } from "../mocks/tasks";
import type { ClickUpTask, ForYouInsight, TaskState } from "../types/for-you";

interface ForYouStore {
  tasks: ClickUpTask[];
  activeTab: TaskState;
  summary: string;
  insights: ForYouInsight[];
  isLoading: boolean;
  errorMessage: string | null;
  setActiveTab: (tab: TaskState) => void;
  generateForYou: () => Promise<void>;
}

const defaultSummary =
  "Generate a personalized briefing to pick the best next task sequence.";

const defaultInsights: ForYouInsight[] = [
  {
    title: "Front-load your critical path",
    reason:
      "Start with the highest business impact item before incoming requests increase noise.",
    actionLabel: "Pick first task"
  },
  {
    title: "Reserve one interruption buffer",
    reason:
      "A pre-allocated buffer prevents side work from hijacking your core timeline.",
    actionLabel: "Block 30m slot"
  }
];

export const useForYouStore = create<ForYouStore>((set, get) => ({
  tasks: seedTasks,
  activeTab: "today",
  summary: defaultSummary,
  insights: defaultInsights,
  isLoading: false,
  errorMessage: null,
  setActiveTab: (tab) => set({ activeTab: tab }),
  generateForYou: async () => {
    if (get().isLoading) {
      return;
    }

    set({ isLoading: true, errorMessage: null });
    try {
      const result = await buildForYouResult(get().tasks);
      set({
        summary: result.summary,
        insights: result.insights,
        isLoading: false
      });
    } catch {
      set({
        errorMessage: "For you generation failed. Please try again.",
        isLoading: false
      });
    }
  }
}));
