# RAMAYANA: The Retro Epic 🏹

A retro 16-bit 2D action RPG built with **Rust** and **Macroquad**, targeting desktop and WebAssembly (**WASM**).

Experience the timeless Indian epic through crisp retro pixel art, responsive keyboard & mouse archery mechanics, procedural 8-bit audio synthesis, and authentic mythological boss battles.

---

## 🎮 Playable Chapter: Level 1 — Tataka's Forest (*Bala Kanda*)

Prince Rama journeys with Sage Vishwamitra into the cursed forest of the Yakshini **Tataka**.

- **Phase 1 — The Invisible Hunt (*Shabdavedhi*):**
  Tataka conceals herself with dark sorcery. Watch for the expanding cyan sound ripples vibrating through the silence of the hermitage and strike her unseen center with precision archery (+20% Pin Bar per hit).
- **Phase 2 — The Final Rush:**
  Once pinned at 100%, Tataka reveals her towering demonic form (100 HP) and charges Rama with vicious claw swipes and ground-stomp shockwaves.
- **Sunset Countdown (90s):**
  The forest lighting shifts from golden twilight to blood-crimson dusk. If the 90-second countdown expires, dusk falls and Tataka enters **Frenzy** (2× speed, 2× damage, halved attack cooldowns).
- **Victory:**
  Defeat Tataka to receive Sage Vishwamitra's blessing and unlock the celestial Astras (*Vayavyastra*, *Manavastra*, *Agneyastra*) for Level 2 (*Siddhashrama*).

---

## 🕹️ Controls (Keyboard & Mouse)

| Action | Control |
|---|---|
| **Move** | `W`, `A`, `S`, `D` or Arrow Keys |
| **Aim** | Mouse Cursor (Custom Golden Crosshair) |
| **Shoot Arrow** | `Left Mouse Button` or `Spacebar` |
| **Select Astra** | `1` (Kodanda Bow), `2–4` (Unlocked in later levels) |
| **Advance Dialogue** | `Left Click`, `Spacebar`, or `Enter` |
| **Retry Level** | `R` key |

---

## 🚀 Getting Started

### 1. Run in Browser (WebAssembly)

Run the included web runner script:

```bash
./run_web.sh
```

Then open your browser at: **[http://localhost:8080](http://localhost:8080)**

To rebuild the release WASM bundle at any time:
```bash
./build_wasm.sh
```

### 2. Run Natively (Desktop)

Ensure you have Rust installed, then run:

```bash
cargo run
```

---

## 🛠️ Tech Stack & Architecture

- **Language / Engine:** Rust + [`macroquad`](https://macroquad.rs/)
- **Resolution:** Logical `640x360` (16:9 widescreen) with integer scaling and offscreen render target letterboxing.
- **Typography:** Authentic retro pixel font (*Press Start 2P*) embedded directly in binary.
- **Audio:** In-memory procedural WAV synthesis (bow twangs, ripple chimes, pin hits, boulder crashes, demonic roars).
- **Target Platforms:** Native Desktop (`Linux`, `macOS`, `Windows`) and Web (`wasm32-unknown-unknown`).

---

## 📜 Master Progression (14 Levels)

1. **L1 — Tataka's Forest (Bala Kanda):** *Shabdavedhi* invisible hunt & Final Rush.
2. **L2 — Siddhashrama (Bala Kanda):** Protect the sacred Yajna; match Astras against Maricha & Subahu.
3. **L3 — Mithila (Bala Kanda):** Swayamvara bow-lifting test of strength.
4. **L4 — Ganga Crossing (Ayodhya Kanda):** Boat navigation gauntlet with Guha.
5. **L5 — Chitrakoot (Ayodhya Kanda):** Gathering resources and building the Parnakuti hermitage.
6. **L6 — Panchavati (Aranya Kanda):** Defeating Khara & Dushana; chasing the Golden Deer.
7. **L7 — Kishkindha (Kishkindha Kanda):** Alliance with Hanuman & Sugriva; duel with Vali.
8. **L8 — The Ocean Leap (Sundara Kanda):** Aerial side-scroll gauntlet to Lanka.
9. **L9 — Ashoka Vatika & Lanka Dahan (Sundara Kanda):** Finding Sita and setting Lanka ablaze.
10. **L10 — Rama Setu (Yuddha Kanda):** Engineering the floating stone bridge.
11. **L11 — Kumbhakarna Siege (Yuddha Kanda):** Multi-hero giant boss battle.
12. **L12 — Sanjeevani Quest (Yuddha Kanda):** Timed mountain flight before sunrise.
13. **L13 — Nikumbhila & Sky Duel (Yuddha Kanda):** Lakshmana vs. Indrajit's illusions.
14. **L14 — The Battle of Lanka (Yuddha Kanda):** Epic final chariot duel vs. Ravana.
