use serde::Deserialize;

use crate::cornucopia::types::public::Musickey;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Deserialize, Debug)]
pub enum Sex {
    Male,
    Female,
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::Male => f.write_str("male"),
            Sex::Female => f.write_str("female"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum MusicKey {
    a_minor,
    a_major,
    b_flat_minor,
    b_flat_major,
    b_minor,
    b_major,
    c_minor,
    c_major,
    c_sharp_minor,
    c_sharp_major,
    d_minor,
    d_major,
    e_flat_minor,
    e_flat_major,
    e_minor,
    e_major,
    f_minor,
    f_major,
    f_sharp_minor,
    f_sharp_major,
    g_minor,
    g_major,
    a_flat_minor,
    a_flat_major,
}

impl From<MusicKey> for Musickey {
    fn from(value: MusicKey) -> Self {
        match value {
            MusicKey::a_minor => Musickey::a_minor,
            MusicKey::a_major => Musickey::a_major,
            MusicKey::b_flat_minor => Musickey::b_flat_minor,
            MusicKey::b_flat_major => Musickey::b_flat_major,
            MusicKey::b_minor => Musickey::b_minor,
            MusicKey::b_major => Musickey::b_major,
            MusicKey::c_minor => Musickey::c_minor,
            MusicKey::c_major => Musickey::c_major,
            MusicKey::c_sharp_minor => Musickey::c_sharp_minor,
            MusicKey::c_sharp_major => Musickey::c_sharp_major,
            MusicKey::d_minor => Musickey::d_minor,
            MusicKey::d_major => Musickey::d_major,
            MusicKey::e_flat_minor => Musickey::e_flat_minor,
            MusicKey::e_flat_major => Musickey::e_flat_major,
            MusicKey::e_minor => Musickey::e_minor,
            MusicKey::e_major => Musickey::e_major,
            MusicKey::f_minor => Musickey::f_minor,
            MusicKey::f_major => Musickey::f_major,
            MusicKey::f_sharp_minor => Musickey::f_sharp_minor,
            MusicKey::f_sharp_major => Musickey::f_sharp_major,
            MusicKey::g_minor => Musickey::g_minor,
            MusicKey::g_major => Musickey::g_major,
            MusicKey::a_flat_minor => Musickey::a_flat_minor,
            MusicKey::a_flat_major => Musickey::a_flat_major,
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum SortBy {
    Relevance,
    NewFirst,
    OldFirst,
    Expensive,
    Budget,
    TopWished,
    TopListened,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Relevance => f.write_str("relevance"),
            SortBy::NewFirst => f.write_str("new_first"),
            SortBy::OldFirst => f.write_str("old_first"),
            SortBy::Expensive => f.write_str("expensive"),
            SortBy::Budget => f.write_str("budget"),
            SortBy::TopWished => f.write_str("top_wished"),
            SortBy::TopListened => f.write_str("top_listened"),
        }
    }
}
