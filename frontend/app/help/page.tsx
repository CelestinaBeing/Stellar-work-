import type { Metadata } from "next";
import HelpClient from "./help-client";

export const metadata: Metadata = {
  title: "Help & Security Center | StellarWork",
  description: "Learn how to manage your Stellar account, secure your keys, and recover access to the platform.",
  robots: { index: true, follow: true },
};

export default function HelpPage() {
  return <HelpClient />;
}
