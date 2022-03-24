use ini::Ini;

pub(crate) fn create_ini() {
    println!("creating file");
    let mut conf = Ini::new();
    conf.with_section(Some("UserInterface"))
        .set("ClickExitX", "1770")
        .set("ClickExitY", "870");
    conf.write_to_file("configuration/poharan.ini").unwrap();
}