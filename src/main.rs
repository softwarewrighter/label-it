use clap::Parser;
use eframe::{NativeOptions, egui};
use egui::{Color32, RichText};

#[derive(Parser, Debug)]
#[command(
    name = "label-it",
    about = "A tiny movable label window for screen recordings."
)]
struct Args {
    #[arg(long, default_value = "Claude Code")]
    text: String,

    #[arg(long, default_value = "#cc0000")]
    bg: String,

    #[arg(long, default_value = "#ffffff")]
    fg: String,

    #[arg(long, default_value_t = 28.0)]
    font_size: f32,

    #[arg(long, default_value_t = 260.0)]
    width: f32,

    #[arg(long, default_value_t = 80.0)]
    height: f32,

    #[arg(long)]
    undecorated: bool,

    #[arg(long)]
    always_on_top: bool,

    #[arg(long, default_value = "Label")]
    title: String,
}

fn parse_hex_color(s: &str) -> Result<Color32, String> {
    let s = s.trim();
    let hex = s.strip_prefix('#').unwrap_or(s);

    let (r, g, b, a) = match hex.len() {
        6 => (
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| "bad R")?,
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| "bad G")?,
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| "bad B")?,
            255,
        ),
        8 => (
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| "bad R")?,
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| "bad G")?,
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| "bad B")?,
            u8::from_str_radix(&hex[6..8], 16).map_err(|_| "bad A")?,
        ),
        _ => return Err("color must be #RRGGBB or #RRGGBBAA".into()),
    };

    Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
}

struct LabelApp {
    text: String,
    bg: Color32,
    fg: Color32,
    font_size: f32,
    undecorated: bool,
    needs_resize: bool,
}

impl eframe::App for LabelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // On first frame, measure text and resize window to fit
        if self.needs_resize {
            self.needs_resize = false;
            let font_id = egui::FontId::proportional(self.font_size);
            let text_width = ctx.fonts(|f| {
                f.layout_no_wrap(self.text.clone(), font_id, Color32::WHITE)
                    .rect
                    .width()
            });
            let padding = 40.0; // horizontal padding
            let new_width = text_width + padding;
            let new_height = self.font_size + 50.0; // vertical padding
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                new_width, new_height,
            )));
        }

        // Start a window drag only once per click (not every frame while held)
        if self.undecorated && ctx.input(|i| i.pointer.primary_pressed()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.bg))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space((ui.available_height() - self.font_size).max(0.0) / 2.0);
                    ui.label(
                        RichText::new(&self.text)
                            .color(self.fg)
                            .size(self.font_size)
                            .strong(),
                    );
                });
            });

        ctx.request_repaint_after(std::time::Duration::from_millis(250));
    }
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();

    let bg = parse_hex_color(&args.bg).unwrap_or(Color32::from_rgb(204, 0, 0));
    let fg = parse_hex_color(&args.fg).unwrap_or(Color32::WHITE);

    let mut options = NativeOptions::default();
    options.viewport.inner_size = Some(egui::vec2(args.width, args.height));
    options.viewport.decorations = Some(!args.undecorated);
    if args.always_on_top {
        options.viewport.window_level = Some(egui::WindowLevel::AlwaysOnTop);
    }

    let app = LabelApp {
        text: args.text,
        bg,
        fg,
        font_size: args.font_size,
        undecorated: args.undecorated,
        needs_resize: true,
    };

    eframe::run_native(&args.title, options, Box::new(|_| Ok(Box::new(app))))
}
