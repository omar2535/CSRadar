use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Offsets {
    pub build_number: usize,
    pub dwBuildNumber: usize,
    pub dwEntityList: usize,
    pub dwLocalPlayer: usize,
    pub dwPlantedC4: usize,
    pub dwViewMatrix: usize,
    pub m_ArmorValue: usize,
    pub m_bIsDefusing: usize,
    pub m_flC4Blow: usize,
    pub m_flFlashOverlayAlpha: usize,
    pub m_flNextBeep: usize,
    pub m_flTimerLength: usize,
    pub m_hPlayerPawn: usize,
    pub m_iAccount: usize,
    pub m_iHealth: usize,
    pub m_iTeamNum: usize,
    pub m_pClippingWeapon: usize,
    pub m_pGameSceneNode: usize,
    pub m_pInGameMoneyServices: usize,
    pub m_sSanitizedPlayerName: usize,
    pub m_szName: usize,
    pub m_vOldOrigin: usize,
    pub m_vecAbsOrigin: usize,
    pub dwLocalPlayerController: usize
}

// gets the offset from the JSON offset file path
pub fn get_offsets(offset_path: &str) -> Offsets {
    let file = File::open(offset_path).unwrap();
    let reader = BufReader::new(file);
    let offsets: Offsets = serde_json::from_reader(reader).unwrap();
    return offsets;
}
