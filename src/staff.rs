use tracing::error;

use druid::{
    kurbo::Line,
    widget::{Container, FillStrat, Flex, Painter, Svg, SvgData, WidgetExt, WidgetWrapper},
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
    WidgetPod, WindowDesc,
};

use super::note;

#[derive(Debug)]
enum ClefType {
    Treble,
    Bass,
    Auto,
}
impl ClefType {
    pub fn svgdata(&self) -> SvgData {
        let file: Result<SvgData, Box<dyn std::error::Error + 'static>>;
        match *self {
            ClefType::Auto => file = include_str!("../assets/treble clef.svg").parse::<SvgData>(),
            ClefType::Treble => file = include_str!("../assets/treble clef.svg").parse::<SvgData>(),
            ClefType::Bass => file = include_str!("../assets/bass clef.svg").parse::<SvgData>(),
        }
        match file {
            Ok(svg) => svg,
            Err(err) => {
                error!("{}", err);
                error!("Using an empty SVG instead.");
                SvgData::default()
            }
        }
    }
}

pub struct Staff {
    midi_note: Option<usize>,
    max_lines: usize,
    clef_svg: WidgetPod<Option<usize>, Svg>,
    note_svg: WidgetPod<Option<usize>, Svg>,
    alt_svg: WidgetPod<Option<usize>, Svg>,
}
impl Staff {
    pub fn new() -> Self {
        Staff {
            midi_note: None,
            max_lines: 9,
            clef_svg: WidgetPod::new(Self::make_clef_svg(ClefType::Treble)),
            note_svg: WidgetPod::new(Self::make_note_svg()),
            alt_svg: WidgetPod::new(Self::make_alt_svg(note::NoteAlt::Sharp)),
        }
    }
    fn make_clef_svg(cleftype: ClefType) -> Svg {
        Svg::new(cleftype.svgdata())
    }
    fn make_note_svg() -> Svg {
        Svg::new(Self::make_svg(
            include_str!("../assets/sharp.svg").parse::<SvgData>(),
        ))
    }
    fn make_alt_svg(alttype: note::NoteAlt) -> Svg {
        Svg::new(alttype.svgdata())
    }
    fn make_svg(file: Result<SvgData, Box<dyn std::error::Error + 'static>>) -> SvgData {
        match file {
            Ok(svg) => svg,
            Err(err) => {
                error!("{}", err);
                error!("Using an empty SVG instead.");
                SvgData::default()
            }
        }
    }
    fn line_w(staff_height: f64) -> f64 {
        let mut w = staff_height / 200.0;
        if w < 2.0 {
            w = 2.0;
        }
        return w;
    }
    fn line_h(&self, staff_height: f64) -> f64 {
        staff_height / (self.max_lines - 1) as f64
    }
    fn lines_rect(&self, staff_rect: Rect) -> Rect {
        let vpad = self.line_h(staff_rect.height()) * ((self.max_lines - 5) / 2) as f64;
        Rect::new(
            staff_rect.x0,
            staff_rect.y0 + vpad,
            staff_rect.x1,
            staff_rect.y1 - vpad,
        )
    }
    fn line_coords(
        &self,
        staff_rect: Rect,
        line_nr: usize,
        width: Option<f64>,
        center: Option<Point>,
    ) -> Rect {
        let lines_rect = self.lines_rect(staff_rect);
        let bottom = lines_rect.y1;
        let m_width = width.unwrap_or(lines_rect.width());
        let m_center = center.unwrap_or(lines_rect.center());
        let pos_y = bottom - self.line_h(staff_rect.height()) * line_nr as f64;
        Rect::new(
            m_center.x - (m_width / 2.0),
            pos_y,
            m_center.x + (m_width / 2.0),
            pos_y,
        )
    }
}

impl Widget<Option<usize>> for Staff {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Option<usize>, env: &Env) {
        self.clef_svg.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Option<usize>,
        env: &Env,
    ) {
        self.clef_svg.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &Option<usize>,
        data: &Option<usize>,
        env: &Env,
    ) {
        self.clef_svg.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Option<usize>,
        env: &Env,
    ) -> Size {
        // For now, just copy of padding.
        // let size = self.clef_svg.layout(ctx, &clef_svg_bc, data, env);
        // let my_size = Size::new(size.width + hpad, size.height + vpad);
        // let my_insets = self.clef_svg.compute_parent_paint_insets(my_size);
        // ctx.set_paint_insets(my_insets);
        // my_size
        let size = bc.max();
        let line_h = self.line_h(size.height);
        let lines_rect = self.lines_rect(Rect::from_origin_size(Point::new(0.0, 0.0), size));
        // let lines_rect = self.lines_rect(size);
        let clef_size = Size::new(size.width, line_h * 6.0);
        let clef_bc = BoxConstraints::new(Size::new(0.0, clef_size.height), clef_size);
        self.clef_svg.layout(ctx, &clef_bc, data, env);
        self.clef_svg.set_origin(
            ctx,
            data,
            env,
            Point::new(lines_rect.x0, lines_rect.y0 - line_h),
        );

        Size::new(size.width, size.height)
        // bc.shrink(diff: impl Into<Size>)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Option<usize>, env: &Env) {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &Color::WHITE);
        for i in 0..5 {
            let line_rect = self.line_coords(bounds, i, None, None);
            let line = Line::new(
                Point::new(line_rect.x0, line_rect.y0),
                Point::new(line_rect.x1, line_rect.y1),
            );
            ctx.stroke(line, &Color::BLACK, Self::line_w(bounds.height()));
        }
        self.clef_svg.paint(ctx, data, env);
    }
}
