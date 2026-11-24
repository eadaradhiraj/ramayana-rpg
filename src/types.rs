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
    L5GangaBoat,
    L6ChitrakootHut,
    L7Panchavati,
    L8Kishkindha,
    L9OceanLeap,
    L10LankaDahan,
    L11RamaSetu,
    L12Kumbhakarna,
    L13Sanjeevani,
    L14Indrajit,
    L15Ravana,
}

#[allow(dead_code)]
impl LevelId {
    pub fn title(&self) -> &'static str {
        match self {
            LevelId::L1Tataka => "Bala Kanda: Tataka's Forest",
            LevelId::L2Siddhashrama => "Bala Kanda: Siddhashrama",
            LevelId::L3Mithila => "Bala Kanda: Mithila (Swayamvara)",
            LevelId::L4Ayodhya => "Ayodhya Kanda: Eve of Coronation",
            LevelId::L5GangaBoat => "Ayodhya Kanda: Ganga Crossing",
            LevelId::L6ChitrakootHut => "Ayodhya Kanda: Chitrakoot",
            LevelId::L7Panchavati => "Aranya Kanda: Panchavati",
            LevelId::L8Kishkindha => "Kishkindha Kanda: Kishkindha",
            LevelId::L9OceanLeap => "Sundara Kanda: The Ocean Leap",
            LevelId::L10LankaDahan => "Sundara Kanda: Ashoka Vatika",
            LevelId::L11RamaSetu => "Yuddha Kanda: Rama Setu",
            LevelId::L12Kumbhakarna => "Yuddha Kanda: Kumbhakarna",
            LevelId::L13Sanjeevani => "Yuddha Kanda: Sanjeevani Quest",
            LevelId::L14Indrajit => "Yuddha Kanda: Nikumbhila & Sky",
            LevelId::L15Ravana => "Yuddha Kanda: Battle of Lanka",
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
    Brahmastra,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AstraProjectileType {
    Kodanda,
    Vayavya,
    Manava,
    Agneya,
}

#[allow(dead_code)]
impl Astra {
    pub fn name(&self) -> &'static str {
        match self {
            Astra::KodandaBow => "Kodanda Bow",
            Astra::Vayavyastra => "Vayavyastra",
            Astra::Manavastra => "Manavastra",
            Astra::Agneyastra => "Agneyastra",
            Astra::Brahmastra => "Brahmastra",
        }
    }

    pub fn key_slot(&self) -> &'static str {
        match self {
            Astra::KodandaBow => "[1]",
            Astra::Vayavyastra => "[2]",
            Astra::Manavastra => "[3]",
            Astra::Agneyastra => "[4]",
            Astra::Brahmastra => "[5]",
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
