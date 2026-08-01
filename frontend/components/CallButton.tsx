"use client";

import { useState, useCallback } from "react";
import CallOverlay from "./CallOverlay";

interface CallButtonProps {
  myAddress: string;
  peerAddress: string;
  userName?: string;
  variant?: "voice" | "video";
  size?: "sm" | "md";
}

export default function CallButton({
  myAddress,
  peerAddress,
  userName = "User",
  variant = "voice",
  size = "md",
}: CallButtonProps) {
  const [showCall, setShowCall] = useState(false);

  const handleStart = useCallback(() => {
    if (!navigator.mediaDevices?.getUserMedia) {
      alert("Your browser does not support calling. Please use a modern browser.");
      return;
    }
    setShowCall(true);
  }, []);

  const handleClose = useCallback(() => {
    setShowCall(false);
  }, []);

  const isVideo = variant === "video";
  const sizeClasses = size === "sm" ? "h-8 w-8" : "h-9 w-9";

  return (
    <>
      <button
        type="button"
        onClick={handleStart}
        className={`inline-flex items-center justify-center rounded-full transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 ${sizeClasses} ${
          isVideo
            ? "bg-blue-600 text-white hover:bg-blue-700"
            : "bg-emerald-600 text-white hover:bg-emerald-700"
        }`}
        aria-label={isVideo ? "Start video call" : "Start voice call"}
        title={isVideo ? "Video call" : "Voice call"}
      >
        {isVideo ? (
          <svg className={size === "sm" ? "h-4 w-4" : "h-5 w-5"} fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5} aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M15 13V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h8a2 2 0 002-2zm0 0l4.5 3V7L15 10" />
          </svg>
        ) : (
          <svg className={size === "sm" ? "h-4 w-4" : "h-5 w-5"} fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5} aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" d="M3.5 16.5c4.5-4.5 12.5-4.5 17 0M5 14c3.5-3 9.5-3 13 0M6.5 11.5c2.5-2.5 7.5-2.5 10 0" />
          </svg>
        )}
      </button>

      {showCall && (
        <CallOverlay
          myAddress={myAddress}
          peerAddress={peerAddress}
          callType={isVideo ? "video" : "audio"}
          userName={userName}
          onClose={handleClose}
        />
      )}
    </>
  );
}
