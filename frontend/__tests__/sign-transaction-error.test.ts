import { beforeEach, describe, expect, it, vi } from "vitest";

// ── Freighter mock ────────────────────────────────────────────────────────────
const freighterSignTransaction = vi.fn();
const isAllowedMock = vi.fn();
const getAddressMock = vi.fn();

vi.mock("@stellar/freighter-api", () => ({
  isAllowed: (...args: unknown[]) => isAllowedMock(...args),
  getAddress: (...args: unknown[]) => getAddressMock(...args),
  requestAccess: vi.fn(),
  signTransaction: (...args: unknown[]) => freighterSignTransaction(...args),
}));

// Minimal stellar-sdk stub so the module resolves without a real SDK install.
vi.mock("@stellar/stellar-sdk", () => ({
  Account: class {
    constructor(public id: string, public sequence: string) {}
    accountId() { return this.id; }
    incrementSequenceNumber() { void 0; }
    sequenceNumber() { return this.sequence; }
  },
  BASE_FEE: "100",
  Contract: class { call() { return {}; } },
  Keypair: { random: () => ({ publicKey: () => "G" }) },
  Networks: { PUBLIC: "Public", TESTNET: "Test SDF Network ; September 2015" },
  nativeToScVal: (v: unknown) => v,
  rpc: {
    Server: vi.fn().mockImplementation(() => ({
      getAccount: vi.fn(),
      simulateTransaction: vi.fn(),
      sendTransaction: vi.fn(),
      prepareTransaction: vi.fn(),
      getTransaction: vi.fn(),
    })),
    Api: {
      isSimulationError: vi.fn().mockReturnValue(false),
      isSimulationSuccess: vi.fn().mockReturnValue(true),
      SendTransactionStatus: { ERROR: "ERROR", PENDING: "PENDING", SUCCESS: "SUCCESS" },
      GetTransactionStatus: { SUCCESS: "SUCCESS", FAILED: "FAILED" },
    },
    assembleTransaction: vi.fn(() => ({ build: vi.fn().mockReturnValue({ toXDR: () => "XDR" }) })),
  },
  scValToNative: (v: unknown) => v,
  TransactionBuilder: class {
    addOperation() { return this; }
    setTimeout() { return this; }
    build() { return { toXDR: () => "XDR" }; }
    static fromXDR() { return {}; }
  },
  xdr: {},
}));

import { signTransaction } from "../lib/stellar";

describe("signTransaction error propagation", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("throws when Freighter returns an error field", async () => {
    freighterSignTransaction.mockResolvedValue({ error: "User rejected the request." });

    await expect(signTransaction("SOME_XDR")).rejects.toThrow(
      "User rejected the request.",
    );
  });

  it("throws when Freighter rejects with a native Error", async () => {
    freighterSignTransaction.mockRejectedValue(new Error("Wallet locked"));

    await expect(signTransaction("SOME_XDR")).rejects.toThrow("Wallet locked");
  });

  it("propagates the error message string intact so the UI can display it", async () => {
    const msg = "Insufficient balance to cover the transaction fee.";
    freighterSignTransaction.mockResolvedValue({ error: msg });

    let caught: Error | null = null;
    try {
      await signTransaction("SOME_XDR");
    } catch (err) {
      caught = err as Error;
    }

    expect(caught).not.toBeNull();
    expect(caught!.message).toBe(msg);
  });

  it("returns the signed XDR string when signing succeeds", async () => {
    freighterSignTransaction.mockResolvedValue({ signedTxXdr: "SIGNED_XDR" });

    const result = await signTransaction("SOME_XDR");

    expect(result).toBe("SIGNED_XDR");
  });
});
