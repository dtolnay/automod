mod dir_mods;
mod file_mods;
mod mixed_mod_types;

#[test]
fn dir_mods_test() {
    assert!(dir_mods::a::mod_test());
    assert!(dir_mods::b::mod_test());
    assert!(dir_mods::c::mod_test());
}
#[test]
fn file_mods_test() {
    assert!(file_mods::a::mod_test());
    assert!(file_mods::b::mod_test());
    assert!(file_mods::c::mod_test());
}
#[test]
fn mixed_mod_types_test() {
    assert!(mixed_mod_types::dir_mod::mod_test());
    assert!(mixed_mod_types::file_mod::mod_test());
}
