use ini::Ini;

pub(crate) fn create_ini() {
    let mut conf = Ini::new();

    // everything related to the bot configuration itself
    conf.with_section(Some("Configuration"))
        .set("FarmStage", "5")
        .set("Clients", "Lunar Tempest,Shãrk")
        .set("CameraFullTurnPixels", "3174")
        .set("AnimationSpeedHackValue", "8")
        .set("SlowAnimationSpeedHackValue", "5.5");

    // all hotkeys, to find the used keys check
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    conf.with_section(Some("Hotkeys"))
        .set("MapTransparency", "0x4E")
        .set("GetIntoCombat", "0x5A")
        .set("ToggleAutoCombat", "0xA0,0x73")
        .set("CheatEngineSpeedHackOn", "0x67")
        .set("CheatEngineSpeedHackOff", "0x63")
        .set("AnimationSpeedHackOn", "0x66")
        .set("SlowAnimationSpeedHackOn", "0x81")
        .set("AnimationSpeedHackOff", "0x63")
        .set("AnimationSpeedHackWarlockOn", "0x84")
        .set("AnimationSpeedHackWarlockOff", "0x63")
        .set("FlyHackBoss1", "0x68")
        .set("FlyHackBoss2", "0x69")
        .set("DisableFlyHack", "0x64")
        .set("ShadowPlay", "0x12,0x79");

    // user interface settings related to the lobby
    conf.with_section(Some("UserInterfaceLobby"))
        .set("PositionClickDungeon", "1561,666")
        .set("PositionStageRightSide", "1747,478")
        .set("PositionStageLeftSide", "1710,478")
        .set("PositionReady", "962,1037")
        .set("PositionChat", "160,883")
        .set("PositionEnter", "1026,1037")
        .set("PositionDuoReady", "984,119")
        .set("DuoReady", "0x38D454")
        .set("PositionMemberInLobby", "965,120")
        .set("MemberInLobby", "0xD4B449")
        .set("PositionIsReady", "940,1036")
        .set("IsReady", "0x898989,0xFFFFFF")
        .set("PositionInF8Lobby", "23,34")
        .set("InF8Lobby", "0xCECECF");

    // user interface settings related to the party management
    conf.with_section(Some("UserInterfaceParty"))
        .set("PositionExit", "1770,870")
        .set("PositionLeaveParty", "321,78");

    // user interface settings related to the dungeon elements
    conf.with_section(Some("UserInterfaceDungeon"))
        .set("PositionPortalIcon", "1152,715")
        .set("PortalIcon", "0xFEAA00")
        .set("PositionExitPortalIcon", "1148,724")
        .set("ExitPortalIcon", "0xFFE10A")
        .set("PositionDynamicQuest", "1590,718")
        .set("DynamicQuest", "0xE38658")
        .set("PositionDynamicReward", "1628,685")
        .set("DynamicReward", "0x463E2C")
        .set("PositionBonusRewardSelection", "1614,676")
        .set("DynamicReward", "0xBCA664");

    // user interface settings related to the player interactions
    conf.with_section(Some("UserInterfacePlayer"))
        .set("PositionReviveVisible", "1042,900")
        .set("ReviveVisible", "0x9B8D71")
        .set("PositionThrallReady", "825,900")
        .set("ThrallReady", "0x01040E")
        .set("PositionOutOfCombat", "841,837")
        .set("OutOfCombat", "0xA0B930");

    // user interface settings related to the map and camera
    conf.with_section(Some("UserInterfacePlayerCamera"))
        .set("PositionMouseOverMap", "1651,251")
        .set("PositionTrackingMapIcon", "1891,51")
        .set("PositionMapOpaque", "1892,278")
        .set("MapOpaque", "0x98896B")
        .set("PositionMapFixpoint", "1512,169")
        .set("MapFixpoint", "0x6E7A60");

    conf.write_to_file("configuration/poharan.ini").unwrap();
}