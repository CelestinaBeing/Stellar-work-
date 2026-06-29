# Video Tutorials & Screencast Guide

This document covers everything needed to produce official StellarWork video tutorials: tooling, recording setup, format standards, voiceover scripts, the planned tutorial series outline, and closed captioning guidelines.

---

## Table of Contents

1. [Recommended Recording Tools](#1-recommended-recording-tools)
2. [Recording Setup & Environment](#2-recording-setup--environment)
3. [Video Format & Resolution Guidelines](#3-video-format--resolution-guidelines)
4. [Voiceover Script Template](#4-voiceover-script-template)
5. [Official Tutorial Series Outline](#5-official-tutorial-series-outline)
6. [Closed Captioning Guidelines](#6-closed-captioning-guidelines)
7. [Publishing Checklist](#7-publishing-checklist)

---

## 1. Recommended Recording Tools

### Screen Recording

| Tool | Platform | Cost | Best For |
|------|----------|------|----------|
| [OBS Studio](https://obsproject.com/) | Windows / macOS / Linux | Free | Full-featured recording + streaming |
| QuickTime Player | macOS | Free (built-in) | Quick single-window captures |
| [Loom](https://www.loom.com/) | Windows / macOS / Browser | Free tier | Fast share-link workflows |
| [Camtasia](https://www.techsmith.com/video-editor.html) | Windows / macOS | Paid | All-in-one record + edit |
| [Kap](https://getkap.co/) | macOS | Free | Lightweight GIF/MP4 captures |

### Video Editing

| Tool | Platform | Cost |
|------|----------|------|
| [DaVinci Resolve](https://www.blackmagicdesign.com/products/davinciresolve) | Windows / macOS / Linux | Free |
| [iMovie](https://www.apple.com/imovie/) | macOS | Free (built-in) |
| [Kdenlive](https://kdenlive.org/) | Windows / macOS / Linux | Free |
| Adobe Premiere Pro | Windows / macOS | Paid |

### Captioning

| Tool | Notes |
|------|-------|
| [YouTube Studio](https://studio.youtube.com/) | Auto-generates captions; manual correction recommended |
| [Rev.com](https://www.rev.com/) | Human-reviewed captions, SRT export |
| [Otter.ai](https://otter.ai/) | AI transcription with speaker labels |
| [MacWhisper](https://goodsnooze.gumroad.com/l/macwhisper) | Local Whisper AI transcription, macOS |

---

## 2. Recording Setup & Environment

### Browser & App Preparation

- Use a **dedicated browser profile** with only the Freighter extension installed — no personal bookmarks, notifications, or other extensions visible.
- Set browser zoom to **100%**. Avoid system scaling above 125% to keep UI elements crisp.
- **Disable notifications** at OS level (Do Not Disturb / Focus mode) before recording.
- Clear any previous wallet state or use a fresh testnet wallet so the walkthrough starts from a clean slate.
- Pre-fund the testnet wallet with enough XLM to cover all demo transactions (at least 100 XLM from [Friendbot](https://friendbot.stellar.org/)).
- Close all tabs except the ones used in the tutorial.

### OBS Studio Setup

```
Settings → Video
  Base (Canvas) Resolution: 1920×1080
  Output (Scaled) Resolution: 1920×1080
  FPS: 30

Settings → Audio
  Sample Rate: 48 kHz
  Desktop Audio: disabled (capture only mic)

Settings → Output
  Recording Format: MP4
  Encoder: x264 (software) or NVENC (hardware if available)
  Rate Control: CRF
  CRF Value: 18–22  (lower = better quality, larger file)

Sources
  Add: Display Capture or Window Capture (browser window)
  Add: Audio Input Capture (microphone)
```

### QuickTime Setup (macOS)

1. File → New Screen Recording.
2. Click the dropdown arrow next to the record button → select your microphone.
3. Choose "Record Selected Portion" and drag to the browser window at 1920×1080.
4. Click Record.

### Microphone Tips

- Use a dedicated USB or XLR microphone rather than a built-in laptop mic.
- Record in a quiet room; close windows and turn off fans/HVAC while speaking.
- Run a 10-second test clip and check levels in your editing software before the full take.
- Target peak volume around **−6 dBFS**, average around **−18 dBFS** (LUFS).

### Cursor & Highlight

- Enable a cursor highlight plugin in OBS or use [Cursor Pro](https://cursor.pro/) (macOS) to make mouse movements easy to follow.
- Slow down mouse movements deliberately — viewers cannot track fast cursor sweeps.

---

## 3. Video Format & Resolution Guidelines

### Delivery Specifications

| Property | Value |
|----------|-------|
| Resolution | 1920×1080 (1080p) minimum; 2560×1440 (1440p) preferred for Retina captures |
| Frame rate | 30 fps (24 fps acceptable for slow UI walkthroughs) |
| Video codec | H.264 (broad compatibility) |
| Audio codec | AAC, stereo, 192 kbps |
| Container | MP4 (.mp4) |
| Max file size | 2 GB per video (split longer content into parts) |
| Color space | sRGB / BT.709 |

### Aspect Ratio

All tutorials must be **16:9**. Avoid recording at 4:3 or ultra-wide ratios — YouTube and embedded players will letterbox or crop them.

### Thumbnail Specification

| Property | Value |
|----------|-------|
| Dimensions | 1280×720 px |
| Format | PNG or JPG |
| Font | Match brand (Geist Sans) |
| Background | White or `#0F172A` (slate-900) |
| Required elements | Tutorial title, StellarWork logo, episode number |

### Naming Convention

```
SW-<episode-number>-<slug>-v<version>.mp4

Examples:
  SW-01-connecting-your-wallet-v1.mp4
  SW-03-accepting-jobs-v2.mp4
```

---

## 4. Voiceover Script Template

Use this template for every tutorial. Fill in the bracketed sections before recording. Aim for a conversational but precise tone — speak as if explaining to a colleague, not reading a manual.

```
============================================================
TUTORIAL SCRIPT — [EPISODE NUMBER]: [TITLE]
Approx. duration: [X] minutes
Last updated: [DATE]
Author: [NAME]
============================================================

── INTRO (≈ 30 seconds) ─────────────────────────────────

"Hi, and welcome to StellarWork. In this video, [one-sentence
summary of what the viewer will learn and accomplish].

By the end, you'll be able to [specific outcome — e.g.,
'connect your Freighter wallet and browse open jobs'].

Let's get started."

── SECTION 1: [SECTION TITLE] (≈ X minutes) ─────────────

[Visual cue: describe what should be visible on screen]

"[Narration text. One sentence per action. Lead with what
the viewer sees, then explain why.]

For example: 'You'll see the Connect Wallet button in the
top-right corner. Click it — this triggers Freighter to ask
for permission to share your public key with the site.'"

── SECTION 2: [SECTION TITLE] (≈ X minutes) ─────────────

[Visual cue: ...]

"[Narration...]"

... (repeat for each section) ...

── OUTRO (≈ 20 seconds) ─────────────────────────────────

"That's everything for this tutorial. If you have questions,
open an issue on GitHub or join the community discussion.

Links to the next video and the written docs are in the
description below. See you there."

============================================================
SCREEN ACTION LOG (for editor reference)
============================================================
00:00  Title card with episode title
00:05  Browser window — [starting URL]
[MM:SS]  [action description]
...
============================================================
```

### Tone Guidelines

- Speak in the **second person** ("you click", "you'll see") — keeps the viewer engaged.
- Use **active voice**: "Click the button" not "The button should be clicked".
- **Pause briefly** after each action to give viewers time to follow along.
- Avoid filler words (um, uh, like, basically).
- Keep each section under 3 minutes; shorter segments are easier to re-watch.

---

## 5. Official Tutorial Series Outline

Each entry below is a planned video. Status column tracks production state.

| # | Title | Audience | Est. Duration | Status |
|---|-------|----------|---------------|--------|
| 01 | Getting Started: Connecting Your Wallet | All | 4 min | Planned |
| 02 | Posting Your First Job | Clients | 6 min | Planned |
| 03 | Finding and Accepting Jobs as a Freelancer | Freelancers | 5 min | Planned |
| 04 | Completing a Job and Getting Paid | Freelancers | 5 min | Planned |
| 05 | Managing Disputes | All | 7 min | Planned |

---

### Episode 01 — Getting Started: Connecting Your Wallet

**Goal:** Viewer installs Freighter, creates/imports a testnet wallet, and connects it to StellarWork.

**Prerequisites:** Chrome or Brave browser.

**Outline:**

1. **Install Freighter** — navigate to the Chrome Web Store, search "Freighter", install the extension. (~1 min)
2. **Create a wallet** — open Freighter, create a new wallet or import an existing one with a mnemonic phrase. (~1 min)
3. **Fund with testnet XLM** — visit Stellar Friendbot, paste the public key, click "Get test XLM". Confirm balance appears in Freighter. (~1 min)
4. **Connect to StellarWork** — open the app, click "Connect Wallet" in the nav header, approve the Freighter popup, confirm the short address appears in the header. (~1 min)

**Key callouts:**
- Explain the difference between testnet and mainnet.
- Warn viewers never to share their secret key or mnemonic.
- Show the green "connected" indicator in the nav.

---

### Episode 02 — Posting Your First Job

**Goal:** Client posts a job with a description, amount, deadline, and token; confirms it appears in the job list.

**Prerequisites:** Connected wallet with testnet XLM balance.

**Outline:**

1. **Navigate to Post Job** — click "Post Job" in the nav. (~15 sec)
2. **Fill in job details** — enter a title/description, set XLM amount (e.g., 50 XLM), choose a deadline, leave token as default native XLM. (~2 min)
3. **Submit and sign** — click "Post Job", Freighter popup appears asking to sign the transaction. Approve it. (~1 min)
4. **Confirm on-chain** — wait for confirmation toast, then navigate to the home job list and find the new job at the top. (~1 min)
5. **View job details** — click into the job to show the escrow details, description, and available actions. (~1 min)

**Key callouts:**
- Explain that funds are locked in escrow immediately on posting.
- Show where to find the transaction hash and link to Stellar Explorer.

---

### Episode 03 — Finding and Accepting Jobs as a Freelancer

**Goal:** Freelancer browses open jobs, filters by amount/deadline, and accepts a job.

**Prerequisites:** A second testnet wallet (freelancer account) connected to StellarWork, and at least one open job posted by another account.

**Outline:**

1. **Switch to freelancer account** — show the "Switch Account" option in the wallet menu, select the freelancer wallet. (~1 min)
2. **Browse the job list** — scroll the home page, show the search bar and filters (amount range, deadline). (~1 min)
3. **Read a job** — click "View Details" on an open job, review the description, amount, and deadline. (~1 min)
4. **Accept the job** — click "Accept Job", approve Freighter signature, wait for confirmation. (~1 min)
5. **Check the dashboard** — navigate to Dashboard, show the job now appears under "Accepted Jobs" with "In Progress" status. (~1 min)

**Key callouts:**
- Explain that accepting locks the freelancer into the job.
- Show the deadline countdown and what "enforce deadline" means for the client.

---

### Episode 04 — Completing a Job and Getting Paid

**Goal:** Freelancer submits work, client approves, payment flows to freelancer minus platform fee.

**Prerequisites:** A job in "In Progress" status, both client and freelancer wallets available.

**Outline:**

1. **Submit work (freelancer)** — on the Dashboard, find the in-progress job, click "Submit Work", sign with Freighter. (~1 min)
2. **Status update** — show status changes to "Submitted for Review" on both client and freelancer dashboards. (~30 sec)
3. **Review and approve (client)** — switch to client account, go to Dashboard, find the job, click "Approve Work", sign with Freighter. (~1 min)
4. **Confirm payment** — navigate to the Transactions page, show the "Payment Received" entry for the freelancer and "Fee Deducted" breakdown. (~1 min)
5. **Check Stellar Explorer** — open the transaction hash link to confirm the on-chain transfer. (~1 min)

**Key callouts:**
- Show the 2.5% platform fee calculation.
- Explain that "Completed" is a terminal state — no further actions are possible.

---

### Episode 05 — Managing Disputes

**Goal:** Show how a dispute is raised, what the dispute state means, and how an admin resolves it.

**Prerequisites:** A job in "In Progress" state, admin wallet configured.

**Outline:**

1. **When to raise a dispute** — briefly explain the conditions (disagreement on work quality, missed deadlines, unresponsive counterparty). (~30 sec)
2. **Raise a dispute** — on the Disputes page or job detail, click "Raise Dispute", sign with Freighter. Show status changes to "Disputed". (~1 min)
3. **What happens next** — explain that funds remain locked in escrow; neither party can withdraw until the dispute is resolved. (~30 sec)
4. **Admin resolution** — switch to admin wallet, go to Admin → Disputes, select the job, enter a client/freelancer split (e.g., 50/50 = 5000 basis points), click "Resolve", sign. (~2 min)
5. **Verify outcome** — show both accounts' transaction histories and confirm the correct amounts landed in each wallet. (~1 min)

**Key callouts:**
- Clarify that only the platform admin can resolve disputes in the current contract version.
- Show that basis points (BPS) represent hundredths of a percent: 10 000 BPS = 100% to client.

---

## 6. Closed Captioning Guidelines

All official tutorial videos must include closed captions (CC) before publishing. This makes content accessible to Deaf and hard-of-hearing viewers, non-native speakers, and anyone watching in a sound-sensitive environment.

### Minimum Requirements

- Every spoken word must have a corresponding caption.
- Captions must be **time-synchronized** — on screen when the words are spoken, off screen when they are not.
- Maximum caption display duration: **7 seconds** per caption line.
- Maximum characters per line: **42 characters** (two lines of 42 each if needed).
- Minimum font size (if burned in): **22 px** at 1080p.

### Caption File Format

Export captions as **SRT** (`.srt`). This is accepted by YouTube, Vimeo, and most video platforms.

SRT format example:

```
1
00:00:04,200 --> 00:00:07,800
Hi, and welcome to StellarWork.

2
00:00:07,900 --> 00:00:11,500
In this video, we'll connect the Freighter
wallet to the platform.

3
00:00:11,600 --> 00:00:15,100
By the end, you'll be ready
to browse and accept open jobs.
```

Name caption files to match the video file:

```
SW-01-connecting-your-wallet-v1.srt
```

### Style Rules

- **Speaker identification**: not required for single-narrator videos; add `[NARRATOR]:` prefix only when multiple voices appear.
- **Non-speech sounds**: caption significant sounds in square brackets: `[keyboard typing]`, `[Freighter popup appears]`.
- **Technical terms**: spell out exactly as intended — `XLM`, `Freighter`, `Soroban`, `testnet`. Do not let auto-captions convert these to phonetic guesses without correcting them.
- **Contractions**: match the spoken audio — if the narrator says "you'll", caption it as "you'll", not "you will".
- **Numbers**: write as digits when referring to amounts (`50 XLM`, `2.5%`) and as words for counts under ten in prose (`three steps`).

### Auto-Caption Correction Workflow

YouTube auto-captions are a useful starting point but always require correction for blockchain/crypto vocabulary.

1. Upload video to YouTube Studio (unlisted).
2. Wait for auto-captions to generate (usually under 10 minutes).
3. Open Subtitles editor, review every line — pay special attention to: `XLM`, `Soroban`, `Freighter`, `escrow`, `testnet`, `mainnet`, `BPS`, `basis points`, wallet addresses.
4. Correct timing drift: captions should not appear more than 200 ms before the word is spoken.
5. Export corrected captions as SRT.
6. Re-upload the corrected SRT file and set it as the primary caption track.
7. Delete the auto-generated track.

### Accessibility Checklist

Before publishing any tutorial, confirm:

- [ ] SRT file attached and set as primary caption track
- [ ] Technical terms reviewed and corrected in captions
- [ ] No caption gap longer than 3 seconds of continuous speech
- [ ] Caption contrast ratio meets WCAG 2.1 AA (if burned-in text is used)
- [ ] Video has a text description in the YouTube/platform description field summarizing what is demonstrated
- [ ] Chapters/timestamps added to the video description for navigation

---

## 7. Publishing Checklist

Complete all items before making a tutorial video public.

### Pre-publish

- [ ] Script reviewed and approved by at least one other contributor
- [ ] Recording resolution is 1920×1080 or higher
- [ ] Audio peaks below −3 dBFS; no clipping; background noise removed
- [ ] Cursor highlights enabled and visible throughout
- [ ] Testnet used — no real funds visible or at risk
- [ ] No personal wallet addresses, private keys, or secrets visible in any frame
- [ ] Intro card includes episode number and title
- [ ] Outro card includes links to next episode and written docs

### Captions

- [ ] SRT file created and attached (see [Section 6](#6-closed-captioning-guidelines))
- [ ] Technical terms corrected in captions
- [ ] Timestamps verified against final video cut

### Metadata

- [ ] Video title format: `StellarWork Tutorial #0X — [Title]`
- [ ] Description includes: one-paragraph summary, prerequisites, timestamps/chapters, links to written docs, and the repository URL
- [ ] Thumbnail created at 1280×720 matching naming convention
- [ ] Tags: `stellar`, `soroban`, `blockchain`, `freelance`, `escrow`, `stellarwork`, `defi`, `tutorial`
- [ ] Video added to "StellarWork Tutorials" playlist

### Post-publish

- [ ] Update the `## Video Tutorials` section in the [root README](../README.md) with the video URL
- [ ] Update the Status column in the [series table above](#5-official-tutorial-series-outline) from `Planned` to `Published`
- [ ] Link from the relevant written doc (e.g., link Episode 01 from `docs/DEPLOY.md` or `CONTRIBUTING.md`)

---

*This guide follows the accessibility standards outlined in [WCAG 2.1](https://www.w3.org/TR/WCAG21/). Full validation requires manual testing with assistive technologies.*
