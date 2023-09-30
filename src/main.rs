//*****************************************************************************
//            _     _     
//      _   _(_)___| |__  
//     | | | | / __| '_ \ 
//     | |_| | \__ \ | | |
//      \__,_|_|___/_| |_|
//                     
// Copyright © J.-P. Liguori 2023
//          jpl@ozf.fr
//
//*****************************************************************************
//
//*****************************************************************************
// IMPORTS
//*****************************************************************************
use std::env;
use chrono::{DateTime, Local};
use cursive::{view::{Nameable, Resizable, scroll::Scroller}, views::{TextView, TextArea, ScrollView, OnEventView, LinearLayout, ResizedView, NamedView}};
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::Cursive;

//*****************************************************************************
// GLOBALS
//*****************************************************************************
const MAIN_TITLE: &str = ":uish:";
const COPYRIGHT: &str = ":uish: © JPL 2023";
const WELCOME: &str = "Welcome to :uish: - F4 to quit";

//*****************************************************************************
// main()
//*****************************************************************************
pub fn main() {
    let mut c = cursive::default();
    
    let core_console = TextView::new("")
        .with_name("console")
        .full_height();
    
    let scroll_console = ScrollView::new(core_console)
        .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
        .with_name("scroll");

    let linear_layout = LinearLayout::vertical()
        // Title bar
        .child(LinearLayout::horizontal()
            .child(TextView::new("00/00/0000")
                .with_name("date"))
            .child(TextView::new( MAIN_TITLE )
                .h_align(HAlign::Center).full_width()
                .with_name("hour"))
            .child(TextView::new("00:00:00").with_name("time"))
        )
        // Body
        .child(scroll_console)
        // Path
        .child(TextView::new("")
            .with_name("path")
            .full_width()
            .fixed_height(1))
        // Prompt
        .child(OnEventView::new(TextArea::new()
            .with_name("prompt").full_width()
            .fixed_height(3))
            // Here, we want to override the "Enter" key event of this TextArea,
            // so we need to embed this TextArea into an OnEventView and add our customized behavior.
            .on_pre_event(Key::Enter, |c| {
                enter_fn(c);
                scroll_to_end(c);
            })
        )
        // Status bar
        .child(LinearLayout::horizontal()
            .child(TextView::new(WELCOME)
                .with_name("statusbar")
                .full_width())
            .child(TextView::new("0/0")
                .with_name("RC"))
        );

    c.add_fullscreen_layer(linear_layout.full_screen());
    c.add_global_callback(Key::F4, |c| c.quit());

    let cb_sink = c.cb_sink().clone();
    let sleep_duration = std::time::Duration::from_millis(1_000);  // 1 second
    std::thread::spawn(move || loop {
        std::thread::sleep(sleep_duration);
        cb_sink.send(Box::new(|s| {
            display_path(s);
            display_time(s);
            display_date(s);
        })).unwrap();
    });    
 
    c.run();

    println!("{}", COPYRIGHT);
}

//*****************************************************************************
// enter_fn()
//*****************************************************************************
fn enter_fn(c: &mut Cursive) {
    let mut statusbar = c.find_name::<TextView>("statusbar").unwrap();
    let mut prompt = c.find_name::<TextArea>("prompt").unwrap();
    let mut console = c.find_name::<TextView>("console").unwrap();
    let cmd = prompt.get_content();

    if cmd != "" {
        match cmd {
            "quit" | "exit" => c.quit(),
            "cls" => console.set_content(""),
            _ => { 
                console.append(cmd);
                console.append("\n");
            },
        }
        statusbar.set_content(cmd);
    } else {
        statusbar.set_content(WELCOME);
    }
    
    prompt.set_content("");
}

//*****************************************************************************
// scroll_to_end()
//*****************************************************************************
fn scroll_to_end(c: &mut Cursive) {
    c.call_on_name("scroll", |view: &mut NamedView<ScrollView<ResizedView<NamedView<TextView>>>>| {
        view.get_mut().get_scroller_mut().scroll_to_bottom();
        view.get_mut().get_scroller_mut().scroll_down(2);
    });
}

//*****************************************************************************
// display_path()
//*****************************************************************************
fn display_path(c: &mut Cursive) {
    let mut out = c.find_name::<TextView>("path").unwrap();
    let cwd = env::current_dir().unwrap();
    let p: String =String::from(cwd.to_string_lossy());
    out.set_content(p);
}

//*****************************************************************************
// display_time()
//*****************************************************************************
fn display_time(c: &mut Cursive) {
    let mut out = c.find_name::<TextView>("time").unwrap();
    let now: DateTime<Local> = Local::now();
    let p = now.format("%H:%M:%S").to_string();
    out.set_content(p);
}

//*****************************************************************************
// display_date()
//*****************************************************************************
fn display_date(c: &mut Cursive) {
    let mut out = c.find_name::<TextView>("date").unwrap();
    let now: DateTime<Local> = Local::now();
    let p = now.format("%d/%m/%Y").to_string();
    out.set_content(p);
}