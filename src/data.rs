#[cfg(not(target_arch = "wasm32"))]
compile_error!{ "This file is only meant to be used in a WASM context" }

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub(crate) struct FormState {
    pub(crate) budget: u16,
    pub(crate) unit_type: UnitType,
    pub(crate) refit_type: RefitType,
    pub(crate) siege: Siege,
    pub(crate) heavy_siege: HeavySiege,
    pub(crate) pack: Pack,
    pub(crate) potion: Potion,
    pub(crate) vessel: Vessel,
    pub(crate) weapons: [Weapon; 5],
    pub(crate) weapon_upgrades: [WeaponUpgrade; 5],
    pub(crate) spells: [Spell; 5],
    pub(crate) structures: [Structure; 5],
    pub(crate) vessel_upgrades: [VesselUpgrade; 5],
}

impl FormState {
    fn count_other_weapons(&self, slot: usize, weapon: Weapon) -> usize {
        self.weapons.iter().enumerate().filter(|(i, w)| *i != slot && **w == weapon).count()
    }
    fn count_other_weapon_upgrades(&self, slot: usize, weapon_upgrade: WeaponUpgrade) -> usize {
        self.weapon_upgrades.iter().enumerate().filter(|(i, w)| *i != slot && **w == weapon_upgrade).count()
    }
    fn count_other_spells(&self, slot: usize, spell: Spell) -> usize {
        self.spells.iter().enumerate().filter(|(i, s)| *i != slot && **s == spell).count()
    }
    fn count_other_structures(&self, slot: usize, structure: Structure) -> usize {
        self.structures.iter().enumerate().filter(|(i, s)| *i != slot && **s == structure).count()
    }
    fn count_other_vessel_upgrades(&self, slot: usize, vessel_upgrade: VesselUpgrade) -> usize {
        self.vessel_upgrades.iter().enumerate().filter(|(i, v)| *i != slot && **v == vessel_upgrade).count()
    }
    pub fn costs(&self) -> Vec<i32> {
        let FormState { budget: _, unit_type: _, refit_type: _, siege, heavy_siege, pack, potion, vessel, weapons, weapon_upgrades, spells, structures, vessel_upgrades} = self;
        let mut costs: Vec<i32> = Vec::with_capacity(33);
        costs.push(siege.cost().into());
        costs.push(heavy_siege.cost().into());
        costs.push(pack.cost().into());
        costs.push(potion.cost().into());
        costs.push(vessel.cost().into());
        costs.extend(weapons.iter().map(|w| w.cost() as i32));
        costs.extend(weapon_upgrades.iter().map(|w| w.cost() as i32));
        costs.extend(spells.iter().map(|s| s.cost() as i32));
        costs.extend(structures.iter().map(|s| s.cost() as i32));
        costs.extend(vessel_upgrades.iter().map(|v| v.cost() as i32));
        costs.retain(|c| *c > 0);
        costs
    }
}

impl From<&[u8]> for FormState {
    fn from(value: &[u8]) -> Self {
        match value {
            &[1, budget_l, budget_h, unit_type, refit_type, siege, heavy_siege, pack, potion, vessel, weapon_1, weapon_2, weapon_3, weapon_4, weapon_5, weapon_upgrade_1, weapon_upgrade_2, weapon_upgrade_3, weapon_upgrade_4, weapon_upgrade_5, spell_1, spell_2, spell_3, spell_4, spell_5, structure_1, structure_2, structure_3, structure_4, structure_5, vessel_upgrade_1, vessel_upgrade_2, vessel_upgrade_3, vessel_upgrade_4, vessel_upgrade_5]
             => Self { budget: u16::from_le_bytes([budget_l, budget_h]), unit_type: unit_type.into(), refit_type: refit_type.into(), siege: siege.into(), heavy_siege: heavy_siege.into(), pack: pack.into(), potion: potion.into(), vessel: vessel.into(), weapons: [weapon_1.into(), weapon_2.into(), weapon_3.into(), weapon_4.into(), weapon_5.into()], weapon_upgrades: [weapon_upgrade_1.into(), weapon_upgrade_2.into(), weapon_upgrade_3.into(), weapon_upgrade_4.into(), weapon_upgrade_5.into()], spells: [spell_1.into(), spell_2.into(), spell_3.into(), spell_4.into(), spell_5.into()], structures: [structure_1.into(), structure_2.into(), structure_3.into(), structure_4.into(), structure_5.into()], vessel_upgrades: [vessel_upgrade_1.into(), vessel_upgrade_2.into(), vessel_upgrade_3.into(), vessel_upgrade_4.into(), vessel_upgrade_5.into()] },
             _ => Self::default(),
        }
    }
}

impl From<FormState> for [u8; 35] {
    fn from(value: FormState) -> Self {
        let mut data = [0u8; 35];
        data[0] = 1;
        let buf: [u8; 34] = unsafe { std::mem::transmute(value) };
        data[1..].copy_from_slice(&buf);
        data
    }
}

