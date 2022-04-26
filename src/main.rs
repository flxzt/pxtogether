mod dialogs;

use std::rc::Rc;

use iced::pure::widget::canvas;
use iced::pure::{self, Application};

#[derive(Debug, Clone)]
struct PxTogetherApp {
    state: State,
}

impl Default for PxTogetherApp {
    fn default() -> Self {
        Self {
            state: State::default(),
        }
    }
}

impl PxTogetherApp {}
#[derive(Debug, Clone)]
struct State {
    rows: usize,
    columns: usize,
    pixel_size: iced::Size,
    grid: Rc<Grid>,
    current_color: iced::Color,
    history: Vec<Rc<Grid>>,
    history_pos: Option<usize>,
}

impl Default for State {
    fn default() -> Self {
        let rows = 16;
        let columns = 16;
        Self {
            rows,
            columns,
            pixel_size: iced::Size::new(40.0, 40.0),
            grid: Rc::new(Grid::new(rows, columns)),
            current_color: iced::Color::new(0.0, 1.0, 1.0, 1.0),
            history: vec![],
            history_pos: None,
        }
    }
}

impl State {
    fn grid_size(&self) -> iced::Size {
        iced::Size::new(
            self.columns as f32 * self.pixel_size.width + 1.0,
            self.rows as f32 * self.pixel_size.height + 1.0,
        )
    }

    /// In coordinate space of  the grid, returns Option<(column, row)>
    fn pos_on_grid(&self, position: iced::Point) -> Option<(usize, usize)> {
        if iced::Rectangle::new(iced::Point::new(0.0, 0.0), self.grid_size()).contains(position) {
            let column = (((position.x) / self.pixel_size.width).floor() as usize)
                .clamp(0, self.columns - 1);
            let row =
                (((position.y) / self.pixel_size.height).floor() as usize).clamp(0, self.rows - 1);

            Some((column, row))
        } else {
            None
        }
    }

    /// saves the current state in the history
    fn record(&mut self) {
        self.history_pos = None;

        if !self
            .history
            .last()
            .map(|last| Rc::ptr_eq(last, &self.grid))
            .unwrap_or(false)
        {
            self.history.push(Rc::clone(&self.grid));
        } else {
            println!("state has not changed, no need to record");
        }
    }

    /// undoes to the last recording (last element of history).
    ///
    /// **emacs style redo:**  
    ///
    /// The current state gets pushed onto the the history before restoring to the previous state.
    /// To redo undone action, you have to record to jump to the latest history position with record() and then undo
    fn undo(&mut self) {
        let index = self.history_pos.unwrap_or(self.history.len());

        if index > 0 {
            let current = Rc::clone(&self.grid);
            let prev = Rc::clone(&self.history[index - 1]);

            self.history.push(current);

            self.grid = prev;
            self.history_pos = Some(index - 1);
        } else {
            println!("no history, can't undo");
        }
    }

    fn clear_history(&mut self) {
        self.history.clear();
        self.history_pos = None
    }
}

#[derive(Debug, Clone, Copy)]
enum CanvasState {
    Idle,
    Drawing,
    Erasing,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self::Idle
    }
}

impl canvas::Program<Message> for State {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(self.grid_size());

        frame.fill_rectangle(iced::Point::new(0.0, 0.0), frame.size(), iced::Color::WHITE);

