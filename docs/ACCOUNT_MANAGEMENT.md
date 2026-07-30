# Stellar Account Recovery & Key Management Guide

This guide explains how to manage your Stellar accounts, secure your keys, and recover access to the StellarWork platform. Since Stellar is a decentralized network, you have sole ownership and control of your account and funds.

---

## 1. What is a Stellar Wallet and How it Works

A Stellar wallet does not actually "store" your funds (like XLM or other assets). Instead, all assets and transaction histories are stored on the **Stellar ledger (blockchain)**. 

Your wallet is a software or hardware tool that stores your **cryptographic keys** and allows you to interact with the blockchain:

*   **Public Key (G...):** This is your account address. Think of it like your email address or bank account number. It is safe to share with others so they can send you payments or view your profile.
*   **Secret Key (S...):** This is your password/signing key. It is used to authorize transactions (e.g., accepting a job, submitting work, releasing escrow). **Anyone who has your secret key has full control over your funds and account.**

StellarWork is **non-custodial**. We do not store your secret keys on our servers. All transaction signing happens locally inside your browser wallet (like Freighter).

---

## 2. Installing and Setting Up Freighter

Freighter is the recommended wallet for StellarWork. It is a secure, non-custodial browser extension developed by the Stellar Development Foundation.

### Step-by-Step Installation:
1.  Visit the official website: [freighter.app](https://www.freighter.app/) or search for "Freighter" in your browser's extension store (Chrome, Firefox, Edge, Brave).
2.  Click **Install** or **Add to Browser**.
3.  Once installed, click the Freighter icon in your browser toolbar to open it.
4.  Click **Create Wallet** if you do not have an existing Stellar account.
5.  Set a strong, unique password to secure the Freighter extension on your device.

---

## 3. Backing Up Your Secret Key / Mnemonic Phrase

During the creation of your Freighter wallet, you will be presented with a **12-word Recovery Phrase** (also known as a mnemonic seed phrase). This phrase is a human-readable representation of your master secret key.

> [!IMPORTANT]
> Your 12-word recovery phrase is the ONLY way to recover your wallet if your computer breaks, is lost, or if you forget your Freighter password.

### Best Practices for Backing Up:
*   **Write it down physically:** Write the 12 words on a piece of paper in the exact order shown.
*   **Store it securely:** Keep this paper in a safe, fireproof, and waterproof location (e.g., a physical safe or deposit box).
*   **Do NOT save it digitally:** Avoid taking screenshots, saving it in notes apps, emailing it to yourself, or storing it on cloud drives. Hackers target digital storage for recovery phrases.
*   **Never share it:** No administrator, support agent, or smart contract will ever ask for your recovery phrase or secret key.

---

## 4. Recovering a Wallet from Backup

If you get a new device, reinstall your browser, or forget your Freighter password, you can easily restore your wallet using your backup.

### How to Restore in Freighter:
1.  Open the Freighter extension.
2.  Instead of creating a new wallet, click **Import Wallet** (or "Import Recovery Phrase").
3.  Enter your **12-word Recovery Phrase** in the correct order.
4.  Set a new password for the extension.
5.  Your public address and all associated balances/history will be restored.

---

## 5. Switching Between Wallets

You may want to use multiple accounts (e.g., one for freelancing work and one for posting jobs as a client). Freighter makes it easy to manage and switch between multiple accounts.

### Adding Another Account:
1.  Open Freighter and unlock it.
2.  Click the account dropdown at the top of the extension.
3.  Click **Create Account** to generate a brand new address, or **Import Account** to add an existing account using its specific **Secret Key (starting with S)**.

### Switching Accounts on StellarWork:
1.  In Freighter, select the account you wish to use.
2.  Go to StellarWork and click your wallet address in the top-right header.
3.  Click **Disconnect** and then **Connect Wallet** again, or simply refresh the page. StellarWork will automatically detect the active account in Freighter and update your session.

---

## 6. Security Best Practices

To keep your assets and identity safe, adhere to these security rules:

*   **Use a Hardware Wallet:** For maximum security, connect a hardware wallet (like Ledger) to Freighter. Your secret keys will never leave the physical device, and you must physically press buttons on the device to sign transactions.
*   **Verify URLs:** Always ensure you are on the official StellarWork domain. Bookmark the site to avoid phishing clones.
*   **Never Input Secret Keys Online:** Never type your secret key or recovery phrase into any website or form. Freighter only requires them during the initial setup inside the extension itself.
*   **Avoid Shared Devices:** Do not access your wallet or sign transactions on public or shared computers.
*   **Double-Check Transaction Details:** Before clicking "Approve" in the Freighter popup, verify the transaction type, amounts, and destination addresses.

---

## 7. What Happens if You Lose Access?

Because StellarWork is decentralized and non-custodial:
*   **We cannot reset your password.**
*   **We cannot recover your recovery phrase or secret key.**
*   **We cannot transfer funds out of your account on your behalf.**

### If you lose your recovery phrase AND your device:
Unfortunately, **your funds and account are permanently lost.** There is no central authority or support team that can recover them.

### If you lose your Freighter password but still have your 12-word recovery phrase:
1.  Uninstall the Freighter extension.
2.  Reinstall Freighter.
3.  Follow the **Recovering a Wallet from Backup** steps above using your 12-word phrase.