impl TryFrom<web_sys::FormData> for FormState {
    type Error = &'static str;
    fn try_from(value: web_sys::FormData) -> Result<Self, Self::Error> {
        let budget = value.get("budget").as_string().unwrap().parse::<u16>().map_err(|_| "gold_budget")?;
        let unit_type: UnitType = value.get("unit_type").as_string().unwrap().parse::<u8>().map_err(|_| "unit_type")?.into();
        let refit_type: RefitType = value.get("refit_type").as_string().unwrap().parse::<u8>().map_err(|_| "refit_type")?.into();
        let siege: Siege = value.get("siege").as_string().unwrap().parse::<u8>().map_err(|_| "siege")?.into();
        let heavy_siege: HeavySiege = value.get("heavy_siege").as_string().unwrap().parse::<u8>().map_err(|_| "heavy_siege")?.into();
        let pack: Pack = value.get("pack").as_string().unwrap().parse::<u8>().map_err(|_| "pack")?.into();
        let potion: Potion = value.get("potion").as_string().unwrap().parse::<u8>().map_err(|_| "potion")?.into();
        let vessel: Vessel = value.get("vessel").as_string().unwrap().parse::<u8>().map_err(|_| "vessel")?.into();
        let weapon_1: Weapon = value.get("weapon_1").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_1")?.into();
        let weapon_2: Weapon = value.get("weapon_2").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_2")?.into();
        let weapon_3: Weapon = value.get("weapon_3").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_3")?.into();
        let weapon_4: Weapon = value.get("weapon_4").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_4")?.into();
        let weapon_5: Weapon = value.get("weapon_5").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_5")?.into();
        let weapon_upgrade_1: WeaponUpgrade = value.get("weapon_1_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_1_upgrade")?.into();
        let weapon_upgrade_2: WeaponUpgrade = value.get("weapon_2_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_2_upgrade")?.into();
        let weapon_upgrade_3: WeaponUpgrade = value.get("weapon_3_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_3_upgrade")?.into();
        let weapon_upgrade_4: WeaponUpgrade = value.get("weapon_4_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_4_upgrade")?.into();
        let weapon_upgrade_5: WeaponUpgrade = value.get("weapon_5_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "weapon_5_upgrade")?.into();
        let spell_1: Spell = value.get("spell_1").as_string().unwrap().parse::<u8>().map_err(|_| "spell_1")?.into();
        let spell_2: Spell = value.get("spell_2").as_string().unwrap().parse::<u8>().map_err(|_| "spell_2")?.into();
        let spell_3: Spell = value.get("spell_3").as_string().unwrap().parse::<u8>().map_err(|_| "spell_3")?.into();
        let spell_4: Spell = value.get("spell_4").as_string().unwrap().parse::<u8>().map_err(|_| "spell_4")?.into();
        let spell_5: Spell = value.get("spell_5").as_string().unwrap().parse::<u8>().map_err(|_| "spell_5")?.into();
        let structure_1: Structure = value.get("structure_1").as_string().unwrap().parse::<u8>().map_err(|_| "structure_1")?.into();
        let structure_2: Structure = value.get("structure_2").as_string().unwrap().parse::<u8>().map_err(|_| "structure_2")?.into();
        let structure_3: Structure = value.get("structure_3").as_string().unwrap().parse::<u8>().map_err(|_| "structure_3")?.into();
        let structure_4: Structure = value.get("structure_4").as_string().unwrap().parse::<u8>().map_err(|_| "structure_4")?.into();
        let structure_5: Structure = value.get("structure_5").as_string().unwrap().parse::<u8>().map_err(|_| "structure_5")?.into();
        let vessel_upgrade_1: VesselUpgrade = value.get("vessel_1_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "vessel_1_upgrade")?.into();
        let vessel_upgrade_2: VesselUpgrade = value.get("vessel_2_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "vessel_2_upgrade")?.into();
        let vessel_upgrade_3: VesselUpgrade = value.get("vessel_3_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "vessel_3_upgrade")?.into();
        let vessel_upgrade_4: VesselUpgrade = value.get("vessel_4_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "vessel_4_upgrade")?.into();
        let vessel_upgrade_5: VesselUpgrade = value.get("vessel_5_upgrade").as_string().unwrap().parse::<u8>().map_err(|_| "vessel_5_upgrade")?.into();
        Ok(Self { budget, unit_type, refit_type, siege, heavy_siege, pack, potion, vessel, weapons: [weapon_1, weapon_2, weapon_3, weapon_4, weapon_5], weapon_upgrades: [weapon_upgrade_1, weapon_upgrade_2, weapon_upgrade_3, weapon_upgrade_4, weapon_upgrade_5], spells: [spell_1, spell_2, spell_3, spell_4, spell_5], structures: [structure_1, structure_2, structure_3, structure_4, structure_5], vessel_upgrades: [vessel_upgrade_1, vessel_upgrade_2, vessel_upgrade_3, vessel_upgrade_4, vessel_upgrade_5] })
    }
}

pub(crate) trait ValidFor: Sized {
    fn valid_for(form_state: &FormState) -> Vec<Self>;
}

pub(crate) trait ValidForArray: Sized {
    fn base_for(form_state: &FormState) -> Vec<Self>;
    fn valid_for(form_state: &FormState) -> [Vec<Self>; 5];
}

pub(crate) trait Field: ValidFor + Asset + Default + Copy + Eq + Debug + From<u8> + Into<u8> + Into<web_sys::HtmlOptionElement> + 'static {
    const ALL: &'static [Self];
    fn cost(&self) -> u8;
    fn resolve_or_default(form_state: &FormState, valid: &[Self]) -> Self;
}

pub(crate) trait FieldArray: ValidForArray + Asset + Default + Copy + Eq + Debug + From<u8> + Into<u8> + Into<web_sys::HtmlOptionElement> + 'static {
    const ALL: &'static [Self];
    fn cost(&self) -> u8;
    fn limit(&self) -> usize;
    fn resolve_or_default_slot(form_state: &FormState, valid: Vec<Self>, slot: usize) -> Self;
}

pub(crate) trait Named {
    fn name(&self) -> &str;
}

pub(crate) trait Asset: Named {
    fn asset(&self) -> String {
        format!("/assets/{}.png", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum UnitType {
    #[default]
    None = 0,
    Soldiers = 1,
    Healers = 2,
    Laborers = 3,
    SiegeCrew = 4,
    Mages = 5,
    SupplyCart = 6,
    LineBreaker = 7,
    SkyBeast = 8,
    Monstrosity = 9,
    Knights = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum RefitType {
    #[default]
    None = 0,
    Champion = 1,
    Infiltrator = 2,
    Archer = 3,
    Artisan = 4,
    WarWagon = 5,
    HeavySiege = 6,
    BattleMage = 7,
    Archmage = 8,
    Caravan = 9,
    Cavalier = 10,
    SkyKnight = 11,
    SiegeBeast = 12,
    SkyChariot = 13,
    Prowler = 14,
    Brute = 15,
    Dragon = 16,
    Architect = 17,
    Trailblazer = 18,
    Surgeon = 19,
    MountedArcher = 20,
    Firehawk = 21,
    PotionMaster = 22,
    Wrangler = 23,
    Sapper = 24,
    ChaosMage = 25,
    GroundChariot = 26,
    RoughRider = 27,
    Paladin = 28,
    LightCav = 29,
    FlatbedCart = 30
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum HeavySiege {
    #[default]
    None = 0,
    Trebuchet = 1,
    Scorpion = 2,
    DragonfireCannon = 3,
    Mangonel = 4,
    GuidedBolt = 5,
    BurningOil = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Pack {
    #[default]
    None = 0,
    Ghost = 1,
    Barding = 2,
    MagicalBarrier = 3,
    BagOfHolding = 4,
    CloakOfTheChameleon = 5,
    GreavesOfTheFox = 6,
    CapeOfTheBat = 7,
    GlovesOfTheTiger = 8,
    ArrowSlits = 9,
    CarriersSaddle = 10,
    ScoutCavalry = 11,
    Veterinarian = 12,
    CaptainsTricorn = 13,
    HeavyArmor = 14,
    Construction = 15,
    HonorGuard = 16,
    MageInitiate = 17,
    FinsOfTheUnagi = 18,
    Vitalist = 19,
    Carpenter = 20,
    MimicCart = 21,
    CloudStrider = 22,
    Sailor = 23,
    BerserkersHelp = 24,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Potion {
    #[default]
    None = 0,
    Healing = 1,
    Mana = 2,
    Enlarging = 3,
    Haste = 4,
    Invisibility = 5,
    Levitation = 6,
    Sharpness = 7,
    Stoneskin = 8,
    Strength = 9,
    Warp = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Siege {
    #[default]
    None = 0,
    Catapult = 1,
    BoltThrower = 2,
    FlameSpitter = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Spell {
    #[default]
    None = 0,
    ArcaneStorm = 1,
    MassBarrier = 2,
    MassHeal = 3,
    Teleport = 4,
    CompleteStructures = 5,
    Divination = 6,
    Resurrection = 7,
    TruePolymorph = 8,
    CreateUndeadHorde = 9,
    MirrorDome = 10,
    SummonUndeadBeast = 11,
    ArcaneBlast = 12,
    Fabricate = 13,
    Barrier = 14,
    CureWounds = 15,
    ScryingEye = 16,
    Haste = 17,
    InscribeCircle = 18,
    ConjurePallisade = 19,
    Fly = 20,
    Invisibility = 21,
    Polymorph = 22,
    Drain = 23,
    MagicMirror = 24,
    SummonZombies = 25,
    ManaConduit = 26,
    Elementalism = 27,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Structure {
    #[default]
    None = 0,
    Stable = 1,
    WatchTower = 2,
    Shrine = 3,
    ExoticStable = 4,
    Hospital = 5,
    Fortress = 6,
    Castle = 7,
    Temple = 8,
    Smith = 9,
    Mill = 10,
    Apothecary = 11,
    Channel = 12,
    Keep = 13,
    Dock = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Vessel {
    #[default]
    None = 0,
    Knarr = 1,
    Caravel = 2,
    Trireme = 3,
    Karve = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum VesselUpgrade {
    #[default]
    None = 0,
    LateenSails = 1,
    RowerBench = 2,
    BoardingSpurs = 3,
    Incendiaries = 4,
    Armory = 5,
    Bulwarks = 6,
    HeavyPlanks = 7,
    Specialization = 8,
    DedicatedVessel = 9,
    CargoHold = 10,
    LaunchPlatform = 11,
    SternRudder = 12,
    SkeletonCrew = 13,
    GalleonC = 14,
    GalleonT = 15,
    GalleonK = 16,
    Corvus = 17,
    Anchor = 18,
    PotionDiffusion = 19,
    CrowsNest = 20,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum Weapon {
    #[default]
    None = 0,
    Spear = 1,
    Sword = 2,
    Hammer = 3,
    Bow = 4,
    Shield = 5,
    Lance = 6,
    Ram = 7,
    Axe = 8,
    Crossbow = 9,
    EaglesTalons = 10,
    OozesSlime = 11,
    HedgehogsSpikes = 12,
    StagsAntlers = 13,
    ScorpionsStinger = 14,
    UnicornsHorn = 15,
    CheetahsSwipe = 16,
    KangaroosKick = 17,
    BatsFangs = 18,
    ManticoresSpines = 19,
    MyrmexsAcid = 20,
    SpidersWeb = 21,
    EaglesEyes = 22,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub(crate) enum WeaponUpgrade {
    #[default]
    None = 0,
    Longspear = 1,
    Javelin = 2,
    Quickblade = 3,
    Greatsword = 4,
    Maul = 5,
    BodkinAmmoB = 6,
    FlamingAmmoB = 7,
    SeekingAmmoB = 8,
    TowerShield = 9,
    LightLance = 10,
    PoleAxe = 11,
    BodkinAmmoC = 12,
    FlamingAmmoC = 13,
    SeekingAmmoC = 14,
    Genitour = 15,
    Longbow = 16,
    HandCrossbow = 17,
    HeavyCrossbow = 18,
    SpikedShield = 19,
    RecurveBow = 20,
    SpikedRams = 21,
    Bolas = 22,
    Glaive = 23,
}

// UnitType

impl Field for UnitType {
    const ALL: &'static [Self] = &[UnitType::None, UnitType::Soldiers, UnitType::Mages, UnitType::Healers, UnitType::Laborers, UnitType::SiegeCrew, UnitType::SupplyCart, UnitType::LineBreaker, UnitType::SkyBeast, UnitType::Monstrosity, UnitType::Knights];
    fn cost(&self) -> u8 {
        match self {
            UnitType::None => 0,
            _ => 1 // a special case, all units cost 1 base gold
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[Self]) -> Self {
        if valid.contains(&form_state.unit_type) { form_state.unit_type } else { Self::default() }
    }
}

impl Named for UnitType {
    fn name(&self) -> &str {
        match self {
            UnitType::None => "NONE",
            UnitType::Soldiers => "SOLDIERS",
            UnitType::Mages => "MAGES",
            UnitType::Healers => "HEALERS",
            UnitType::Laborers => "LABORERS",
            UnitType::SiegeCrew => "SIEGE_CREW",
            UnitType::SupplyCart => "SUPPLY_CART",
            UnitType::LineBreaker => "LINE_BREAKER",
            UnitType::SkyBeast => "SKY_BEAST",
            UnitType::Monstrosity => "MONSTROSITY",
            UnitType::Knights => "KNIGHTS",
        }
    }
}

impl Asset for UnitType {}

impl ValidFor for UnitType {
    fn valid_for(_form_state: &FormState) -> Vec<Self> {
        Self::ALL.to_vec()
    }
}

impl UnitType {
    pub(crate) fn unit_gold(&self) -> u8 {
        match self {
            UnitType::None => 0,
            UnitType::Soldiers => 1,
            UnitType::Healers => 0,
            UnitType::Laborers => 1,
            UnitType::SiegeCrew => 2,
            UnitType::Mages => 1,
            UnitType::SupplyCart => 0,
            UnitType::LineBreaker => 0,
            UnitType::SkyBeast => 0,
            UnitType::Monstrosity => 0,
            UnitType::Knights => 1,
        }
    }
}

impl From<UnitType> for u8 {
    fn from(value: UnitType) -> Self {
        value as u8
    }
}

impl From<u8> for UnitType {
    fn from(value: u8) -> Self {
        match value {
            0 => UnitType::None,
            1 => UnitType::Soldiers,
            2 => UnitType::Healers,
            3 => UnitType::Laborers,
            4 => UnitType::SiegeCrew,
            5 => UnitType::Mages,
            6 => UnitType::SupplyCart,
            7 => UnitType::LineBreaker,
            8 => UnitType::SkyBeast,
            9 => UnitType::Monstrosity,
            10 => UnitType::Knights,
            _ => UnitType::default(),
        }
    }
}

impl From<UnitType> for web_sys::HtmlOptionElement {
    fn from(value: UnitType) -> Self { 
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// RefitType

impl Field for RefitType {
    const ALL: &'static [Self] = &[RefitType::None, RefitType::Champion, RefitType::Infiltrator, RefitType::Archer, RefitType::Artisan, RefitType::WarWagon, RefitType::HeavySiege, RefitType::BattleMage, RefitType::Archmage, RefitType::Caravan, RefitType::Cavalier, RefitType::SkyKnight, RefitType::SiegeBeast, RefitType::SkyChariot, RefitType::Prowler, RefitType::Brute, RefitType::Dragon, RefitType::Architect, RefitType::Trailblazer, RefitType::Surgeon, RefitType::MountedArcher, RefitType::Firehawk, RefitType::PotionMaster, RefitType::Wrangler, RefitType::Sapper, RefitType::ChaosMage, RefitType::GroundChariot, RefitType::RoughRider, RefitType::Paladin, RefitType::LightCav, RefitType::FlatbedCart];
    fn cost(&self) -> u8 {
        match self {
            RefitType::None => 0,
            _ => 3,
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[RefitType]) -> RefitType {
        if valid.contains(&form_state.refit_type) { form_state.refit_type } else { RefitType::default() }
    }
}

impl Named for RefitType {
    fn name(&self) -> &str {
        match self {
            RefitType::None => "NONE",
            RefitType::Champion => "CHAMPION",
            RefitType::Infiltrator => "INFILTRATOR",
            RefitType::Archer => "ARCHER",
            RefitType::Artisan => "ARTISAN",
            RefitType::WarWagon => "WAR_WAGON",
            RefitType::HeavySiege => "HEAVY_SIEGE",
            RefitType::BattleMage => "BATTLE_MAGE",
            RefitType::Archmage => "ARCHMAGE",
            RefitType::Caravan => "CARAVAN",
            RefitType::Cavalier => "CAVALIER",
            RefitType::SkyKnight => "SKY_KNIGHT",
            RefitType::SiegeBeast => "SIEGE_BEAST",
            RefitType::SkyChariot => "SKY_CHARIOT",
            RefitType::Prowler => "PROWLER",
            RefitType::Brute => "BRUTE",
            RefitType::Dragon => "DRAGON",
            RefitType::Architect => "ARCHITECT",
            RefitType::Trailblazer => "TRAILBLAZER",
            RefitType::Surgeon => "SURGEON",
            RefitType::MountedArcher => "MOUNTED_ARCHER",
            RefitType::Firehawk => "FIREHAWK",
            RefitType::PotionMaster => "POTION_MASTER",
            RefitType::Wrangler => "WRANGLER",
            RefitType::Sapper => "SAPPER",
            RefitType::ChaosMage => "CHAOS_MAGE",
            RefitType::GroundChariot => "GROUND_CHARIOT",
            RefitType::RoughRider => "ROUGH_RIDER",
            RefitType::Paladin => "PALADIN",
            RefitType::LightCav => "LIGHT_CAV",
            RefitType::FlatbedCart => "FLATBED_CART",
        }
    }
}

impl Asset for RefitType {}

impl ValidFor for RefitType {
    fn valid_for(form_state: &FormState) -> Vec<RefitType> {
        match form_state.unit_type {
            UnitType::None => vec![RefitType::None],
            UnitType::Soldiers => vec![RefitType::None, RefitType::Champion, RefitType::Infiltrator, RefitType::Archer],
            UnitType::Mages => vec![RefitType::None, RefitType::BattleMage, RefitType::Archmage, RefitType::ChaosMage],
            UnitType::Healers => vec![RefitType::None, RefitType::Surgeon, RefitType::PotionMaster, RefitType::Paladin],
            UnitType::Knights => vec![RefitType::None, RefitType::MountedArcher, RefitType::Cavalier, RefitType::LightCav],
            UnitType::Laborers => vec![RefitType::None, RefitType::Artisan, RefitType::Wrangler, RefitType::Architect],
            UnitType::SiegeCrew => vec![RefitType::None, RefitType::WarWagon, RefitType::HeavySiege, RefitType::Sapper],
            UnitType::SupplyCart => vec![RefitType::None, RefitType::FlatbedCart, RefitType::Caravan, RefitType::GroundChariot],
            UnitType::LineBreaker => vec![RefitType::None, RefitType::RoughRider, RefitType::SiegeBeast, RefitType::Trailblazer],
            UnitType::SkyBeast => vec![RefitType::None, RefitType::Firehawk, RefitType::SkyKnight, RefitType::SkyChariot],
            UnitType::Monstrosity => vec![RefitType::None, RefitType::Brute, RefitType::Prowler, RefitType::Dragon],
        }
    }
}

impl RefitType {
    pub(crate) fn unit_gold(&self) -> u8 {
        match self {
            RefitType::None => 0,
            RefitType::Champion => 2,
            RefitType::Infiltrator => 1,
            RefitType::Archer => 1,
            RefitType::Artisan => 2,
            RefitType::WarWagon => 0,
            RefitType::HeavySiege => 0,
            RefitType::BattleMage => 2,
            RefitType::Archmage => 3,
            RefitType::Caravan => 0,
            RefitType::Cavalier => 0,
            RefitType::SkyKnight => 1,
            RefitType::SiegeBeast => 0,
            RefitType::SkyChariot => 0,
            RefitType::Prowler => 0,
            RefitType::Brute => 0,
            RefitType::Dragon => 0,
            RefitType::Architect => 3,
            RefitType::Trailblazer => 0,
            RefitType::Surgeon => 0,
            RefitType::MountedArcher => 1,
            RefitType::Firehawk => 0,
            RefitType::PotionMaster => 0,
            RefitType::Wrangler => 2,
            RefitType::Sapper => 1,
            RefitType::ChaosMage => 1,
            RefitType::GroundChariot => 1,
            RefitType::RoughRider => 1,
            RefitType::Paladin => 1,
            RefitType::LightCav => 1,
            RefitType::FlatbedCart => 0,
        }
    }
}

impl From<RefitType> for u8 {
    fn from(value: RefitType) -> Self {
        value as u8
    }
}

impl From<u8> for RefitType {
    fn from(value: u8) -> Self {
        match value {
            0 => RefitType::None,
            1 => RefitType::Champion,
            2 => RefitType::Infiltrator,
            3 => RefitType::Archer,
            4 => RefitType::Artisan,
            5 => RefitType::WarWagon,
            6 => RefitType::HeavySiege,
            7 => RefitType::BattleMage,
            8 => RefitType::Archmage,
            9 => RefitType::Caravan,
            10 => RefitType::Cavalier,
            11 => RefitType::SkyKnight,
            12 => RefitType::SiegeBeast,
            13 => RefitType::SkyChariot,
            14 => RefitType::Prowler,
            15 => RefitType::Brute,
            16 => RefitType::Dragon,
            17 => RefitType::Architect,
            18 => RefitType::Trailblazer,
            19 => RefitType::Surgeon,
            20 => RefitType::MountedArcher,
            21 => RefitType::Firehawk,
            22 => RefitType::PotionMaster,
            23 => RefitType::Wrangler,
            24 => RefitType::Sapper,
            25 => RefitType::ChaosMage,
            26 => RefitType::GroundChariot,
            27 => RefitType::RoughRider,
            28 => RefitType::Paladin,
            29 => RefitType::LightCav,
            30 => RefitType::FlatbedCart,
            _ => RefitType::default(),
        }
    }
}

impl From<RefitType> for web_sys::HtmlOptionElement {
    fn from(value: RefitType) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// HEAVY_SIEGE

impl Field for HeavySiege {
    const ALL: &'static [Self] = &[HeavySiege::None, HeavySiege::Trebuchet, HeavySiege::Scorpion, HeavySiege::DragonfireCannon, HeavySiege::Mangonel, HeavySiege::GuidedBolt, HeavySiege::BurningOil];
    fn cost(&self) -> u8 {
        match self {
            HeavySiege::None => 0,
            HeavySiege::Trebuchet => 0,
            HeavySiege::Scorpion => 0,
            HeavySiege::DragonfireCannon => 0,
            HeavySiege::Mangonel => 2,
            HeavySiege::GuidedBolt => 1,
            HeavySiege::BurningOil => 2,
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[HeavySiege]) -> HeavySiege {
        if valid.contains(&form_state.heavy_siege) { form_state.heavy_siege } else { HeavySiege::default() }
    }
}

impl Named for HeavySiege {
    fn name(&self) -> &str {
        match self {
            HeavySiege::None => "NONE",
            HeavySiege::Trebuchet => "TREBUCHET",
            HeavySiege::Scorpion => "SCORPION",
            HeavySiege::DragonfireCannon => "DRAGONFIRE_CANNON",
            HeavySiege::Mangonel => "MANGONEL",
            HeavySiege::GuidedBolt => "GUIDED_BOLT",
            HeavySiege::BurningOil => "BURNING_OIL",
        }
    }
}

impl Asset for HeavySiege {}

impl ValidFor for HeavySiege {
    fn valid_for(form_state: &FormState) -> Vec<HeavySiege> {
        match (form_state.refit_type, form_state.siege) {
            (_, Siege::None) => vec![HeavySiege::None],
            (RefitType::HeavySiege, Siege::Catapult) => vec![HeavySiege::None, HeavySiege::Trebuchet, HeavySiege::Mangonel],
            (_, Siege::Catapult) => vec![HeavySiege::None, HeavySiege::Mangonel],
            (RefitType::HeavySiege, Siege::BoltThrower) => vec![HeavySiege::None, HeavySiege::Scorpion, HeavySiege::GuidedBolt],
            (_, Siege::BoltThrower) => vec![HeavySiege::None, HeavySiege::GuidedBolt],
            (RefitType::HeavySiege, Siege::FlameSpitter) => vec![HeavySiege::None, HeavySiege::DragonfireCannon, HeavySiege::BurningOil],
            (_, Siege::FlameSpitter) => vec![HeavySiege::None, HeavySiege::BurningOil]

        }
    }
}

impl From<HeavySiege> for u8 {
    fn from(value: HeavySiege) -> Self {
        value as u8
    }
}

impl From<u8> for HeavySiege {
    fn from(value: u8) -> Self {
        match value {
            0 => HeavySiege::None,
            1 => HeavySiege::Trebuchet,
            2 => HeavySiege::Scorpion,
            3 => HeavySiege::DragonfireCannon,
            4 => HeavySiege::Mangonel,
            5 => HeavySiege::GuidedBolt,
            6 => HeavySiege::BurningOil,
            _ => HeavySiege::default(),
        }
    }
}

impl From<HeavySiege> for web_sys::HtmlOptionElement {
    fn from(value: HeavySiege) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// PACK

impl Field for Pack {
    const ALL: &'static [Self] = &[Pack::None, Pack::Ghost, Pack::Barding, Pack::MagicalBarrier, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::ArrowSlits, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::Veterinarian, Pack::HeavyArmor, Pack::Construction, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Vitalist, Pack::Carpenter, Pack::MimicCart, Pack::CloudStrider, Pack::Sailor, Pack::BerserkersHelp, Pack::BagOfHolding, Pack::CaptainsTricorn];
    fn cost(&self) -> u8 {
        match self {
            Pack::None => 0,
            Pack::Ghost => 1,
            Pack::Barding => 2,
            Pack::MagicalBarrier => 2,
            Pack::BagOfHolding => 2,
            Pack::CloakOfTheChameleon => 2,
            Pack::GreavesOfTheFox => 3,
            Pack::CapeOfTheBat => 3,
            Pack::GlovesOfTheTiger => 2,
            Pack::ArrowSlits => 2,
            Pack::CarriersSaddle => 2,
            Pack::ScoutCavalry => 2,
            Pack::Veterinarian => 2,
            Pack::CaptainsTricorn => 2,
            Pack::HeavyArmor => 2,
            Pack::Construction => 2,
            Pack::HonorGuard => 2,
            Pack::MageInitiate => 1,
            Pack::FinsOfTheUnagi => 1,
            Pack::Vitalist => 2,
            Pack::Carpenter => 2,
            Pack::MimicCart => 1,
            Pack::CloudStrider => 2,
            Pack::Sailor => 1,
            Pack::BerserkersHelp => 3,
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[Pack]) -> Pack {
        if valid.contains(&form_state.pack) { form_state.pack } else { Pack::default() }
    }
}

impl Named for Pack {
    fn name(&self) -> &str {
        match self {
            Pack::None => "NONE",
            Pack::Ghost => "GHOST",
            Pack::Barding => "BARDING",
            Pack::MagicalBarrier => "MAGICAL_BARRIER",
            Pack::CloakOfTheChameleon => "CLOAK_OF_THE_CHAMELEON",
            Pack::GreavesOfTheFox => "GREAVES_OF_THE_FOX",
            Pack::CapeOfTheBat => "CAPE_OF_THE_BAT",
            Pack::GlovesOfTheTiger => "GLOVES_OF_THE_TIGER",
            Pack::ArrowSlits => "ARROW_SLITS",
            Pack::CarriersSaddle => "CARRIERS_SADDLE",
            Pack::ScoutCavalry => "SCOUT_CAVALRY",
            Pack::Veterinarian => "VETERNARIAN",
            Pack::HeavyArmor => "HVY_ARMOR",
            Pack::Construction => "CONSTRUCTION",
            Pack::HonorGuard => "HONOR_GUARD",
            Pack::MageInitiate => "MAGE_INITIATE",
            Pack::FinsOfTheUnagi => "FINS_UNAGI",
            Pack::Vitalist => "VITALIST",
            Pack::Carpenter => "CARPENTER",
            Pack::MimicCart => "MIMIC_CART",
            Pack::CloudStrider => "CLOUD_STRIDER",
            Pack::Sailor => "SAILOR",
            Pack::BerserkersHelp => "BERSERKERS_HELM",
            Pack::BagOfHolding => "BAG_OF_HOLDING",
            Pack::CaptainsTricorn => "CAPTAINS_TRICORNE",
        }
    }
}

impl Asset for Pack {}

impl ValidFor for Pack {
    fn valid_for(form_state: &FormState) -> Vec<Pack> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![Pack::None],
                UnitType::Soldiers => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::Construction, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
                UnitType::Mages => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
                UnitType::SupplyCart => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::ArrowSlits, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::MimicCart, Pack::Sailor],
                UnitType::LineBreaker => vec![Pack::None, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
                UnitType::Monstrosity => vec![Pack::None, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
                UnitType::SkyBeast => vec![Pack::None, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::CloudStrider, Pack::Sailor],
                UnitType::Knights => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor, Pack::BerserkersHelp],
                UnitType::Laborers => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
                UnitType::SiegeCrew => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
                UnitType::Healers => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::Veterinarian, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Vitalist, Pack::Carpenter, Pack::Sailor],
            }
        }
            RefitType::Champion => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::Construction, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::Infiltrator => vec![Pack::None, Pack::Ghost, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::Construction, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::BattleMage => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::HonorGuard, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::Archmage => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
            RefitType::Architect => vec![Pack::None, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
            RefitType::ChaosMage => vec![Pack::None, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
            RefitType::SkyChariot => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GlovesOfTheTiger, Pack::ArrowSlits, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::CloudStrider, Pack::Sailor],
            RefitType::Trailblazer => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::ArrowSlits, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::Sailor],
            RefitType::Wrangler => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
            RefitType::Caravan => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::ArrowSlits, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::MimicCart, Pack::Sailor],
            RefitType::Dragon => vec![Pack::None, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
            RefitType::FlatbedCart => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::MimicCart, Pack::Sailor],
            RefitType::GroundChariot => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::MimicCart, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::Sapper => vec![Pack::None, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::WarWagon => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::ArrowSlits, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::MimicCart, Pack::Sailor],
            RefitType::Brute => vec![Pack::None, Pack::Barding, Pack::CapeOfTheBat, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
            RefitType::Firehawk => vec![Pack::None, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::CloudStrider, Pack::Sailor],
            RefitType::SiegeBeast => vec![Pack::None, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
            RefitType::Artisan => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor],
            RefitType::Cavalier => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::LightCav => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::MountedArcher => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::Prowler => vec![Pack::None, Pack::Ghost, Pack::Barding, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::CarriersSaddle, Pack::CaptainsTricorn, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor],
            RefitType::RoughRider => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::Paladin => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::GlovesOfTheTiger, Pack::CarriersSaddle, Pack::Veterinarian, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Vitalist, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::SkyKnight => vec![Pack::None, Pack::Barding, Pack::BagOfHolding, Pack::GreavesOfTheFox, Pack::CarriersSaddle, Pack::ScoutCavalry, Pack::CaptainsTricorn, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::CloudStrider, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::Surgeon => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::Veterinarian, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Vitalist, Pack::Carpenter, Pack::Sailor],
            RefitType::HeavySiege => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
            RefitType::PotionMaster => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::Veterinarian, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Vitalist, Pack::Carpenter, Pack::Sailor],
            RefitType::Archer => vec![Pack::None, Pack::MagicalBarrier, Pack::BagOfHolding, Pack::CloakOfTheChameleon, Pack::GreavesOfTheFox, Pack::CapeOfTheBat, Pack::GlovesOfTheTiger, Pack::CaptainsTricorn, Pack::HeavyArmor, Pack::Construction, Pack::HonorGuard, Pack::MageInitiate, Pack::FinsOfTheUnagi, Pack::Carpenter, Pack::Sailor, Pack::BerserkersHelp],
        }
    }
}

impl From<Pack> for u8 {
    fn from(value: Pack) -> Self {
        value as u8
    }
}

impl From<u8> for Pack {
    fn from(value: u8) -> Self {
        match value {
            0 => Pack::None,
            1 => Pack::Ghost,
            2 => Pack::Barding,
            3 => Pack::MagicalBarrier,
            4 => Pack::BagOfHolding,
            5 => Pack::CloakOfTheChameleon,
            6 => Pack::GreavesOfTheFox,
            7 => Pack::CapeOfTheBat,
            8 => Pack::GlovesOfTheTiger,
            9 => Pack::ArrowSlits,
            10 => Pack::CarriersSaddle,
            11 => Pack::ScoutCavalry,
            12 => Pack::Veterinarian,
            13 => Pack::CaptainsTricorn,
            14 => Pack::HeavyArmor,
            15 => Pack::Construction,
            16 => Pack::HonorGuard,
            17 => Pack::MageInitiate,
            18 => Pack::FinsOfTheUnagi,
            19 => Pack::Vitalist,
            20 => Pack::Carpenter,
            21 => Pack::MimicCart,
            22 => Pack::CloudStrider,
            23 => Pack::Sailor,
            24 => Pack::BerserkersHelp,
            _ => Pack::default(),
        }
    }
}

impl From<Pack> for web_sys::HtmlOptionElement {
    fn from(value: Pack) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// POTION

impl Field for Potion {
    const ALL: &'static [Self] = &[Potion::None, Potion::Healing, Potion::Mana, Potion::Enlarging, Potion::Haste, Potion::Invisibility, Potion::Levitation, Potion::Sharpness, Potion::Stoneskin, Potion::Strength, Potion::Warp];
    fn cost(&self) -> u8 {
        match self {
            Potion::None => 0,
            _ => 1,
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[Potion]) -> Potion {
        if valid.contains(&form_state.potion) { form_state.potion } else { Potion::default() }
    }
}

impl Named for Potion {
    fn name(&self) -> &str {
        match self {
            Potion::None => "NONE",
            Potion::Healing => "POTION_OF_HEALING",
            Potion::Mana => "POTION_OF_MANA",
            Potion::Enlarging => "POTION_OF_ENLARGING",
            Potion::Haste => "POTION_OF_HASTE",
            Potion::Invisibility => "POTION_OF_INVISIBILITY",
            Potion::Levitation => "POTION_OF_LEVITATION",
            Potion::Sharpness => "POTION_OF_SHARPNESS",
            Potion::Stoneskin => "POTION_OF_STONESKIN",
            Potion::Strength => "POTION_OF_STRENGTH",
            Potion::Warp => "POTION_OF_WARP",
        }
    }
}

impl Asset for Potion {}

impl ValidFor for Potion {
    fn valid_for(form_state: &FormState) -> Vec<Potion> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![Potion::None],
                UnitType::Soldiers => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::Mages => vec![Potion::None, Potion::Healing, Potion::Mana, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::Knights => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::SkyBeast => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::SupplyCart => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::Healers => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::Laborers => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::LineBreaker => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::Monstrosity => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
                UnitType::SiegeCrew => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],

            }}
            RefitType::Champion => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Infiltrator => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::BattleMage => vec![Potion::None, Potion::Healing, Potion::Mana, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Archmage => vec![Potion::None, Potion::Healing, Potion::Mana, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Archer => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Architect => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Artisan => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Brute => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Strength, Potion::Sharpness],
            RefitType::Caravan => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Cavalier => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::ChaosMage => vec![Potion::None, Potion::Healing, Potion::Mana, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Dragon => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Firehawk => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::FlatbedCart => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::GroundChariot => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::HeavySiege => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::LightCav => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::MountedArcher => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Paladin => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::PotionMaster => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Prowler => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::RoughRider => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Sapper => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::SiegeBeast => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::SkyChariot => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::SkyKnight => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Surgeon => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Trailblazer => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Invisibility, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::WarWagon => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
            RefitType::Wrangler => vec![Potion::None, Potion::Healing, Potion::Haste, Potion::Levitation, Potion::Stoneskin, Potion::Warp, Potion::Enlarging, Potion::Strength, Potion::Sharpness],
        }
    }
}

impl From<Potion> for u8 {
    fn from(value: Potion) -> Self {
        value as u8
    }
}

impl From<u8> for Potion {
    fn from(value: u8) -> Self {
        match value {
            0 => Potion::None,
            1 => Potion::Healing,
            2 => Potion::Mana,
            3 => Potion::Enlarging,
            4 => Potion::Haste,
            5 => Potion::Invisibility,
            6 => Potion::Levitation,
            7 => Potion::Sharpness,
            8 => Potion::Stoneskin,
            9 => Potion::Strength,
            10 => Potion::Warp,
            _ => Potion::default(),
        }
    }
}

impl From<Potion> for web_sys::HtmlOptionElement {
    fn from(value: Potion) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// SEIGE

impl Field for Siege {
    const ALL: &[Siege] = &[Siege::None, Siege::Catapult, Siege::BoltThrower, Siege::FlameSpitter];
    fn cost(&self) -> u8 {
        match self {
            Siege::None => 0,
            Siege::Catapult => 2,
            Siege::BoltThrower => 2,
            Siege::FlameSpitter => 2,
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[Siege]) -> Siege {
        if valid.contains(&form_state.siege) { form_state.siege } else { Siege::default() }
    }
}

impl Named for Siege {
    fn name(&self) -> &str {
        match self {
            Siege::None => "NONE",
            Siege::Catapult => "CATAPULT",
            Siege::BoltThrower => "BOLT_THROWER",
            Siege::FlameSpitter => "FLAME_SPITTER",
        }
    }
}

impl Asset for Siege {}

impl ValidFor for Siege {
    fn valid_for(form_state: &FormState) -> Vec<Siege> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![Siege::None],
                UnitType::SiegeCrew => Siege::ALL.to_vec(),
                _ => vec![Siege::None],
            }}
            RefitType::HeavySiege => Siege::ALL.to_vec(),
            RefitType::Sapper => Siege::ALL.to_vec(),
            RefitType::WarWagon => Siege::ALL.to_vec(),
            _ => vec![Siege::None],
        }
    }
}

impl From<Siege> for u8 {
    fn from(value: Siege) -> Self {
        value as u8
    }
}

impl From<u8> for Siege {
    fn from(value: u8) -> Self {
        match value {
            0 => Siege::None,
            1 => Siege::Catapult,
            2 => Siege::BoltThrower,
            3 => Siege::FlameSpitter,
            _ => Siege::None,
        }
    }
}

impl From<Siege> for web_sys::HtmlOptionElement {
    fn from(value: Siege) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// SPELL

impl FieldArray for Spell {
    const ALL: &[Spell] = &[Spell::None, Spell::ArcaneBlast, Spell::Fabricate, Spell::Barrier, Spell::CureWounds, Spell::ScryingEye, Spell::Haste, Spell::InscribeCircle, Spell::ConjurePallisade, Spell::Fly, Spell::Invisibility, Spell::Polymorph, Spell::Drain, Spell::MagicMirror, Spell::SummonZombies, Spell::ManaConduit, Spell::Elementalism, Spell::ArcaneStorm, Spell::MassBarrier, Spell::MassHeal, Spell::Teleport, Spell::CompleteStructures, Spell::Divination, Spell::Resurrection, Spell::TruePolymorph, Spell::CreateUndeadHorde, Spell::MirrorDome, Spell::SummonUndeadBeast];
    fn cost(&self) -> u8 {
        match self {
            Spell::None => 0,
            Spell::ArcaneStorm => 3,
            Spell::MassBarrier => 3,
            Spell::MassHeal => 3,
            Spell::Teleport => 3,
            Spell::CompleteStructures => 3,
            Spell::Divination => 3,
            Spell::Resurrection => 3,
            Spell::TruePolymorph => 3,
            Spell::CreateUndeadHorde => 3,
            Spell::MirrorDome => 3,
            Spell::SummonUndeadBeast => 3,
            _ => 1,
        }
    }
    fn limit(&self) -> usize {
        match self {
            Spell::None => 5,
            _ => 1,
        }
    }
    fn resolve_or_default_slot(form_state: &FormState, valid: Vec<Spell>, slot: usize) -> Spell {
        if valid.contains(&form_state.spells[slot]) { form_state.spells[slot] } else { Spell::default() }
    }
}

impl Named for Spell {
    fn name(&self) -> &str {
        match self {
            Spell::None => "NONE",
            Spell::ArcaneStorm => "ARCANE_STORM",
            Spell::MassBarrier => "MASS_BARRIER",
            Spell::MassHeal => "MASS_HEAL",
            Spell::Teleport => "TELEPORT",
            Spell::CompleteStructures => "COMPLETE_STRUCTURES",
            Spell::Divination => "DIVINATION",
            Spell::Resurrection => "RESURRECTION",
            Spell::TruePolymorph => "TRUE_POLYMORPH",
            Spell::CreateUndeadHorde => "CREATE_UNDEAD_HORDE",
            Spell::MirrorDome => "MIRROR_DOME",
            Spell::SummonUndeadBeast => "SUMMON_UNDEAD_BEAST",
            Spell::ArcaneBlast => "ARCANE_BLAST",
            Spell::Fabricate => "FABRICATE",
            Spell::Barrier => "BARRIER",
            Spell::CureWounds => "CURE_WOUNDS",
            Spell::ScryingEye => "SCRYING_EYE",
            Spell::Haste => "HASTE",
            Spell::InscribeCircle => "INSCRIBE_CIRCLE",
            Spell::ConjurePallisade => "CONJURE_PALLISADE",
            Spell::Fly => "FLY",
            Spell::Invisibility => "INVISIBILITY",
            Spell::Polymorph => "POLYMORPH",
            Spell::Drain => "DRAIN",
            Spell::MagicMirror => "MAGIC_MIRROR",
            Spell::SummonZombies => "SUMMON_ZOMBIES",
            Spell::ManaConduit => "MANA_CONDUIT",
            Spell::Elementalism => "ELEMENTALISM",
        }
    }
}

impl Asset for Spell {}

impl ValidForArray for Spell {
    fn base_for(form_state: &FormState) -> Vec<Spell> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![Spell::None],
                UnitType::Mages => Spell::ALL.to_vec(),
                _ => vec![Spell::None],
            }}
            RefitType::BattleMage => Spell::ALL.to_vec(),
            RefitType::Archmage => Spell::ALL.to_vec(),
            RefitType::ChaosMage => Spell::ALL.to_vec(),
            _ => vec![Spell::None],
        }
    }
    fn valid_for(form_state: &FormState) -> [Vec<Spell>; 5] {
        let base = Self::base_for(form_state);
        std::array::from_fn(|i| {
            base.iter()
            .copied()
            .filter(|spell| form_state.count_other_spells(i, *spell) < spell.limit())
            .collect()
        })
    }
    
}

impl From<Spell> for u8 {
    fn from(value: Spell) -> Self {
        value as u8
    }
}

impl From<u8> for Spell {
    fn from(value: u8) -> Self {
        match value {
            0 => Spell::None,
            1 => Spell::ArcaneStorm,
            2 => Spell::MassBarrier,
            3 => Spell::MassHeal,
            4 => Spell::Teleport,
            5 => Spell::CompleteStructures,
            6 => Spell::Divination,
            7 => Spell::Resurrection,
            8 => Spell::TruePolymorph,
            9 => Spell::CreateUndeadHorde,
            10 => Spell::MirrorDome,
            11 => Spell::SummonUndeadBeast,
            12 => Spell::ArcaneBlast,
            13 => Spell::Fabricate,
            14 => Spell::Barrier,
            15 => Spell::CureWounds,
            16 => Spell::ScryingEye,
            17 => Spell::Haste,
            18 => Spell::InscribeCircle,
            19 => Spell::ConjurePallisade,
            20 => Spell::Fly,
            21 => Spell::Invisibility,
            22 => Spell::Polymorph,
            23 => Spell::Drain,
            24 => Spell::MagicMirror,
            25 => Spell::SummonZombies,
            26 => Spell::ManaConduit,
            27 => Spell::Elementalism,
            _ => Spell::None,
        }
    }
}

impl From<Spell> for web_sys::HtmlOptionElement {
    fn from(value: Spell) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// STRUCTURE

impl FieldArray for Structure {
    const ALL: &[Structure] = &[Structure::None, Structure::Stable, Structure::WatchTower, Structure::Shrine, Structure::ExoticStable, Structure::Hospital, Structure::Fortress, Structure::Castle, Structure::Temple, Structure::Smith, Structure::Mill, Structure::Apothecary, Structure::Channel, Structure::Keep, Structure::Dock];
    fn cost(&self) -> u8 {
        match self {
            Structure::None => 0,
            Structure::Stable => 1,
            Structure::WatchTower => 1,
            Structure::Shrine => 1,
            Structure::ExoticStable => 2,
            Structure::Hospital => 2,
            Structure::Fortress => 2,
            Structure::Castle => 2,
            Structure::Temple => 2,
            Structure::Smith => 1,
            Structure::Mill => 1,
            Structure::Apothecary => 1,
            Structure::Channel => 0,
            Structure::Keep => 2,
            Structure::Dock => 1,
        }
    }
    fn limit(&self) -> usize {
        match self {
            Structure::None => 5,
            _ => 1,
        }
    }
    fn resolve_or_default_slot(form_state: &FormState, valid: Vec<Structure>, slot: usize) -> Structure {
        if valid.contains(&form_state.structures[slot]) { form_state.structures[slot] } else { Structure::default() }
    }
}

impl Named for Structure {
    fn name(&self) -> &str {
        match self {
            Structure::None => "NONE",
            Structure::Stable => "STABLE",
            Structure::WatchTower => "WATCH_TOWER",
            Structure::Shrine => "SHRINE",
            Structure::ExoticStable => "EXOTIC_STABLE",
            Structure::Hospital => "HOSPITAL",
            Structure::Fortress => "FORTRESS",
            Structure::Castle => "CASTLE",
            Structure::Temple => "TEMPLE",
            Structure::Smith => "SMITH",
            Structure::Mill => "MILL",
            Structure::Apothecary => "APOTHECARY",
            Structure::Channel => "CHANNEL",
            Structure::Keep => "KEEP",
            Structure::Dock => "DOCK",
        }
    }
}

impl Asset for Structure {}

impl ValidForArray for Structure {
    fn base_for(form_state: &FormState) -> Vec<Structure> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![Structure::None],
                UnitType::Laborers => Structure::ALL.to_vec(),
                _ => vec![Structure::None],
            }}
            RefitType::Architect => Structure::ALL.to_vec(),
            RefitType::Artisan => Structure::ALL.to_vec(),
            RefitType::Wrangler => Structure::ALL.to_vec(),
            _ => vec![Structure::None],
        }
    }
    fn valid_for(form_state: &FormState) -> [Vec<Structure>; 5] {
        let base = Self::base_for(form_state);
        std::array::from_fn(|i| {
            base.iter()
            .copied()
            .filter(|structure| form_state.count_other_structures(i, *structure) < structure.limit())
            .collect()
        })
    }
}

impl From<Structure> for u8 {
    fn from(value: Structure) -> Self {
        value as u8
    }
}

impl From<u8> for Structure {
    fn from(value: u8) -> Self {
        match value {
            0 => Structure::None,
            1 => Structure::Stable,
            2 => Structure::WatchTower,
            3 => Structure::Shrine,
            4 => Structure::ExoticStable,
            5 => Structure::Hospital,
            6 => Structure::Fortress,
            7 => Structure::Castle,
            8 => Structure::Temple,
            9 => Structure::Smith,
            10 => Structure::Mill,
            11 => Structure::Apothecary,
            12 => Structure::Channel,
            13 => Structure::Keep,
            14 => Structure::Dock,
            _ => Structure::None,
        }
    }
}

impl From<Structure> for web_sys::HtmlOptionElement {
    fn from(value: Structure) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// VESSEL

impl Field for Vessel {
    const ALL: &[Vessel] = &[Vessel::None, Vessel::Knarr, Vessel::Caravel, Vessel::Trireme, Vessel::Karve];
    fn cost(&self) -> u8 {
        match self {
            Vessel::None => 0,
            Vessel::Knarr => 0,
            Vessel::Caravel => 2,
            Vessel::Trireme => 2,
            Vessel::Karve => 2,
        }
    }
    fn resolve_or_default(form_state: &FormState, valid: &[Vessel]) -> Vessel {
        match form_state.pack {
            Pack::CaptainsTricorn => if valid.contains(&form_state.vessel) { form_state.vessel } else { Vessel::Knarr }
            _ => Vessel::default(),
        }
    }
}

impl Named for Vessel {
    fn name(&self) -> &str {
        match self {
            Vessel::None => "NONE",
            Vessel::Knarr => "KNARR",
            Vessel::Caravel => "CARAVEL",
            Vessel::Trireme => "TRIREME",
            Vessel::Karve => "KARVE",
        }
    }
}

impl Asset for Vessel {}

impl ValidFor for Vessel {
    fn valid_for(form_state: &FormState) -> Vec<Vessel> {
        match form_state.pack {
            Pack::CaptainsTricorn => vec![Vessel::Knarr, Vessel::Caravel, Vessel::Trireme, Vessel::Karve],
            _ => vec![Vessel::None],
        }
    }
}

impl From<Vessel> for u8 {
    fn from(value: Vessel) -> Self {
        value as u8
    }
}

impl From<u8> for Vessel {
    fn from(value: u8) -> Self {
        match value {
            0 => Vessel::None,
            1 => Vessel::Knarr,
            2 => Vessel::Caravel,
            3 => Vessel::Trireme,
            4 => Vessel::Karve,
            _ => Vessel::None,
        }
    }
}

impl From<Vessel> for web_sys::HtmlOptionElement {
    fn from(value: Vessel) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// VESSEL_UP

impl FieldArray for VesselUpgrade {
    const ALL: &[VesselUpgrade] = &[VesselUpgrade::None, VesselUpgrade::LateenSails, VesselUpgrade::RowerBench, VesselUpgrade::Anchor, VesselUpgrade::Incendiaries, VesselUpgrade::Armory, VesselUpgrade::Bulwarks, VesselUpgrade::HeavyPlanks, VesselUpgrade::Specialization, VesselUpgrade::DedicatedVessel, VesselUpgrade::CargoHold, VesselUpgrade::LaunchPlatform, VesselUpgrade::SternRudder, VesselUpgrade::SkeletonCrew, VesselUpgrade::GalleonC, VesselUpgrade::GalleonT, VesselUpgrade::GalleonK, VesselUpgrade::Corvus, VesselUpgrade::PotionDiffusion, VesselUpgrade::CrowsNest];
    fn cost(&self) -> u8 {
        match self {
            VesselUpgrade::None => 0,
            VesselUpgrade::LateenSails => 2,
            VesselUpgrade::RowerBench => 2,
            VesselUpgrade::BoardingSpurs => 1,
            VesselUpgrade::Incendiaries => 1,
            VesselUpgrade::Armory => 1,
            VesselUpgrade::Bulwarks => 2,
            VesselUpgrade::HeavyPlanks => 1,
            VesselUpgrade::Specialization => 2,
            VesselUpgrade::DedicatedVessel => 3,
            VesselUpgrade::CargoHold => 1,
            VesselUpgrade::LaunchPlatform => 1,
            VesselUpgrade::SternRudder => 1,
            VesselUpgrade::SkeletonCrew => 1,
            VesselUpgrade::GalleonC => 2,
            VesselUpgrade::GalleonT => 2,
            VesselUpgrade::GalleonK => 2,
            VesselUpgrade::Corvus => 1,
            VesselUpgrade::Anchor => 1,
            VesselUpgrade::PotionDiffusion => 2,
            VesselUpgrade::CrowsNest => 1,
        }
    }
    fn limit(&self) -> usize {
        match self {
            VesselUpgrade::None => 5,
            VesselUpgrade::Armory => 2,
            VesselUpgrade::HeavyPlanks => 2,
            VesselUpgrade::CargoHold => 4,
            VesselUpgrade::Specialization => 3,
            _ => 1,
        }
    }
    fn resolve_or_default_slot(form_state: &FormState, valid: Vec<VesselUpgrade>, slot: usize) -> VesselUpgrade {
        if valid.contains(&form_state.vessel_upgrades[slot]) { form_state.vessel_upgrades[slot] } else { VesselUpgrade::default() }
    }
}

impl Named for VesselUpgrade {
    fn name(&self) -> &str {
        match self {
            VesselUpgrade::None => "NONE",
            VesselUpgrade::LateenSails => "LATEEN_SAILS",
            VesselUpgrade::RowerBench => "ROWER_BENCH",
            VesselUpgrade::BoardingSpurs => "BOARDING_SPURS",
            VesselUpgrade::Incendiaries => "INCENDIARIES",
            VesselUpgrade::Armory => "ARMORY",
            VesselUpgrade::Bulwarks => "BULWARKS",
            VesselUpgrade::HeavyPlanks => "HEAVY_PLANKS",
            VesselUpgrade::Specialization => "SPECIALIZATION",
            VesselUpgrade::DedicatedVessel => "DEDICATED_VESSEL",
            VesselUpgrade::CargoHold => "CARGO_HOLD",
            VesselUpgrade::LaunchPlatform => "LAUNCH_PLATFORM",
            VesselUpgrade::SternRudder => "STERN_RUDDER",
            VesselUpgrade::SkeletonCrew => "SKELETON_CREW",
            VesselUpgrade::GalleonC => "GALEON_C",
            VesselUpgrade::GalleonT => "GALEON_T",
            VesselUpgrade::GalleonK => "GALEON_K",
            VesselUpgrade::Corvus => "CORVUS",
            VesselUpgrade::Anchor => "ANCHOR",
            VesselUpgrade::PotionDiffusion => "POTION_DIFFUSION",
            VesselUpgrade::CrowsNest => "CROWS_NEST",
        }
    }
}

impl Asset for VesselUpgrade {}

impl ValidForArray for VesselUpgrade {
    fn base_for(form_state: &FormState) -> Vec<VesselUpgrade> {
        match form_state.vessel {
            Vessel::None => vec![VesselUpgrade::None],
            Vessel::Knarr => VesselUpgrade::ALL.to_vec(),
            Vessel::Caravel => VesselUpgrade::ALL.to_vec(),
            Vessel::Trireme => VesselUpgrade::ALL.to_vec(),
            Vessel::Karve => VesselUpgrade::ALL.to_vec(),
        }
    }
    fn valid_for(form_state: &FormState) -> [Vec<VesselUpgrade>; 5] {
        let base = Self::base_for(form_state);
        std::array::from_fn(|i| {
            base.iter()
            .copied()
            .filter(|vessel_up| form_state.count_other_vessel_upgrades(i, *vessel_up) < vessel_up.limit())
            .collect()
        })
    }
}

impl From<VesselUpgrade> for u8 {
    fn from(value: VesselUpgrade) -> Self {
        value as u8
    }
}

impl From<u8> for VesselUpgrade {
    fn from(value: u8) -> Self {
        match value {
            0 => VesselUpgrade::None,
            1 => VesselUpgrade::LateenSails,
            2 => VesselUpgrade::RowerBench,
            3 => VesselUpgrade::BoardingSpurs,
            4 => VesselUpgrade::Incendiaries,
            5 => VesselUpgrade::Armory,
            6 => VesselUpgrade::Bulwarks,
            7 => VesselUpgrade::HeavyPlanks,
            8 => VesselUpgrade::Specialization,
            9 => VesselUpgrade::DedicatedVessel,
            10 => VesselUpgrade::CargoHold,
            11 => VesselUpgrade::LaunchPlatform,
            12 => VesselUpgrade::SternRudder,
            13 => VesselUpgrade::SkeletonCrew,
            14 => VesselUpgrade::GalleonC,
            15 => VesselUpgrade::GalleonT,
            16 => VesselUpgrade::GalleonK,
            17 => VesselUpgrade::Corvus,
            18 => VesselUpgrade::Anchor,
            19 => VesselUpgrade::PotionDiffusion,
            20 => VesselUpgrade::CrowsNest,
            _ => VesselUpgrade::None,
        }
    }
}

impl From<VesselUpgrade> for web_sys::HtmlOptionElement {
    fn from(value: VesselUpgrade) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// WEAPON

impl FieldArray for Weapon {
    const ALL: &[Self] = &[Weapon::None, Weapon::Sword, Weapon::Shield, Weapon::Spear, Weapon::Hammer, Weapon::Bow, Weapon::Lance, Weapon::Ram, Weapon::Axe, Weapon::Crossbow, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs, Weapon::ManticoresSpines, Weapon::MyrmexsAcid, Weapon::SpidersWeb, Weapon::EaglesEyes];
    fn cost(&self) -> u8 {
        match self {
            Weapon::None => 0,
            Weapon::Sword => 1,
            Weapon::Shield => 1,
            Weapon::Spear => 1,
            Weapon::Hammer => 1,
            Weapon::Bow => 1,
            Weapon::Lance => 1,
            Weapon::Ram => 1,
            Weapon::Axe => 1,
            Weapon::Crossbow => 1,
            Weapon::EaglesTalons => 1,
            Weapon::OozesSlime => 1,
            Weapon::HedgehogsSpikes => 2,
            Weapon::StagsAntlers => 2,
            Weapon::ScorpionsStinger => 2,
            Weapon::UnicornsHorn => 2,
            Weapon::CheetahsSwipe => 2,
            Weapon::KangaroosKick => 2,
            Weapon::BatsFangs => 2,
            Weapon::ManticoresSpines => 2,
            Weapon::MyrmexsAcid => 2,
            Weapon::SpidersWeb => 2,
            Weapon::EaglesEyes => 2,
        }
    }
    fn limit(&self) -> usize {
        match self {
            Weapon::None => 5,
            _ => 1,
        }
    }
    fn resolve_or_default_slot(form_state: &FormState, valid: Vec<Weapon>, slot: usize) -> Weapon {
        if valid.contains(&form_state.weapons[slot]) { form_state.weapons[slot] } else { Weapon::default() }
    }
}

impl Named for Weapon {
    fn name(&self) -> &str {
        match self {
            Weapon::None => "NONE",
            Weapon::Sword => "SWORD",
            Weapon::Shield => "SHIELD",
            Weapon::Spear => "SPEAR",
            Weapon::Hammer => "HAMMER",
            Weapon::Bow => "BOW",
            Weapon::Lance => "LANCE",
            Weapon::Ram => "RAM",
            Weapon::Axe => "AXE",
            Weapon::Crossbow => "CROSSBOW",
            Weapon::EaglesTalons => "EAGLES_TALONS",
            Weapon::OozesSlime => "OOZES_SLIME",
            Weapon::HedgehogsSpikes => "HEDGEHOGS_SPIKES",
            Weapon::StagsAntlers => "STAGS_ANTLERS",
            Weapon::ScorpionsStinger => "SCORPIONS_STINGER",
            Weapon::UnicornsHorn => "UNICORNS_HORN",
            Weapon::CheetahsSwipe => "CHEETAHS_SWIPE",
            Weapon::KangaroosKick => "KANGAROOS_KICK",
            Weapon::BatsFangs => "BATS_FANGS",
            Weapon::ManticoresSpines => "MANICORES_SPINES",
            Weapon::MyrmexsAcid => "MYRMEXS_ACID",
            Weapon::SpidersWeb => "SPIDERS_WEB",
            Weapon::EaglesEyes => "EAGLES_EYES",
        }
    }
}

impl Asset for Weapon {}

impl ValidForArray for Weapon {
    fn base_for(form_state: &FormState) -> Vec<Weapon> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![Weapon::None],
                UnitType::SupplyCart => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance],
                UnitType::Knights => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
                UnitType::Healers => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
                UnitType::Laborers => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
                UnitType::SiegeCrew => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
                UnitType::Mages => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
                UnitType::Soldiers => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
                UnitType::LineBreaker => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs],
                UnitType::SkyBeast => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs],
                UnitType::Monstrosity => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs, Weapon::ManticoresSpines, Weapon::MyrmexsAcid, Weapon::SpidersWeb, Weapon::EaglesEyes],

            }}
            RefitType::Caravan => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance],
            RefitType::Cavalier => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::LightCav => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::MountedArcher => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::Paladin => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::RoughRider => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::SkyKnight => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::Wrangler => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::FlatbedCart => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe],
            RefitType::GroundChariot => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Axe, Weapon::Crossbow],
            RefitType::WarWagon => vec![Weapon::None, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Ram, Weapon::Axe],
            RefitType::Archer => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Artisan => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Archmage => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::BattleMage => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Champion => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::HeavySiege => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Infiltrator => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::PotionMaster => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Surgeon => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Architect => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::ChaosMage => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Sapper => vec![Weapon::None, Weapon::Spear, Weapon::Sword, Weapon::Hammer, Weapon::Bow, Weapon::Shield, Weapon::Lance, Weapon::Ram, Weapon::Axe, Weapon::Crossbow],
            RefitType::Firehawk => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs],
            RefitType::SiegeBeast => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs],
            RefitType::SkyChariot => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs],
            RefitType::Trailblazer => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs],
            RefitType::Brute => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs, Weapon::ManticoresSpines, Weapon::MyrmexsAcid, Weapon::SpidersWeb, Weapon::EaglesEyes],
            RefitType::Dragon => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs, Weapon::ManticoresSpines, Weapon::MyrmexsAcid, Weapon::SpidersWeb, Weapon::EaglesEyes],
            RefitType::Prowler => vec![Weapon::None, Weapon::EaglesTalons, Weapon::OozesSlime, Weapon::HedgehogsSpikes, Weapon::StagsAntlers, Weapon::ScorpionsStinger, Weapon::UnicornsHorn, Weapon::CheetahsSwipe, Weapon::KangaroosKick, Weapon::BatsFangs, Weapon::ManticoresSpines, Weapon::MyrmexsAcid, Weapon::SpidersWeb, Weapon::EaglesEyes],
        }
    }
    fn valid_for(form_state: &FormState) -> [Vec<Weapon>; 5] {
        let base = Self::base_for(form_state);
        std::array::from_fn(|i| {
            base.iter()
            .copied()
            .filter(|weapon| form_state.count_other_weapons(i, *weapon) < weapon.limit())
            .collect()
        })
    }
}

