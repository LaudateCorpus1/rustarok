use std::collections::HashMap;
use encoding;
use encoding::types::Encoding;
use encoding::{DecoderTrap};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum JobId {
    NOVICE = 0,
    SWORDMAN = 1,
    MAGICIAN = 2,
    ARCHER = 3,
    ACOLYTE = 4,
    MERCHANT = 5,
    THIEF = 6,
    KNIGHT = 7,
    PRIEST = 8,
    WIZARD = 9,
    BLACKSMITH = 10,
    HUNTER = 11,
    ASSASSIN = 12,
    KNIGHT2 = 13,
    CRUSADER = 14,
    MONK = 15,
    SAGE = 16,
    ROGUE = 17,
    ALCHEMIST = 18,
    BARD = 19,
    DANCER = 20,
    CRUSADER2 = 21,
    MARRIED = 22,
    SUPERNOVICE = 23,
    GUNSLINGER = 24,
    NINJA = 25,
    XMAS = 26,
    SUMMER = 27,
    NOVICE_H = 4001,
    SWORDMAN_H = 4002,
    MAGICIAN_H = 4003,
    ARCHER_H = 4004,
    ACOLYTE_H = 4005,
    MERCHANT_H = 4006,
    THIEF_H = 4007,
    KNIGHT_H = 4008,
    PRIEST_H = 4009,
    WIZARD_H = 4010,
    BLACKSMITH_H = 4011,
    HUNTER_H = 4012,
    ASSASSIN_H = 4013,
    KNIGHT2_H = 4014,
    CRUSADER_H = 4015,
    MONK_H = 4016,
    SAGE_H = 4017,
    ROGUE_H = 4018,
    ALCHEMIST_H = 4019,
    BARD_H = 4020,
    DANCER_H = 4021,
    CRUSADER2_H = 4022,
    NOVICE_B = 4023,
    SWORDMAN_B = 4024,
    MAGICIAN_B = 4025,
    ARCHER_B = 4026,
    ACOLYTE_B = 4027,
    MERCHANT_B = 4028,
    THIEF_B = 4029,
    KNIGHT_B = 4030,
    PRIEST_B = 4031,
    WIZARD_B = 4032,
    BLACKSMITH_B = 4033,
    HUNTER_B = 4034,
    ASSASSIN_B = 4035,
    KNIGHT2_B = 4036,
    CRUSADER_B = 4037,
    MONK_B = 4038,
    SAGE_B = 4039,
    ROGUE_B = 4040,
    ALCHEMIST_B = 4041,
    BARD_B = 4042,
    DANCER_B = 4043,
    CRUSADER2_B = 4044,
    SUPERNOVICE_B = 4045,
    TAEKWON = 4046,
    STAR = 4047,
    STAR2 = 4048,
    LINKER = 4049,
    /*
    not used yet=
    Job_Gangsi	4050
    Job_Death_Knight	4051
    Job_Dark_Collector	4052
    */
    RUNE_KNIGHT = 4054,
    WARLOCK = 4055,
    RANGER = 4056,
    ARCHBISHOP = 4057,
    MECHANIC = 4058,
    GUILLOTINE_CROSS = 4059,
    RUNE_KNIGHT_H = 4060,
    WARLOCK_H = 4061,
    RANGER_H = 4062,
    ARCHBISHOP_H = 4063,
    MECHANIC_H = 4064,
    GUILLOTINE_CROSS_H = 4065,
    ROYAL_GUARD = 4066,
    SORCERER = 4067,
    MINSTREL = 4068,
    WANDERER = 4069,
    SURA = 4070,
    GENETIC = 4071,
    SHADOW_CHASER = 4072,
    ROYAL_GUARD_H = 4073,
    SORCERER_H = 4074,
    MINSTREL_H = 4075,
    WANDERER_H = 4076,
    SURA_H = 4077,
    GENETIC_H = 4078,
    SHADOW_CHASER_H = 4079,
    RUNE_KNIGHT2 = 4080,
    RUNE_KNIGHT2_H = 4081,
    ROYAL_GUARD2 = 4082,
    ROYAL_GUARD2_H = 4083,
    RANGER2 = 4084,
    RANGER2_H = 4085,
    MECHANIC2 = 4086,
    MECHANIC2_H = 4087,

    RUNE_KNIGHT_B = 4096,
    WARLOCK_B = 4097,
    RANGER_B = 4098,
    ARCHBISHOP_B = 4099,
    MECHANIC_B = 4100,
    GUILLOTINE_CROSS_B = 4101,
    ROYAL_GUARD_B = 4102,
    SORCERER_B = 4103,
    MINSTREL_B = 4104,
    WANDERER_B = 4105,
    SURA_B = 4106,
    GENETIC_B = 4107,
    SHADOW_CHASER_B = 4108,
    RUNE_KNIGHT2_B = 4109,
    ROYAL_GUARD2_B = 4110,
    RANGER2_B = 4111,
    MECHANIC2_B = 4112,
    // 4113 ?
    FROG_NINJA = 4114,
    PECO_GUNNER = 4115,
    PECO_SWORD = 4116,
    // 4117 ?
    PIG_WHITESMITH = 4118,
    PIG_MERCHANT = 4119,
    PIG_GENETIC = 4120,
    PIG_CREATOR = 4121,
    OSTRICH_ARCHER = 4122,
    PORING_STAR = 4123,
    PORING_NOVICE = 4124,
    SHEEP_MONK = 4125,
    SHEEP_ACO = 4126,
    SHEEP_SURA = 4127,
    PORING_SNOVICE = 4128,
    SHEEP_ARCB = 4129,
    FOX_MAGICIAN = 4130,
    FOX_SAGE = 4131,
    FOX_SORCERER = 4132,
    FOX_WARLOCK = 4133,
    FOX_WIZ = 4134,
    // 4135 ?
    FOX_HWIZ = 4136,
    PIG_ALCHE = 4137,
    PIG_BLACKSMITH = 4138,
    SHEEP_CHAMP = 4139,
    DOG_G_CROSS = 4140,
    DOG_THIEF = 4141,
    DOG_ROGUE = 4142,
    DOG_CHASER = 4143,
    DOG_STALKER = 4144,
    DOG_ASSASSIN = 4145,
    DOG_ASSA_X = 4146,
    OSTRICH_DANCER = 4147,
    OSTRICH_MINSTREL = 4148,
    OSTRICH_BARD = 4149,
    OSTRICH_SNIPER = 4150,
    OSTRICH_WANDER = 4151,
    OSTRICH_ZIPSI = 4152,
    OSTRICH_CROWN = 4153,
    OSTRICH_HUNTER = 4154,
    PORING_TAEKWON = 4155,
    SHEEP_PRIEST = 4156,
    SHEEP_HPRIEST = 4157,
    PORING_NOVICE_B = 4158,
    // 4159 ?
    FOX_MAGICIAN_B = 4160,
    OSTRICH_ARCHER_B = 4161,
    SHEEP_ACO_B = 4162,
    PIG_MERCHANT_B = 4163,
    OSTRICH_HUNTER_B = 4164,
    DOG_ASSASSIN_B = 4165,
    SHEEP_MONK_B = 4166,
    FOX_SAGE_B = 4167,
    DOG_ROGUE_B = 4168,
    PIG_ALCHE_B = 4169,
    OSTRICH_BARD_B = 4170,
    OSTRICH_DANCER_B = 4171,
    PORING_SNOVICE_B = 4172,
    FOX_WARLOCK_B = 4173,
    SHEEP_ARCB_B = 4174,
    DOG_G_CROSS_B = 4175,
    FOX_SORCERER_B = 4176,
    OSTRICH_MINSTREL_B = 4177,
    OSTRICH_WANDER_B = 4178,
    SHEEP_SURA_B = 4179,
    PIG_GENETIC_B = 4180,
    DOG_THIEF_B = 4181,
    DOG_CHASER_B = 4182,
    PORING_NOVICE_H = 4183,
    // 4184 ?
    FOX_MAGICIAN_H = 4185,
    OSTRICH_ARCHER_H = 4186,
    SHEEP_ACO_H = 4187,
    PIG_MERCHANT_H = 4188,
    DOG_THIEF_H = 4189,
    SUPERNOVICE2 = 4190,
    SUPERNOVICE2_B = 4191,
    PORING_SNOVICE2 = 4192,
    PORING_SNOVICE2_B = 4193,
    SHEEP_PRIEST_B = 4194,
    FOX_WIZ_B = 4195,
    PIG_BLACKSMITH_B = 4196,

    KAGEROU = 4211,
    OBORO = 4212,
    FROG_KAGEROU = 4213,
    FROG_OBORO = 4214,
}

