use tracing::error;

use druid::{
    kurbo::Line,
    widget::{
        Button, Container, CrossAxisAlignment, FillStrat, Flex, Label, LabelText, Padding, Painter,
        SizedBox, Svg, SvgData, WidgetExt, WidgetWrapper,
    },
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
    WidgetPod, WindowDesc,
};
mod midi;
mod note;
mod staff;

struct Root {
    staff: WidgetPod<Option<note::Note>, staff::Staff>,
}
impl Widget<Option<note::Note>> for Root {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<note::Note>,
        env: &Env,
    ) {
        self.staff.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Option<note::Note>,
        env: &Env,
    ) {
        self.staff.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &Option<note::Note>,
        data: &Option<note::Note>,
        env: &Env,
    ) {
        self.staff.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Option<note::Note>,
        env: &Env,
    ) -> Size {
        const PADDING: f64 = 20.0;
        let size = bc.max();
        let staff_origin = Point::new(PADDING, PADDING);
        let staff_size = Size::new(size.width - PADDING * 2.0, size.height - PADDING * 2.0);
        let staff_bc = BoxConstraints::new(staff_size, staff_size);
        self.staff.layout(ctx, &staff_bc, data, env);
        self.staff.set_origin(ctx, data, env, staff_origin);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Option<note::Note>, env: &Env) {
        let size = ctx.size().to_rect();
        ctx.fill(size, &Color::WHITE);
        self.staff.paint(ctx, data, env);
    }
}

fn build_ui() -> impl Widget<Option<note::Note>> {
    Root {
        staff: WidgetPod::new(
            // staff::Staff::new(),
            staff::Staff::from_note(note::Note::new(73)),
        ),
    }
}
fn main() {
    let data = None;
    let window = WindowDesc::new(build_ui())
    // .window_size(Size::new(100.0, 100.0)) 
    ;
    AppLauncher::with_window(window).launch(data);
}
