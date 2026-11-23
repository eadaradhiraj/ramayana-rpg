Save this as **`SPEC.md`** in your project root:

```markdown
# SPEC.md — RAMAYANA: THE RETRO EPIC (2D Rust + WASM)

## 1. TECH STACK & CONVENTIONS
- **Language / Engine:** Rust + `macroquad` (target: `wasm32-unknown-unknown`)
- **Resolution:** Logical `640x480` viewport (`20x15` grid of `32x32` tiles)
- **Visual Style:** 8-bit/16-bit retro pixel art (geometric placeholders first, `.png` spritesheets later)
- **Controls:** `WASD`/Arrows (Move) | `Mouse` or `Space` (Aim/Shoot/Interact) | `1-4` (Select Astra/Action)

---

## 2. CORE RUST ARCHITECTURE
```rust
pub enum Hero { Rama, Lakshmana, Hanuman }

pub enum LevelId {
    L1Tataka, L2Siddhashrama, L3Mithila,
    L4GangaBoat, L5ChitrakootHut, L6Panchavati,
    L7Kishkindha, L8OceanLeap, L9LankaDahan,
    L10RamaSetu, L11Kumbhakarna, L12Sanjeevani,
    L13Indrajit, L14Ravana,
}

pub enum GameState {
    TitleScreen,
    Dialogue { speaker: String, lines: Vec<String>, idx: usize },
    Playing(LevelId),
    LevelComplete(LevelId),
    GameOver(LevelId),
}

pub struct PlayerStats {
    pub prana: i32,      // HP
    pub max_prana: i32,
    pub tejas: i32,      // MP / Astra energy
    pub max_tejas: i32,
    pub unlocked_astras: Vec<Astra>,
}

pub enum Astra {
    KodandaBow, Vayavyastra, Manavastra, Agneyastra, Brahmastra
}
```

---

## 3. MASTER LEVEL PROGRESSION (14 LEVELS)

| Lvl | Kanda | Location / Title | Hero | Core Mechanic & Goal |
|-----|-------|------------------|------|----------------------|
| **L1** | Bala | Tataka's Forest | Rama | *Shabdavedhi* (shoot invisible sound ripples) → Final Charge |
| **L2** | Bala | Siddhashrama | Rama | Guard Yajna altar; match Astras (*Vayavya* horde, *Manava* Maricha, *Agni* Subahu) |
| **L3** | Bala | Mithila (Swayamvara) | Rama | Timing/power bar mini-game to lift & string Shiva's bow |
| **L4** | Ayodhya | Ganga Crossing | Rama | Boat ferry mini-game with Guha; dodge currents & rocks |
| **L5** | Ayodhya | Chitrakoot | Rama/Lakshmana | Gather wood/leaves & build the *Parnakuti* hut |
| **L6** | Aranya | Panchavati | Rama | Defeat Khara-Dushana → chase Golden Deer → Jatayu scene |
| **L7** | Kishkindha | Kishkindha | Rama | Ally with Hanuman & Sugriva → Boss duel vs. Vali |
| **L8** | Sundara | The Ocean Leap | Hanuman | Side-scroll gauntlet: Mainaka rest, Surasa size puzzle, Simhika, Lankini |
| **L9** | Sundara | Ashoka Vatika | Hanuman | Stealth to Sita → beat Akshaya Kumara → escape *Lanka Dahan* |
| **L10** | Yuddha | Rama Setu | Rama/Vanaras | Bridge-building mini-game: place floating inscribed stones |
| **L11** | Yuddha | Siege & Kumbhakarna | Rama/Vanaras | Multi-character boss battle against the waking giant |
| **L12** | Yuddha | Sanjeevani Quest | Hanuman | Timed flight obstacle course; fetch mountain before sunrise |
| **L13** | Yuddha | Nikumbhila & Sky | Lakshmana | Disrupt Indrajit's Yajna → defeat shadow/illusion boss |
| **L14** | Yuddha | Battle of Lanka | Rama | Chariot boss vs. Ravana; cut heads → *Aditya Hridayam* → *Brahmastra* |

---

## 4. DETAILED BUILD SPEC: LEVEL 1 (TATAKA'S FOREST)
*Immediate vertical slice target before expanding to L2–L14.*

- **Arena:** `640x480` forest clearing with tree borders.
- **Sunset Timer:** `90.0s` countdown. If dusk hits (`0.0s`), Tataka enters Frenzy (2x speed & damage).
- **Phase 1 — Invisible Hunt (*Shabdavedhi*):**
  - Tataka moves invisibly (`visible = false`), changing direction periodically.
  - Every `1.2s`, she emits a **Sound Ripple** (expanding circle at her position, fading over `1.0s`) and occasionally throws a boulder projectile.
  - Arrows hitting within `28px` of her invisible center add `+20%` to the **Pin Bar** (`0%` → `100%`).
- **Phase 2 — Final Rush:**
  - At `100%` Pin Bar, Tataka turns visible (`100 HP` final bar) and charges directly toward Rama.
  - Rama dodges and shoots arrows (`20 dmg` each) until `HP == 0`.
- **Win Condition:** Tataka defeated → Dialogue (*Vishwamitra bestows Astras*) → Unlock **L2 (Siddhashrama)**.
```

---

Say **"build Level 1"** whenever you're ready, and I'll generate the complete `src/main.rs` for Tataka's boss fight! (Also let me know: **Mouse-aim** or **Keyboard-only**?)
