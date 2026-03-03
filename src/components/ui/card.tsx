import type { HTMLAttributes, PropsWithChildren } from "react";

import { cn } from "../../lib/cn";

interface CardProps extends HTMLAttributes<HTMLDivElement> {}

export function Card({
  className,
  children,
  ...props
}: PropsWithChildren<CardProps>) {
  return (
    <section
      className={cn(
        "rounded-2xl border bg-[var(--card)] p-5 text-[var(--card-foreground)] shadow-sm",
        className
      )}
      {...props}
    >
      {children}
    </section>
  );
}
