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

fn build_ui() -> impl Widget<Option<usize>> {
    Container::new(staff::Staff::new())
}

fn main() {
    let data = None;
    let window = WindowDesc::new(build_ui()).window_size(Size::new(100.0, 100.0));
    AppLauncher::with_window(window).launch(data);
}
