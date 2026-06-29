import type { Metadata } from "next";
import SettingsClient from "./settings-client";

export const metadata: Metadata = {
  title: "Settings",
  description: "Manage your display, notification, privacy, and account preferences.",
  robots: { index: false, follow: false },
};

export default function SettingsPage() {
  return <SettingsClient />;
}
