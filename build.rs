fn main() {
    slint_build::compile("ui/fileselector.slint").unwrap();
    slint_build::compile("ui/appwindow.slint").unwrap();
}