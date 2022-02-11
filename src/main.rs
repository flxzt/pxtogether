pub mod canvasstate;
pub mod canvaswidget;
use canvasstate::CanvasState;
use canvaswidget::CanvasWidget;
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};

const APP_NAME: LocalizedString<CanvasState> = LocalizedString::new("CollaDraw");

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(APP_NAME)
        .window_size((400.0, 400.0));

    // create the initial app state
    let canvas_state = CanvasState::default();

    // start the application
    AppLauncher::with_window(main_window)
        .launch(canvas_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<CanvasState> {
    CanvasWidget::default()
}