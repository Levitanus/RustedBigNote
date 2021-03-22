use tracing::error;

use druid::{
    kurbo::Line,
    widget::{Container, FillStrat, Flex, Painter, Svg, SvgData, WidgetExt, WidgetWrapper},
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
    WidgetPod, WindowDesc,
};
mod note;
mod staff;

pub struct ZSvg<T> {
    child: WidgetPod<T, Svg>,
}

impl<T: Data> ZSvg<T> {
    fn new(child: Svg) -> Self {
        ZSvg {
            child: WidgetPod::new(child),
        }
    }
}

impl<T: Data> Widget<T> for ZSvg<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        // For now, just copy of padding.
        bc.debug_check("Padding");

        let hpad = 10.0;
        let vpad = 10.0;

        let child_bc = bc.shrink((hpad, vpad));
        let size = self.child.layout(ctx, &child_bc, data, env);
        let origin = Point::new(10.0, 10.0);
        self.child.set_origin(ctx, data, env, origin);

        let my_size = Size::new(size.width + hpad, size.height + vpad);
        let my_insets = self.child.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);
        my_size
        // Size::new(Default::default(), Default::default())
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let bounds = ctx.size().to_rect();
        println!("{:?}", bounds);
        ctx.fill(bounds, &Color::YELLOW);
        let y = (bounds.y1 - bounds.y0) / 2.0;
        let line = Line::new(Point::new(bounds.x0, y), Point::new(bounds.x1, y));
        ctx.stroke(line, &env.get(druid::theme::PRIMARY_DARK), 1.0);
        self.child.paint(ctx, data, env);
    }
}

fn build_ui() -> impl Widget<Color> {
    let clef_svg = match include_str!("../assets/treble clef.svg").parse::<SvgData>() {
        Ok(svg) => svg,
        Err(err) => {
            error!("{}", err);
            error!("Using an empty SVG instead.");
            SvgData::default()
        }
    };
    let svg_widget = Svg::new(clef_svg.clone());
    Container::new(ZSvg::new(svg_widget)).background(Color::WHITE)
}

fn main() {
    let data = Color::BLACK;
    let window = WindowDesc::new(build_ui());
    AppLauncher::with_window(window).launch((data));
}
