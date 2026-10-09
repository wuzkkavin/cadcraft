//! Menus (in-window menu bar; the native macOS menu is built from the same tree), UI-only
//! commands and keyboard shortcuts.

use serde_json::{Value, json};

use crate::CadApp;

/// The top-level menus, in order. （繁體中文顯示名稱；選單樹內部組裝仍以英文路徑比對，
/// 經 [`tr`] 轉換後才與此處的中文頂層名稱對上。）
pub const MENUS: &[&str] = &["檔案", "編輯", "檢視", "插入", "格式", "工具", "繪圖", "標註", "修改", "視窗", "說明"];

/// UI-only commands: (id, label, menu path, shortcut).
/// 注意：id、menu path（英文，組裝時經 [`tr`] 翻譯）與 shortcut 不可翻譯，只翻 label。
pub const UI_COMMANDS: &[(&str, &str, &[&str], Option<&str>)] = &[
    ("ui.open", "開啟…", &[], Some("Cmd+O")),
    ("ui.saveas", "另存新檔…", &[], None),
    ("ui.sample", "開啟範例圖面", &["Help", "Open Sample Drawing"], None),
    ("ui.toggle.toolsets", "工具組", &["Window", "Tool Sets"], Some("Cmd+3")),
    ("ui.toggle.palettes", "性質檢視器", &["Window", "Properties Inspector"], Some("Cmd+1")),
    ("ui.toggle.toolbar", "工具列", &["Window", "Tool Bar"], None),
    ("ui.toggle.filetabs", "檔案分頁", &["Window", "File Tab"], None),
    ("ui.toggle.statusbar", "狀態列", &["Window", "Status Bar"], None),
    ("ui.toggle.cmdline", "指令行", &["Window", "Command Line"], Some("Cmd+9")),
    ("ui.toggle.viewcube", "ViewCube", &["View", "ViewCube", "On"], None),
    ("ui.toggle.ucsicon", "UCS 圖示", &["View", "UCS Icon", "On"], None),
    ("ui.toggle.menubar", "視窗內選單列", &["Window", "In-window Menu Bar"], None),
    ("ui.hidepalettes", "隱藏面板", &["Window", "Hide Palettes"], None),
    ("ui.resetpalettes", "重設面板", &["Window", "Reset Palettes"], None),
    ("ui.start", "開始", &["Window", "Start"], None),
    ("ui.dialog.layers", "圖層性質管理員", &["Window", "Layers"], None),
    ("ui.dialog.blocks", "圖塊", &["Window", "Blocks"], None),
    ("ui.dialog.qselect", "快速選取…", &[], None),
    ("ui.dialog.parameters", "參數管理員", &["Window", "Parameters Manager"], None),
    ("ui.dialog.dsettings", "繪圖設定…", &[], None),
    ("ui.dialog.about", "關於 CADCraft", &["Help", "About CADCraft"], None),
    ("ui.dialog.commands", "指令參考", &["Help", "CADCraft Help"], Some("F1")),
    ("ui.noop", "", &[], None),
    ("ui.quit", "結束 CADCraft", &[], Some("Cmd+Q")),
];

pub fn is_ui_command(id: &str) -> bool {
    UI_COMMANDS.iter().any(|c| c.0 == id)
}