impl From<Weapon> for u8 {
    fn from(value: Weapon) -> Self {
        value as u8
    }
}

impl From<u8> for Weapon {
    fn from(value: u8) -> Self {
        match value {
            0 => Weapon::None,
            1 => Weapon::Spear,
            2 => Weapon::Sword,
            3 => Weapon::Hammer,
            4 => Weapon::Bow,
            5 => Weapon::Shield,
            6 => Weapon::Lance,
            7 => Weapon::Ram,
            8 => Weapon::Axe,
            9 => Weapon::Crossbow,
            10 => Weapon::EaglesTalons,
            11 => Weapon::OozesSlime,
            12 => Weapon::HedgehogsSpikes,
            13 => Weapon::StagsAntlers,
            14 => Weapon::ScorpionsStinger,
            15 => Weapon::UnicornsHorn,
            16 => Weapon::CheetahsSwipe,
            17 => Weapon::KangaroosKick,
            18 => Weapon::BatsFangs,
            19 => Weapon::ManticoresSpines,
            20 => Weapon::MyrmexsAcid,
            21 => Weapon::SpidersWeb,
            22 => Weapon::EaglesEyes,
            _ => Weapon::None,
        }
    }
}

impl From<Weapon> for web_sys::HtmlOptionElement {
    fn from(value: Weapon) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}

