enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: {}", s),
        WebEvent::Click { x, y } => println!("Click: ({}, {})", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');

    let pasted = WebEvent::Paste(("my_text".to_owned()));
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unloaded = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unloaded);
}
