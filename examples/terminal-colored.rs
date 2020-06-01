use fltk::{app, text::*, window::*};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
struct Term {
    pub term: TextDisplay,
    current_dir: String,
    cmd: String,
    sbuf: TextBuffer,
}

impl Term {
    pub fn new(mut buf: &mut TextBuffer) -> Term {
        let mut current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();

        current_dir.push_str("/ $ ");

        let mut term = TextDisplay::new(5, 5, 630, 470, &mut buf);

        let mut sbuf = TextBuffer::default();

        // Enable different colored text in TestDisplay
        let styles: Vec<StyleTableEntry> = vec![
            StyleTableEntry {
                color: Color::Green,
                font: Font::Courrier,
                size: 18,
            },
            StyleTableEntry {
                color: Color::Red,
                font: Font::Courrier,
                size: 18,
            },
        ];
        term.set_styly_table_entry(&mut sbuf, &styles);

        Term {
            term: term,
            current_dir: current_dir,
            cmd: String::from(""),
            sbuf: sbuf,
        }
    }

    pub fn style(&mut self) {
        self.term.set_color(Color::Black);
        self.term.set_text_color(Color::Green);
        self.term.set_text_font(Font::Courrier);
        self.term.set_cursor_color(Color::Green);
        self.term.set_cursor_style(CursorStyle::BlockCursor);
        self.term.show_cursor(true);
    }

    fn append(&mut self, txt: &str) {
        self.term.buffer().append(txt);
        self.sbuf.append(&"A".repeat(txt.len()));
        self.term.set_insert_position(self.term.buffer().length());
        self.term.scroll(
            self.term.count_lines(0, self.term.buffer().length(), true),
            0,
        );
    }

    fn append_error(&mut self, txt: &str) {
        self.term.buffer().append(txt);
        self.sbuf.append(&"B".repeat(txt.len()));
        self.term.set_insert_position(self.term.buffer().length());
        self.term.scroll(
            self.term.count_lines(0, self.term.buffer().length(), true),
            0,
        );
    }

    fn run_command(&mut self) -> String {
        let args = self.cmd.clone();
        let args: Vec<&str> = args.split_whitespace().collect();
        
        if args.len() > 0 {
            let mut cmd = Command::new(args[0]);
            if args.len() > 1 {
                if args[0] == "cd" {
                    let path = args[1];
                    return self.change_dir(&PathBuf::from(path));
                } else {
                    cmd.args(&args[1..]);
                }
            }
            let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            if out.is_err() {
                let msg = format!("{}: command not found!\n", self.cmd);
                return msg;
            } else {
                let stdout = out.unwrap().stdout;
                let stdout = String::from_utf8_lossy(&stdout).to_string();
                return stdout;
            }
        } else {
            return String::from("");
        }
    }

    pub fn change_dir(&mut self, path: &Path) -> String {
        if path.exists() && path.is_dir() {
            std::env::set_current_dir(path).unwrap();
            let mut current_dir = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
            current_dir.push_str("/ $ ");
            self.current_dir = current_dir.clone();
            return String::from("");
        } else {
            return String::from("Path does not exist!\n");
        }
    }
}

fn main() {
    let app = app::App::default().set_scheme(app::AppScheme::Plastic);
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let mut tbuf = TextBuffer::default();

    let mut term = Term::new(&mut tbuf);
    term.style();

    let dir = term.current_dir.clone();
    term.append(&dir);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let mut term_c = term.clone();
    term_c.term.handle(Box::new(move |ev| {
        // println!("{:?}", app::event());
        // println!("{:?}", app::event_key());
        // println!("{:?}", app::event_text());
        match ev {
            app::Event::KeyDown => match app::event_key() {
                app::Key::Enter => {
                    term.append("\n");
                    let out = term.run_command();
                    if out.contains("not found") {
                        term.append_error(&out);
                    } else {
                        term.append(&out);
                    }
                    let current_dir = term.current_dir.clone();
                    term.append(&current_dir);
                    term.cmd.clear();
                    true
                }
                app::Key::BackSpace => {
                    if term.cmd.len() != 0 {
                        let text_len = term.term.buffer().text().len() as u32;
                        term
                            .term
                            .buffer()
                            .remove(text_len - 1, text_len as u32);
                        term.cmd.pop().unwrap();
                        return true;
                    } else {
                        return false;
                    }
                }
                _ => {
                    let temp = app::event_text();
                    term.cmd.push_str(&temp);
                    term.append(&temp);
                    true
                }
            },
            _ => false,
        }
    }));
    
    app.run().unwrap();
}
