//! Tool Sets (left) and the Layers / Properties palettes (right).

use cadcraft_color::Color;
use cadcraft_doc::EntityKind;
use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::{Value, json};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

type Tool = (Icon, &'static str, &'static str);

/// Tool Sets groups: (name, large tools, small tools).
pub const DRAFTING: &[(&str, &[Tool], &[Tool])] = &[
    (
        "繪圖",
        &[(Icon::Line, "line", "直線"), (Icon::Polyline, "pline", "聚合線"), (Icon::Circle, "circle", "圓"), (Icon::Rectangle, "rectang", "矩形")],
        &[
            (Icon::Arc, "arc", "弧"),
            (Icon::Spline, "spline", "雲形線"),
            (Icon::XLine, "xline", "建構線"),
            (Icon::Ray, "ray", "射線"),
            (Icon::Ellipse, "ellipse", "橢圓"),
            (Icon::Polygon, "polygon", "多邊形"),
            (Icon::Point, "point.multiple", "多點"),
            (Icon::Donut, "donut", "環"),
            (Icon::RevCloud, "revcloud", "修訂雲形"),
            (Icon::Region, "region", "面域"),
            (Icon::Wipeout, "wipeout", "覆蓋"),
        ],
    ),
    ("填充線", &[(Icon::Hatch, "hatch", "填充線"), (Icon::Gradient, "gradient", "漸層"), (Icon::Boundary, "boundary", "邊界")], &[]),
    (
        "圖塊",
        &[
            (Icon::BlockCreate, "block", "建立圖塊"),
            (Icon::Insert, "insert", "插入圖塊"),
            (Icon::BlockEdit, "bedit", "圖塊編輯器"),
            (Icon::AttDef, "attdef", "定義屬性"),
        ],
        &[],
    ),
    (
        "修改",
        &[(Icon::Move, "move", "移動")],
        &[
            (Icon::Copy, "copy", "複製"),
            (Icon::Rotate, "rotate", "旋轉"),
            (Icon::Trim, "trim", "修剪"),
            (Icon::Fillet, "fillet", "圓角"),
            (Icon::ArrayRect, "arrayrect", "矩形陣列"),
            (Icon::Mirror, "mirror", "鏡射"),
            (Icon::Scale, "scale", "比例"),
            (Icon::Extend, "extend", "延伸"),
            (Icon::Offset, "offset", "偏移"),
            (Icon::ArrayPolar, "arraypolar", "環形陣列"),
            (Icon::Stretch, "stretch", "拉伸"),
            (Icon::Chamfer, "chamfer", "倒角"),
            (Icon::Explode, "explode", "分解"),
            (Icon::Break, "break", "打斷"),
            (Icon::Join, "join", "接合"),
            (Icon::Erase, "erase", "刪除"),
            (Icon::Lengthen, "lengthen", "拉長"),
            (Icon::MatchProp, "matchprop", "性質複製"),
        ],
    ),
    (
        "文字",
        &[(Icon::Text, "text", "單行文字"), (Icon::MText, "mtext", "多行文字")],
        &[(Icon::Table, "table", "表格"), (Icon::Search, "find", "尋找與取代")],
    ),
    (
        "尺寸標註",
        &[(Icon::DimLinear, "dimlinear", "線性"), (Icon::DimAligned, "dimaligned", "對齊式")],
        &[
            (Icon::DimRadius, "dimradius", "半徑"),
            (Icon::DimDiameter, "dimdiameter", "直徑"),
            (Icon::DimAngular, "dimangular", "角度"),
            (Icon::DimArc, "dimarc", "弧長"),
            (Icon::DimOrdinate, "dimordinate", "座標"),
            (Icon::DimBaseline, "dimbaseline", "基線式"),
            (Icon::DimContinue, "dimcontinue", "連續式"),
            (Icon::QDim, "qdim", "快速標註"),
        ],
    ),
    (
        "引線",
        &[(Icon::MLeader, "mleader", "多重引線")],
        &[
            (Icon::AddLeader, "mleaderedit", "加入引線"),
            (Icon::RemoveLeader, "mleaderedit.remove", "移除引線"),
            (Icon::AlignLeaders, "mleaderalign", "對齊"),
        ],
    ),
    ("表格", &[(Icon::Table, "table", "表格")], &[]),
    (
        "參數式",
        &[(Icon::Constraint, "autoconstrain", "自動約束")],
        &[
            (Icon::Coincident, "gccoincident", "重合"),
            (Icon::Parallel, "gcparallel", "平行"),
            (Icon::Perpendicular, "gcperpendicular", "垂直"),
            (Icon::Horizontal, "gchorizontal", "水平"),
            (Icon::Vertical, "gcvertical", "垂直"),
            (Icon::Tangent, "gctangent", "相切"),
            (Icon::Concentric, "gcconcentric", "同心"),
            (Icon::Equal, "gcequal", "相等"),
        ],
    ),
];

