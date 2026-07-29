"use client";

import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  useRef,
  type ReactNode,
} from "react";
import {
  connectWallet as stellarConnectWallet,
  getPublicKey,
} from "@/lib/stellar";
import LegalConsentModal, { hasAcceptedLegal, acceptLegal } from "@/components/LegalConsentModal";

interface WalletContextType {
  wallet: string | null;
  connectWallet: () => Promise<void>;
  disconnectWallet: () => void;
}

type WalletDisplayMode = "short" | "full";

const WalletContext = createContext<WalletContextType>({
  wallet: null,
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  connectWallet: async () => {},
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  disconnectWallet: () => {},
});

export function WalletProvider({ children }: { children: ReactNode }) {
  const [wallet, setWallet] = useState<string | null>(null);
  const [showLegalModal, setShowLegalModal] = useState(false);
  const connectPromiseRef = useRef<Promise<string> | null>(null);

  useEffect(() => {
    getPublicKey().then((key) => {
      if (key) setWallet(key);
    });
  }, []);

  useEffect(() => {
    if (wallet && !hasAcceptedLegal()) {
      setShowLegalModal(true);
    }
  }, [wallet]);

  const connectWallet = useCallback(async () => {
    if (wallet) {
      return;
    }

    if (!connectPromiseRef.current) {
      connectPromiseRef.current = stellarConnectWallet().finally(() => {
        connectPromiseRef.current = null;
      });
    }

    const key = await connectPromiseRef.current;
    setWallet(key);
  }, [wallet]);

  const disconnectWallet = useCallback(() => {
    setWallet(null);
  }, []);

  const declineLegal = useCallback(() => {
    setShowLegalModal(false);
    setWallet(null);
  }, []);

  return (
    <WalletContext.Provider value={{ wallet, connectWallet, disconnectWallet }}>
      {children}
      {showLegalModal && (
        <LegalConsentModal
          onAccept={() => setShowLegalModal(false)}
          onClose={declineLegal}
        />
      )}
    </WalletContext.Provider>
  );
}

export function useWallet() {
  return useContext(WalletContext);
}

export function WalletButton() {
  const { wallet, connectWallet, disconnectWallet } = useWallet();
  const [connecting, setConnecting] = useState(false);
  const [displayMode, setDisplayMode] = useState<WalletDisplayMode>("short");

  useEffect(() => {
    const stored = sessionStorage.getItem("wallet-display-mode");
    if (stored === "short" || stored === "full") {
      setDisplayMode(stored);
    }
  }, []);

  useEffect(() => {
    if (!wallet) {
      setDisplayMode("short");
    }
  }, [wallet]);

  const toggleDisplayMode = useCallback(() => {
    setDisplayMode((current) => {
      const next: WalletDisplayMode = current === "short" ? "full" : "short";
      sessionStorage.setItem("wallet-display-mode", next);
      return next;
    });
  }, []);

  if (wallet) {
    const visibleAddress =
      displayMode === "full" ? wallet : `${wallet.slice(0, 6)}...${wallet.slice(-4)}`;

    return (
      <div className="flex items-center gap-2">
        <span className="rounded-md bg-slate-100 px-3 py-1.5 font-mono text-xs text-slate-700">
          {visibleAddress}
        </span>
        <button
          type="button"
          onClick={toggleDisplayMode}
          className="rounded-md border border-slate-300 px-3 py-1.5 text-xs text-slate-600 hover:bg-slate-50"
          aria-label={
            displayMode === "short"
              ? "Show full wallet address"
              : "Show shortened wallet address"
          }
        >
          {displayMode === "short" ? "Show full" : "Show short"}
        </button>
        <button
          type="button"
          onClick={disconnectWallet}
          className="rounded-md border border-slate-300 px-3 py-1.5 text-sm text-slate-600 hover:bg-slate-50"
        >
          Disconnect
        </button>
      </div>
    );
  }

  return (
    <button
      onClick={async () => {
        setConnecting(true);
        try {
          await connectWallet();
        } catch {
          /* user cancelled or Freighter unavailable */
        } finally {
          setConnecting(false);
        }
      }}
      className="rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white disabled:cursor-not-allowed disabled:opacity-60"
      disabled={connecting}
      aria-busy={connecting}
    >
      {connecting ? "Connecting..." : "Connect Wallet"}
    </button>
  );
}
