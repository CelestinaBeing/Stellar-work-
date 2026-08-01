import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  MAX_RECENT_SEARCHES,
  clearRecentSearches,
  loadRecentSearches,
  removeRecentSearch,
  saveRecentSearches,
  updateRecentSearches,
} from "@/lib/recent-searches";

describe("recent search helpers", () => {
  beforeEach(() => {
    vi.stubGlobal("window", window);
    localStorage.clear();
  });

  it("stores the last ten unique searches with the newest first", () => {
    const searches = Array.from({ length: 12 }, (_, index) => `term-${index + 1}`).reduce(
      (current, term) => updateRecentSearches(current, term),
      [] as string[],
    );

    expect(searches).toHaveLength(MAX_RECENT_SEARCHES);
    expect(searches[0]).toBe("term-12");
    expect(searches).not.toContain("term-1");
  });

  it("moves an existing search to the front instead of duplicating it", () => {
    const searches = updateRecentSearches(["design", "frontend"], "Frontend");

    expect(searches).toEqual(["Frontend", "design"]);
  });

  it("removes an individual recent search", () => {
    expect(removeRecentSearch(["design", "frontend", "contract"], "frontend")).toEqual([
      "design",
      "contract",
    ]);
  });

  it("loads, saves, and clears persisted searches", () => {
    saveRecentSearches(["one", "two"]);
    expect(loadRecentSearches()).toEqual(["one", "two"]);

    clearRecentSearches();
    expect(loadRecentSearches()).toEqual([]);
  });
});