pub fn job_name_table() -> HashMap<JobId, String> {
    let mut table = HashMap::new();

    table.insert(JobId::NOVICE, encoding::all::WINDOWS_1252.decode(&[0xC3, 0xCA, 0xBA, 0xB8, 0xC0, 0xDA], DecoderTrap::Strict).unwrap());


    table.insert(JobId::SWORDMAN, encoding::all::WINDOWS_1252.decode(&[0xB0, 0xCB, 0xBB, 0xE7], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MAGICIAN, encoding::all::WINDOWS_1252.decode(&[0xB8, 0xB6, 0xB9, 0xFD, 0xBB, 0xE7], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ARCHER, encoding::all::WINDOWS_1252.decode(&[0xB1, 0xC3, 0xBC, 0xF6], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ACOLYTE, encoding::all::WINDOWS_1252.decode(&[0xBC, 0xBA, 0xC1, 0xF7, 0xC0, 0xDA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MERCHANT, encoding::all::WINDOWS_1252.decode(&[0xBB, 0xF3, 0xC0, 0xCE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::THIEF, encoding::all::WINDOWS_1252.decode(&[0xB5, 0xB5, 0xB5, 0xCF], DecoderTrap::Strict).unwrap());

    table.insert(JobId::KNIGHT, encoding::all::WINDOWS_1252.decode(&[0xB1, 0xE2, 0xBB, 0xE7], DecoderTrap::Strict).unwrap());
    table.insert(JobId::PRIEST, encoding::all::WINDOWS_1252.decode(&[0xC7, 0xC1, 0xB8, 0xAE, 0xBD, 0xBA, 0xC6, 0xAE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::WIZARD, encoding::all::WINDOWS_1252.decode(&[0xC0, 0xA7, 0xC0, 0xFA, 0xB5, 0xE5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::BLACKSMITH, encoding::all::WINDOWS_1252.decode(&[0xC1, 0xA6, 0xC3, 0xB6, 0xB0, 0xF8], DecoderTrap::Strict).unwrap());
    table.insert(JobId::HUNTER, encoding::all::WINDOWS_1252.decode(&[0xC7, 0xE5, 0xC5, 0xCD], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ASSASSIN, encoding::all::WINDOWS_1252.decode(&[0xBE, 0xEE, 0xBC, 0xBC, 0xBD, 0xC5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::KNIGHT2, encoding::all::WINDOWS_1252.decode(&[0xC6, 0xE4, 0xC4, 0xDA, 0xC6, 0xE4, 0xC4, 0xDA, 0x5f, 0xB1, 0xE2, 0xBB, 0xE7], DecoderTrap::Strict).unwrap());

    table.insert(JobId::CRUSADER, encoding::all::WINDOWS_1252.decode(&[0xC5, 0xA9, 0xB7, 0xE7, 0xBC, 0xBC, 0xC0, 0xCC, 0xB4, 0xF5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MONK, encoding::all::WINDOWS_1252.decode(&[0xB8, 0xF9, 0xC5, 0xA9], DecoderTrap::Strict).unwrap());
    table.insert(JobId::SAGE, encoding::all::WINDOWS_1252.decode(&[0xBC, 0xBC, 0xC0, 0xCC, 0xC1, 0xF6], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ROGUE, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xCE, 0xB1, 0xD7], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ALCHEMIST, encoding::all::WINDOWS_1252.decode(&[0xBF, 0xAC, 0xB1, 0xDD, 0xBC, 0xFA, 0xBB, 0xE7], DecoderTrap::Strict).unwrap());
    table.insert(JobId::BARD, encoding::all::WINDOWS_1252.decode(&[0xB9, 0xD9, 0xB5, 0xE5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::DANCER, encoding::all::WINDOWS_1252.decode(&[0xB9, 0xAB, 0xC8, 0xF1], DecoderTrap::Strict).unwrap());
    table.insert(JobId::CRUSADER2, encoding::all::WINDOWS_1252.decode(&[0xBD, 0xC5, 0xC6, 0xE4, 0xC4, 0xDA, 0xC5, 0xA9, 0xB7, 0xE7, 0xBC, 0xBC, 0xC0, 0xCC, 0xB4, 0xF5], DecoderTrap::Strict).unwrap());

    table.insert(JobId::SUPERNOVICE, encoding::all::WINDOWS_1252.decode(&[0xBD, 0xB4, 0xC6, 0xDB, 0xB3, 0xEB, 0xBA, 0xF1, 0xBD, 0xBA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::GUNSLINGER, encoding::all::WINDOWS_1252.decode(&[0xB0, 0xC7, 0xB3, 0xCA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::NINJA, encoding::all::WINDOWS_1252.decode(&[0xB4, 0xD1, 0xC0, 0xDA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::TAEKWON, encoding::all::WINDOWS_1252.decode(&[0xc5, 0xc2, 0xb1, 0xc7, 0xbc, 0xd2, 0xb3, 0xe2], DecoderTrap::Strict).unwrap());
    table.insert(JobId::STAR, encoding::all::WINDOWS_1252.decode(&[0xb1, 0xc7, 0xbc, 0xba], DecoderTrap::Strict).unwrap());
    table.insert(JobId::STAR2, encoding::all::WINDOWS_1252.decode(&[0xb1, 0xc7, 0xbc, 0xba, 0xc0, 0xb6, 0xc7, 0xd5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::LINKER, encoding::all::WINDOWS_1252.decode(&[0xbc, 0xd2, 0xbf, 0xef, 0xb8, 0xb5, 0xc4, 0xbf], DecoderTrap::Strict).unwrap());

    table.insert(JobId::MARRIED, encoding::all::WINDOWS_1252.decode(&[0xB0, 0xE1, 0xC8, 0xA5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::XMAS, encoding::all::WINDOWS_1252.decode(&[0xBB, 0xEA, 0xC5, 0xB8], DecoderTrap::Strict).unwrap());
    table.insert(JobId::SUMMER, encoding::all::WINDOWS_1252.decode(&[0xBF, 0xA9, 0xB8, 0xA7], DecoderTrap::Strict).unwrap());

    table.insert(JobId::KNIGHT_H, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xCE, 0xB5, 0xE5, 0xB3, 0xAA, 0xC0, 0xCC, 0xC6, 0xAE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::PRIEST_H, encoding::all::WINDOWS_1252.decode(&[0xC7, 0xCF, 0xC0, 0xCC, 0xC7, 0xC1, 0xB8, 0xAE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::WIZARD_H, encoding::all::WINDOWS_1252.decode(&[0xC7, 0xCF, 0xC0, 0xCC, 0xC0, 0xA7, 0xC0, 0xFA, 0xB5, 0xE5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::BLACKSMITH_H, encoding::all::WINDOWS_1252.decode(&[0xC8, 0xAD, 0xC0, 0xCC, 0xC6, 0xAE, 0xBD, 0xBA, 0xB9, 0xCC, 0xBD, 0xBA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::HUNTER_H, encoding::all::WINDOWS_1252.decode(&[0xBD, 0xBA, 0xB3, 0xAA, 0xC0, 0xCC, 0xC6, 0xDB], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ASSASSIN_H, encoding::all::WINDOWS_1252.decode(&[0xBE, 0xEE, 0xBD, 0xD8, 0xBD, 0xC5, 0xC5, 0xA9, 0xB7, 0xCE, 0xBD, 0xBA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::KNIGHT2_H, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xCE, 0xB5, 0xE5, 0xC6, 0xE4, 0xC4, 0xDA], DecoderTrap::Strict).unwrap());
    table.insert(JobId::CRUSADER_H, encoding::all::WINDOWS_1252.decode(&[0xC6, 0xC8, 0xB6, 0xF3, 0xB5, 0xF2], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MONK_H, encoding::all::WINDOWS_1252.decode(&[0xC3, 0xA8, 0xC7, 0xC7, 0xBF, 0xC2], DecoderTrap::Strict).unwrap());
    table.insert(JobId::SAGE_H, encoding::all::WINDOWS_1252.decode(&[0xC7, 0xC1, 0xB7, 0xCE, 0xC6, 0xE4, 0xBC, 0xAD], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ROGUE_H, encoding::all::WINDOWS_1252.decode(&[0xBD, 0xBA, 0xC5, 0xE4, 0xC4, 0xBF], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ALCHEMIST_H, encoding::all::WINDOWS_1252.decode(&[0xC5, 0xA9, 0xB8, 0xAE, 0xBF, 0xA1, 0xC0, 0xCC, 0xC5, 0xCD], DecoderTrap::Strict).unwrap());
    table.insert(JobId::BARD_H, encoding::all::WINDOWS_1252.decode(&[0xC5, 0xAC, 0xB6, 0xF3, 0xBF, 0xEE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::DANCER_H, encoding::all::WINDOWS_1252.decode(&[0xC1, 0xFD, 0xBD, 0xC3], DecoderTrap::Strict).unwrap());
    table.insert(JobId::CRUSADER2_H, encoding::all::WINDOWS_1252.decode(&[0xC6, 0xE4, 0xC4, 0xDA, 0xC6, 0xC8, 0xB6, 0xF3, 0xB5, 0xF2], DecoderTrap::Strict).unwrap());

    table.insert(JobId::RUNE_KNIGHT, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xE9, 0xB3, 0xAA, 0xC0, 0xCC, 0xC6, 0xAE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::WARLOCK, encoding::all::WINDOWS_1252.decode(&[0xBF, 0xF6, 0xB7, 0xCF], DecoderTrap::Strict).unwrap());
    table.insert(JobId::RANGER, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xB9, 0xC0, 0xCE, 0xC1, 0xAE], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ARCHBISHOP, encoding::all::WINDOWS_1252.decode(&[0xBE, 0xC6, 0xC5, 0xA9, 0xBA, 0xF1, 0xBC, 0xF3], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MECHANIC, encoding::all::WINDOWS_1252.decode(&[0xB9, 0xCC, 0xC4, 0xC9, 0xB4, 0xD0], DecoderTrap::Strict).unwrap());
    table.insert(JobId::GUILLOTINE_CROSS, encoding::all::WINDOWS_1252.decode(&[0xB1, 0xE6, 0xB7, 0xCE, 0xC6, 0xBE, 0xC5, 0xA9, 0xB7, 0xCE, 0xBD, 0xBA], DecoderTrap::Strict).unwrap());

    table.insert(JobId::ROYAL_GUARD, encoding::all::WINDOWS_1252.decode(&[0xB0, 0xA1, 0xB5, 0xE5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::SORCERER, encoding::all::WINDOWS_1252.decode(&[0xBC, 0xD2, 0xBC, 0xAD, 0xB7, 0xAF], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MINSTREL, encoding::all::WINDOWS_1252.decode(&[0xB9, 0xCE, 0xBD, 0xBA, 0xC6, 0xAE, 0xB7, 0xB2], DecoderTrap::Strict).unwrap());
    table.insert(JobId::WANDERER, encoding::all::WINDOWS_1252.decode(&[0xBF, 0xF8, 0xB4, 0xF5, 0xB7, 0xAF], DecoderTrap::Strict).unwrap());
    table.insert(JobId::SURA, encoding::all::WINDOWS_1252.decode(&[0xBD, 0xB4, 0xB6, 0xF3], DecoderTrap::Strict).unwrap());
    table.insert(JobId::GENETIC, encoding::all::WINDOWS_1252.decode(&[0xC1, 0xA6, 0xB3, 0xD7, 0xB8, 0xAF], DecoderTrap::Strict).unwrap());
    table.insert(JobId::SHADOW_CHASER, encoding::all::WINDOWS_1252.decode(&[0xBD, 0xA6, 0xB5, 0xB5, 0xBF, 0xEC, 0xC3, 0xBC, 0xC0, 0xCC, 0xBC, 0xAD], DecoderTrap::Strict).unwrap());

    table.insert(JobId::RUNE_KNIGHT2, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xE9, 0xB3, 0xAA, 0xC0, 0xCC, 0xC6, 0xAE, 0xBB, 0xDA, 0xB6, 0xEC], DecoderTrap::Strict).unwrap());
    table.insert(JobId::ROYAL_GUARD2, encoding::all::WINDOWS_1252.decode(&[0xB1, 0xD7, 0xB8, 0xAE, 0xC6, 0xF9, 0xB0, 0xA1, 0xB5, 0xE5], DecoderTrap::Strict).unwrap());
    table.insert(JobId::RANGER2, encoding::all::WINDOWS_1252.decode(&[0xB7, 0xB9, 0xC0, 0xCE, 0xC1, 0xAE, 0xB4, 0xC1, 0xB4, 0xEB], DecoderTrap::Strict).unwrap());
    table.insert(JobId::MECHANIC2, encoding::all::WINDOWS_1252.decode(&[0xB8, 0xB6, 0xB5, 0xB5, 0xB1, 0xE2, 0xBE, 0xEE], DecoderTrap::Strict).unwrap());

    return table;
}