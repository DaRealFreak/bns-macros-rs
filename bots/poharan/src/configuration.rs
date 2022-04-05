use ini::Ini;

pub(crate) fn create_ini() {
    let mut conf = Ini::new();

    // everything related to the bot configuration itself
    conf.with_section(Some("Configuration"))
        .set("FarmStage", "5")
        .set("Clients", "Lunar Tempest")
        .set("AnimationSpeedHackValue", "8")
        .set("SlowAnimationSpeedHackValue", "5.5")
        .set("CameraFullTurnPixels", "3174")
        .set("LogFile", "poharan_multibox.log");

    // all hotkeys, to find the used keys check
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    conf.with_section(Some("Hotkeys"))
        .set("MapTransparency", "0x4E")
        .set("GetIntoCombat", "0x5A")
        .set("ToggleAutoCombat", "0xA0,0x73")
        .set("CheatEngineSpeedHackOn", "0x91")
        .set("CheatEngineSpeedHackOff", "0x63")
        .set("AnimationSpeedHackOn", "0x66")
        .set("SlowAnimationSpeedHackOn", "0x81")
        .set("AnimationSpeedHackOff", "0x63")
        .set("AnimationSpeedHackWarlockOn", "0x84")
        .set("AnimationSpeedHackWarlockOff", "0x63")
        .set("FlyHackBoss1", "0x68")
        .set("FlyHackBoss2", "0x69")
        .set("DisableFlyHack", "0x64")
        .set("ShadowPlay", "0x12,0x79")
        .set("TurnCameraTo0Degrees", "0x7D")
        .set("TurnCameraTo90Degrees", "0x7E")
        .set("TurnCameraTo270Degrees", "0x7F");

    // user interface settings related to the lobby
    conf.with_section(Some("UserInterfaceLobby"))
        .set("PositionDungeonSelected", "1686,317")
        .set("DungeonSelected", "0x3E261C")
        .set("PositionClickDungeon", "1561,666")
        .set("PositionStageSelected", "1732,485")
        .set("StageSelected", "0xE2E2E2")
        .set("PositionStageRightSide", "1747,478")
        .set("PositionStageLeftSide", "1710,478")
        .set("PositionReady", "962,1037")
        .set("PositionChat", "160,883")
        .set("PositionEnter", "990,1036")
        .set("Enter", "0xFFFFFF")
        .set("PositionHasInvite", "1265,964")
        .set("HasInvite", "0xFFFFFF")
        .set("PositionHasPartyJoinRequest", "886,588")
        .set("HasPartyJoinRequest", "0xFFFFFF")
        .set("PositionIsReady", "940,1036")
        .set("IsReady", "0x898989,0xFFFFFF")
        .set("PositionInF8Lobby", "23,34")
        .set("InF8Lobby", "0xCECECF");

    // user interface settings related to the party management
    conf.with_section(Some("UserInterfaceGeneral"))
        .set("PositionOutOfLoadingScreen", "811,794")
        .set("OutOfLoadingScreen", "0xED5E11")
        .set("PositionLoadingScreen", "1861,1062")
        .set("LoadingScreen", "0x090303")
        .set("PositionExit", "1770,870")
        .set("PositionLeaveParty", "321,78");

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

    // user interface settings related to the party management
    conf.with_section(Some("UserInterfaceCamera"))
        .set("PositionOverMap", "1725,283")
        .set("PositionTrackingMap", "1889,54")
        .set("TrackingMap", "0xAEA698,0x6F778F")
        .set("PositionMapNotTransparent", "1892,278")
        .set("MapNotTransparent", "0x98896B")
        .set("PositionCrossServerLobby", "1705,299")
        .set("CrossServerLobby", "0xD53C17")
        .set("PositionMap0Degrees", "1725,283")
        .set("Map0Degrees", "0xED5E11");

    conf.write_to_file("configuration/poharan.ini").unwrap();
}