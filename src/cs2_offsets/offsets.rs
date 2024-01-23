/*
 * All offsets dumped with: https://frezzyhook.com
 * Mon, 22 Jan 2024 17:40:39 +0000
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod client_dll { // client.dll
    pub const dwEntityList: usize = 0x17CE6A0;
    pub const dwForceBackward: usize = 0x16CE150;
    pub const dwForceCrouch: usize = 0x16CE420;
    pub const dwForceForward: usize = 0x16CE0C0;
    pub const dwForceJump: usize = 0x16CE390;
    pub const dwForceLeft: usize = 0x16CE1E0;
    pub const dwForceRight: usize = 0x16CE270;
    pub const dwGameEntitySystem: usize = 0x18FEDD0;
    pub const dwGameEntitySystem_getHighestEntityIndex: usize = 0x1510;
    pub const dwGameRules: usize = 0x182AD98;
    pub const dwGlobalVars: usize = 0x16C9CA8;
    pub const dwGlowManager: usize = 0x182A500;
    pub const dwInterfaceLinkList: usize = 0x192CA68;
    pub const dwLocalPlayerController: usize = 0x181DC98;
    pub const dwLocalPlayerPawn: usize = 0x16D4F48;
    pub const dwPlantedC4: usize = 0x18317D8;
    pub const dwPrediction: usize = 0x16D4E10;
    pub const dwSensitivity: usize = 0x182BA98;
    pub const dwSensitivity_sensitivity: usize = 0x40;
    pub const dwViewAngles: usize = 0x188E140;
    pub const dwViewMatrix: usize = 0x182CEA0;
    pub const dwViewRender: usize = 0x182D6E8;
}

pub mod engine2_dll { // engine2.dll
    pub const dwBuildNumber: usize = 0x4E13D4;
    pub const dwNetworkGameClient: usize = 0x4E0988;
    pub const dwNetworkGameClient_getLocalPlayer: usize = 0xF0;
    pub const dwNetworkGameClient_maxClients: usize = 0x250;
    pub const dwNetworkGameClient_signOnState: usize = 0x240;
    pub const dwWindowHeight: usize = 0x597E0C;
    pub const dwWindowWidth: usize = 0x597E08;
}

pub mod game_info { // Some additional information about the game at dump time
    pub const buildNumber: usize = 0x36A1; // Game build number
}

pub mod inputsystem_dll { // inputsystem.dll
    pub const dwInputSystem: usize = 0x35760;
}