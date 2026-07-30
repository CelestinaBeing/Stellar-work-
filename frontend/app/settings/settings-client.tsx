"use client";

import { useTheme } from "@/components/ThemeProvider";
import { useWallet } from "@/lib/wallet-context";
import {
  FIAT_CURRENCIES,
  getPreferredFiatCurrency,
  savePreferredFiatCurrency,
  type FiatCurrency,
} from "@/lib/format";
import {
  useNotifications,
  getEventLabel,
} from "@/lib/notifications-context";
import type { NotificationEvent } from "@/lib/types";
import { getNetwork } from "@/lib/stellar";
import { useCallback, useEffect, useId, useState } from "react";

const NOTIFICATION_EVENTS: NotificationEvent[] = [
  "job_accepted",
  "work_submitted",
  "work_approved",
  "job_cancelled",
  "dispute_raised",
  "dispute_resolved",
];

const PROFILE_VISIBILITY_KEY = "stellarwork:settings:profile_visible";
const SHOW_EMAIL_KEY = "stellarwork:settings:show_email";
const READ_RECEIPTS_KEY = "stellarwork:settings:read_receipts";

function readBool(key: string, defaultValue: boolean): boolean {
  if (typeof window === "undefined") return defaultValue;
  const v = localStorage.getItem(key);
  if (v === null) return defaultValue;
  return v === "true";
}

function Toggle({
  id,
  checked,
  onChange,
  label,
  description,
}: {
  id: string;
  checked: boolean;
  onChange: (v: boolean) => void;
  label: string;
  description?: string;
}) {
  return (
    <label
      htmlFor={id}
      className="flex cursor-pointer items-center justify-between gap-4"
    >
      <span className="flex flex-col gap-0.5">
        <span className="text-sm font-medium text-slate-800 dark:text-slate-200">
          {label}
        </span>
        {description && (
          <span className="text-xs text-slate-500 dark:text-slate-400">
            {description}
          </span>
        )}
      </span>
      <button
        id={id}
        role="switch"
        aria-checked={checked}
        type="button"
        onClick={() => onChange(!checked)}
        className={`relative inline-flex h-6 w-11 shrink-0 rounded-full border-2 border-transparent transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 ${
          checked
            ? "bg-blue-600"
            : "bg-slate-200 dark:bg-slate-700"
        }`}
      >
        <span
          aria-hidden="true"
          className={`inline-block h-5 w-5 transform rounded-full bg-white shadow-md ring-0 transition-transform ${
            checked ? "translate-x-5" : "translate-x-0"
          }`}
        />
      </button>
    </label>
  );
}

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section className="rounded-xl border border-slate-200 bg-white p-6 shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <h2 className="mb-4 text-base font-semibold text-slate-900 dark:text-slate-100">
        {title}
      </h2>
      <div className="space-y-4">{children}</div>
    </section>
  );
}

