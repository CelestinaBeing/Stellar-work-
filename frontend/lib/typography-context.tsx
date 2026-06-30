"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";

// ─── Types ────────────────────────────────────────────────────────────────────

export type FontSize = "small" | "medium" | "large" | "x-large";
export type LineSpacing = "compact" | "normal" | "relaxed";

export interface TypographySettings {
  fontSize: FontSize;
  lineSpacing: LineSpacing;
}

interface TypographyContextValue extends TypographySettings {
  setFontSize: (size: FontSize) => void;
  setLineSpacing: (spacing: LineSpacing) => void;
  reset: () => void;
}

// ─── Constants ────────────────────────────────────────────────────────────────

const STORAGE_KEY = "stellarwork:typography";

/** Root font-size in px for each preset. All rem values in the app scale with this. */
export const FONT_SIZE_MAP: Record<FontSize, string> = {
  small: "14px",
  medium: "16px",
  large: "18px",
  "x-large": "20px",
};

export const LINE_SPACING_MAP: Record<LineSpacing, string> = {
  compact: "1.4",
  normal: "1.6",
  relaxed: "1.8",
};

const DEFAULTS: TypographySettings = {
  fontSize: "medium",
  lineSpacing: "normal",
};

// ─── Helpers ──────────────────────────────────────────────────────────────────

function loadSettings(): TypographySettings {
  if (typeof window === "undefined") return DEFAULTS;
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return DEFAULTS;
    const parsed = JSON.parse(raw) as Partial<TypographySettings>;
    return {
      fontSize: parsed.fontSize ?? DEFAULTS.fontSize,
      lineSpacing: parsed.lineSpacing ?? DEFAULTS.lineSpacing,
    };
  } catch {
    return DEFAULTS;
  }
}

function applyToDocument(settings: TypographySettings) {
  if (typeof document === "undefined") return;
  const root = document.documentElement;
  root.style.setProperty("--font-size-base", FONT_SIZE_MAP[settings.fontSize]);
  root.style.setProperty(
    "--line-height-base",
    LINE_SPACING_MAP[settings.lineSpacing],
  );
}

// ─── Context ──────────────────────────────────────────────────────────────────

const TypographyContext = createContext<TypographyContextValue>({
  ...DEFAULTS,
  setFontSize: () => {},
  setLineSpacing: () => {},
  reset: () => {},
});

export function useTypography() {
  return useContext(TypographyContext);
}

// ─── Provider ─────────────────────────────────────────────────────────────────

export function TypographyProvider({ children }: { children: ReactNode }) {
  const [settings, setSettings] = useState<TypographySettings>(DEFAULTS);

  // Hydrate from localStorage on mount and apply to <html>
  useEffect(() => {
    const stored = loadSettings();
    setSettings(stored);
    applyToDocument(stored);
  }, []);

  const update = useCallback((next: TypographySettings) => {
    setSettings(next);
    applyToDocument(next);
    if (typeof window !== "undefined") {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
    }
  }, []);

  const setFontSize = useCallback(
    (fontSize: FontSize) => update({ ...settings, fontSize }),
    [settings, update],
  );

  const setLineSpacing = useCallback(
    (lineSpacing: LineSpacing) => update({ ...settings, lineSpacing }),
    [settings, update],
  );

  const reset = useCallback(() => update(DEFAULTS), [update]);

  return (
    <TypographyContext.Provider
      value={{ ...settings, setFontSize, setLineSpacing, reset }}
    >
      {children}
    </TypographyContext.Provider>
  );
}
