fn main() {
    // Linkerskript von esp-hal (Speicherlayout, Startcode)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