fn command_known(id: &str) -> bool {
    cadcraft_engine::find_command(id).is_some() || crate::menus::is_ui_command(id)
}

pub fn toolsets(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    egui::Panel::left("cc_toolsets").exact_size(220.0).resizable(false).frame(egui::Frame::NONE.fill(t.panel)).show(ui, |ui| {
        let r = ui.max_rect();
        let p = ui.painter().clone();
        // Tabs.
        let tab_h = 28.0;
        let mut x = r.left();
        for name in ["Drafting", "Modeling"] {
            let w = 92.0;
            let tr = Rect::from_min_size(pos2(x, r.top()), vec2(w, tab_h));
            let active = app.ui.toolset_tab == name;
            let disp = match name {
                "Drafting" => "繪圖",
                "Modeling" => "建模",
                _ => name,
            };
            let resp = ui.interact(tr, ui.id().with(("ts", name)), Sense::click());
            p.rect_filled(tr, 0.0, if active { t.tab_active } else { t.chrome });
            p.text(tr.center(), egui::Align2::CENTER_CENTER, disp, egui::FontId::proportional(13.5), if active { t.text } else { t.text_dim });
            if resp.clicked() {
                app.ui.toolset_tab = name.into();
            }
            x += w;
        }
        p.rect_filled(Rect::from_min_max(pos2(x, r.top()), pos2(r.right(), r.top() + tab_h)), 0.0, t.chrome);
        let cr = Rect::from_center_size(pos2(r.right() - 14.0, r.top() + tab_h / 2.0), vec2(14.0, 14.0));
        let cresp = ui.interact(cr, ui.id().with("ts_collapse"), Sense::click());
        icons::paint(&p, cr, Icon::ChevronLeft, false);
        if cresp.on_hover_text("收合工具集").clicked() {
            app.ui.show_toolsets = false;
        }
        let body = Rect::from_min_max(pos2(r.left(), r.top() + tab_h), r.max);
        let mut clicked: Option<&'static str> = None;
        let mut toggle_group = None;
        ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                let groups: &[(&str, &[Tool], &[Tool])] = if app.ui.toolset_tab == "Drafting" { DRAFTING } else { MODELING };
                for (name, large, small) in groups {
                    let collapsed = app.ui.collapsed_groups.iter().any(|g| g == name);
                    // Header.
                    let (hr, hresp) = ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::click());
                    let pp = ui.painter();
                    icons::paint(
                        pp,
                        Rect::from_center_size(pos2(hr.left() + 11.0, hr.center().y), vec2(11.0, 11.0)),
                        if collapsed { Icon::ChevronRight } else { Icon::ChevronDown },
                        false,
                    );
                    pp.text(pos2(hr.left() + 22.0, hr.center().y), egui::Align2::LEFT_CENTER, *name, egui::FontId::proportional(12.5), t.text);
                    icons::paint(pp, Rect::from_center_size(pos2(hr.right() - 12.0, hr.center().y), vec2(11.0, 11.0)), Icon::Gear, false);
                    if hresp.clicked() {
                        toggle_group = Some(name.to_string());
                    }
                    if !collapsed {
                        ui.add_space(4.0);
                        // Large icons.
                        if !large.is_empty() {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
                                for (icon, cmd, tip) in large.iter() {
                                    ui.add_enabled_ui(command_known(cmd), |ui| {
                                        if icons::button(ui, *icon, 40.0, tip, false).clicked() {
                                            clicked = Some(cmd);
                                        }
                                    });
                                }
                            });
                        }
                        // Small icons, 8 per row.
                        for row in small.chunks(8) {
                            ui.add_space(2.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
                                for (icon, cmd, tip) in row.iter() {
                                    ui.add_enabled_ui(command_known(cmd), |ui| {
                                        if icons::button(ui, *icon, 23.0, tip, false).clicked() {
                                            clicked = Some(cmd);
                                        }
                                    });
                                }
                            });
                        }
                        ui.add_space(6.0);
                    }
                    let (sr, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
                    ui.painter().hline(sr.x_range().shrink(6.0), sr.center().y, Stroke::new(1.0, t.border));
                }
            });
        });
        if let Some(g) = toggle_group {
            if let Some(i) = app.ui.collapsed_groups.iter().position(|x| *x == g) {
                app.ui.collapsed_groups.remove(i);
            } else {
                app.ui.collapsed_groups.push(g);
            }
        }
        if let Some(c) = clicked {
            app.cmd.buffer.clear();
            app.start(c);
        }
    });
}

