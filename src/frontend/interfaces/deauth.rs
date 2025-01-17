use crate::types::*;

use gtk4::prelude::*;
use gtk4::*;

pub struct DeauthGui {
    pub window: Window,
    pub store: ListStore,
    pub view: TreeView,
    pub toggle: CellRendererToggle,
    pub scroll: ScrolledWindow,
    pub all_cli_but: CheckButton,
    pub sel_cli_but: CheckButton,
    pub attack_but: Button,
}

impl DeauthGui {
    pub fn new(parent: &impl IsA<Window>) -> Self {
        let window = Window::builder()
            .title("Deauth")
            .hide_on_close(true)
            .default_width(300)
            .default_height(300)
            .resizable(false)
            .transient_for(parent)
            .modal(true)
            .build();

        let all_cli_but = CheckButton::with_label("Deauth all clients");
        let sel_cli_but = CheckButton::with_label("Deauth selected clients");

        all_cli_but.set_active(true);
        sel_cli_but.set_group(Some(&all_cli_but));

        all_cli_but.set_margin_start(15);
        all_cli_but.set_margin_top(15);

        sel_cli_but.set_margin_start(15);
        sel_cli_but.set_margin_bottom(15);

        let store = ListStore::new(&[glib::Type::BOOL, glib::Type::STRING]);

        let column = TreeViewColumn::new();
        column.set_title("Clients");

        let view = TreeView::new();
        view.set_sensitive(false);
        view.set_vexpand(true);
        view.set_model(Some(&store));
        view.append_column(&column);

        let toggle = CellRendererToggle::new();
        column.pack_start(&toggle, false);
        column.add_attribute(&toggle, "active", 0);

        let text_ren = CellRendererText::new();
        column.pack_start(&text_ren, true);
        column.add_attribute(&text_ren, "text", 1);

        let scroll = ScrolledWindow::new();
        scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
        scroll.set_child(Some(&view));

        let deauth_box = Box::new(Orientation::Vertical, 2);
        deauth_box.append(&all_cli_but);
        deauth_box.append(&sel_cli_but);
        deauth_box.append(&scroll);

        let frame = Frame::new(None);
        frame.set_child(Some(&deauth_box));

        let attack_but = Button::with_label("Attack");

        let main_box = Box::new(Orientation::Vertical, 10);
        main_box.append(&frame);
        main_box.append(&attack_but);

        main_box.set_margin_bottom(10);
        main_box.set_margin_end(10);
        main_box.set_margin_start(10);
        main_box.set_margin_top(10);

        window.set_child(Some(&main_box));

        Self {
            window,
            store,
            view,
            toggle,
            scroll,
            all_cli_but,
            sel_cli_but,
            attack_but,
        }
    }

    pub fn show(&self, ap: AP) {
        self.window
            .set_title(Some(&format!("Deauth \"{}\"", ap.essid)));

        self.sel_cli_but.set_active(false);
        self.all_cli_but.set_active(true);
        self.view.set_sensitive(false);
        self.attack_but.set_sensitive(true);

        self.store.clear();
        for (_, cli) in ap.clients.iter() {
            self.store
                .set(&self.store.append(), &[(0, &false), (1, &cli.mac)]);
        }

        self.window.show();
    }
}
