//! macOS: CADCraft's menu tree as the native system menu bar. Items dispatch through the same
//! command path as the in-window menus; enablement is refreshed a few times per second.

use std::collections::HashMap;
use std::str::FromStr;

use cadcraft_ui_egui::CadApp;
use cadcraft_ui_egui::menus::{self, Entry};
use muda::accelerator::Accelerator;
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};

pub struct NativeMenu {
    _menu: Menu,
    items: HashMap<String, (String, MenuItem)>,
    last_refresh: f64,
}

/// Accelerator for "Cmd+Shift+S" (modifier-less shortcuts stay in the app so typing works).
fn accel(sc: &str) -> Option<Accelerator> {
    if !(sc.contains("Cmd") || sc.contains("Ctrl") || sc.contains("Alt")) {
        return None;
    }
    Accelerator::from_str(&sc.replace("Cmd", "CMD").replace("Alt", "ALT").replace("Shift", "SHIFT").replace("Ctrl", "CTRL")).ok()
}

impl NativeMenu {
    pub fn install(app: &mut CadApp) -> Self {
        let menu = Menu::new();
        let mut items = HashMap::new();
        let mut counter = 0usize;
        let app_menu = Submenu::new("CADCraft", true);
        let about = MenuItem::with_id("cc-about", "關於 CADCraft", true, None);
        let discord = MenuItem::with_id("cc-discord", "加入 ArtCraft Discord…", true, None);
        let _ = app_menu.append_items(&[
            &about,
            &discord,
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::services(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ]);
        items.insert("cc-about".into(), ("ui.dialog.about".into(), about));
        items.insert("cc-discord".into(), ("ui.discord".into(), discord));
        let _ = menu.append(&app_menu);
        for (title, entries) in menus::tree(app) {
            let sub = Submenu::new(&title, true);
            build(&sub, &entries, &mut items, &mut counter);
            let _ = menu.append(&sub);
        }
        menu.init_for_nsapp();
        Self { _menu: menu, items, last_refresh: 0.0 }
    }

    pub fn poll(&mut self, app: &mut CadApp, ctx: &egui::Context) {
        while let Ok(ev) = MenuEvent::receiver().try_recv() {
            if let Some((cmd, _)) = self.items.get(ev.id.as_ref()) {
                if cmd == "ui.discord" {
                    ctx.open_url(egui::OpenUrl::new_tab("https://discord.gg/artcraft"));
                } else {
                    menus::activate(app, &cmd.clone());
                }
                ctx.request_repaint();
            }
        }
        let now = cadcraft_ui_egui::now_ms();
        if now - self.last_refresh < 300.0 {
            return;
        }
        self.last_refresh = now;
        for (cmd, item) in self.items.values() {
            let en = cadcraft_engine::find_command(cmd).is_none_or(|c| (c.enabled)(&app.session).is_ok());
            item.set_enabled(en);
        }
    }
}

fn build(parent: &Submenu, entries: &[Entry], items: &mut HashMap<String, (String, MenuItem)>, counter: &mut usize) {
    for e in entries {
        match e {
            Entry::Sub { label, children } => {
                let sub = Submenu::new(label, true);
                build(&sub, children, items, counter);
                let _ = parent.append(&sub);
            }
            Entry::Item { label, id, shortcut, enabled } => {
                *counter += 1;
                let mid = format!("cc{counter}");
                let i = MenuItem::with_id(mid.clone(), label, *enabled, shortcut.as_deref().and_then(accel));
                let _ = parent.append(&i);
                items.insert(mid, (id.clone(), i));
            }
        }
    }
}
