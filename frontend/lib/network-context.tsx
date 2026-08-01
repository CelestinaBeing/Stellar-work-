"use client";

import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  type ReactNode,
} from "react";
import {
  type StellarNetwork,
  getDefaultNetwork,
  getPersistedNetwork,
  persistNetwork,
  getNetworkConfig,
  type NetworkConfig,
  NETWORK_LIST,
} from "@/lib/network-config";

interface NetworkContextType {
  network: StellarNetwork;
  setNetwork: (network: StellarNetwork) => void;
  config: NetworkConfig;
  isPendingSwitch: boolean;
  confirmSwitch: (network: StellarNetwork) => void;
  cancelSwitch: () => void;
}

const NetworkContext = createContext<NetworkContextType>({
  network: "testnet",
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  setNetwork: () => {},
  config: getNetworkConfig("testnet"),
  isPendingSwitch: false,
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  confirmSwitch: () => {},
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  cancelSwitch: () => {},
});

export function NetworkProvider({ children }: { children: ReactNode }) {
  const [network, setNetworkState] = useState<StellarNetwork>(getDefaultNetwork);
  const [pendingNetwork, setPendingNetwork] = useState<StellarNetwork | null>(null);
  const [hydrated, setHydrated] = useState(false);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    setNetworkState(getPersistedNetwork());
    setHydrated(true);
  }, []);

  const setNetwork = useCallback((newNetwork: StellarNetwork) => {
    setNetworkState(newNetwork);
    persistNetwork(newNetwork);
  }, []);

  const confirmSwitch = useCallback((newNetwork: StellarNetwork) => {
    setPendingNetwork(newNetwork);
  }, []);

  const cancelSwitch = useCallback(() => {
    setPendingNetwork(null);
  }, []);

  useEffect(() => {
    if (pendingNetwork !== null) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setNetwork(pendingNetwork);
      setPendingNetwork(null);
      window.location.reload();
    }
  }, [pendingNetwork, setNetwork]);

  const config = getNetworkConfig(network);

  if (!hydrated) {
    return (
      <NetworkContext.Provider
        value={{
          network: getDefaultNetwork(),
          // eslint-disable-next-line @typescript-eslint/no-empty-function
          setNetwork: () => {},
          config: getNetworkConfig(getDefaultNetwork()),
          isPendingSwitch: false,
          // eslint-disable-next-line @typescript-eslint/no-empty-function
          confirmSwitch: () => {},
          // eslint-disable-next-line @typescript-eslint/no-empty-function
          cancelSwitch: () => {},
        }}
      >
        {children}
      </NetworkContext.Provider>
    );
  }

  return (
    <NetworkContext.Provider
      value={{
        network,
        setNetwork,
        config,
        isPendingSwitch: pendingNetwork !== null,
        confirmSwitch,
        cancelSwitch,
      }}
    >
      {children}
    </NetworkContext.Provider>
  );
}

export function useNetwork() {
  return useContext(NetworkContext);
}

export { NETWORK_LIST };
