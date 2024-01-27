use eframe::egui::{self, Style, Ui, Visuals};

use crate::error::FilelistError;

#[derive(Default)]
pub struct Summary {
  pub old_content: String,
  pub new_content: String,
}

pub fn display_summary(summary: Summary, errors: Vec<FilelistError>) {
  let lines_count: f32 = usize::max(
    summary.new_content.lines().count(),
    summary.old_content.lines().count(),
  )
  .max(errors.len()) as f32;

  let width = if errors.is_empty() { 320.0 } else { 640.0 };
  let icon = include_bytes!("../../icon.png");
  let icon = eframe::icon_data::from_png_bytes(icon);

  let mut viewport =
    egui::ViewportBuilder::default().with_inner_size([width, 60.0 + lines_count * 18.0]);

  if let Ok(icon) = icon {
    viewport = viewport.with_icon(icon);
  }

  let options = eframe::NativeOptions {
    viewport,

    ..Default::default()
  };

  // eframe::WindowBuilder::new()
  //   .with_inner_size([width, 60.0 + lines_count * 18.0])
  //   .build(window_target)

  // let icon = {
  //   let (icon_rgba, icon_width, icon_height) = {
  //     let icon = include_bytes!("../../icon.png");
  //     let image = image::load_from_memory(icon)
  //       .expect("Failed to open icon path")
  //       .into_rgba8();
  //     let (width, height) = image.dimensions();
  //     let rgba = image.into_raw();
  //     (rgba, width, height)
  //   };

  //   Some( Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon"))
  // };

  eframe::run_native(
    "tw3-menufilelist-updater",
    options,
    Box::new(|creation_context| {
      let style = Style {
        visuals: Visuals::dark(),
        ..Style::default()
      };
      creation_context.egui_ctx.set_style(style);

      Box::<App>::new(App {
        summary,
        errors: errors.into_iter().map(|e| e.to_string()).collect(),
      })
    }),
  )
  .expect("Unable to open native GUI window");
}

struct App {
  pub summary: Summary,
  pub errors: Vec<String>,
}

impl App {
  fn render_summary(&mut self, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
      ui.label("Filelists updated");
      ui.separator();

      ui.columns(2, |columns| {
        columns[0].vertical_centered(|ui| {
          ui.label("BEFORE");
          ui.text_edit_multiline(&mut self.summary.old_content);
        });

        columns[1].vertical_centered(|ui| {
          ui.label("AFTER");
          ui.text_edit_multiline(&mut self.summary.new_content);
        });
      });
    });
  }

  fn render_errors(&mut self, ui: &mut Ui) {
    ui.vertical_centered_justified(|ui| {
      ui.label("Error(s) while updating the filelists:");

      for error in &self.errors {
        ui.label(error);
      }
    });
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      if self.errors.is_empty() {
        self.render_summary(ui);
      } else {
        self.render_errors(ui);
      }
    });
  }
}
