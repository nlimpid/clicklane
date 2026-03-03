import { createOpenAI } from "@ai-sdk/openai";
import { generateObject } from "ai";
import { z } from "zod";

import type { ClickUpTask, ForYouResult } from "../types/for-you";

const insightSchema = z.object({
  title: z.string().min(3),
  reason: z.string().min(6),
  actionLabel: z.string().min(3)
});

const forYouSchema = z.object({
  summary: z.string().min(12),
  insights: z.array(insightSchema).min(2).max(4)
});

function localFallback(tasks: ClickUpTask[]): ForYouResult {
  const todayTasks = tasks.filter((task) => task.state === "today");
  const blockedTasks = tasks.filter((task) => task.state === "blocked");
  const totalFocusMinutes = todayTasks.reduce(
    (sum, task) => sum + task.estimateMinutes,
    0
  );

  return {
    summary: todayTasks.length
      ? `Today has ${todayTasks.length} high-value tasks (${totalFocusMinutes} min). Clear blockers first, then finish the urgent deliverable before context switching.`
      : "No tasks tagged for today yet. Start by promoting one meaningful task into today.",
    insights: [
      {
        title: "Start with one urgent outcome",
        reason:
          "A single priority anchor lowers context switching and gives the day a clear success condition.",
        actionLabel: "Pin top task"
      },
      {
        title: "Bundle medium tasks into one block",
        reason:
          "Grouping related smaller tasks protects deep-focus windows and avoids fragmented work.",
        actionLabel: "Create 45m block"
      },
      {
        title: "Address blockers early",
        reason:
          blockedTasks.length > 0
            ? `You have ${blockedTasks.length} blocked item(s). Removing dependency risk early prevents late-day stalls.`
            : "No blocker detected. Keep this slot as buffer for ad-hoc issues.",
        actionLabel: "Review blockers"
      }
    ]
  };
}

export async function buildForYouResult(
  tasks: ClickUpTask[]
): Promise<ForYouResult> {
  const token = import.meta.env.VITE_CLICKUP_OPENAI_TOKEN;
  if (!token) {
    return localFallback(tasks);
  }

  const openai = createOpenAI({ apiKey: token });
  const taskDigest = tasks
    .map(
      (task) =>
        `- ${task.title} | list:${task.list} | state:${task.state} | priority:${task.priority} | estimate:${task.estimateMinutes}m | due:${task.dueLabel ?? "n/a"}`
    )
    .join("\n");

  try {
    const result = await generateObject({
      model: openai("gpt-4.1-mini"),
      schema: forYouSchema,
      prompt: `You are an execution coach for ClickUp users.
Create a "For you" briefing that is concrete and short.
Use crisp language and focus on sequencing.
Tasks:
${taskDigest}`
    });

    return result.object;
  } catch {
    return localFallback(tasks);
  }
}