pub const MODELING: &[(&str, &[Tool], &[Tool])] = &[
    (
        "基本實體",
        &[(Icon::Blocks, "box", "方塊"), (Icon::Circle, "cylinder", "圓柱"), (Icon::Polygon, "pyramid", "角錐")],
        &[(Icon::Ellipse, "sphere", "球體"), (Icon::Donut, "torus", "圓環體"), (Icon::Arc, "cone", "圓錐")],
    ),
    (
        "實體",
        &[(Icon::Extend, "extrude", "擠出"), (Icon::Rotate, "revolve", "回轉")],
        &[(Icon::Spline, "sweep", "掃掠"), (Icon::Offset, "loft", "斷面混成")],
    ),
    ("布林運算", &[(Icon::Join, "union", "聯集"), (Icon::Trim, "subtract", "差集"), (Icon::Region, "intersect", "交集")], &[]),
    ("檢視", &[(Icon::Orbit, "3dorbit", "環繞"), (Icon::ZoomExtents, "zoom.extents", "縮放至實際範圍")], &[]),
];

fn section_header(ui: &mut egui::Ui, title: &str) -> Rect {
    let t = Tokens::get();
    let (hr, _) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), Sense::hover());
    ui.painter().text(pos2(hr.left() + 10.0, hr.center().y), egui::Align2::LEFT_CENTER, title, egui::FontId::proportional(14.0), t.text);
    hr
}

fn swatch(p: &egui::Painter, r: Rect, c: Color) {
    let rgb = c.resolve(Color::Index(7), Color::Index(7));
    p.rect_filled(r, 1.0, Color32::from_rgb(rgb.0, rgb.1, rgb.2));
    p.rect_stroke(r, 1.0, Stroke::new(1.0, Color32::from_gray(30)), egui::StrokeKind::Inside);
}

pub fn right_palettes(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    egui::Panel::right("cc_palettes").exact_size(300.0).resizable(false).frame(egui::Frame::NONE.fill(t.panel)).show(ui, |ui| {
        let r = ui.max_rect();
        // Palette icon tabs.
        let p = ui.painter().clone();
        let tab_h = 30.0;
        p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), tab_h)), 0.0, t.chrome);
        for (i, icon) in [Icon::Layers, Icon::Blocks, Icon::Properties].iter().enumerate() {
            let br = Rect::from_center_size(pos2(r.left() + 30.0 + i as f32 * 52.0, r.top() + tab_h / 2.0), vec2(20.0, 20.0));
            icons::paint(&p, br, *icon, false);
        }
        icons::paint(&p, Rect::from_center_size(pos2(r.right() - 14.0, r.top() + tab_h / 2.0), vec2(14.0, 14.0)), Icon::ChevronRight, false);
        let body = Rect::from_min_max(pos2(r.left(), r.top() + tab_h), r.max);
        ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                layers_section(app, ui);
                ui.add_space(8.0);
                properties_section(app, ui);
            });
        });
    });
}

