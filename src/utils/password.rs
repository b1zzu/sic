pub fn ask_password() -> String {
    rpassword::read_password_from_tty(Some("Password: ")).unwrap()
}