// WEP_UP

impl FieldArray for WeaponUpgrade {
    const ALL: &[Self] = &[
        WeaponUpgrade::None,
        WeaponUpgrade::Longspear,
        WeaponUpgrade::Javelin,
        WeaponUpgrade::Quickblade,
        WeaponUpgrade::Greatsword,
        WeaponUpgrade::Maul,
        WeaponUpgrade::BodkinAmmoB,
        WeaponUpgrade::FlamingAmmoB,
        WeaponUpgrade::SeekingAmmoB,
        WeaponUpgrade::TowerShield,
        WeaponUpgrade::LightLance,
        WeaponUpgrade::PoleAxe,
        WeaponUpgrade::BodkinAmmoC,
        WeaponUpgrade::FlamingAmmoC,
        WeaponUpgrade::SeekingAmmoC,
        WeaponUpgrade::Genitour,
        WeaponUpgrade::Longbow,
        WeaponUpgrade::HandCrossbow,
        WeaponUpgrade::HeavyCrossbow,
        WeaponUpgrade::SpikedShield,
        WeaponUpgrade::RecurveBow,
        WeaponUpgrade::SpikedRams,
        WeaponUpgrade::Bolas,
        WeaponUpgrade::Glaive,
    ];
    fn cost(&self) -> u8 {
        match self {
            WeaponUpgrade::None => 0,
            WeaponUpgrade::Longspear => 2,
            WeaponUpgrade::Javelin => 2,
            WeaponUpgrade::Quickblade => 2,
            WeaponUpgrade::Greatsword => 2,
            WeaponUpgrade::Maul => 2,
            WeaponUpgrade::BodkinAmmoB => 2,
            WeaponUpgrade::FlamingAmmoB => 2,
            WeaponUpgrade::SeekingAmmoB => 2,
            WeaponUpgrade::TowerShield => 2,
            WeaponUpgrade::LightLance => 2,
            WeaponUpgrade::PoleAxe => 2,
            WeaponUpgrade::BodkinAmmoC => 2,
            WeaponUpgrade::FlamingAmmoC => 2,
            WeaponUpgrade::SeekingAmmoC => 2,
            WeaponUpgrade::Genitour => 2,
            WeaponUpgrade::Longbow => 2,
            WeaponUpgrade::HandCrossbow => 2,
            WeaponUpgrade::HeavyCrossbow => 2,
            WeaponUpgrade::SpikedShield => 2,
            WeaponUpgrade::RecurveBow => 2,
            WeaponUpgrade::SpikedRams => 2,
            WeaponUpgrade::Bolas => 2,
            WeaponUpgrade::Glaive => 2,
        }
    }
    fn limit(&self) -> usize {
        match self {
            WeaponUpgrade::None => 5,
            _ => 1,
        }
    }
    fn resolve_or_default_slot(form_state: &FormState, valid: Vec<WeaponUpgrade>, slot: usize) -> WeaponUpgrade {
        if valid.contains(&form_state.weapon_upgrades[slot]) { form_state.weapon_upgrades[slot] } else { WeaponUpgrade::default() }
    }
}