/// Run a UI-only command. `None` if `id` isn't one.
pub fn run_ui_command(app: &mut CadApp, id: &str, params: &Value) -> Option<Result<Value, String>> {
    let toggle = |b: &mut bool, p: &Value| {
        *b = p.get("on").and_then(Value::as_bool).unwrap_or(!*b);
    };
    let no_path = params.is_null() || (params.get("path").is_none() && params.get("data").is_none());
    let r = match id {
        "ui.open" | "open" if no_path => {
            let picked = app.services.pick_open.as_ref().and_then(|f| f());
            if let Some(p) = picked {
                app.open_path(&p);
            }
            Ok(Value::Null)
        }
        "ui.saveas" | "saveas" if no_path => {
            let name = app.session.state().map(|s| s.title.clone()).unwrap_or_else(|_| "Drawing.dxf".into());
            let name = if name.contains('.') { name } else { format!("{name}.dxf") };
            if let Some(p) = app.services.pick_save.as_ref().and_then(|f| f(&name)) {
                return Some(app.session.execute("saveas", &json!({ "path": p })).map_err(|e| e.to_string()));
            }
            Ok(Value::Null)
        }
        "qsave" if no_path && app.session.state().is_ok_and(|s| s.path.is_none()) => return run_ui_command(app, "ui.saveas", &Value::Null),
        "ui.sample" => {
            let d = cadcraft_engine::sample::default_sample();
            app.session.open_drawing(d, "支架", None);
            app.canvas.zoom_pending = true;
            app.ui.start_tab = false;
            Ok(Value::Null)
        }
        "ui.toggle.toolsets" => {
            toggle(&mut app.ui.show_toolsets, params);
            Ok(Value::Null)
        }
        "ui.toggle.palettes" => {
            toggle(&mut app.ui.show_palettes, params);
            Ok(Value::Null)
        }
        "ui.toggle.toolbar" => {
            toggle(&mut app.ui.show_toolbar, params);
            Ok(Value::Null)
        }
        "ui.toggle.filetabs" => {
            toggle(&mut app.ui.show_file_tabs, params);
            Ok(Value::Null)
        }
        "ui.toggle.statusbar" => {
            toggle(&mut app.ui.show_status_bar, params);
            Ok(Value::Null)
        }
        "ui.toggle.cmdline" => {
            toggle(&mut app.ui.show_command_line, params);
            Ok(Value::Null)
        }
        "ui.toggle.viewcube" => {
            toggle(&mut app.ui.show_viewcube, params);
            Ok(Value::Null)
        }
        "ui.toggle.ucsicon" => {
            toggle(&mut app.ui.show_ucs_icon, params);
            Ok(Value::Null)
        }
        "ui.toggle.menubar" => {
            toggle(&mut app.ui.in_window_menu, params);
            Ok(Value::Null)
        }
        "ui.hidepalettes" => {
            app.ui.show_palettes = false;
            app.ui.show_toolsets = false;
            Ok(Value::Null)
        }
        "ui.resetpalettes" => {
            let menu = app.ui.in_window_menu;
            app.ui = crate::UiState { in_window_menu: menu, ..Default::default() };
            Ok(Value::Null)
        }
        "ui.start" => {
            app.ui.start_tab = true;
            Ok(Value::Null)
        }
        "ui.dialog.layers"
        | "ui.dialog.blocks"
        | "ui.dialog.dsettings"
        | "ui.dialog.about"
        | "ui.dialog.commands"
        | "ui.dialog.qselect"
        | "ui.dialog.parameters" => {
            app.ui.dialog = Some(id.trim_start_matches("ui.dialog.").to_string());
            Ok(Value::Null)
        }
        "ui.dialog.close" => {
            app.ui.dialog = None;
            Ok(Value::Null)
        }
        "ui.quit" => {
            app.quit_requested = true;
            Ok(Value::Null)
        }
        "ui.noop" => Ok(Value::Null),
        "layer" | "la" | "layers" if params.is_null() => {
            app.ui.dialog = Some("layers".into());
            Ok(Value::Null)
        }
        // Typed or menu-invoked (no parameters) these open their dialogs; JSON calls run the command.
        "qselect" | "qs" if params.is_null() => {
            app.ui.dialog = Some("qselect".into());
            Ok(Value::Null)
        }
        "parameters" | "par" if params.is_null() => {
            app.ui.dialog = Some("parameters".into());
            Ok(Value::Null)
        }
        "parametersclose" if params.is_null() => {
            if app.ui.dialog.as_deref() == Some("parameters") {
                app.ui.dialog = None;
            }
            Ok(Value::Null)
        }
        _ => return None,
    };
    Some(r)
}

