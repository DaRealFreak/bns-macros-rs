use ini::Ini;

pub(crate) fn create_ini() {
    let mut conf = Ini::new();
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
        .set("IsReady", "0x898989,0xFFFFFF");
    conf.write_to_file("configuration/poharan.ini").unwrap();
}