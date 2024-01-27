use eframe::egui::{self, Style, Ui, Visuals};

use crate::error::FilelistError;

#[derive(Default)]
pub struct Summary {
  dx11: Option<ContentDiff>,
  dx12: Option<ContentDiff>,
}

impl Summary {
  pub fn set_dx11_diff(&mut self, old: String, new: String) {
    self.dx11 = Some(ContentDiff { old, new });
  }

  pub fn set_dx12_diff(&mut self, old: String, new: String) {
    self.dx12 = Some(ContentDiff { old, new });
  }

  pub fn is_empty(&self) -> bool {
    self.dx11.is_none() && self.dx12.is_none()
  }

  pub fn lines(&self) -> usize {
    self.dx11.as_ref().map(|d| d.lines()).unwrap_or_default()
      + self.dx12.as_ref().map(|d| d.lines()).unwrap_or_default()
  }

  pub fn render(&mut self, ui: &mut Ui) {
    if let Some(diff) = &mut self.dx11 {
      ui.label("DX11");
      diff.render(ui);
    }

    if let Some(diff) = &mut self.dx12 {
      if self.dx11.is_some() {
        ui.separator();
      }

      ui.label("DX12");
      diff.render(ui);
    }
  }
}

struct ContentDiff {
  old: String,
  new: String,
}

impl ContentDiff {
  pub fn lines(&self) -> usize {
    self.new.lines().count().max(self.old.lines().count())
  }

  pub fn render(&mut self, ui: &mut Ui) {
    ui.columns(2, |columns| {
      columns[0].vertical_centered(|ui| {
        ui.label("BEFORE");
        ui.text_edit_multiline(&mut self.old);
      });

      columns[1].vertical_centered(|ui| {
        ui.label("AFTER");
        ui.text_edit_multiline(&mut self.new);
      });
    });
  }
}

pub fn display_summary(summary: Summary, errors: Vec<FilelistError>) {
  let lines_count: f32 = summary.lines().max(errors.len()) as f32;

  let width = if errors.is_empty() { 320.0 } else { 640.0 };
  let icon = include_bytes!("../../icon.png");
  let icon = eframe::icon_data::from_png_bytes(icon);

  let mut viewport =
    egui::ViewportBuilder::default().with_inner_size([width, 90.0 + lines_count * 18.0]);

  if let Ok(icon) = icon {
    viewport = viewport.with_icon(icon);
  }

  let options = eframe::NativeOptions {
    viewport,

    ..Default::default()
  };

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
      ui.label("Filelists successfully updated");
      ui.separator();

      self.summary.render(ui);
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
