import type { ButtonHTMLAttributes, PropsWithChildren } from "react";

import { cn } from "../../lib/cn";

type ButtonVariant = "primary" | "ghost";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
}

const variantClasses: Record<ButtonVariant, string> = {
  primary:
    "bg-[var(--accent)] text-[var(--accent-foreground)] hover:opacity-90 disabled:bg-[var(--muted)] disabled:text-[var(--muted-foreground)]",
  ghost:
    "bg-transparent text-[var(--foreground)] hover:bg-[var(--muted)] disabled:text-[var(--muted-foreground)]"
};

export function Button({
  className,
  children,
  variant = "primary",
  ...props
}: PropsWithChildren<ButtonProps>) {
  return (
    <button
      className={cn(
        "inline-flex h-10 items-center justify-center rounded-lg border border-transparent px-4 text-sm font-medium transition-opacity disabled:cursor-not-allowed",
        variantClasses[variant],
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
}
