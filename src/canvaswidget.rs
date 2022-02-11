use druid::kurbo::{PathEl, BezPath};
use druid::{Color, Event, MouseButton, Point, RenderContext, Size, Widget};

use crate::canvasstate::{CanvasState, Stroke};

#[derive(Debug, Clone, Default)]
pub struct CanvasWidget {
    pub drawing: bool,
    pub elem_buf: Vec<Point>,
}

impl Widget<CanvasState> for CanvasWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut CanvasState,
        _env: &druid::Env,
    ) {
        match event {
            Event::MouseDown(e) => {
                if e.button == MouseButton::Left {
                    self.drawing = true;
                    data.strokes.borrow_mut().push(Stroke::new_w_move(e.pos));
                }
            }
            Event::MouseMove(e) => {
                if self.drawing {
                    if self.elem_buf.len() < 3 {
                        self.elem_buf.push(e.pos);
                    } else {
                        if let Some(last_stroke) = data.strokes.borrow_mut().last_mut() {
                            last_stroke.bez_path.borrow_mut().push(PathEl::CurveTo(
                                self.elem_buf[0],
                                self.elem_buf[1],
                                self.elem_buf[2],
                            ));
                        }
                        self.elem_buf.clear();
                    }
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_e) => {
                self.drawing = false;
                self.elem_buf.clear();
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &CanvasState,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &CanvasState,
        _data: &CanvasState,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &CanvasState,
        _env: &druid::Env,
    ) -> druid::Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &CanvasState, _env: &druid::Env) {
        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        // Note: ctx also has a `clear` method, but that clears the whole context,
        // and we only want to clear this widget's area.
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        // Create a color
        let stroke_color = Color::rgb8(0, 0, 192);

        data.strokes.borrow().iter().for_each(|stroke| {
            let mut bezpath = BezPath::new();
            stroke.bez_path.borrow().flatten(0.25, |line_elem| {
                bezpath.push(line_elem);
            });
            ctx.stroke(bezpath, &stroke_color, 2.0);
        });
    }
}
