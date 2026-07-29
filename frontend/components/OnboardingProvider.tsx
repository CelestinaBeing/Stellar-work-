"use client";

import { useState } from "react";
import OnboardingWizard from "./OnboardingWizard";

/**
 * Renders the onboarding wizard (auto-shown on first visit) plus a persistent
 * help button that lets returning users re-open the tour at any time.
 */
export default function OnboardingProvider() {
  const [forceOpen, setForceOpen] = useState(false);

  return (
    <>
      <OnboardingWizard
        forceOpen={forceOpen}
        onClose={() => setForceOpen(false)}
      />
      {/* Help button — fixed to bottom-right corner */}
      <button
        type="button"
        aria-label="Open onboarding tour"
        title="Platform tour"
        onClick={() => setForceOpen(true)}
        className="fixed bottom-6 right-6 z-40 flex h-10 w-10 items-center justify-center rounded-full bg-blue-600 text-white shadow-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
      >
        <span aria-hidden="true" className="text-lg font-bold">?</span>
      </button>
    </>
  );
}