/// 繁體中文（台灣）選單翻譯：把 engine `command_specs` 的英文 label 與 menu path
/// segment、以及本檔 UI 指令的英文路徑，翻成使用者看得到的中文。
/// 對照表涵蓋盤點到的全部選單字串（約 320 條）；沒有對到的字串原樣回傳英文，
/// 保證不會把未知標籤弄丟。指令 id、JSON key 與快捷鍵不經此函式。
pub fn tr(s: &str) -> String {
    let out = match s {
        // 頂層選單
        "File" => "檔案",
        "Edit" => "編輯",
        "View" => "檢視",
        "Insert" => "插入",
        "Format" => "格式",
        "Tools" => "工具",
        "Draw" => "繪圖",
        "Dimension" => "標註",
        "Modify" => "修改",
        "Window" => "視窗",
        "Help" => "說明",
        // 檔案／編輯
        "New Drawing..." => "新圖面…",
        "New" => "新增",
        "Open..." => "開啟…",
        "Open" => "開啟",
        "Close" => "關閉",
        "Close All" => "全部關閉",
        "Save" => "儲存",
        "Save As..." => "另存新檔…",
        "Export..." => "輸出…",
        "Export" => "輸出",
        "Export to PDF..." => "輸出至 PDF…",
        "Import..." => "輸入…",
        "Print..." => "列印…",
        "Undo" => "復原",
        "Redo" => "重做",
        "Cut" => "剪下",
        "Copy" => "複製",
        "Copy with Base Point" => "以基點複製",
        "Copy Nested Objects" => "複製巢狀物件",
        "Paste" => "貼上",
        "Paste to Original Coordinates" => "貼至原座標",
        "Clear" => "清除",
        "Select" => "選取",
        "Select All" => "全部選取",
        "Select Objects" => "選取物件",
        "Selection Cycling" => "選取循環",
        "Find..." => "尋找…",
        "Purge" => "清理",
        // 檢視
        "Zoom" => "縮放",
        "Zoom Window" => "窗框縮放",
        "Zoom Extents" => "縮放範圍",
        "Zoom All" => "縮放全部",
        "Zoom Object" => "縮放物件",
        "Zoom Previous" => "前次縮放",
        "Zoom In" => "放大",
        "Zoom Out" => "縮小",
        "Pan" => "平移",
        "Pan Left" => "向左平移",
        "Pan Right" => "向右平移",
        "Pan Up" => "向上平移",
        "Pan Down" => "向下平移",
        "Realtime" => "即時",
        "Previous" => "上一個",
        "Dynamic" => "動態",
        "Scale" => "比例",
        "Center" => "中心",
        "Object" => "物件",
        "In" => "放大",
        "Out" => "縮小",
        "All" => "全部",
        "Extents" => "範圍",
        "Up" => "上",
        "Down" => "下",
        "Left" => "左",
        "Right" => "右",
        "Regen" => "重生",
        "Regen All" => "全部重生",
        "Redraw" => "重繪",
        "Viewports" => "視埠",
        "New Viewports..." => "新視埠…",
        "1 Viewport" => "1 個視埠",
        "2 Viewports" => "2 個視埠",
        "3 Viewports" => "3 個視埠",
        "4 Viewports" => "4 個視埠",
        "Polygonal" => "多邊形",
        "Set View" => "設定檢視",
        "Get View" => "取得檢視",
        "Switch Layout" => "切換配置",
        "Switch Drawing" => "切換圖面",
        // 繪圖
        "Line" => "直線",
        "Polyline" => "聚合線",
        "3D Polyline" => "3D 聚合線",
        "3D Operations" => "3D 操作",
        "Circle" => "圓",
        "Arc" => "弧",
        "Ellipse" => "橢圓",
        "Elliptical Arc" => "橢圓弧",
        "Rectangle" => "矩形",
        "Rectangular" => "矩形",
        "Polygon" => "多邊形",
        "Spline" => "雲形線",
        "Point" => "點",
        "Single Point" => "單點",
        "Multiple Point" => "多點",
        "Construction Line" => "建構線",
        "Ray" => "射線",
        "Donut" => "環",
        "Helix" => "螺旋",
        "Region" => "面域",
        "Revision Cloud" => "修訂雲形",
        "Freehand" => "手繪",
        "Wipeout" => "覆蓋",
        "Multiline" => "多線",
        "Text" => "文字",
        "Single Line Text" => "單行文字",
        "Multiline Text" => "多行文字",
        "Multiline Text..." => "多行文字…",
        "Edit Text..." => "編輯文字…",
        "Table..." => "表格…",
        "Table" => "表格",
        "Hatch..." => "填充線…",
        "Hatch" => "填充線",
        "Hatch Edit" => "填充線編輯",
        "Gradient..." => "漸層…",
        "Boundary..." => "邊界…",
        "Block" => "圖塊",
        "Blocks" => "圖塊",
        "Make..." => "製作…",
        "Block..." => "圖塊…",
        "Define Attributes..." => "定義屬性…",
        "Base" => "基點",
        "Center Line" => "中心線",
        "Center Mark" => "中心標記",
        "Center, Radius" => "中心、半徑",
        "Center, Diameter" => "中心、直徑",
        "Center, Start, Angle" => "中心、起點、角度",
        "Center, Start, End" => "中心、起點、端點",
        "Center, Start, Length" => "中心、起點、長度",
        "Start, Center, Angle" => "起點、中心、角度",
        "Start, Center, End" => "起點、中心、端點",
        "Start, Center, Length" => "起點、中心、長度",
        "Start, End, Angle" => "起點、端點、角度",
        "Start, End, Direction" => "起點、端點、方向",
        "Start, End, Radius" => "起點、端點、半徑",
        "2 Points" => "2 點",
        "3 Points" => "3 點",
        "Tan, Tan, Radius" => "相切、相切、半徑",
        "Tan, Tan, Tan" => "相切、相切、相切",
        "Axis, End" => "軸、端點",
        "Control Vertices" => "控制頂點",
        "Fit Points" => "擬合點",
        "Continue" => "連續",
        // 標註
        "Linear" => "線性",
        "Aligned" => "對齊式",
        "Angular" => "角度式",
        "Angle" => "角度",
        "Radius" => "半徑",
        "Diameter" => "直徑",
        "Arc Length" => "弧長",
        "Ordinate" => "座標式",
        "Baseline" => "基線式",
        "Quick Dimension" => "快速標註",
        "Multileader" => "多重引線",
        "Multileader Style..." => "多重引線型式…",
        "Leader" => "引線",
        "Align Text" => "對齊文字",
        "Home" => "原位",
        "Dimension Style..." => "標註型式…",
        "Dimension Style Override" => "標註型式取代",
        "Dimension Space" => "標註間距",
        "Dimension Text Edit" => "標註文字編輯",
        "Reassociate Dimensions" => "重建標註關聯",
        "Disassociate Dimensions" => "解除標註關聯",
        "Update" => "更新",
        "Override" => "取代",
        // 修改
        "Erase" => "刪除",
        "Move" => "移動",
        "Rotate" => "旋轉",
        "Mirror" => "鏡射",
        "Offset" => "偏移",
        "Trim" => "修剪",
        "Extend" => "延伸",
        "Fillet" => "圓角",
        "Chamfer" => "倒角",
        "Array" => "陣列",
        "Rectangular Array" => "矩形陣列",
        "Polar Array" => "環形陣列",
        "Path Array" => "路徑陣列",
        "Explode" => "分解",
        "Stretch" => "拉伸",
        "Lengthen" => "加長",
        "Break" => "切斷",
        "Break At Point" => "在一點切斷",
        "Join" => "接合",
        "Reverse" => "反轉",
        "Blend" => "混成",
        "Align" => "對齊",
        "Draw Order" => "繪製順序",
        "Bring to Front" => "移到最上層",
        "Send to Back" => "移到最下層",
        "Grip Stretch" => "夾點拉伸",
        "Grip Mirror" => "夾點鏡射",
        "Grip Rotate" => "夾點旋轉",
        "Grip Scale" => "夾點比例",
        "Change Space" => "變更空間",
        "Flatten Objects" => "平面化物件",
        "Delete Duplicate Objects" => "刪除重複物件",
        "Properties" => "性質",
        "Match Properties" => "相符性質",
        "Set Properties" => "設定性質",
        "Attribute" => "屬性",
        "Single..." => "單一…",
        "Block Attribute Manager..." => "圖塊屬性管理員…",
        // 格式／圖層／性質
        "Layer" => "圖層",
        "Layers" => "圖層",
        "Layer Properties Manager" => "圖層性質管理員",
        "Layer States Manager..." => "圖層狀態管理員…",
        "Layer Tools" => "圖層工具",
        "New Layer" => "新圖層",
        "Delete Layer" => "刪除圖層",
        "Make Current" => "設為目前",
        "Make Object's Layer Current" => "將物件圖層設為目前",
        "Change to Current Layer" => "改為目前圖層",
        "Layer Match" => "圖層相符",
        "Isolate Layer" => "隔離圖層",
        "Unisolate Layer" => "取消隔離圖層",
        "Freeze Layer" => "凍結圖層",
        "Thaw All Layers" => "解凍所有圖層",
        "Turn All Layers On" => "開啟所有圖層",
        "Layer Off" => "關閉圖層",
        "Lock Layer" => "鎖定圖層",
        "Unlock Layer" => "解鎖圖層",
        "Previous Layer" => "上一個圖層",
        "Save Layer State" => "儲存圖層狀態",
        "Restore Layer State" => "回復圖層狀態",
        "Delete Layer State" => "刪除圖層狀態",
        "Rename Layer State" => "重新命名圖層狀態",
        "List Layer States" => "列出圖層狀態",
        "Set Layer Properties" => "設定圖層性質",
        "Color..." => "顏色…",
        "Color" => "顏色",
        "Linetype..." => "線型…",
        "Linetype" => "線型",
        "Linetype Scale" => "線型比例",
        "Lineweight..." => "線寬…",
        "Lineweight" => "線寬",
        "Text Style..." => "文字型式…",
        "Table Style..." => "表格型式…",
        "Point Style..." => "點型式…",
        "Units..." => "單位…",
        "Drawing Limits" => "圖面範圍",
        "Rename..." => "重新命名…",
        // 工具／查詢／參數式
        "Inquiry" => "查詢",
        "Distance" => "距離",
        "Area" => "面積",
        "List" => "列表",
        "List Blocks" => "列出圖塊",
        "List Dimension Styles" => "列出標註型式",
        "List Text Styles" => "列出文字型式",
        "List Layouts" => "列出配置",
        "List System Variables" => "列出系統變數",
        "Measure" => "量測",
        "Measure Geometry" => "量測幾何",
        "Divide" => "等分",
        "Count" => "計數",
        "ID Point" => "點座標",
        "Time" => "時間",
        "Status" => "狀態",
        "QuickCalc" => "快速計算機",
        "Quick Select..." => "快速選取…",
        "Quick Select Properties" => "快速選取性質",
        "Quick Properties" => "快速性質",
        "Quick" => "快速",
        "Query Entities" => "查詢圖元",
        "Inspect Drawing" => "檢查圖面",
        "Inspect Constraints" => "檢查約束",
        "Parametric" => "參數式",
        "Geometric Constraint" => "幾何約束",
        "Geometric Constraints" => "幾何約束",
        "Dimensional Constraint" => "尺寸約束",
        "Dimensional Constraints" => "尺寸約束",
        "AutoConstrain" => "自動約束",
        "Constraint Bars" => "約束列",
        "Constraint Settings" => "約束設定",
        "Dynamic Dimensions" => "動態尺寸",
        "Delete Constraints" => "刪除約束",
        "Parameters Manager" => "參數管理員",
        "Coincident" => "重合",
        "Collinear" => "共線",
        "Concentric" => "同心",
        "Fix" => "固定",
        "Parallel" => "平行",
        "Perpendicular" => "垂直",
        "Horizontal" => "水平",
        "Vertical" => "垂直",
        "Tangent" => "相切",
        "Smooth" => "平滑",
        "Symmetric" => "對稱",
        "Equal" => "相等",
        "Show All" => "全部顯示",
        "Hide All" => "全部隱藏",
        "Show All Constraint Bars" => "顯示所有約束列",
        "Hide All Constraint Bars" => "隱藏所有約束列",
        "Show All Dynamic Dimensions" => "顯示所有動態尺寸",
        "Hide All Dynamic Dimensions" => "隱藏所有動態尺寸",
        // 設定／配置／出圖
        "Settings" => "設定",
        "Drafting Settings..." => "繪圖設定…",
        "Drafting Settings" => "繪圖設定",
        "Grid Display" => "格點顯示",
        "Snap Mode" => "鎖點模式",
        "Ortho Mode" => "正交模式",
        "Polar Tracking" => "極座標追蹤",
        "Object Snap" => "物件鎖點",
        "Object Snap Tracking" => "物件鎖點追蹤",
        "Dynamic Input" => "動態輸入",
        "Isometric Drafting" => "等角圖",
        "Show/Hide Lineweight" => "顯示/隱藏線寬",
        "Show/Hide Transparency" => "顯示/隱藏透明度",
        "Layout" => "配置",
        "New Layout" => "新配置",
        "Copy Layout" => "複製配置",
        "Delete Layout" => "刪除配置",
        "Rename Layout" => "重新命名配置",
        "Page Setup Manager..." => "頁面設定管理員…",
        "Page Setup Manager" => "頁面設定管理員",
        "Model Space (in viewport)" => "模型空間（視埠內）",
        "Paper Space" => "圖紙空間",
        "Viewport Layer Freeze" => "視埠圖層凍結",
        "Viewport Properties" => "視埠性質",
        "Insert Block..." => "插入圖塊…",
        "Write Block" => "寫入圖塊",
        "Insert Row" => "插入列",
        "Insert Column" => "插入欄",
        "Delete Row" => "刪除列",
        "Delete Column" => "刪除欄",
        "Merge Cells" => "合併儲存格",
        "Unmerge Cells" => "取消合併儲存格",
        "Set Table Cell" => "設定表格儲存格",
        "Set Current Dimension Style" => "設定目前標註型式",
        "Set Current Text Style" => "設定目前文字型式",
        "Delete Dimension Style" => "刪除標註型式",
        "Delete Text Style" => "刪除文字型式",
        "Rename Dimension Style" => "重新命名標註型式",
        "Rename Text Style" => "重新命名文字型式",
        "Get Variable" => "取得變數",
        "Set Variable" => "設定變數",
        "Region/Mass Properties" => "面域/質量性質",
        "Drawing as bytes" => "圖面位元組資料",
        // 視窗／說明（UI 指令路徑 segment）
        "Tool Sets" => "工具組",
        "Properties Inspector" => "性質檢視器",
        "Tool Bar" => "工具列",
        "File Tab" => "檔案分頁",
        "Status Bar" => "狀態列",
        "Command Line" => "指令行",
        "UCS Icon" => "UCS 圖示",
        // ViewCube 是 AutoCAD 慣用的功能名稱，中文版亦保留原名。
        "ViewCube" => "ViewCube",
        "On" => "開啟",
        "In-window Menu Bar" => "視窗內選單列",
        "Hide Palettes" => "隱藏面板",
        "Reset Palettes" => "重設面板",
        "Start" => "開始",
        "Open Sample Drawing" => "開啟範例圖面",
        "About CADCraft" => "關於 CADCraft",
        "CADCraft Help" => "CADCraft 說明",
        "Command Reference" => "指令參考",
        _ => return s.to_string(),
    };
    out.to_string()
}

