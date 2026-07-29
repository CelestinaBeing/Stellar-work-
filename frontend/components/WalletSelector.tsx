"use client";

import { useState } from "react";
import { walletManager, WalletType } from "@/lib/unified-wallet";

interface WalletSelectorProps {
  onConnect: (walletType: WalletType) => Promise<void>;
  onDisconnect: () => Promise<void>;
  isConnected: boolean;
  currentWalletType: WalletType | null;
}

export function WalletSelector({
  onConnect,
  onDisconnect,
  isConnected,
  currentWalletType,
}: WalletSelectorProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedWallet, setSelectedWallet] = useState<WalletType | null>(null);

  const wallets: { type: WalletType; name: string; icon: string }[] = [
    { type: "freighter", name: "Freighter", icon: "🚢" },
    { type: "walletconnect", name: "WalletConnect", icon: "📱" },
    { type: "ledger", name: "Ledger", icon: "🔐" },
  ];

  const handleConnect = async (walletType: WalletType) => {
    setSelectedWallet(walletType);
    setIsOpen(false);
    try {
      await onConnect(walletType);
    } catch (error) {
      console.error(`Failed to connect ${walletType}:`, error);
      setSelectedWallet(null);
    }
  };

  const handleDisconnect = async () => {
    await onDisconnect();
    setSelectedWallet(null);
  };

  if (isConnected && currentWalletType) {
    const wallet = wallets.find((w) => w.type === currentWalletType);
    return (
      <div className="flex items-center gap-2">
        <span className="text-sm text-slate-600">
          {wallet?.icon} {wallet?.name} Connected
        </span>
        <button
          onClick={handleDisconnect}
          className="rounded-md border border-slate-300 px-3 py-1.5 text-sm text-slate-600 hover:bg-slate-50"
        >
          Disconnect
        </button>
      </div>
    );
  }

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="rounded-md bg-slate-900 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800"
      >
        Connect Wallet
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-2 w-56 rounded-md border border-slate-200 bg-white shadow-lg">
          <div className="p-2">
            {wallets.map((wallet) => (
              <button
                key={wallet.type}
                onClick={() => handleConnect(wallet.type)}
                className="flex w-full items-center gap-3 rounded-md px-3 py-2 text-left text-sm text-slate-700 hover:bg-slate-50"
              >
                <span className="text-lg">{wallet.icon}</span>
                <span>{wallet.name}</span>
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
