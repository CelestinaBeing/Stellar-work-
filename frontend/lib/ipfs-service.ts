const IPFS_GATEWAY = process.env.NEXT_PUBLIC_IPFS_GATEWAY_URL ?? "https://dweb.link/ipfs/";
const WEB3_STORAGE_TOKEN = process.env.NEXT_PUBLIC_WEB3_STORAGE_TOKEN ?? "";

function normalizeGateway(url: string): string {
  return url.endsWith("/") ? url : `${url}/`;
}

export async function uploadToIpfs(text: string): Promise<string> {
  if (WEB3_STORAGE_TOKEN) {
    return uploadViaWeb3Storage(text);
  }
  return uploadFallback(text);
}

export async function fetchFromIpfs(cid: string): Promise<string> {
  const gateway = normalizeGateway(IPFS_GATEWAY);
  const url = `${gateway}${cid}`;
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`IPFS fetch failed: ${response.status} ${response.statusText}`);
  }
  return response.text();
}

async function uploadViaWeb3Storage(text: string): Promise<string> {
  const response = await fetch("https://api.web3.storage/upload", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${WEB3_STORAGE_TOKEN}`,
      "Content-Type": "text/plain",
    },
    body: text,
  });
  if (!response.ok) {
    throw new Error(`web3.storage upload failed: ${response.status} ${response.statusText}`);
  }
  const data = await response.json() as { cid?: string };
  if (!data.cid) {
    throw new Error("web3.storage did not return a CID");
  }
  return data.cid;
}

async function uploadFallback(text: string): Promise<string> {
  const bytes = new TextEncoder().encode(text);
  const hash = await crypto.subtle.digest("SHA-256", bytes);
  const hashHex = Array.from(new Uint8Array(hash))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  localStorage.setItem(`job-desc:${hashHex}`, text);
  return `fallback:${hashHex}`;
}

export function isFallbackCid(cid: string): boolean {
  return cid.startsWith("fallback:");
}

export function getFallbackHash(cid: string): string {
  return cid.replace("fallback:", "");
}
