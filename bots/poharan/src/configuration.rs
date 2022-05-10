use ini::Ini;

pub(crate) fn create_ini() {
    let mut conf = Ini::new();

    // everything related to the bot configuration itself
    conf.with_section(Some("Configuration"))
        .set("FarmStage", "5")
        .set("AnimationSpeedHackValue", "10")
        .set("SlowAnimationSpeedHackValue", "6")
        .set("LogFile", "poharan_multibox.log");

    conf.with_section(Some("Pointers"))
        .set("BaseAddressPlayer", "0x07536B40")
        .set("OffsetsAnimationSpeed", "0x30,0x2C0,0x98")
        .set("OffsetsCameraYaw", "")
        .set("OffsetsPlayerX", "0x30,0x2C0,0x130,0x1F0")
        .set("OffsetsPlayerY", "0x30,0x2C0,0x130,0x1F4")
        .set("OffsetsPlayerZ", "0x30,0x2C0,0x130,0x1F8")
        .set("BaseAddressDungeon", "0x074B5AF0")
        .set("OffsetsDungeonStage", "0xA0,0x8D8,0x28,0x1D0")
        .set("OffsetsLobbyNumber", "0xA0,0x8D8,0x28,0x20");

    // all hotkeys, to find the used keys check
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    conf.with_section(Some("Hotkeys"))
        .set("AfterTaeJangum", "0x42,0x42")
        .set("ToggleAutoCombat", "0xA0,0x73")
        .set("CheatEngineSpeedHackOn", "0x91")
        .set("CheatEngineSpeedHackOff", "0x63")
        .set("FlyHackBoss1", "0x68")
        .set("FlyHackBoss2", "0x69")
        .set("DisableFlyHack", "0x64")
        .set("ShadowPlay", "0x12,0x79");

    // user interface settings related to the lobby
    conf.with_section(Some("UserInterfaceLobby"))
        .set("PositionInF8Lobby", "23,34")
        .set("InF8Lobby", "0xCECECF")
        .set("PositionFindLobby", "1800,1025")
        .set("PositionHasJoinLobbyDialogue", "985,602")
        .set("HasJoinLobbyDialogue", "0xFFFFFF")
        .set("PositionIsReady", "940,1036")
        .set("IsReady", "0x898989,0xFFFFFF")
        .set("PositionReady", "962,1037")
        .set("PositionDungeonSelected", "1686,317")
        .set("DungeonSelected", "0x3E261C")
        .set("PositionClickDungeon", "1561,666")
        .set("PositionStageSelected", "1732,485")
        .set("StageSelected", "0xE2E2E2")
        .set("PositionStageRightSide", "1747,478")
        .set("PositionEnter", "990,1036")
        .set("Enter", "0xFFFFFF");

    // user interface settings related to the party management
    conf.with_section(Some("UserInterfaceGeneral"))
        .set("PositionOutOfLoadingScreen", "811,794;75,1048")
        .set("OutOfLoadingScreen", "0xED5E11,0x000001")
        .set("PositionLoadingScreen", "1861,1062")
        .set("LoadingScreen", "0x090303")
        .set("PositionExit", "1770,870");

    // user interface settings related to the dungeon elements
    conf.with_section(Some("UserInterfaceDungeon"))
        .set("PositionPortalIcon", "1152,715")
        .set("PortalIcon", "0xFEAA00")
        .set("PositionExitPortalIcon", "1148,724")
        .set("ExitPortalIcon", "0xFFE10A")
        .set("PositionDynamicQuest", "1590,718;1594,968")
        .set("DynamicQuest", "0xE38658,0x58B54C")
        .set("PositionDynamicReward", "1628,685")
        .set("DynamicReward", "0x463E2C")
        .set("PositionBonusRewardSelection", "1614,676")
        .set("BonusRewardSelection", "0xBCA664");

    // user interface settings related to the player interactions
    conf.with_section(Some("UserInterfacePlayer"))
        .set("PositionReviveVisible", "1042,900")
        .set("ReviveVisible", "0x9B8D71")
        .set("PositionThrallReady", "825,900")
        .set("ThrallReady", "0x01040E")
        .set("PositionOutOfCombat", "841,837")
        .set("OutOfCombat", "0xA0B930");

    conf.write_to_file("configuration/poharan.ini").unwrap();
}