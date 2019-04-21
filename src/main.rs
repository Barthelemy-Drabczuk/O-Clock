extern crate time;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Label};

use time::now;

fn get_time () -> String {
	let time_now = now ();
	return format! ("{}:{}:{}", time_now.tm_hour, time_now.tm_min, time_now.tm_sec);
}

fn main() {

	if gtk::init ().is_err () {
        println! ("Failed to initialize GTK.");
        return;
    }

	//window part
    let window = Window::new (WindowType::Toplevel);
    window.set_title ("O'Clock");
    window.set_default_size (350, 70);

	//time label part
	let time_now = get_time ();
    let time_label = Label::new (None);
	time_label.set_text (&time_now);
	window.add (&time_label);
    window.show_all ();

    window.connect_delete_event (|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

	// we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        let time = get_time ();
        time_label.set_text (&time);
        // we could return gtk::Continue(false) to stop our clock after this tick
        gtk::Continue (true)
    };

    // executes the closure once every second
	gtk::timeout_add_seconds (1, tick);

    gtk::main ();
}
