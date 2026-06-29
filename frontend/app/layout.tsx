import type { Metadata, Viewport } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { NextIntlClientProvider } from "next-intl";
import { getLocale, getMessages } from "next-intl/server";
import { WalletProvider } from "@/lib/wallet-context";
import { ToastProvider } from "@/components/ToastProvider";
import { NotificationProvider } from "@/lib/notifications-context";
import { MessagingProvider } from "@/lib/messaging-context";
import { ThemeProvider } from "@/components/ThemeProvider";
import { Navigation } from "./navigation";
import { ScrollRestorer } from "@/components/ScrollRestorer";
import ErrorBoundary from "@/components/ErrorBoundary";
import CommandPalette from "@/components/CommandPalette";
import OnboardingProvider from "@/components/OnboardingProvider";
import AnnouncementBanner from "@/components/AnnouncementBanner";
import Link from "next/link";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
  viewportFit: "cover",
};

export const metadata: Metadata = {
  title: "StellarWork",
  description: "Decentralized escrow freelance marketplace on Stellar",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const locale = await getLocale();
  const messages = await getMessages();

  return (
    <html
      lang={locale}
      className={`${geistSans.variable} ${geistMono.variable} h-full antialiased`}
      suppressHydrationWarning
    >
      <body className="min-h-full flex flex-col bg-slate-50 text-slate-900 dark:bg-slate-950 dark:text-slate-100">
        <NextIntlClientProvider messages={messages} locale={locale}>
        <ThemeProvider>
        <WalletProvider>
          <NotificationProvider>
          <MessagingProvider>
          <ToastProvider>
          <a
            href="#main-content"
            className="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50 focus:rounded-md focus:bg-white focus:px-3 focus:py-2 focus:text-slate-900 focus:outline-none dark:focus:bg-slate-800 dark:focus:text-slate-100"
          >
            Skip to main content
          </a>
          <AnnouncementBanner />
          <Navigation />
          <CommandPalette />
          <OnboardingProvider />
          <ScrollRestorer />
          <main id="main-content" tabIndex={-1} className="mx-auto w-full max-w-5xl flex-1 px-3 py-6 sm:px-4 sm:py-8">
            <ErrorBoundary>{children}</ErrorBoundary>
          </main>
          <footer className="mt-auto border-t border-slate-200 bg-white py-8 pb-[calc(2rem+env(safe-area-inset-bottom))] dark:border-slate-800 dark:bg-slate-900">
            <div className="mx-auto max-w-5xl px-4">
              <div className="flex flex-col items-center justify-between gap-6 md:flex-row">
                <div className="flex flex-col items-center gap-2 md:items-start">
                  <span className="text-lg font-bold text-slate-900 dark:text-slate-100">StellarWork</span>
                  <p className="text-sm text-slate-500 dark:text-slate-400">Decentralized Escrow Marketplace</p>
                </div>

                <nav className="flex flex-wrap justify-center gap-8 text-sm font-medium text-slate-600 dark:text-slate-400">
                  <a href="https://github.com/anumukul/Stellar-work-" target="_blank" rel="noopener noreferrer" className="hover:text-blue-600 dark:hover:text-blue-400 transition-colors">GitHub</a>
                  <Link href="/docs" className="hover:text-blue-600 dark:hover:text-blue-400 transition-colors">Documentation</Link>
                  <a href="/LICENSE" target="_blank" rel="noopener noreferrer" className="hover:text-blue-600 dark:hover:text-blue-400 transition-colors">License</a>
                </nav>

                <div className="flex items-center gap-2 rounded-full bg-slate-50 px-4 py-2 border border-slate-100 dark:bg-slate-800 dark:border-slate-700">
                  <span className="text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">Built on</span>
                  <span className="text-sm font-bold text-slate-800 dark:text-slate-200">Stellar</span>
                </div>
              </div>
              <div className="mt-8 border-t border-slate-100 pt-8 text-center text-xs text-slate-400 dark:border-slate-800 dark:text-slate-500">
                &copy; {new Date().getFullYear()} StellarWork. All rights reserved.
              </div>
            </div>
          </footer>
          </ToastProvider>
          </MessagingProvider>
          </NotificationProvider>
        </WalletProvider>
        </ThemeProvider>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
