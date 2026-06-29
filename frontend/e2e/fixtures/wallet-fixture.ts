/* eslint-disable react-hooks/rules-of-hooks */

import { test as base, type Page } from "@playwright/test";

const MOCK_WALLET_ADDRESS =
  "GBZXM4PURFDMDPPCYFQSPH3LZODXWMFY2VAWIPKAIHHQEA2XBGV5WQJM";

export interface WalletFixture {
  mockWallet: string;
  connectWallet: (page: Page) => Promise<void>;
}

export const test = base.extend<WalletFixture>({
  mockWallet: MOCK_WALLET_ADDRESS,

  connectWallet: async ({}, use) => {
    const connect = async (p: Page) => {
      await p.evaluate(
        (addr) => {
          Object.defineProperty(window, "freighter", {
            value: {
              isConnected: () => Promise.resolve(true),
              getPublicKey: () => Promise.resolve(addr),
              signTransaction: () =>
                Promise.resolve(
                  "AAAAAgAAAAD...mock_signature_base64...",
                ),
              getNetwork: () => Promise.resolve("TESTNET"),
            },
            writable: true,
            configurable: true,
          });
        },
        MOCK_WALLET_ADDRESS,
      );
    };
    await use(connect);
  },
});

export { expect } from "@playwright/test";