/// A menu entry.
#[derive(Clone, Debug)]
pub enum Entry {
    Item { label: String, id: String, shortcut: Option<String>, enabled: bool },
    Sub { label: String, children: Vec<Entry> },
}

/// The full menu tree from the command registry plus UI commands, in registration order.
pub fn tree(app: &CadApp) -> Vec<(String, Vec<Entry>)> {
    let mut out: Vec<(String, Vec<Entry>)> = MENUS.iter().map(|m| (m.to_string(), Vec::new())).collect();
    let mut add = |path: &[&str], label: &str, id: &str, shortcut: Option<&str>, enabled: bool| {
        let Some((top, rest)) = path.split_first() else { return };
        // 頂層名稱在 MENUS 裡是中文，engine/UI 路徑是英文：先翻譯再比對。
        let top = tr(top);
        let Some((_, entries)) = out.iter_mut().find(|(m, _)| *m == top) else { return };
        let mut cur = entries;
        let n = rest.len();
        for (i, seg) in rest.iter().enumerate() {
            // 每個路徑 segment 與最後的標籤都翻成中文後才存入選單樹；
            // 子選單的去重比對也用翻譯後的字串，同一個英文 segment 只會建一個子選單。
            let seg = tr(seg);
            if i + 1 == n {
                cur.push(Entry::Item {
                    label: if seg.is_empty() { tr(label) } else { seg },
                    id: id.to_string(),
                    shortcut: shortcut.map(str::to_string),
                    enabled,
                });
            } else {
                let pos = cur.iter().position(|e| matches!(e, Entry::Sub { label, .. } if *label == seg));
                let idx = match pos {
                    Some(p) => p,
                    None => {
                        cur.push(Entry::Sub { label: seg, children: Vec::new() });
                        cur.len() - 1
                    }
                };
                let Some(Entry::Sub { children, .. }) = cur.get_mut(idx) else { return };
                cur = children;
            }
        }
    };
    for c in cadcraft_engine::command_specs() {
        if !c.menu.is_empty() {
            add(c.menu, c.label, c.id, c.shortcut, (c.enabled)(&app.session).is_ok());
        }
    }
    for (id, label, menu, sc) in UI_COMMANDS {
        if !menu.is_empty() {
            add(menu, label, id, *sc, true);
        }
    }
    out
}

