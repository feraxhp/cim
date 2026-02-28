fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("wix/cim.ico");
        res.compile().unwrap();
    }
}