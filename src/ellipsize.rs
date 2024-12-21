// //! Draw ellipsizing paragraphs.
use cosmic_text::{Buffer, Metrics};
use iced::advanced::graphics::text;
use iced::advanced::text::{Shaping as IcedShaping, Text, Wrapping};
use iced::widget::text::LineHeight;
use iced::{Element, Font, Pixels, Size};

pub struct Content {
    text: String,
    size: Pixels,
    max_size: Option<Size>,
    font: Option<Font>,
    line_height: LineHeight,
    horizontal_alignment: iced::alignment::Horizontal,
    vertical_alignment: iced::alignment::Vertical,
}

impl Content {
    pub fn new(text: String, max_size: Option<Size>) -> Self {
        Self {
            text,
            size: 14.0.into(),
            max_size,
            font: None,
            line_height: 1.5.into(),
            horizontal_alignment: iced::alignment::Horizontal::Left,
            vertical_alignment: iced::alignment::Vertical::Center,
        }
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn line_height(mut self, line_height: LineHeight) -> Self {
        self.line_height = line_height;
        self
    }

    pub fn align_x(mut self, alignment: impl Into<iced::alignment::Horizontal>) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    pub fn align_y(mut self, alignment: impl Into<iced::alignment::Vertical>) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    pub fn center(mut self) -> Self {
        self.horizontal_alignment = iced::alignment::Horizontal::Center;
        self.vertical_alignment = iced::alignment::Vertical::Center;
        self
    }
}

impl<'a, Message, Theme, Renderer> From<Content> for Element<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::text::Renderer<Font = iced::Font> + 'a,
{
    fn from(content: Content) -> Element<'a, Message, Theme, Renderer> {
        let font = content.font.unwrap_or(iced::Font::DEFAULT);
        if let Some(max_size) = content.max_size {
            let text = Text {
                content: content.text,
                size: content.size,
                font,
                line_height: content.line_height,
                horizontal_alignment: content.horizontal_alignment,
                vertical_alignment: content.vertical_alignment,
                wrapping: Wrapping::WordOrGlyph,
                shaping: IcedShaping::Advanced,
                bounds: max_size,
            };

            iced::widget::text(ellipsize(text, max_size))
                .font(font)
                .size(content.size)
                .line_height(content.line_height)
                .align_x(content.horizontal_alignment)
                .align_y(content.vertical_alignment)
                .wrapping(Wrapping::WordOrGlyph)
                .shaping(IcedShaping::Advanced)
                .into()
        } else {
            iced::widget::text(content.text)
                .font(font)
                .size(content.size)
                .line_height(content.line_height)
                .align_x(content.horizontal_alignment)
                .align_y(content.vertical_alignment)
                .wrapping(Wrapping::WordOrGlyph)
                .shaping(IcedShaping::Advanced)
                .into()
        }
    }
}

fn ellipsize(text: Text, size: iced::Size) -> String {
    let metrics = Metrics::new(
        text.size.into(),
        text.line_height.to_absolute(text.size).into(),
    );
    eprintln!("\nInitial metrics: {:?}", metrics);

    let mut buffer = Buffer::new_empty(metrics);
    let mut font_system = text::font_system().write().expect("Write font system");

    // Set up buffer with infinite height to measure full content
    buffer.set_size(font_system.raw(), Some(size.width), Some(f32::INFINITY));
    buffer.set_wrap(font_system.raw(), text::to_wrap(text.wrapping));
    buffer.set_text(
        font_system.raw(),
        text.content.as_str(),
        text::to_attributes(text.font),
        text::to_shaping(text.shaping),
    );

    let full_measure = text::measure(&buffer);
    eprintln!("Full text measure: {:?}, target: {:?}", full_measure, size);

    // If text fits completely, return as-is
    if full_measure.height <= size.height {
        eprintln!("Text fits completely, returning wrapped text");
        return buffer
            .lines
            .iter()
            .map(|l| l.text())
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Collect visible text until we hit our height limit
    let mut truncated = String::new();
    let mut runs = buffer.layout_runs().peekable();

    eprintln!("\nBegin collecting layout runs:");
    while let Some(run) = runs.next() {
        // Find the text boundaries for this line using glyphs
        let start = run.glyphs.first().map(|g| g.start).unwrap_or(0);
        let end = run.glyphs.last().map(|g| g.end).unwrap_or(0);
        let line_text = &run.text[start..end];

        let line_top = run.line_top;
        let line_bottom = run.line_y;

        eprintln!(
            "\nRun: line_top={:.1}, line_bottom={:.1}, height={:.1}\n  text: '{}'",
            line_top, line_bottom, size.height, line_text
        );

        // If this line's full height would exceed our limit, stop
        if line_bottom > size.height {
            eprintln!("  Line would exceed height limit, breaking");
            // Add ellipsis to previous content if we have any
            if !truncated.is_empty() && !truncated.ends_with('…') {
                truncated.push('…');
            }
            break;
        }

        // Add the line to our truncated text
        if !truncated.is_empty() {
            truncated.push('\n');
        }
        truncated.push_str(line_text);
    }

    eprintln!("\nFinal truncated text:");
    eprintln!("Length: {}", truncated.len());
    eprintln!("Result: '{}'", truncated);

    truncated
}