fn entry_ui(ui: &mut egui::Ui, e: &Entry, clicked: &mut Option<String>) {
    match e {
        Entry::Item { label, id, shortcut, enabled } => {
            let mut b = egui::Button::new(label);
            if let Some(s) = shortcut {
                b = b.shortcut_text(s.replace("Cmd+", "⌘").replace("Shift+", "⇧"));
            }
            if ui.add_enabled(*enabled, b).clicked() {
                *clicked = Some(id.clone());
                ui.close();
            }
        }
        Entry::Sub { label, children } => {
            ui.menu_button(label, |ui| {
                for c in children {
                    entry_ui(ui, c, clicked);
                }
            });
        }
    }
}

pub fn menu_bar(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = crate::theme::Tokens::get();
    let tree = tree(app);
    let mut clicked = None;
    egui::Panel::top("cc_menubar").exact_size(22.0).frame(egui::Frame::NONE.fill(t.chrome_dark).inner_margin(egui::Margin::symmetric(6, 0))).show(
        ui,
        |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                for (name, entries) in &tree {
                    ui.menu_button(name, |ui| {
                        for e in entries {
                            entry_ui(ui, e, &mut clicked);
                        }
                    });
                }
            });
        },
    );
    if let Some(id) = clicked {
        activate(app, &id);
    }
}

