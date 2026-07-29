import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Terms of Service - StellarWork",
};

export default function TermsPage() {
  return (
    <div className="mx-auto max-w-3xl space-y-8">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold text-slate-900">Terms of Service</h1>
        <p className="text-sm text-slate-500">Version 1.0 &mdash; Last updated: July 2026</p>
      </div>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">1. Acceptance of Terms</h2>
        <p className="text-sm leading-6 text-slate-600">
          By accessing or using StellarWork (&ldquo;the Platform&rdquo;), you agree to be bound by these Terms of Service.
          If you do not agree, do not use the Platform.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">2. User Rights and Responsibilities</h2>
        <p className="text-sm leading-6 text-slate-600">
          Users are responsible for maintaining the security of their Stellar wallet and for all activities under their account.
          You agree to use the Platform only for lawful purposes and in compliance with applicable laws.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">3. Fee Disclosure</h2>
        <p className="text-sm leading-6 text-slate-600">
          Platform fees are disclosed at the time of job posting and approval. The current fee rate is displayed
          on the Platform and may be updated by the platform admin with notice.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">4. Dispute Resolution</h2>
        <p className="text-sm leading-6 text-slate-600">
          Disputes between clients and freelancers may be raised on-chain. The platform admin may resolve disputes
          by splitting the escrowed amount between parties as deemed fair. All dispute resolutions are final.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">5. Data Handling and Privacy</h2>
        <p className="text-sm leading-6 text-slate-600">
          StellarWork stores minimal personal data. Wallet addresses and job metadata are stored on the Stellar
          blockchain and are publicly visible. Off-chain data (notification preferences, cached IPFS metadata)
          is stored locally in your browser. See our Privacy Policy for details.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">6. Limitation of Liability</h2>
        <p className="text-sm leading-6 text-slate-600">
          StellarWork is provided &ldquo;as is&rdquo; without warranty of any kind. The platform administrators
          are not liable for any losses arising from the use of the Platform, including but not limited to
          smart contract bugs, blockchain network issues, or user error.
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-xl font-semibold text-slate-800">7. Termination Policy</h2>
        <p className="text-sm leading-6 text-slate-600">
          The platform admin reserves the right to suspend or terminate access to the Platform for any user
          who violates these Terms. Users may stop using the Platform at any time.
        </p>
      </section>
    </div>
  );
}
