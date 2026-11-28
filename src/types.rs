#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hero {
    Rama,
    Lakshmana,
    Hanuman,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LevelId {
    L1Tataka,
    L2Siddhashrama,
    L3Mithila,
    L4Ayodhya,
    L5Chitrakuta,
    L6Viradha,
    L7Panchavati,
    L8MarichaJatayu,
    L9Kishkindha,
    L10OceanLeap,
    L11LankaDahan,
    L12RamaSetu,
    L13Kumbhakarna,
    L14Sanjeevani,
    L15Indrajit,
    L16Ravana,
}

#[allow(dead_code)]
impl LevelId {
    pub fn title(&self) -> &'static str {
        match self {
            LevelId::L1Tataka => "Bala Kanda: Tataka's Forest",
            LevelId::L2Siddhashrama => "Bala Kanda: Siddhashrama",
            LevelId::L3Mithila => "Bala Kanda: Mithila (Swayamvara)",
            LevelId::L4Ayodhya => "Ayodhya Kanda: Eve of Coronation",
            LevelId::L5Chitrakuta => "Ayodhya Kanda: Chitrakuta (Boat & Hermitage)",
            LevelId::L6Viradha => "Aranya Kanda: Dandakaranya (Ambush of Viradha)",
            LevelId::L7Panchavati => "Aranya Kanda: Panchavati (Khara & Dushana)",
            LevelId::L8MarichaJatayu => "Aranya Kanda: Golden Deer & Jatayu",
            LevelId::L9Kishkindha => "Kishkindha Kanda: Hanuman, Sugriva & Vali",
            LevelId::L10OceanLeap => "Sundara Kanda: The Ocean Leap",
            LevelId::L11LankaDahan => "Sundara Kanda: Ashoka Vatika",
            LevelId::L12RamaSetu => "Yuddha Kanda: Rama Setu",
            LevelId::L13Kumbhakarna => "Yuddha Kanda: Kumbhakarna",
            LevelId::L14Sanjeevani => "Yuddha Kanda: Sanjeevani Quest",
            LevelId::L15Indrajit => "Yuddha Kanda: Nikumbhila & Sky",
            LevelId::L16Ravana => "Yuddha Kanda: Battle of Lanka",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Astra {
    KodandaBow,
    Vayavyastra,
    Manavastra,
    Agneyastra,
    Gandharvastra,
    Brahmastra,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AstraProjectileType {
    Kodanda,
    Vayavya,
    Manava,
    Agneya,
    Gandharva,
}

#[allow(dead_code)]
impl Astra {
    pub fn name(&self) -> &'static str {
        match self {
            Astra::KodandaBow => "Kodanda Bow",
            Astra::Vayavyastra => "Vayavyastra",
            Astra::Manavastra => "Manavastra",
            Astra::Agneyastra => "Agneyastra",
            Astra::Gandharvastra => "Gandharvastra",
            Astra::Brahmastra => "Brahmastra",
        }
    }

    pub fn key_slot(&self) -> &'static str {
        match self {
            Astra::KodandaBow => "[1]",
            Astra::Vayavyastra => "[2]",
            Astra::Manavastra => "[3]",
            Astra::Agneyastra => "[4]",
            Astra::Gandharvastra => "[5]",
            Astra::Brahmastra => "[6]",
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PlayerStats {
    pub prana: i32,      // HP
    pub max_prana: i32,
    pub tejas: i32,      // MP / Astra energy
    pub max_tejas: i32,
    pub unlocked_astras: Vec<Astra>,
    pub selected_astra: Astra,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            prana: 100,
            max_prana: 100,
            tejas: 100,
            max_tejas: 100,
            unlocked_astras: vec![Astra::KodandaBow],
            selected_astra: Astra::KodandaBow,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CutsceneType {
    RamaBirth,
    SitaBirth,
    SitaKalyanam,
    ParashuramaEncounter,
    AyodhyaExile,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DialogueTarget {
    StartFight(LevelId),
    Victory(LevelId),
    StartCutscene(CutsceneType),
}

#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    TitleScreen,
    LevelSelect {
        selected: usize,
    },
    Paused {
        level_id: LevelId,
        selected: usize,
    },
    Dialogue {
        speaker: String,
        lines: Vec<String>,
        idx: usize,
        target: DialogueTarget,
    },
    Cutscene {
        cutscene_type: CutsceneType,
        slide_idx: usize,
    },
    Playing(LevelId),
    LevelComplete(LevelId),
    GameOver(LevelId),
}