/// Like choosing a menu item: starts the command interactively.
pub fn activate(app: &mut CadApp, id: &str) {
    app.cmd.buffer.clear();
    app.start(id);
}

/// Keyboard shortcuts and function keys.
pub fn shortcuts(app: &mut CadApp, ctx: &egui::Context) {
    use egui::{Key, KeyboardShortcut, Modifiers};
    let sc = |m: Modifiers, k: Key| KeyboardShortcut::new(m, k);
    let cmd = Modifiers::COMMAND;
    let cmd_shift = Modifiers::COMMAND | Modifiers::SHIFT;
    let pairs: &[(KeyboardShortcut, &str)] = &[
        (sc(cmd_shift, Key::Z), "redo"),
        (sc(cmd, Key::Z), "undo"),
        (sc(cmd, Key::Y), "redo"),
        (sc(cmd, Key::N), "new"),
        (sc(cmd, Key::O), "ui.open"),
        (sc(cmd_shift, Key::S), "ui.saveas"),
        (sc(cmd, Key::S), "qsave"),
        (sc(cmd, Key::A), "selectall"),
        (sc(cmd_shift, Key::C), "copybase"),
        (sc(cmd, Key::C), "copyclip"),
        (sc(cmd, Key::X), "cutclip"),
        (sc(cmd, Key::V), "pasteclip"),
        (sc(cmd, Key::Num1), "ui.toggle.palettes"),
        (sc(cmd, Key::Num3), "ui.toggle.toolsets"),
        (sc(cmd, Key::Num9), "ui.toggle.cmdline"),
        (sc(Modifiers::NONE, Key::F1), "ui.dialog.commands"),
        (sc(Modifiers::NONE, Key::F3), "osnap"),
        (sc(Modifiers::NONE, Key::F7), "grid"),
        (sc(Modifiers::NONE, Key::F8), "ortho"),
        (sc(Modifiers::NONE, Key::F9), "snap"),
        (sc(Modifiers::NONE, Key::F10), "polar"),
        (sc(Modifiers::NONE, Key::F11), "otrack"),
        (sc(Modifiers::NONE, Key::F12), "dynmode"),
    ];
    if ctx.egui_wants_keyboard_input() {
        return;
    }
    let mut fire = Vec::new();
    ctx.input_mut(|i| {
        for (s, id) in pairs {
            if i.consume_shortcut(s) {
                fire.push(*id);
            }
        }
    });
    for id in fire {
        // Toggles run transparently (they don't cancel the active command).
        let transparent = ["osnap", "grid", "ortho", "snap", "polar", "otrack", "dynmode"].contains(&id);
        if transparent || id.starts_with("ui.") {
            match app.session.execute(id, &json!({})) {
                Ok(v) => {
                    if let Some(m) = v.get("message").and_then(Value::as_str) {
                        app.session.echo(m.to_string());
                    }
                }
                Err(_) => {
                    let _ = app.run(id, json!({}));
                }
            }
        } else {
            activate(app, id);
        }
    }
}
