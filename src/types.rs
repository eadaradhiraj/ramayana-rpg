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
    L4GangaBoat,
    L5ChitrakootHut,
    L6Panchavati,
    L7Kishkindha,
    L8OceanLeap,
    L9LankaDahan,
    L10RamaSetu,
    L11Kumbhakarna,
    L12Sanjeevani,
    L13Indrajit,
    L14Ravana,
}

#[allow(dead_code)]
impl LevelId {
    pub fn title(&self) -> &'static str {
        match self {
            LevelId::L1Tataka => "Bala Kanda: Tataka's Forest",
            LevelId::L2Siddhashrama => "Bala Kanda: Siddhashrama",
            LevelId::L3Mithila => "Bala Kanda: Mithila (Swayamvara)",
            LevelId::L4GangaBoat => "Ayodhya Kanda: Ganga Crossing",
            LevelId::L5ChitrakootHut => "Ayodhya Kanda: Chitrakoot",
            LevelId::L6Panchavati => "Aranya Kanda: Panchavati",
            LevelId::L7Kishkindha => "Kishkindha Kanda: Kishkindha",
            LevelId::L8OceanLeap => "Sundara Kanda: The Ocean Leap",
            LevelId::L9LankaDahan => "Sundara Kanda: Ashoka Vatika",
            LevelId::L10RamaSetu => "Yuddha Kanda: Rama Setu",
            LevelId::L11Kumbhakarna => "Yuddha Kanda: Kumbhakarna",
            LevelId::L12Sanjeevani => "Yuddha Kanda: Sanjeevani Quest",
            LevelId::L13Indrajit => "Yuddha Kanda: Nikumbhila & Sky",
            LevelId::L14Ravana => "Yuddha Kanda: Battle of Lanka",
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