        for column in 0..self.columns {
            for row in 0..self.rows {
                let pixel_rect = canvas::Path::rectangle(
                    iced::Point::new(
                        self.pixel_size.width * row as f32 + 1.0,
                        self.pixel_size.height * column as f32 + 1.0,
                    ),
                    self.pixel_size,
                );

                frame.fill(
                    &pixel_rect,
                    iced::Color::from(self.grid.pixels[row][column].color),
                );
                frame.stroke(
                    &pixel_rect,
                    canvas::Stroke::default().with_color(iced::Color::new(0.7, 0.7, 0.7, 1.0)),
                );
            }
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        cursor: iced::canvas::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        if let canvas::Event::Mouse(mouse_event) = event {
            match (state, mouse_event) {
                (state, iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => {
                    if let Some(position) = cursor.position() {
                        if let Some((column, row)) = self.pos_on_grid(iced::Point {
                            x: position.x - bounds.x,
                            y: position.y - bounds.y,
                        }) {
                            *state = CanvasState::Drawing;

                            return (
                                canvas::event::Status::Captured,
                                Some(Message::PutPixel {
                                    new_pixel: Pixel {
                                        color: self.current_color.into(),
                                    },
                                    pos: Pos { row, column },
                                    record: true,
                                }),
                            );
                        }
                    }
                }
                (state, iced::mouse::Event::ButtonPressed(iced::mouse::Button::Right)) => {
                    if let Some(position) = cursor.position() {
                        if let Some((column, row)) = self.pos_on_grid(iced::Point {
                            x: position.x - bounds.x,
                            y: position.y - bounds.y,
                        }) {
                            *state = CanvasState::Erasing;

                            return (
                                canvas::event::Status::Captured,
                                Some(Message::PutPixel {
                                    new_pixel: Pixel {
                                        color: iced::Color::TRANSPARENT.into(),
                                    },
                                    pos: Pos { row, column },
                                    record: true,
                                }),
                            );
                        }
                    }
                }
                (CanvasState::Drawing, iced::mouse::Event::CursorMoved { position }) => {
                    if let Some((column, row)) = self.pos_on_grid(iced::Point {
                        x: position.x - bounds.x,
                        y: position.y - bounds.y,
                    }) {
                        return (
                            canvas::event::Status::Captured,
                            Some(Message::PutPixel {
                                new_pixel: Pixel {
                                    color: self.current_color.into(),
                                },
                                pos: Pos { row, column },
                                record: false,
                            }),
                        );
                    }
                }
                (CanvasState::Erasing, iced::mouse::Event::CursorMoved { position }) => {
                    if let Some((column, row)) = self.pos_on_grid(iced::Point {
                        x: position.x - bounds.x,
                        y: position.y - bounds.y,
                    }) {
                        return (
                            canvas::event::Status::Captured,
                            Some(Message::PutPixel {
                                new_pixel: Pixel {
                                    color: iced::Color::TRANSPARENT.into(),
                                },
                                pos: Pos { row, column },
                                record: false,
                            }),
                        );
                    }
                }
                (
                    state @ CanvasState::Drawing | state @ CanvasState::Erasing,
                    iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)
                    | iced::mouse::Event::ButtonReleased(iced::mouse::Button::Right),
                ) => {
                    *state = CanvasState::Idle;

                    return (canvas::event::Status::Captured, None);
                }
                _ => {}
            }
        }

        (canvas::event::Status::Ignored, None)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Grid {
    pixels: Vec<Vec<Rc<Pixel>>>,
}

impl Grid {
    fn new(rows: usize, columns: usize) -> Self {
        Self {
            pixels: vec![vec![Rc::new(Pixel::default()); rows]; columns],
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct Pixel {
    color: PixelColor,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
struct PixelColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl From<iced::Color> for PixelColor {
    fn from(color: iced::Color) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl From<PixelColor> for iced::Color {
    fn from(color: PixelColor) -> Self {
        Self::new(color.r, color.g, color.b, color.a)
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            color: iced::Color::TRANSPARENT.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    column: usize,
}

#[derive(Debug, Clone)]
enum Message {
    None,
    PutPixel {
        new_pixel: Pixel,
        pos: Pos,
        record: bool,
    },
    ChangeRed(f32),
    ChangeGreen(f32),
    ChangeBlue(f32),
    Record,
    Undo,
    Clear,
    OpenFileDialog,
    OpenFileData(Vec<u8>),
    SaveFile,
}

impl pure::Application for PxTogetherApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (PxTogetherApp::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("PxTogether")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::None => {}
            Message::PutPixel {
                new_pixel,
                pos,
                record,
            } => {
                if record {
                    self.state.record();
                }

                *Rc::make_mut(
                    &mut Rc::make_mut(&mut self.state.grid).pixels[pos.column][pos.row],
                ) = new_pixel;
            }
            Message::Record => {
                println!("record");
                self.state.record();
            }
            Message::Undo => {
                println!("undo");
                self.state.undo();
                //self.state.record();
            }
            Message::Clear => {
                println!("clear");

                self.state.record();

                *Rc::make_mut(&mut self.state.grid) =
                    Grid::new(self.state.rows, self.state.columns);
            }
            Message::ChangeRed(red) => {
                self.state.current_color.r = red;
            }
            Message::ChangeGreen(green) => {
                self.state.current_color.g = green;
            }
            Message::ChangeBlue(blue) => {
                self.state.current_color.b = blue;
            }
            Message::OpenFileDialog => {
                return iced::Command::perform(dialogs::open_file(), |data| match data {
                    Some(data) => Message::OpenFileData(data),
                    None => Message::None,
                })
            }
            Message::OpenFileData(grid_data) => match serde_json::from_slice(&grid_data) {
                Ok(grid) => {
                    self.state.grid = Rc::new(grid);
                    self.state.clear_history();
                }
                Err(e) => log::error!("reading file failed with Err {}", e),
            },
            Message::SaveFile => match serde_json::to_string(&*self.state.grid) {
                Ok(grid_data) => {
                    return iced::Command::perform(
                        dialogs::save_file(grid_data.into_bytes()),
                        |res| {
                            if let Err(e) = res {
                                log::error!("saving file failed with Err {}", e);
                            }
                            Message::None
                        },
                    );
                }
                Err(e) => log::error!(
                    "serializing grid while saving to file failed with Err {}",
                    e
                ),
            },
        }

        iced::Command::none()
    }

    fn view(&self) -> pure::Element<'_, Self::Message> {
        let content = pure::column()
            .push(
                pure::row()
                    .padding(12)
                    .spacing(12)
                    .align_items(iced::Alignment::Start)
                    .push(
                        pure::container(
                            pure::button("Open file").on_press(Message::OpenFileDialog),
                        )
                        .center_y(),
                    )
                    .push(
                        pure::container(pure::button("Save to file").on_press(Message::SaveFile))
                            .center_y(),
                    ),
            )
            .push(pure::horizontal_rule(0))
            .push(
                pure::container(
                    pure::widget::Canvas::new(&self.state)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
                .padding(6)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
            )
            .push(pure::horizontal_rule(0))
            .push(
                pure::row()
                    .align_items(iced::Alignment::Start)
                    .push(
                        pure::row()
                            .padding(12)
                            .spacing(12)
                            .align_items(iced::Alignment::Start)
                            .width(iced::Length::FillPortion(1))
                            .push(
                                pure::container(pure::button("Clear").on_press(Message::Clear))
                                    .center_y(),
                            )
                            .push(
                                pure::container(pure::button("Record").on_press(Message::Record))
                                    .center_y(),
                            )
                            .push(
                                pure::container(pure::button("Undo").on_press(Message::Undo))
                                    .center_y(),
                            ),
                    )
                    .push(
                        pure::row()
                            .padding(12)
                            .spacing(12)
                            .align_items(iced::Alignment::Fill)
                            .width(iced::Length::FillPortion(3))
                            .push(pure::text(format!(
                                "red: {:.3}",
                                self.state.current_color.r
                            )))
                            .push(
                                pure::slider(0.0..=1.0, self.state.current_color.r, |value| {
                                    Message::ChangeRed(value)
                                })
                                .step(0.01),
                            )
                            .push(pure::text(format!(
                                "green: {:.3}",
                                self.state.current_color.g
                            )))
                            .push(
                                pure::slider(0.0..=1.0, self.state.current_color.g, |value| {
                                    Message::ChangeGreen(value)
                                })
                                .step(0.01),
                            )
                            .push(pure::text(format!(
                                "blue: {:.3}",
                                self.state.current_color.b
                            )))
                            .push(
                                pure::slider(0.0..=1.0, self.state.current_color.b, |value| {
                                    Message::ChangeBlue(value)
                                })
                                .step(0.01),
                            ),
                    ),
            );

        content.into()
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    PxTogetherApp::run(iced::Settings::default())?;

    Ok(())
}
