use ini::Ini;

pub(crate) fn create_ini() {
    let mut conf = Ini::new();

    // everything related to the bot configuration itself
    conf.with_section(Some("Configuration"))
        .set("FarmStage", "41")
        .set("AnimationSpeedHackValue", "5.5")
        .set("LogFile", "aerodrome_exp.log");

    conf.with_section(Some("Pointers"))
        .set("BaseAddressPlayer", "0x07537B40")
        .set("OffsetsAnimationSpeed", "0x30,0x2C0,0x98")
        .set("OffsetsCameraYaw", "");

    // all hotkeys, to find the used keys check
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    conf.with_section(Some("Hotkeys"))
        .set("UseSoup", "0x35")
        .set("UseExpCharm", "0x36")
        .set("UseRepairTools", "0x37")
        .set("CcDummies", "0x5A")
        .set("ToggleAutoCombat", "0xA0,0x73")
        .set("CheatEngineSpeedHackOn", "0x91")
        .set("CheatEngineSpeedHackOff", "0x63")
        .set("ShadowPlay", "0x12,0x79");

    // user interface settings related to the lobby
    conf.with_section(Some("UserInterfaceLobby"))
        .set("PositionDungeonSelected", "1686,317")
        .set("DungeonSelected", "0x4C3C23")
        .set("PositionClickDungeon", "1555,813")
        .set("PositionStageSelected", "1732,485")
        .set("StageSelected", "0xE2E2E2")
        .set("PositionStageRightSide", "1747,478")
        .set("PositionEnter", "990,1036")
        .set("Enter", "0xFFFFFF")
        .set("PositionInF8Lobby", "23,34")
        .set("InF8Lobby", "0xCECECF");

    // user interface settings related to the party management
    conf.with_section(Some("UserInterfaceGeneral"))
        .set("PositionOutOfLoadingScreen", "811,794")
        .set("OutOfLoadingScreen", "0xED5E11")
        .set("PositionLoadingScreen", "1861,1062")
        .set("LoadingScreen", "0x090303")
        .set("PositionSoupActive", "20,6;55,7")
        .set("SoupActive", "0x553A54,0x85693B")
        .set("PositionExpCharmActive", "20,6;55,7")
        .set("ExpCharmActive", "0x4D2C1A,0x9E6135")
        .set("PositionEscape", "1520,842");

    // user interface settings related to the player interactions
    conf.with_section(Some("UserInterfacePlayer"))
        .set("PositionReviveVisible", "1042,900")
        .set("ReviveVisible", "0x9B8D71")
        .set("PositionOutOfCombat", "841,837")
        .set("OutOfCombat", "0xA0B930");

    conf.write_to_file("configuration/aerodrome-exp.ini").unwrap();
}