export default function SettingsClient() {
  const { theme, setTheme } = useTheme();
  const { wallet } = useWallet();
  const { preferences, setPreference } = useNotifications();
  const network = getNetwork();
  const themeId = useId();
  const currencyId = useId();

  const [currency, setCurrency] = useState<FiatCurrency>("USD");
  const [profileVisible, setProfileVisible] = useState(true);
  const [showEmail, setShowEmail] = useState(false);
  const [readReceipts, setReadReceipts] = useState(true);

  useEffect(() => {
    setCurrency(getPreferredFiatCurrency());
    setProfileVisible(readBool(PROFILE_VISIBILITY_KEY, true));
    setShowEmail(readBool(SHOW_EMAIL_KEY, false));
    setReadReceipts(readBool(READ_RECEIPTS_KEY, true));
  }, []);

  const handleCurrencyChange = useCallback((c: FiatCurrency) => {
    setCurrency(c);
    savePreferredFiatCurrency(c);
  }, []);

  const handleProfileVisible = useCallback((v: boolean) => {
    setProfileVisible(v);
    localStorage.setItem(PROFILE_VISIBILITY_KEY, String(v));
  }, []);

  const handleShowEmail = useCallback((v: boolean) => {
    setShowEmail(v);
    localStorage.setItem(SHOW_EMAIL_KEY, String(v));
  }, []);

  const handleReadReceipts = useCallback((v: boolean) => {
    setReadReceipts(v);
    localStorage.setItem(READ_RECEIPTS_KEY, String(v));
  }, []);

  const resetDisplay = useCallback(() => {
    setTheme("system");
    handleCurrencyChange("USD");
  }, [setTheme, handleCurrencyChange]);

  const resetNotifications = useCallback(() => {
    for (const event of NOTIFICATION_EVENTS) {
      setPreference(event, true);
    }
  }, [setPreference]);

  const resetPrivacy = useCallback(() => {
    handleProfileVisible(true);
    handleShowEmail(false);
    handleReadReceipts(true);
  }, [handleProfileVisible, handleShowEmail, handleReadReceipts]);

  return (
    <div className="mx-auto max-w-2xl space-y-6 py-2">
      <div>
        <h1 className="text-2xl font-bold text-slate-900 dark:text-slate-100">
          Settings
        </h1>
        <p className="mt-1 text-sm text-slate-500 dark:text-slate-400">
          Preferences are saved automatically and persist across sessions.
        </p>
      </div>

      {/* Display */}
      <Section title="Display">
        <div className="flex flex-col gap-1.5">
          <label
            htmlFor={themeId}
            className="text-sm font-medium text-slate-800 dark:text-slate-200"
          >
            Theme
          </label>
          <select
            id={themeId}
            value={theme}
            onChange={(e) =>
              setTheme(e.target.value as "light" | "dark" | "system")
            }
            className="w-48 rounded-md border border-slate-300 bg-white px-3 py-1.5 text-sm text-slate-800 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-200"
          >
            <option value="system">System default</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </div>

        <div className="flex flex-col gap-1.5">
          <label
            htmlFor={currencyId}
            className="text-sm font-medium text-slate-800 dark:text-slate-200"
          >
            Fiat currency
          </label>
          <select
            id={currencyId}
            value={currency}
            onChange={(e) => handleCurrencyChange(e.target.value as FiatCurrency)}
            className="w-48 rounded-md border border-slate-300 bg-white px-3 py-1.5 text-sm text-slate-800 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-200"
          >
            {FIAT_CURRENCIES.map((c) => (
              <option key={c} value={c}>
                {c}
              </option>
            ))}
          </select>
          <p className="text-xs text-slate-500 dark:text-slate-400">
            Used for XLM fiat equivalents shown across the app.
          </p>
        </div>

        <button
          type="button"
          onClick={resetDisplay}
          className="text-xs text-slate-400 underline hover:text-slate-600 dark:hover:text-slate-300"
        >
          Reset to defaults
        </button>
      </Section>

      {/* Notifications */}
      <Section title="Notifications">
        <p className="text-xs text-slate-500 dark:text-slate-400">
          Choose which in-app events trigger a notification.
        </p>
        {NOTIFICATION_EVENTS.map((event) => (
          <Toggle
            key={event}
            id={`notif-${event}`}
            checked={preferences[event]}
            onChange={(v) => setPreference(event, v)}
            label={getEventLabel(event)}
          />
        ))}
        <button
          type="button"
          onClick={resetNotifications}
          className="text-xs text-slate-400 underline hover:text-slate-600 dark:hover:text-slate-300"
        >
          Enable all notifications
        </button>
      </Section>

      {/* Privacy */}
      <Section title="Privacy">
        <Toggle
          id="privacy-profile-visible"
          checked={profileVisible}
          onChange={handleProfileVisible}
          label="Public profile"
          description="Allow others to view your on-chain activity."
        />
        <Toggle
          id="privacy-show-email"
          checked={showEmail}
          onChange={handleShowEmail}
          label="Show email on profile"
          description="Display your email address on your public profile page."
        />
        <Toggle
          id="privacy-read-receipts"
          checked={readReceipts}
          onChange={handleReadReceipts}
          label="Read receipts"
          description="Let others see when you've read their messages."
        />
        <button
          type="button"
          onClick={resetPrivacy}
          className="text-xs text-slate-400 underline hover:text-slate-600 dark:hover:text-slate-300"
        >
          Reset to defaults
        </button>
      </Section>

      {/* Account */}
      <Section title="Account">
        <div className="space-y-1">
          <p className="text-sm font-medium text-slate-800 dark:text-slate-200">
            Connected wallet
          </p>
          {wallet ? (
            <p className="break-all font-mono text-xs text-slate-600 dark:text-slate-400">
              {wallet}
            </p>
          ) : (
            <p className="text-xs text-slate-400 dark:text-slate-500">
              No wallet connected.
            </p>
          )}
        </div>
        <div className="space-y-1">
          <p className="text-sm font-medium text-slate-800 dark:text-slate-200">
            Network
          </p>
          <p className="text-xs capitalize text-slate-600 dark:text-slate-400">
            {network}
          </p>
        </div>
      </Section>
    </div>
  );
}
