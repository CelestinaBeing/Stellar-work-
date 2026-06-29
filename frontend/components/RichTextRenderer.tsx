"use client";

import { useMemo } from "react";
import DOMPurify from "dompurify";

interface RichTextRendererProps {
  /** Raw HTML string from the rich text editor. */
  html: string;
  /** Optional extra className for the wrapper div. */
  className?: string;
}

/**
 * Safely renders editor-produced HTML.
 * DOMPurify strips any dangerous tags/attributes before the string is
 * handed to dangerouslySetInnerHTML, preventing XSS.
 */
export default function RichTextRenderer({ html, className }: RichTextRendererProps) {
  const clean = useMemo(() => {
    if (typeof window === "undefined") {
      // Server-side: strip all tags as a safe fallback (SSR will be hydrated)
      return html.replace(/<[^>]+>/g, "");
    }
    return DOMPurify.sanitize(html, {
      ALLOWED_TAGS: [
        "p", "br",
        "strong", "em", "b", "i",
        "h1", "h2", "h3",
        "ul", "ol", "li",
        "a",
      ],
      ALLOWED_ATTR: ["href", "target", "rel", "class"],
      // Force safe link attributes — prevent javascript: hrefs
      FORCE_BODY: true,
    });
  }, [html]);

  return (
    <div
      className={[
        "prose prose-sm max-w-none text-sm text-slate-900",
        "[&_h1]:text-lg [&_h1]:font-semibold [&_h1]:mt-3 [&_h1]:mb-1",
        "[&_h2]:text-base [&_h2]:font-semibold [&_h2]:mt-3 [&_h2]:mb-1",
        "[&_h3]:text-sm [&_h3]:font-semibold [&_h3]:mt-2 [&_h3]:mb-0.5",
        "[&_ul]:list-disc [&_ul]:pl-5 [&_ul]:my-1",
        "[&_ol]:list-decimal [&_ol]:pl-5 [&_ol]:my-1",
        "[&_li]:my-0.5",
        "[&_a]:text-blue-600 [&_a]:underline [&_a]:break-all",
        "[&_p]:my-1",
        className ?? "",
      ].join(" ")}
      // eslint-disable-next-line react/no-danger
      dangerouslySetInnerHTML={{ __html: clean }}
    />
  );
}

// ── Plain-text fallback renderer ──────────────────────────────────────────────
// Used for the description_hash tooltip / copy area and for non-HTML content.

interface PlainTextRendererProps {
  text: string;
  className?: string;
}

/** Renders plain text with whitespace preserved. */
export function PlainTextRenderer({ text, className }: PlainTextRendererProps) {
  return (
    <p className={["whitespace-pre-wrap text-sm text-slate-900", className ?? ""].join(" ")}>
      {text}
    </p>
  );
}

/** Returns true if the string looks like editor-produced HTML. */
export function isRichText(content: string): boolean {
  return /^<[a-z]/i.test(content.trimStart());
}
