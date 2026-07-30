import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Privacy Policy - StellarWork",
};

export default function PrivacyPage() {
  return (
    <div className="mx-auto max-w-3xl space-y-8">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold text-slate-900">Privacy Policy</h1>
        <p className="text-sm text-slate-500">Version 1.0 &mdash; Last updated: July 2026</p>
      </div>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">1. Information We Collect</h2>
        <p className="text-sm leading-6 text-slate-600">
          We collect your Stellar wallet address when you connect to the Platform. Job-related data
          (descriptions, amounts, deadlines) is stored on the Stellar blockchain and in IPFS.
          Notification preferences and cached data are stored locally in your browser.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">2. How We Use Your Information</h2>
        <p className="text-sm leading-6 text-slate-600">
          Your wallet address is used to identify you on the Platform and to facilitate escrow transactions.
          Job data is used to display listings and enable the freelance marketplace functionality.
          Local storage data is used to remember your preferences and improve your experience.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">3. Data Storage and Security</h2>
        <p className="text-sm leading-6 text-slate-600">
          No sensitive personal information (such as private keys, passwords, or email addresses) is stored
          by the Platform. All financial transactions are secured by the Stellar blockchain. Local storage
          data remains in your browser and is not transmitted to our servers.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">4. Third-Party Services</h2>
        <p className="text-sm leading-6 text-slate-600">
          The Platform interacts with the Stellar blockchain and IPFS. These services have their own privacy
          policies. We do not share your data with any other third parties.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">5. Changes to This Policy</h2>
        <p className="text-sm leading-6 text-slate-600">
          We may update this Privacy Policy from time to time. Users will be notified of material changes
          via the Platform and may be required to re-accept the updated terms.
        </p>
      </section>
    </div>
  );
}