fn layers_section(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    section_header(ui, "圖層");
    // Layer tools row.
    let mut cmd = None;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.spacing_mut().item_spacing = vec2(3.0, 0.0);
        for (icon, c, tip) in [
            (Icon::LayerProps, "ui.dialog.layers", "圖層性質"),
            (Icon::MakeCurrent, "laymcur", "將物件圖層設為目前圖層"),
            (Icon::LayerMatch, "laymch", "符合圖層"),
            (Icon::LayerPrev, "layerp", "前一個圖層"),
            (Icon::LayerIso, "layiso", "隔離圖層"),
            (Icon::LayerOff, "layoff", "關閉圖層"),
            (Icon::LayerFreeze, "layfrz", "凍結圖層"),
            (Icon::LayerLock, "laylck", "鎖定圖層"),
            (Icon::Unlock, "layulk", "解鎖圖層"),
        ] {
            if icons::button(ui, icon, 26.0, tip, false).clicked() {
                cmd = Some(c);
            }
        }
    });
    if let Some(c) = cmd {
        if c.starts_with("ui.") {
            app.start(c);
        } else {
            let _ = app.run(c, json!({}));
        }
    }
    ui.add_space(4.0);
    // Current layer combo.
    let Ok(d) = app.session.doc() else { return };
    let cur = d.header.str("CLAYER", "0");
    let layers: Vec<(String, bool, bool, bool, Color)> = d.layers.iter().map(|l| (l.name.clone(), l.on, l.frozen, l.locked, l.color)).collect();
    let cur_info = layers.iter().find(|l| l.0.eq_ignore_ascii_case(&cur)).cloned();
    let mut set_current = None;
    let mut toggle: Option<(String, &str, bool)> = None;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let w = ui.available_width() - 10.0;
        let (rect, resp) = ui.allocate_exact_size(vec2(w, 24.0), Sense::click());
        let p = ui.painter();
        p.rect_filled(rect, 3.0, t.control);
        if let Some((name, on, frozen, locked, color)) = &cur_info {
            let mut x = rect.left() + 6.0;
            for icon in [
                if *on { Icon::Bulb } else { Icon::BulbOff },
                if *locked { Icon::Lock } else { Icon::Unlock },
                if *frozen { Icon::Snowflake } else { Icon::Sun },
            ] {
                icons::paint(p, Rect::from_min_size(pos2(x, rect.top() + 4.0), vec2(16.0, 16.0)), icon, false);
                x += 18.0;
            }
            swatch(p, Rect::from_min_size(pos2(x + 2.0, rect.top() + 7.0), vec2(10.0, 10.0)), *color);
            p.text(pos2(x + 20.0, rect.center().y), egui::Align2::LEFT_CENTER, name, crate::theme::body(), t.text);
        }
        icons::paint(p, Rect::from_center_size(pos2(rect.right() - 12.0, rect.center().y), vec2(12.0, 12.0)), Icon::ChevronDown, false);
        egui::Popup::from_toggle_button_response(&resp).width(w).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
            for (name, on, frozen, locked, color) in &layers {
                ui.horizontal(|ui| {
                    if icons::button(ui, if *on { Icon::Bulb } else { Icon::BulbOff }, 18.0, "開/關", false).clicked() {
                        toggle = Some((name.clone(), "on", !*on));
                    }
                    if icons::button(ui, if *frozen { Icon::Snowflake } else { Icon::Sun }, 18.0, "凍結/解凍", false).clicked() {
                        toggle = Some((name.clone(), "frozen", !*frozen));
                    }
                    if icons::button(ui, if *locked { Icon::Lock } else { Icon::Unlock }, 18.0, "鎖定/解鎖", false).clicked() {
                        toggle = Some((name.clone(), "locked", !*locked));
                    }
                    let (sr, _) = ui.allocate_exact_size(vec2(12.0, 12.0), Sense::hover());
                    swatch(ui.painter(), sr, *color);
                    if ui.selectable_label(name.eq_ignore_ascii_case(&cur), name).clicked() {
                        set_current = Some(name.clone());
                    }
                });
            }
        });
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let w = ui.available_width() - 10.0;
        let (rect, _) = ui.allocate_exact_size(vec2(w, 24.0), Sense::hover());
        ui.painter().rect_filled(rect, 3.0, t.control);
        ui.painter().text(pos2(rect.left() + 8.0, rect.center().y), egui::Align2::LEFT_CENTER, "未儲存的圖層狀態", crate::theme::body(), t.text);
        icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.right() - 12.0, rect.center().y), vec2(12.0, 12.0)), Icon::ChevronDown, false);
    });
    ui.add_space(4.0);
    let label = if app.ui.show_layer_list { "隱藏圖層清單" } else { "顯示圖層清單" };
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let (rect, resp) = ui.allocate_exact_size(vec2(160.0, 20.0), Sense::click());
        icons::paint(
            ui.painter(),
            Rect::from_center_size(pos2(rect.left() + 7.0, rect.center().y), vec2(11.0, 11.0)),
            if app.ui.show_layer_list { Icon::ChevronDown } else { Icon::ChevronRight },
            false,
        );
        ui.painter().text(pos2(rect.left() + 20.0, rect.center().y), egui::Align2::LEFT_CENTER, label, crate::theme::body(), t.text);
        if resp.clicked() {
            app.ui.show_layer_list = !app.ui.show_layer_list;
        }
    });
    if app.ui.show_layer_list {
        ui.add_space(2.0);
        for (name, on, frozen, locked, color) in &layers {
            ui.horizontal(|ui| {
                ui.add_space(14.0);
                if icons::button(ui, if *on { Icon::Bulb } else { Icon::BulbOff }, 18.0, "開/關", false).clicked() {
                    toggle = Some((name.clone(), "on", !*on));
                }
                if icons::button(ui, if *frozen { Icon::Snowflake } else { Icon::Sun }, 18.0, "凍結/解凍", false).clicked() {
                    toggle = Some((name.clone(), "frozen", !*frozen));
                }
                if icons::button(ui, if *locked { Icon::Lock } else { Icon::Unlock }, 18.0, "鎖定/解鎖", false).clicked() {
                    toggle = Some((name.clone(), "locked", !*locked));
                }
                let (sr, _) = ui.allocate_exact_size(vec2(12.0, 12.0), Sense::hover());
                swatch(ui.painter(), sr, *color);
                if ui.selectable_label(name.eq_ignore_ascii_case(&cur), name).clicked() {
                    set_current = Some(name.clone());
                }
            });
        }
    }
    if let Some(n) = set_current {
        let _ = app.run("layer.current", json!({ "name": n }));
    }
    if let Some((n, k, v)) = toggle {
        let _ = app.run("layer.set", json!({ "name": n, k: v }));
    }
}