impl Named for WeaponUpgrade {
    fn name(&self) -> &str {
        match self {
            WeaponUpgrade::None => "NONE",
            WeaponUpgrade::Longspear => "LONGSPEAR",
            WeaponUpgrade::Javelin => "JAVELIN",
            WeaponUpgrade::Quickblade => "QUICKBLADE",
            WeaponUpgrade::Greatsword => "GREATSWORD",
            WeaponUpgrade::Maul => "MAUL",
            WeaponUpgrade::BodkinAmmoB => "BODKIN_AMMO_B",
            WeaponUpgrade::FlamingAmmoB => "FLAMING_AMMO_B",
            WeaponUpgrade::SeekingAmmoB => "SEEKING_AMMO_B",
            WeaponUpgrade::TowerShield => "TOWER_SHIELD",
            WeaponUpgrade::LightLance => "LIGHT_LANCE",
            WeaponUpgrade::PoleAxe => "POLEAXE",
            WeaponUpgrade::BodkinAmmoC => "BODKIN_AMMO_C",
            WeaponUpgrade::FlamingAmmoC => "FLAMING_AMMO_C",
            WeaponUpgrade::SeekingAmmoC => "SEEKING_AMMO_C",
            WeaponUpgrade::Genitour => "GENITOUR",
            WeaponUpgrade::Longbow => "LONGBOW",
            WeaponUpgrade::HandCrossbow => "HAND_CROSSBOW",
            WeaponUpgrade::HeavyCrossbow => "HEAVY_CROSSBOW",
            WeaponUpgrade::SpikedShield => "SPIKED_SHIELD",
            WeaponUpgrade::RecurveBow => "RECURVE_BOW",
            WeaponUpgrade::SpikedRams => "SPIKED_RAMS",
            WeaponUpgrade::Bolas => "BOLAS",
            WeaponUpgrade::Glaive => "GLAIVE",
        }
    }
}

