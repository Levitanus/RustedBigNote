use druid::{
    kurbo::Line,
    widget::{Container, FillStrat, Flex, Painter, Svg, SvgData, WidgetExt, WidgetWrapper},
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
    WidgetPod, WindowDesc,
};

const LINES_AMOUTN: usize = 24;
const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const NOTE_LINES: [NoteLine; 24] = [
    NoteLine::new(0.0, false), // C
    NoteLine::new(0.0, true),  // C#
    NoteLine::new(0.5, false), // D
    NoteLine::new(0.5, true),  // D#
    NoteLine::new(1.0, false), // E
    NoteLine::new(1.5, false), // F
    NoteLine::new(1.5, true),  // F#
    NoteLine::new(2.0, false), // G
    NoteLine::new(2.0, true),  // G#
    NoteLine::new(2.5, false), // A
    NoteLine::new(2.5, true),  // A#
    NoteLine::new(3.0, false), // B
    NoteLine::new(3.5, false), // C
    NoteLine::new(3.5, true),  // C#
    NoteLine::new(4.0, false), // D
    NoteLine::new(4.0, true),  // D#
    NoteLine::new(4.5, false), // E
    NoteLine::new(5.0, false), // F
    NoteLine::new(5.0, true),  // F#
    NoteLine::new(5.5, false), // G
    NoteLine::new(5.5, true),  // G#
    NoteLine::new(6.0, false), // A
    NoteLine::new(6.0, true),  // A#
    NoteLine::new(6.5, false), // B
];

#[derive(PartialEq)]
enum NoteAlt {
    White,
    Sharp,
    Flat,
}

pub struct NoteLine {
    root: f32,
    alterated: bool,
}
impl NoteLine {
    fn new(root: f32, alterated: bool) -> Self {
        NoteLine {
            root: root,
            alterated: alterated,
        }
    }
    fn from_alteration(&self, alteration: &mut NoteAlt) -> f32 {
        if NoteAlt::Sharp == alteration || self.alterated == false {
            return self.root;
        } else {
            alteration = NoteAlt::Flat;
            return self.root + 1.0;
        }
    }
}

pub struct Note {
    midi_nr: u8,
}
impl Note {
    fn spec(&self, alteration: Option<NoteAlt>) -> (f32, NoteAlt, &&str) {
        let midi_nr = self.midi_nr as usize;
        let modulo = ((midi_nr % LINES_AMOUTN) + LINES_AMOUTN) % LINES_AMOUTN;
        let remainder = midi_nr % LINES_AMOUTN;
        let mut alt = alteration.unwrap_or(NoteAlt::Sharp);
        let line = &NOTE_LINES[remainder].from_alteration(&mut alt) + ((modulo * 7) as f32);
        let name = &NOTE_NAMES[remainder];
        (line, alt, name)
    }
}

pub struct ZStack {
    clef: Option<Svg>,
    note: Option<Svg>,
    key_signature: Option<Svg>,
    max_lines: u8,
}

// impl ZStack {
//     fn new(max_lines: Option<u8>) -> Self {
//         ZStack {
//             clef: None,
//             note: None,
//             key_signature: None,
//             max_lines: max_lines.unwrap_or(11_u8),
//         }
//     }
//     fn add_child(self, key: String, child: W) -> Self {
//         self.children.insert(key, child);
//         self
//     }
// }

// impl<T: Data> Widget<T> for ZStack<T> {
//     fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
//         self.child.event(ctx, event, data, env)
//     }

//     fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
//         self.child.lifecycle(ctx, event, data, env)
//     }

//     fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
//         self.child.update(ctx, data, env);
//     }

//     fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
//         // For now, just copy of padding.
//         bc.debug_check("Padding");

//         let hpad = 10.0;
//         let vpad = 10.0;

//         let child_bc = bc.shrink((hpad, vpad));
//         let size = self.child.layout(ctx, &child_bc, data, env);
//         let origin = Point::new(10.0, 10.0);
//         self.child.set_origin(ctx, data, env, origin);

//         let my_size = Size::new(size.width + hpad, size.height + vpad);
//         let my_insets = self.child.compute_parent_paint_insets(my_size);
//         ctx.set_paint_insets(my_insets);
//         my_size
//     }

//     fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
//         let bounds = ctx.size().to_rect();
//         println!("{:?}", bounds);
//         ctx.fill(bounds, &Color::YELLOW);
//         let y = (bounds.y1 - bounds.y0) / 2.0;
//         let line = Line::new(Point::new(bounds.x0, y), Point::new(bounds.x1, y));
//         ctx.stroke(line, &env.get(druid::theme::PRIMARY_DARK), 1.0);
//         self.child.paint(ctx, data, env);
//     }
// }