fn prop_row(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
    let t = Tokens::get();
    ui.horizontal(|ui| {
        let (lr, _) = ui.allocate_exact_size(vec2(118.0, 22.0), Sense::hover());
        ui.painter().text(pos2(lr.right() - 6.0, lr.center().y), egui::Align2::RIGHT_CENTER, label, crate::theme::body(), t.text_dim);
        add(ui);
    });
}

fn value_box(ui: &mut egui::Ui, text: &str, combo: bool, enabled: bool) -> egui::Response {
    let t = Tokens::get();
    let w = ui.available_width() - 10.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 20.0), Sense::click());
    ui.painter().rect_filled(rect, 2.0, if enabled { t.control } else { t.panel });
    if !enabled {
        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, t.control), egui::StrokeKind::Inside);
    }
    ui.painter().text(
        pos2(rect.left() + 6.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        crate::theme::body(),
        if enabled { t.text } else { t.text_faint },
    );
    if combo {
        icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.right() - 10.0, rect.center().y), vec2(11.0, 11.0)), Icon::ChevronDown, false);
    }
    resp
}

/// An editable numeric/text field that commits on Enter or focus loss.
fn edit_field(ui: &mut egui::Ui, id: egui::Id, value: String) -> Option<String> {
    let mut buf = ui.data_mut(|d| d.get_temp::<String>(id)).unwrap_or_else(|| value.clone());
    let w = ui.available_width() - 10.0;
    let resp = ui.add_sized([w, 20.0], egui::TextEdit::singleline(&mut buf).id(id.with("te")).font(crate::theme::body()));
    let mut out = None;
    if resp.has_focus() {
        ui.data_mut(|d| d.insert_temp(id, buf.clone()));
    } else {
        ui.data_mut(|d| d.remove::<String>(id));
    }
    if resp.lost_focus() && buf != value {
        out = Some(buf);
    }
    out
}

