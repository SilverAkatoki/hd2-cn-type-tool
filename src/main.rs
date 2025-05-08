#![windows_subsystem = "windows"] // 隐藏控制台窗口
use eframe::egui::{self, FontDefinitions, FontFamily, FontTweak};
use enigo::{Enigo, Keyboard, Settings};
use rdev::{EventType, listen};
use std::sync::{Arc, RwLock};

fn configure_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "chinese".to_owned(),
        Arc::new(
            egui::FontData::from_static(include_bytes!(
                "..\\assets\\fonts\\SourceHanSansCN-Regular.otf"
            ))
            .tweak(FontTweak {
                scale: 1.25,
                baseline_offset_factor: 0.25,
                ..Default::default()
            }),
        ),
    );

    // 将中文字体设置为优先字体
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "chinese".to_owned());

    fonts
}

fn main() -> eframe::Result {
    // 窗口参数
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 140.0])
            .with_resizable(false), // 不可缩放
        ..Default::default()
    };

    let fonts = configure_fonts();
    let input_text = Arc::new(RwLock::new(String::new()));
    let input_text_clone = Arc::clone(&input_text);

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // 监听键盘事件的线程
    std::thread::spawn(move || {
        listen(move |event| match event.event_type {
            EventType::KeyPress(key) => {
                if key == rdev::Key::F8 {
                    if let Ok(text) = input_text_clone.read() {
                        let _ = enigo.text(&text.clone());
                    }
                }
            }
            _ => {}
        })
        .unwrap();
    });

    eframe::run_simple_native("Alt 输入器", options, move |ctx, _| {
        ctx.set_fonts(fonts.clone());
        // ctx.set_visuals(egui::Visuals::dark());
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("用 F8 键将下面的文本发送到当前窗口。");
                let mut text = input_text.write().unwrap();
                ui.text_edit_multiline(&mut *text);
                *text = text.replace('\n', ""); // 去掉换行符
            });
        });
    })
}