impl Asset for WeaponUpgrade {}

impl ValidForArray for WeaponUpgrade {
    fn base_for(form_state: &FormState) -> Vec<WeaponUpgrade> {
        match form_state.refit_type {
            RefitType::None => { match form_state.unit_type {
                UnitType::None => vec![WeaponUpgrade::None],
                UnitType::Healers => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
                UnitType::Laborers => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
                UnitType::Mages => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
                UnitType::SiegeCrew => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
                UnitType::Knights => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
                UnitType::Soldiers => vec![WeaponUpgrade::None, WeaponUpgrade::Longspear, WeaponUpgrade::Javelin, WeaponUpgrade::Quickblade, WeaponUpgrade::Greatsword, WeaponUpgrade::Maul, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::TowerShield, WeaponUpgrade::PoleAxe, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
                UnitType::LineBreaker => vec![WeaponUpgrade::None],
                UnitType::Monstrosity => vec![WeaponUpgrade::None],
                UnitType::SkyBeast => vec![WeaponUpgrade::None],
                UnitType::SupplyCart => vec![WeaponUpgrade::None],
            }}
            RefitType::Architect => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::Archmage => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::Artisan => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::ChaosMage => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::PotionMaster => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::Surgeon => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::WarWagon => vec![WeaponUpgrade::None, WeaponUpgrade::SpikedRams],
            RefitType::GroundChariot => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::LightCav => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::MountedArcher => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Paladin => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::SkyKnight => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Sapper => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Cavalier => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::RoughRider => vec![WeaponUpgrade::None, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::LightLance, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Genitour, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Archer => vec![WeaponUpgrade::None, WeaponUpgrade::Longspear, WeaponUpgrade::Javelin, WeaponUpgrade::Quickblade, WeaponUpgrade::Greatsword, WeaponUpgrade::Maul, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::TowerShield, WeaponUpgrade::PoleAxe, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::BattleMage => vec![WeaponUpgrade::None, WeaponUpgrade::Longspear, WeaponUpgrade::Javelin, WeaponUpgrade::Quickblade, WeaponUpgrade::Greatsword, WeaponUpgrade::Maul, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::TowerShield, WeaponUpgrade::PoleAxe, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Champion => vec![WeaponUpgrade::None, WeaponUpgrade::Longspear, WeaponUpgrade::Javelin, WeaponUpgrade::Quickblade, WeaponUpgrade::Greatsword, WeaponUpgrade::Maul, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::TowerShield, WeaponUpgrade::PoleAxe, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::HeavySiege => vec![WeaponUpgrade::None, WeaponUpgrade::Longspear, WeaponUpgrade::Javelin, WeaponUpgrade::Quickblade, WeaponUpgrade::Greatsword, WeaponUpgrade::Maul, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::TowerShield, WeaponUpgrade::PoleAxe, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Infiltrator => vec![WeaponUpgrade::None, WeaponUpgrade::Longspear, WeaponUpgrade::Javelin, WeaponUpgrade::Quickblade, WeaponUpgrade::Greatsword, WeaponUpgrade::Maul, WeaponUpgrade::BodkinAmmoB, WeaponUpgrade::FlamingAmmoB, WeaponUpgrade::SeekingAmmoB, WeaponUpgrade::TowerShield, WeaponUpgrade::PoleAxe, WeaponUpgrade::BodkinAmmoC, WeaponUpgrade::FlamingAmmoC, WeaponUpgrade::SeekingAmmoC, WeaponUpgrade::Longbow, WeaponUpgrade::HandCrossbow, WeaponUpgrade::HeavyCrossbow, WeaponUpgrade::SpikedShield, WeaponUpgrade::RecurveBow, WeaponUpgrade::SpikedRams, WeaponUpgrade::Bolas, WeaponUpgrade::Glaive],
            RefitType::Brute => vec![WeaponUpgrade::None],
            RefitType::Caravan => vec![WeaponUpgrade::None],
            RefitType::Dragon => vec![WeaponUpgrade::None],
            RefitType::Firehawk => vec![WeaponUpgrade::None],
            RefitType::FlatbedCart => vec![WeaponUpgrade::None],
            RefitType::Prowler => vec![WeaponUpgrade::None],
            RefitType::SiegeBeast => vec![WeaponUpgrade::None],
            RefitType::SkyChariot => vec![WeaponUpgrade::None],
            RefitType::Trailblazer => vec![WeaponUpgrade::None],
            RefitType::Wrangler => vec![WeaponUpgrade::None],
            
        }
    }
    fn valid_for(form_state: &FormState) -> [Vec<WeaponUpgrade>; 5] {
        let base = Self::base_for(form_state);
        std::array::from_fn(|i| {
            base.iter()
            .copied()
            .filter(|weapon_up| form_state.count_other_weapon_upgrades(i, *weapon_up) < weapon_up.limit())
            .filter(|w| w.required_weapon().is_none_or(|req| form_state.weapons.contains(&req)))
            .collect()
        })
    }
}

impl WeaponUpgrade {
    fn required_weapon(&self) -> Option<Weapon> {
        match self {
            WeaponUpgrade::None => None,
            WeaponUpgrade::Longspear => Some(Weapon::Spear),
            WeaponUpgrade::Javelin => Some(Weapon::Spear),
            WeaponUpgrade::Quickblade => Some(Weapon::Sword),
            WeaponUpgrade::Greatsword => Some(Weapon::Sword),
            WeaponUpgrade::Maul => Some(Weapon::Hammer),
            WeaponUpgrade::Bolas => Some(Weapon::Hammer),
            WeaponUpgrade::BodkinAmmoB => Some(Weapon::Bow),
            WeaponUpgrade::FlamingAmmoB => Some(Weapon::Bow),
            WeaponUpgrade::SeekingAmmoB => Some(Weapon::Bow),
            WeaponUpgrade::Longbow => Some(Weapon::Bow),
            WeaponUpgrade::RecurveBow => Some(Weapon::Bow),
            WeaponUpgrade::TowerShield => Some(Weapon::Shield),
            WeaponUpgrade::SpikedShield => Some(Weapon::Shield),
            WeaponUpgrade::LightLance => Some(Weapon::Lance),
            WeaponUpgrade::Genitour => Some(Weapon::Lance),
            WeaponUpgrade::PoleAxe => Some(Weapon::Axe),
            WeaponUpgrade::Glaive => Some(Weapon::Axe),
            WeaponUpgrade::BodkinAmmoC => Some(Weapon::Crossbow),
            WeaponUpgrade::FlamingAmmoC => Some(Weapon::Crossbow),
            WeaponUpgrade::SeekingAmmoC => Some(Weapon::Crossbow),
            WeaponUpgrade::HandCrossbow => Some(Weapon::Crossbow),
            WeaponUpgrade::HeavyCrossbow => Some(Weapon::Crossbow),
            WeaponUpgrade::SpikedRams => Some(Weapon::Ram),
        }
    }
}

impl From<WeaponUpgrade> for u8 {
    fn from(value: WeaponUpgrade) -> Self {
        value as u8
    }
}

impl From<u8> for WeaponUpgrade {
    fn from(value: u8) -> Self {
        match value {
            0 => WeaponUpgrade::None,
            1 => WeaponUpgrade::Longspear,
            2 => WeaponUpgrade::Javelin,
            3 => WeaponUpgrade::Quickblade,
            4 => WeaponUpgrade::Greatsword,
            5 => WeaponUpgrade::Maul,
            6 => WeaponUpgrade::BodkinAmmoB,
            7 => WeaponUpgrade::FlamingAmmoB,
            8 => WeaponUpgrade::SeekingAmmoB,
            9 => WeaponUpgrade::TowerShield,
            10 => WeaponUpgrade::LightLance,
            11 => WeaponUpgrade::PoleAxe,
            12 => WeaponUpgrade::BodkinAmmoC,
            13 => WeaponUpgrade::FlamingAmmoC,
            14 => WeaponUpgrade::SeekingAmmoC,
            15 => WeaponUpgrade::Genitour,
            16 => WeaponUpgrade::Longbow,
            17 => WeaponUpgrade::HandCrossbow,
            18 => WeaponUpgrade::HeavyCrossbow,
            19 => WeaponUpgrade::SpikedShield,
            20 => WeaponUpgrade::RecurveBow,
            21 => WeaponUpgrade::SpikedRams,
            22 => WeaponUpgrade::Bolas,
            23 => WeaponUpgrade::Glaive,
            _ => WeaponUpgrade::None,
        }
    }
}

impl From<WeaponUpgrade> for web_sys::HtmlOptionElement {
    fn from(value: WeaponUpgrade) -> Self {
        let option = web_sys::HtmlOptionElement::new().unwrap();
        option.set_value(&(value as u8).to_string());
        option.set_text(value.name());
        option
    }
}