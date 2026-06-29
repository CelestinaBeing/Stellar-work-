# Security Policy

## Supported Versions

Only the latest release and the current `main` branch receive security patches. Older versions and unmaintained branches are not supported.

| Version | Supported          |
| ------- | ------------------ |
| Latest release (main branch) | :white_check_mark: |
| Older versions / tags        | :x:                |

## Reporting a Vulnerability

If you discover a security issue, please report it privately and do not open a public issue.

- **Email:** bandanadivya.opensource@gmail.com
- **PGP Key:** Not available at this time — use encrypted email if possible, or request a secure channel in your initial report.
- **Include:** affected component, version/commit, impact severity, reproduction steps, and any proof-of-concept details.
- If available, include suggested remediation or mitigation guidance.

## Disclosure Process

- We will acknowledge receipt of a report within **3 business days**.
- We will provide an initial severity assessment and next-step plan within **7 business days**.
- We will coordinate disclosure timing with the reporter after a fix or mitigation is available.
- Please avoid sharing exploit details publicly until coordinated disclosure is complete.

## Bug Bounty

We do not currently offer a monetary bug bounty program. However, we publicly acknowledge responsible disclosures in release notes (with the reporter's consent) and welcome contributions to fix validated issues.

## Scope Guidance

Please report vulnerabilities affecting:

- Soroban contract logic and state transition safety
- Frontend auth/session assumptions or wallet interaction risks
- Dependency or supply-chain issues that impact this repository

Out-of-scope examples:

- Social engineering attempts
- Denial-of-service scenarios without a practical exploit path
- Reports requiring non-default insecure local configuration
