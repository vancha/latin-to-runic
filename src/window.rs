use gtk::prelude::*;
use gtk::TextBuffer;
use gtk::TextBufferExt;
use std::io::Write;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        let knop: gtk::TextBuffer = builder
            .get_object("textbuffer1")
            .expect("Failed to find the window object");
        knop.set_text("");
        let viewthing: gtk::TextView = builder
            .get_object("input")
            .expect("Failed to find the window object");
        let fertût: gtk::Label = builder
            .get_object("output")
            .expect("Failed to find the window object");

        let oersetknop: gtk::Button = builder
            .get_object("convert")
            .expect("Failed to find the window object");

        let kloand = viewthing.clone();
        oersetknop.connect_clicked(move |_| {
            let test = gtk::test_text_get(&kloand).unwrap();

            let s = &test
                .to_string()
                .chars()
                .map(|c| match c {
                    'a' => 'ᚪ',
                    'b' => 'ᛒ',
                    'c' => 'ᚳ',
                    'd' => 'ᛞ',
                    'e' => 'ᛖ',
                    'f' => 'ᚠ',
                    'g' => 'ᚷ',
                    'h' => 'ᚻ',
                    'i' => 'ᛁ',
                    'j' => 'ᛡ',
                    'k' => 'ᚳ',
                    'l' => 'ᛚ',
                    'm' => 'ᛗ',
                    'n' => 'ᚾ',
                    'o' => 'ᚩ',
                    'p' => 'ᛈ',
                    'q' => 'ᚳ',
                    'r' => 'ᚱ',
                    's' => 'ᛋ',
                    't' => 'ᛏ',
                    'u' => 'ᚢ',
                    'v' => 'ᚠ',
                    'w' => 'ᚹ',
                    'x' => 'ᚷ',
                    'y' => 'ᛁ',
                    'z' => 'ᛉ',
                    _ => c,
                })
                .collect::<String>(); //<Vec<char>>();
            fertût.set_text(&s);
            let stru = &test.to_string();
            println!("length of vec: {}: {:?}", s.len(), s);
        });

        viewthing.set_size_request(300, 200);

        Self { widget }
    }
}
