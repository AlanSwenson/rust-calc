extern crate atk;
extern crate gtk;
extern crate gtk_sys;
use glib::GString;
use gtk::*;
use std::cell::RefCell;
use std::process;
use std::rc::Rc;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let calc_display = Arc::new(CalcComponent::new(0));
    let op = Rc::new(RefCell::new("".to_string()));
    let first_num: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));
    let second_num: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));

    // Initialize the UI's initial state
    let app = App::new(&calc_display);

    {
        // Program the subtraction button.
        let info = app.content.calc_display.clone();
        let captured_op = op.clone();
        let captured_num = first_num.clone();
        app.content
            .op_buttons
            .minus_button
            .clone()
            .connect_clicked(move |_| {
                change_op("-", captured_op.clone(), captured_num.clone(), info.clone());

                println!("{:?}", captured_op);
                println!("{:?}", captured_num);
            });
    }
    {
        // Program the addition button.
        let info = app.content.calc_display.clone();
        let captured_op = op.clone();
        let captured_num = first_num.clone();
        app.content
            .op_buttons
            .plus_button
            .clone()
            .connect_clicked(move |_| {
                change_op("+", captured_op.clone(), captured_num.clone(), info.clone());
            });
    }
    {
        // Program the equals button.
        let captured_op = op.clone();
        app.content
            .op_buttons
            .enter_button
            .clone()
            .connect_clicked(move |_| {
                println!("{:?}", captured_op);
                *captured_op.borrow_mut() = "=".to_string();
                println!("{:?}", captured_op);
            });
    }

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

pub struct CalcComponent {
    pub display: AtomicIsize,
}

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
}

pub struct Content {
    pub container: Box,
    pub calc_display: Label,
    pub op_buttons: OpButtons,
}

pub struct OpButtons {
    pub minus_button: Button,
    pub plus_button: Button,
    pub enter_button: Button,
}

impl OpButtons {
    fn new() -> OpButtons {
        let plus_button = make_button("+");
        let minus_button = make_button("-");
        let enter_button = make_button("=");

        OpButtons {
            minus_button,
            plus_button,
            enter_button,
        }
    }
}

impl App {
    fn new(calc_display: &CalcComponent) -> App {
        // Create a new top level window.
        let window = Window::new(WindowType::Toplevel);
        // Create a the headerbar and it's associated content.
        let header = Header::new();
        let content = Content::new(calc_display);

        // Set the headerbar as the title bar widget.
        window.set_titlebar(Some(&header.container));
        // Set the title of the window.
        window.set_title("Rust Calc");
        // Set the window manager class.
        window.set_wmclass("app-name", "App name");
        // The icon the app will display.
        Window::set_default_icon_name("iconname");

        // Add the content box into the window.
        window.add(&content.container);

        // Programs what to do when the exit button is used.
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return our main application state
        App {
            window,
            header,
            content,
        }
    }
}

impl Header {
    fn new() -> Header {
        // Creates the main header bar container widget.
        let container = HeaderBar::new();

        // Sets the text to display in the title section of the header bar.
        container.set_title(Some("Rust Calc"));
        // Enable the window controls within this headerbar.
        container.set_show_close_button(true);

        // Returns the header and all of it's state
        Header { container }
    }
}

impl Content {
    fn new(calc_display: &CalcComponent) -> Content {
        // Creates the main content bar container widget.
        let container = Box::new(Orientation::Vertical, 0);
        let calc_info = Box::new(Orientation::Horizontal, 0);

        let calc_display = Label::new(Some(calc_display.get_health().to_string().as_str()));

        let table = gtk::Grid::new();

        for n in 1..10 {
            let new_button = make_button(&n.to_string());
            let info = calc_display.clone();
            new_button.connect_clicked(move |_| {
                // concat existing with new
                let original = info.get_text().unwrap();
                let mut new_display: String = format!("{}{}", original, n);
                new_display = new_display.trim_start_matches('0').to_string();
                info.set_label(new_display.to_string().as_str());
                // info.set_display(n);
            });
            let grid_width = 3;
            let width = 1;
            let height = 1;
            let row_offset = 1;
            let col = ((n + (grid_width - 1)) % grid_width) + 1;
            let row = row_offset + ((n - 1) / grid_width + 1);
            table.attach(&new_button, col, row, width, height);
        }
        let op_buttons = OpButtons::new();

        table.attach(&op_buttons.plus_button, 4, 3, 1, 1);
        table.attach(&op_buttons.minus_button, 4, 2, 1, 1);
        let multiply_button = make_button("*");
        table.attach(&multiply_button, 3, 1, 1, 1);
        let division_button = make_button("/");
        table.attach(&division_button, 2, 1, 1, 1);
        let mod_button = make_button("%");
        table.attach(&mod_button, 1, 1, 1, 1);
        let zero_button = make_button("0");
        table.attach(&zero_button, 1, 5, 3, 1);
        table.attach(&op_buttons.enter_button, 4, 4, 1, 2);
        let back_button = make_button("back");
        table.attach(&back_button, 4, 1, 1, 1);

        calc_info.pack_end(&calc_display, false, false, 5);
        container.pack_start(&calc_info, true, false, 5);
        container.pack_end(&table, true, false, 0);

        // Returns the content and all of it's state
        Content {
            calc_display,
            container,
            op_buttons,
        }
    }
}

impl<'a> CalcComponent {
    fn new(initial: isize) -> CalcComponent {
        CalcComponent {
            display: AtomicIsize::new(initial),
        }
    }

    fn get_health(&self) -> isize {
        self.display.load(Ordering::SeqCst)
    }

    fn subtract(&self, value: isize) -> isize {
        let current: isize = self.display.load(Ordering::SeqCst);
        let new = current - value;
        self.display.store(new, Ordering::SeqCst);
        new
    }

    fn set_display(&self, value: isize) -> isize {
        self.display.store(value, Ordering::SeqCst);
        value
    }

    fn add(&self, value: isize) -> isize {
        let original = self.display.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}

fn make_button(x: &str) -> Button {
    let new_button = Button::new_with_label(x);
    return new_button;
}

fn convert_gstring(x: Option<GString>) -> Option<i64> {
    //convert to string
    let orig_string = String::from(x.unwrap());
    // convert to Option<i64> or None
    let new_int: Option<i64> = orig_string.parse().ok();
    return new_int;
}

fn change_op(
    op: &str,
    captured_op: std::rc::Rc<std::cell::RefCell<std::string::String>>,
    captured_num: std::rc::Rc<std::cell::RefCell<std::option::Option<i64>>>,
    info: gtk::Label,
) {
    *captured_op.borrow_mut() = op.to_string();
    println!("new operator: {:?}", op);
    let c = *captured_num.borrow_mut();
    if let Some(value) = c {
        println!("x has value: {}", value);
        *captured_num.borrow_mut() = convert_gstring(info.get_text());
        println!("c has changed value: {:?}", c);
    } else {
        *captured_num.borrow_mut() = convert_gstring(info.get_text());
        println!("c has value: {:?}", c);
    }
}
