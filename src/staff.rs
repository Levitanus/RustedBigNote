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
    midi_note: Option<note::Note>,
    max_lines: i32,
    clef_svg: WidgetPod<Option<note::Note>, Svg>,
    note_svg: WidgetPod<Option<note::Note>, Svg>,
    sharp_svg: WidgetPod<Option<note::Note>, Svg>,
    flat_svg: WidgetPod<Option<note::Note>, Svg>,
}
const MAX_LINES: i32 = 11;
impl Staff {
    pub fn new() -> Self {
        Staff {
            midi_note: None,
            max_lines: MAX_LINES,
            clef_svg: WidgetPod::new(Self::make_clef_svg(ClefType::Treble)),
            note_svg: WidgetPod::new(Self::make_note_svg()),
            sharp_svg: WidgetPod::new(Self::make_alt_svg(note::NoteAlt::Sharp)),
            flat_svg: WidgetPod::new(Self::make_alt_svg(note::NoteAlt::Flat)),
        }
    }
    pub fn from_note(note: note::Note) -> Self {
        Staff {
            midi_note: Some(note),
            max_lines: MAX_LINES,
            clef_svg: WidgetPod::new(Self::make_clef_svg(ClefType::Treble)),
            note_svg: WidgetPod::new(Self::make_note_svg()),
            sharp_svg: WidgetPod::new(Self::make_alt_svg(note::NoteAlt::Sharp)),
            flat_svg: WidgetPod::new(Self::make_alt_svg(note::NoteAlt::Flat)),
        }
    }
    fn make_clef_svg(cleftype: ClefType) -> Svg {
        Svg::new(cleftype.svgdata()).fill_mode(FillStrat::Fill)
    }
    fn make_note_svg() -> Svg {
        Svg::new(Self::make_svg(
            include_str!("../assets/note.svg").parse::<SvgData>(),
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
        line_nr: i32,
        width: Option<f64>,
        center: Option<f64>,
    ) -> Rect {
        let lines_rect = self.lines_rect(staff_rect);
        let bottom = lines_rect.y1;
        let m_width = width.unwrap_or(lines_rect.width());
        let m_center = center.unwrap_or(lines_rect.center().x);
        let pos_y = bottom - self.line_h(staff_rect.height()) * line_nr as f64;
        Rect::new(
            m_center - (m_width / 2.0),
            pos_y,
            m_center + (m_width / 2.0),
            pos_y,
        )
    }
    fn note_line(&self) -> f64 {
        let note = self.midi_note.clone().unwrap();
        note.line(self.note_alt())
    }
    fn note_alt(&self) -> note::NoteAlt {
        let note = self.midi_note.clone().unwrap();
        note.alteration(note::NoteAlt::Flat)
    }
    fn staff_line(&self) -> f64 {
        note::Note::new(64).line(self.note_alt())
    }
}

impl Widget<Option<note::Note>> for Staff {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Option<note::Note>,
        env: &Env,
    ) {
        self.clef_svg.event(ctx, event, data, env);
        self.note_svg.event(ctx, event, data, env);
        self.sharp_svg.event(ctx, event, data, env);
        self.flat_svg.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Option<note::Note>,
        env: &Env,
    ) {
        self.clef_svg.lifecycle(ctx, event, data, env);
        self.note_svg.lifecycle(ctx, event, data, env);
        self.sharp_svg.lifecycle(ctx, event, data, env);
        self.flat_svg.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &Option<note::Note>,
        data: &Option<note::Note>,
        env: &Env,
    ) {
        self.clef_svg.update(ctx, data, env);
        self.note_svg.update(ctx, data, env);
        self.sharp_svg.update(ctx, data, env);
        self.flat_svg.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Option<note::Note>,
        env: &Env,
    ) -> Size {
        let size = bc.max();
        let staff_origin = Point::new(0.0, 0.0);
        let line_h = self.line_h(size.height);
        let lines_rect = self.lines_rect(Rect::from_origin_size(staff_origin, size));
        let clef_size = Size::new(size.width, line_h * 6.0);
        let clef_bc = BoxConstraints::new(Size::new(0.0, clef_size.height), clef_size);
        self.clef_svg.layout(ctx, &clef_bc, data, env);
        self.clef_svg.set_origin(
            ctx,
            data,
            env,
            Point::new(lines_rect.x0, lines_rect.y0 - line_h),
        );

        if self.midi_note.is_some() {
            let diff = self.note_line() - self.staff_line();
            // let staff_line = ;
            let note_size = Size::new(size.width, line_h);
            let note_bc = BoxConstraints::new(Size::new(0.0, note_size.height), note_size);
            let note_size = self.note_svg.layout(ctx, &note_bc, data, env);
            let note_origin = Point::new(
                lines_rect.center().x - note_size.width / 2.0,
                lines_rect.y1 - (line_h * diff + line_h * 0.5),
            );
            self.note_svg.set_origin(ctx, data, env, note_origin);
            let note_alt = self.note_alt();
            if note_alt != note::NoteAlt::White {
                let mut curr_svg = &mut self.sharp_svg;
                let mut up_coeff = 4.0;
                if note_alt == note::NoteAlt::Flat {
                    curr_svg = &mut self.flat_svg;
                    up_coeff = 1.5;
                }
                let alt_size = Size::new(size.width, line_h * 1.5);
                let alt_bc = BoxConstraints::new(Size::new(0.0, alt_size.height), alt_size);
                let alt_size = curr_svg.layout(ctx, &alt_bc, data, env);
                let alt_origin = Point::new(
                    note_origin.x - alt_size.width * 2.0,
                    note_origin.y - line_h / up_coeff,
                );
                curr_svg.set_origin(ctx, data, env, alt_origin);
            }
        }

        size
        // bc.shrink(diff: impl Into<Size>)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Option<note::Note>, env: &Env) {
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

        if self.midi_note.is_some() {
            self.note_svg.paint(ctx, data, env);
            let bounds = ctx.size().to_rect();
            let note_line = self.note_line();
            let staff_line = self.staff_line();
            let diff = (note_line - staff_line) as i32;
            let diff_range: Option<std::ops::Range<i32>>;
            if diff < 0 {
                diff_range = Some(diff..0);
            } else if diff > 4 {
                diff_range = Some(5..(diff + 1));
            } else {
                diff_range = None;
            }
            if diff_range.is_some() {
                for i in diff_range.unwrap() {
                    let line_rect = self.line_coords(
                        bounds,
                        i,
                        Some(bounds.width() / 7.0),
                        Some(bounds.center().x),
                    );
                    let line = Line::new(
                        Point::new(line_rect.x0, line_rect.y0),
                        Point::new(line_rect.x1, line_rect.y1),
                    );
                    ctx.stroke(line, &Color::BLACK, Self::line_w(bounds.height()));
                }
            }
            match self.note_alt() {
                note::NoteAlt::Sharp => {
                    self.sharp_svg.paint(ctx, data, env);
                }
                note::NoteAlt::Flat => {
                    self.flat_svg.paint(ctx, data, env);
                }
                note::NoteAlt::White => {}
            }
        }
    }
}
