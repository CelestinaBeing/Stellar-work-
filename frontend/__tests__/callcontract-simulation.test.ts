import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

// Spy state shared with the stellar-sdk mock.
const simulateTransaction = vi.fn();
const sendTransaction = vi.fn();
const prepareTransaction = vi.fn();
const getAccount = vi.fn();
const assembleTransactionBuild = vi.fn();
const isSimulationError = vi.fn();
const isSimulationSuccess = vi.fn();
const signTransactionSpy = vi.fn();

const isAllowedMock = vi.fn();
const getAddressMock = vi.fn();

class FakeAccount {
  constructor(public id: string, public sequence: string) {}
  accountId() {
    return this.id;
  }
  incrementSequenceNumber() { void 0; }
  sequenceNumber() {
    return this.sequence;
  }
}

class FakeContract {
  constructor(public contractId: string) {}
  call(method: string, ...args: unknown[]) {
    return { __op: "call", method, args };
  }
}

class FakeTransactionBuilder {
  private operations: unknown[] = [];
  constructor(public source: unknown, public opts: unknown) {}
  addOperation(op: unknown) {
    this.operations.push(op);
    return this;
  }
  setTimeout() {
    return this;
  }
  build() {
    return { __tx: true, source: this.source, operations: this.operations };
  }
  static fromXDR(xdrValue: string) {
    return { __tx: true, fromXdr: xdrValue };
  }
}

const sdkMocks = {
  Account: FakeAccount,
  BASE_FEE: "100",
  Contract: FakeContract,
  Keypair: { random: () => ({ publicKey: () => "GANY" }) },
  Networks: { PUBLIC: "Public", TESTNET: "Test SDF Network ; September 2015" },
  nativeToScVal: (value: unknown) => ({ __scval: value }),
  rpc: {
    Server: vi.fn().mockImplementation(() => ({
      getAccount,
      simulateTransaction,
      sendTransaction,
      prepareTransaction,
      getTransaction: vi.fn(),
    })),
    Api: {
      isSimulationError: (sim: unknown) => isSimulationError(sim),
      isSimulationSuccess: (sim: unknown) => isSimulationSuccess(sim),
      SendTransactionStatus: { ERROR: "ERROR", PENDING: "PENDING", SUCCESS: "SUCCESS" },
      GetTransactionStatus: { SUCCESS: "SUCCESS", FAILED: "FAILED" },
    },
    assembleTransaction: vi.fn(() => ({ build: assembleTransactionBuild })),
  },
  scValToNative: (value: unknown) => {
    if (value && typeof value === "object" && "__decoded" in value) {
      return (value as { __decoded: unknown }).__decoded;
    }
    return value;
  },
  TransactionBuilder: FakeTransactionBuilder,
  xdr: {},
};

vi.mock("@stellar/stellar-sdk", () => sdkMocks);

vi.mock("@stellar/freighter-api", () => ({
  getAddress: (...args: unknown[]) => getAddressMock(...args),
  isAllowed: (...args: unknown[]) => isAllowedMock(...args),
  requestAccess: vi.fn(),
  signTransaction: (...args: unknown[]) => signTransactionSpy(...args),
}));

// Import the module under test AFTER mocks are registered.
import { callContract } from "../lib/stellar";

describe("callContract read-only simulation path", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    isSimulationError.mockReturnValue(false);
    isSimulationSuccess.mockReturnValue(true);
    assembleTransactionBuild.mockReturnValue({ toXDR: () => "ASSEMBLED_XDR" });
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it("simulates without signing and decodes the simulation retval", async () => {
    // No wallet connected -> falls back to placeholder source.
    isAllowedMock.mockResolvedValue({ isAllowed: false });
    getAddressMock.mockResolvedValue({ address: null, error: "not allowed" });

    simulateTransaction.mockResolvedValue({
      result: { retval: { __decoded: 42 } },
    });

    const result = await callContract("CCONTRACT", "get_job_count", [], {
      readOnly: true,
    });

    expect(result).toEqual({ status: "SUCCESS", data: 42 });
    expect(simulateTransaction).toHaveBeenCalledTimes(1);

    // No signing or sending should happen on a read-only path.
    expect(signTransactionSpy).not.toHaveBeenCalled();
    expect(sendTransaction).not.toHaveBeenCalled();
    expect(prepareTransaction).not.toHaveBeenCalled();
  });

  it("returns an error result when the simulation has no retval", async () => {
    isAllowedMock.mockResolvedValue({ isAllowed: false });
    getAddressMock.mockResolvedValue({ address: null, error: "not allowed" });

    simulateTransaction.mockResolvedValue({ result: {} });

    const result = await callContract("CCONTRACT", "get_job", [], {
      readOnly: true,
    });

    expect(result.status).toBe("ERROR");
    expect(signTransactionSpy).not.toHaveBeenCalled();
  });

  it("propagates simulation errors as thrown exceptions", async () => {
    isAllowedMock.mockResolvedValue({ isAllowed: false });
    getAddressMock.mockResolvedValue({ address: null, error: "not allowed" });

    isSimulationError.mockReturnValue(true);
    simulateTransaction.mockResolvedValue({ error: "boom" });

    await expect(
      callContract("CCONTRACT", "get_job", [], { readOnly: true }),
    ).rejects.toThrow("boom");

    expect(signTransactionSpy).not.toHaveBeenCalled();
  });

  it("throws when wallet is not connected and the call is not read-only", async () => {
    isAllowedMock.mockResolvedValue({ isAllowed: false });
    getAddressMock.mockResolvedValue({ address: null, error: "not allowed" });

    await expect(
      callContract("CCONTRACT", "post_job", []),
    ).rejects.toThrow("Connect Freighter before calling contract.");

    // It must not even attempt to simulate or sign.
    expect(simulateTransaction).not.toHaveBeenCalled();
    expect(signTransactionSpy).not.toHaveBeenCalled();
  });

  it("uses the wallet's account as the simulation source when one is connected", async () => {
    isAllowedMock.mockResolvedValue({ isAllowed: true });
    getAddressMock.mockResolvedValue({ address: "GWALLET", error: undefined });
    getAccount.mockResolvedValue(new FakeAccount("GWALLET", "0"));

    simulateTransaction.mockResolvedValue({
      result: { retval: { __decoded: "value" } },
    });

    const result = await callContract("CCONTRACT", "get_job", [], {
      readOnly: true,
    });

    expect(getAccount).toHaveBeenCalledWith("GWALLET");
    expect(result).toEqual({ status: "SUCCESS", data: "value" });
    expect(signTransactionSpy).not.toHaveBeenCalled();
  });
});