fn properties_section(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let hr = section_header(ui, "性質");
    // All | My segmented control.
    let seg = Rect::from_min_size(pos2(hr.right() - 120.0, hr.top() + 4.0), vec2(70.0, 18.0));
    ui.painter().rect_filled(seg, 3.0, t.chrome_dark);
    let half = Rect::from_min_size(seg.min, vec2(35.0, 18.0));
    ui.painter().rect_filled(if app.ui.properties_all { half } else { half.translate(vec2(35.0, 0.0)) }, 3.0, t.control);
    ui.painter().text(half.center(), egui::Align2::CENTER_CENTER, "全部", crate::theme::small(), t.text);
    ui.painter().text(half.center() + vec2(35.0, 0.0), egui::Align2::CENTER_CENTER, "我的", crate::theme::small(), t.text);
    let sel = app.session.selection();
    let Ok(d) = app.session.doc() else { return };
    // Selection type combo.
    let types: Vec<&'static str> = sel.iter().filter_map(|h| d.entity(*h).map(|e| e.kind.type_name())).collect();
    let header = match types.len() {
        0 => "無選取".to_string(),
        1 => types.first().map(|s| s.to_string()).unwrap_or_default(),
        n => {
            let first = types.first().copied().unwrap_or("");
            if types.iter().all(|t| *t == first) { format!("{first} ({n})") } else { format!("全部 ({n})") }
        }
    };
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        value_box(ui, &header, true, !sel.is_empty());
    });
    ui.add_space(4.0);
    let h = &d.header;
    if sel.is_empty() {
        let rows: Vec<(&str, String, bool)> = vec![
            ("顏色", Color::from_aci(h.i64("CECOLOR", 256) as i16).name(), true),
            ("圖層", h.str("CLAYER", "0"), true),
            ("線型", h.str("CELTYPE", "ByLayer"), true),
            ("線型比例", format!("{:.4}", h.f64("CELTSCALE", 1.0)), true),
            ("線寬", cadcraft_doc::Lineweight::from_dxf(h.i64("CELWEIGHT", -1) as i16).name(), true),
            ("透明度", "依圖層".into(), true),
            ("厚度", format!("{:.4}", h.f64("THICKNESS", 0.0)), true),
            ("文字型式", h.str("TEXTSTYLE", "Standard"), true),
            ("標註型式", h.str("DIMSTYLE", "Standard"), true),
            ("多重引線型式", h.str("CMLEADERSTYLE", "Standard"), true),
            ("表格型式", h.str("CTABLESTYLE", "Standard"), true),
            ("註解比例", "1:1".into(), true),
            ("文字高", format!("{:.4}", h.f64("TEXTSIZE", 0.2)), true),
            ("出圖型式", "依顏色".into(), true),
            ("出圖型式表", "無".into(), true),
            ("出圖型式附著於", "模型".into(), false),
            ("出圖表類型", "不可用".into(), false),
        ];
        let layers: Vec<String> = d.layers.iter().map(|l| l.name.clone()).collect();
        let mut action: Option<(&str, Value)> = None;
        for (label, val, en) in rows {
            prop_row(ui, label, |ui| match label {
                "顏色" => {
                    ui.menu_button(format!("■ {val}"), |ui| {
                        for (n, c) in [("依圖層", 256), ("依圖塊", 0), ("紅", 1), ("黃", 2), ("綠", 3), ("青", 4), ("藍", 5), ("洋紅", 6), ("白", 7)]
                        {
                            if ui.button(n).clicked() {
                                action = Some(("color", json!({ "color": c })));
                                ui.close();
                            }
                        }
                    });
                }
                "圖層" => {
                    ui.menu_button(val.clone(), |ui| {
                        for l in &layers {
                            if ui.button(l).clicked() {
                                action = Some(("layer.current", json!({ "name": l })));
                                ui.close();
                            }
                        }
                    });
                }
                "文字高" => {
                    if let Some(v) = edit_field(ui, ui.id().with("cur_textsize"), val.clone())
                        && let Ok(f) = v.trim().parse::<f64>()
                    {
                        action = Some(("setvar", json!({ "name": "TEXTSIZE", "value": f })));
                    }
                }
                "線型比例" => {
                    if let Some(v) = edit_field(ui, ui.id().with("cur_celtscale"), val.clone())
                        && let Ok(f) = v.trim().parse::<f64>()
                    {
                        action = Some(("setvar", json!({ "name": "CELTSCALE", "value": f })));
                    }
                }
                _ => {
                    value_box(ui, &val, en, en);
                }
            });
        }
        if let Some((c, p)) = action {
            let _ = app.run(c, p);
        }
        return;
    }
    // Selected objects: common + geometry for a single object.
    let first = sel.first().and_then(|h| d.entity(*h)).map(|e| (**e).clone());
    let Some(e) = first else { return };
    let same = |f: &dyn Fn(&cadcraft_doc::Entity) -> String| -> String {
        let vals: Vec<String> = sel.iter().filter_map(|h| d.entity(*h)).map(|e| f(e)).collect();
        match vals.first() {
            Some(v0) if vals.iter().all(|v| v == v0) => v0.clone(),
            _ => "*多種*".into(),
        }
    };
    let color = same(&|e| e.common.color.name());
    let layer = same(&|e| e.common.layer.clone());
    let linetype = same(&|e| e.common.linetype.clone());
    let lts = same(&|e| format!("{:.4}", e.common.ltscale));
    let lw = same(&|e| e.common.lineweight.name());
    let layers: Vec<String> = d.layers.iter().map(|l| l.name.clone()).collect();
    let linetypes: Vec<String> = d.linetypes.iter().map(|l| l.name.clone()).collect();
    let ids: Vec<String> = sel.iter().map(|h| h.hex()).collect();
    let mut set: Option<Value> = None;
    let group = |ui: &mut egui::Ui, title: &str| {
        let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 20.0), Sense::hover());
        ui.painter().rect_filled(r.shrink2(vec2(6.0, 1.0)), 2.0, t.chrome_dark);
        ui.painter().text(pos2(r.left() + 12.0, r.center().y), egui::Align2::LEFT_CENTER, title, crate::theme::small(), t.text_dim);
    };
    group(ui, "一般");
    prop_row(ui, "顏色", |ui| {
        ui.menu_button(format!("■ {color}"), |ui| {
            for (n, c) in [
                ("依圖層", 256),
                ("依圖塊", 0),
                ("紅", 1),
                ("黃", 2),
                ("綠", 3),
                ("青", 4),
                ("藍", 5),
                ("洋紅", 6),
                ("白", 7),
                ("顏色 8", 8),
                ("顏色 9", 9),
            ] {
                if ui.button(n).clicked() {
                    set = Some(json!({ "handles": ids, "color": c }));
                    ui.close();
                }
            }
        });
    });
    prop_row(ui, "圖層", |ui| {
        ui.menu_button(layer.clone(), |ui| {
            for l in &layers {
                if ui.button(l).clicked() {
                    set = Some(json!({ "handles": ids, "layer": l }));
                    ui.close();
                }
            }
        });
    });
    prop_row(ui, "線型", |ui| {
        ui.menu_button(linetype.clone(), |ui| {
            for l in std::iter::once("ByLayer".to_string())
                .chain(std::iter::once("ByBlock".to_string()))
                .chain(linetypes.iter().filter(|l| !["ByLayer", "ByBlock"].contains(&l.as_str())).cloned())
            {
                if ui.button(&l).clicked() {
                    set = Some(json!({ "handles": ids, "linetype": l }));
                    ui.close();
                }
            }
        });
    });
    prop_row(ui, "線型比例", |ui| {
        if let Some(v) = edit_field(ui, ui.id().with(("lts", &ids)), lts.clone())
            && let Ok(f) = v.trim().parse::<f64>()
        {
            set = Some(json!({ "handles": ids, "ltscale": f }));
        }
    });
    prop_row(ui, "線寬", |ui| {
        ui.menu_button(lw.clone(), |ui| {
            if ui.button("依圖層").clicked() {
                set = Some(json!({ "handles": ids, "lineweight": "ByLayer" }));
                ui.close();
            }
            for v in cadcraft_doc::Lineweight::STANDARD {
                if ui.button(format!("{:.2} mm", f64::from(v) / 100.0)).clicked() {
                    set = Some(json!({ "handles": ids, "lineweight": f64::from(v) / 100.0 }));
                    ui.close();
                }
            }
        });
    });
    prop_row(ui, "透明度", |ui| {
        value_box(ui, "依圖層", true, true);
    });
    if sel.len() == 1 {
        group(ui, "幾何");
        let num = |ui: &mut egui::Ui, label: &str, key: &str, v: f64, set: &mut Option<Value>, ids: &Vec<String>| {
            prop_row(ui, label, |ui| {
                if let Some(s) = edit_field(ui, ui.id().with((key, ids)), format!("{v:.4}"))
                    && let Ok(f) = s.trim().parse::<f64>()
                {
                    *set = Some(json!({ "handles": ids, key: f }));
                }
            });
        };
        match &e.kind {
            EntityKind::Line(l) => {
                let fixed = |ui: &mut egui::Ui, label: &str, v: f64| {
                    prop_row(ui, label, |ui| {
                        value_box(ui, &format!("{v:.4}"), false, true);
                    })
                };
                prop_row(ui, "起點 X", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("sx", &ids)), format!("{:.4}", l.a.x))
                        && let Ok(f) = s.trim().parse::<f64>()
                    {
                        set = Some(json!({ "handles": ids, "start": [f, l.a.y] }));
                    }
                });
                prop_row(ui, "起點 Y", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("sy", &ids)), format!("{:.4}", l.a.y))
                        && let Ok(f) = s.trim().parse::<f64>()
                    {
                        set = Some(json!({ "handles": ids, "start": [l.a.x, f] }));
                    }
                });
                prop_row(ui, "終點 X", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("ex", &ids)), format!("{:.4}", l.b.x))
                        && let Ok(f) = s.trim().parse::<f64>()
                    {
                        set = Some(json!({ "handles": ids, "end": [f, l.b.y] }));
                    }
                });
                prop_row(ui, "終點 Y", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("ey", &ids)), format!("{:.4}", l.b.y))
                        && let Ok(f) = s.trim().parse::<f64>()
                    {
                        set = Some(json!({ "handles": ids, "end": [l.b.x, f] }));
                    }
                });
                let dlt = l.b.xy() - l.a.xy();
                fixed(ui, "X 差值", dlt.x);
                fixed(ui, "Y 差值", dlt.y);
                fixed(ui, "長度", dlt.len());
                fixed(ui, "Angle", dlt.angle().to_degrees());
            }
            EntityKind::Circle(c) => {
                num(ui, "中心點 X", "cx", c.center.x, &mut set, &ids);
                num(ui, "中心點 Y", "cy", c.center.y, &mut set, &ids);
                num(ui, "半徑", "radius", c.radius, &mut set, &ids);
                num(ui, "直徑", "diameter", c.radius * 2.0, &mut set, &ids);
                prop_row(ui, "圓周長", |ui| {
                    value_box(ui, &format!("{:.4}", c.radius * std::f64::consts::TAU), false, true);
                });
                prop_row(ui, "面積", |ui| {
                    value_box(ui, &format!("{:.4}", c.radius * c.radius * std::f64::consts::PI), false, true);
                });
                if let Some(v) = &mut set
                    && let Some(o) = v.as_object_mut()
                {
                    if let Some(x) = o.remove("cx") {
                        o.insert("center".into(), json!([x, c.center.y]));
                    }
                    if let Some(y) = o.remove("cy") {
                        o.insert("center".into(), json!([c.center.x, y]));
                    }
                }
            }
            EntityKind::Arc(a) => {
                num(ui, "半徑", "radius", a.radius, &mut set, &ids);
                num(ui, "起始角度", "startAngle", a.start.to_degrees(), &mut set, &ids);
                num(ui, "終止角度", "endAngle", a.end.to_degrees(), &mut set, &ids);
            }
            EntityKind::Text(tx) => {
                prop_row(ui, "內容", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("txt", &ids)), tx.value.clone()) {
                        set = Some(json!({ "handles": ids, "text": s }));
                    }
                });
                num(ui, "文字高", "height", tx.height, &mut set, &ids);
                num(ui, "旋轉角度", "rotation", tx.rotation.to_degrees(), &mut set, &ids);
                num(ui, "寬度係數", "widthFactor", tx.width_factor, &mut set, &ids);
            }
            EntityKind::MText(tx) => {
                prop_row(ui, "內容", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("mtxt", &ids)), tx.contents.clone()) {
                        set = Some(json!({ "handles": ids, "text": s }));
                    }
                });
                num(ui, "文字高", "height", tx.height, &mut set, &ids);
                num(ui, "定義寬度", "width", tx.width, &mut set, &ids);
            }
            EntityKind::LwPolyline(pl) => {
                num(ui, "整體寬度", "width", pl.const_width, &mut set, &ids);
                let g = cadcraft_geom::Polyline { vertices: pl.vertices.clone(), closed: pl.closed };
                prop_row(ui, "長度", |ui| {
                    value_box(ui, &format!("{:.4}", g.len()), false, true);
                });
                if pl.closed {
                    prop_row(ui, "面積", |ui| {
                        value_box(ui, &format!("{:.4}", g.area().abs()), false, true);
                    });
                }
                prop_row(ui, "封閉", |ui| {
                    let mut c = pl.closed;
                    let label = if c { "是" } else { "否" };
                    if ui.checkbox(&mut c, label).changed() {
                        set = Some(json!({ "handles": ids, "closed": c }));
                    }
                });
            }
            EntityKind::Hatch(hh) => {
                prop_row(ui, "圖案", |ui| {
                    ui.menu_button(hh.pattern.clone(), |ui| {
                        for pat in cadcraft_doc::library::standard_patterns() {
                            if ui.button(pat.name).clicked() {
                                set = Some(json!({ "handles": ids, "pattern": pat.name }));
                                ui.close();
                            }
                        }
                    });
                });
                num(ui, "比例", "scale", hh.scale, &mut set, &ids);
                num(ui, "角度", "angle", hh.angle.to_degrees(), &mut set, &ids);
            }
            EntityKind::Dimension(dm) => {
                prop_row(ui, "文字覆寫", |ui| {
                    if let Some(s) = edit_field(ui, ui.id().with(("dimtxt", &ids)), dm.text.clone()) {
                        set = Some(json!({ "handles": ids, "textOverride": s }));
                    }
                });
                prop_row(ui, "標註型式", |ui| {
                    value_box(ui, &dm.style, true, true);
                });
            }
            EntityKind::Insert(i) => {
                prop_row(ui, "名稱", |ui| {
                    value_box(ui, &i.block, false, true);
                });
                num(ui, "旋轉角度", "rotation", i.rotation.to_degrees(), &mut set, &ids);
                num(ui, "比例", "scale", i.scale.x, &mut set, &ids);
            }
            _ => {}
        }
    }
    if let Some(p) = set {
        let _ = app.run("properties.set", p);
    }
}
