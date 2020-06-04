extern crate atk;
extern crate gtk;
extern crate gtk_sys;
use gtk::*;
use std::process;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

fn main() {
    // Initialize GTK before proceeding.
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let calc_display = Arc::new(CalcComponent::new(0));

    // Initialize the UI's initial state
    let app = App::new(&calc_display);

    // Make all the widgets within the UI visible.
    app.window.show_all();

    // Start the GTK main event loop
    gtk::main();
}

pub struct CalcComponent(AtomicUsize);

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
            let grid_width = 3;
            let width = 1;
            let height = 1;
            let row_offset = 1;
            let col = ((n + (grid_width - 1)) % grid_width) + 1;
            let row = row_offset + ((n - 1) / grid_width + 1);
            // println!("{} {}", col, row);
            table.attach(&new_button, col, row, width, height);
        }
        let plus_button = make_button("+");
        table.attach(&plus_button, 4, 3, 1, 1);
        let minus_button = make_button("-");
        table.attach(&minus_button, 4, 2, 1, 1);
        let multiply_button = make_button("*");
        table.attach(&multiply_button, 3, 1, 1, 1);
        let division_button = make_button("/");
        table.attach(&division_button, 2, 1, 1, 1);
        let mod_button = make_button("%");
        table.attach(&mod_button, 1, 1, 1, 1);
        let zero_button = make_button("0");
        table.attach(&zero_button, 1, 5, 3, 1);
        let enter_button = make_button("=");
        table.attach(&enter_button, 4, 4, 1, 2);
        let back_button = make_button("back");
        table.attach(&back_button, 4, 1, 1, 1);

        // Add the corresponding style classes to those buttons.
        // num_1_button
        //     .get_style_context()
        //     .map(|c| c.add_class("destructive-action"));
        // num_2_button
        //     .get_style_context()
        //     .map(|c| c.add_class("suggested-action"));

        // THen add them to the header bar.
        // container.pack_start(&num_1_button, true, false, 0);
        // container.pack_end(&num_2_button, true, false, 0);
        calc_info.pack_end(&calc_display, false, false, 5);
        container.pack_start(&calc_info, true, false, 5);
        container.pack_end(&table, true, false, 0);

        // Returns the content and all of it's state
        Content {
            calc_display,
            container,
        }
    }
}

impl CalcComponent {
    fn new(initial: usize) -> CalcComponent {
        CalcComponent(AtomicUsize::new(initial))
    }

    fn get_health(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    fn subtract(&self, value: usize) -> usize {
        let current = self.0.load(Ordering::SeqCst);
        let new = if current < value { 0 } else { current - value };
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn heal(&self, value: usize) -> usize {
        let original = self.0.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}

fn make_button(x: &str) -> Button {
    let new_button = Button::new_with_label(x);
    return new_button;
}
