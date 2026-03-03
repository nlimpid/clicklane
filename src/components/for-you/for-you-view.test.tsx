import { render, screen } from "@testing-library/react";

import { ForYouView } from "./for-you-view";

describe("ForYouView", () => {
  it("renders page heading and default task queue", () => {
    render(<ForYouView />);

    expect(screen.getByRole("heading", { name: "For you" })).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { name: "Work queue" })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("heading", { name: "Suggested moves" })
    ).toBeInTheDocument();
    expect(screen.getByText("Today")).toBeInTheDocument();
  });
});
