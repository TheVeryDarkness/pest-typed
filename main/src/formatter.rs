use crate::Span;
use alloc::{format, string::String, vec::Vec};
use core::{fmt, marker::PhantomData};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
struct Pos {
    line: usize,
    col: usize,
}

fn visualize_white_space(line: &str) -> String {
    // \r ␍
    // \n ␊
    line.replace('\n', "␊").replace('\r', "␍")
}

#[derive(Debug)]
struct Partition2<'i> {
    line: usize,
    former: String,
    middle: String,
    latter: String,
    _p: PhantomData<&'i str>,
}
impl<'i> Partition2<'i> {
    fn new<'s: 'i>(line: usize, s: &'s str, col_start: usize, col_end: usize) -> Self {
        let (former, latter) = s.split_at(col_end);
        let (former, middle) = former.split_at(col_start);
        let former = visualize_white_space(former);
        let middle = visualize_white_space(middle);
        let latter = visualize_white_space(latter);
        let _p = PhantomData;
        Self {
            line,
            former,
            middle,
            latter,
            _p,
        }
    }
}

#[derive(Debug)]
struct Partition<'i> {
    line: usize,
    former: String,
    latter: String,
    _p: PhantomData<&'i str>,
}
impl<'i> Partition<'i> {
    fn new<'s: 'i>(line: usize, s: &'s str, col: usize) -> Self {
        let (former, latter) = s.split_at(col);
        let former = visualize_white_space(former);
        let latter = visualize_white_space(latter);
        let _p = PhantomData;
        Self {
            line,
            former,
            latter,
            _p,
        }
    }
}

/// Formatter options for [Span](crate::Span).
pub struct FormatOption<SpanFormatter, MarkerFormatter, NumberFormatter> {
    pub span_formatter: SpanFormatter,
    pub marker_formatter: MarkerFormatter,
    pub number_formatter: NumberFormatter,
}

type FmtPtr<Writer> = fn(&str, &mut Writer) -> fmt::Result;
impl<Writer: fmt::Write> Default for FormatOption<FmtPtr<Writer>, FmtPtr<Writer>, FmtPtr<Writer>> {
    fn default() -> Self {
        Self {
            span_formatter: |s, f| write!(f, "{s}"),
            marker_formatter: |m, f| write!(f, "{m}"),
            number_formatter: |n, f| write!(f, "{n}"),
        }
    }
}

impl<SF, MF, NF> FormatOption<SF, MF, NF> {
    /// Create option with given functions.
    pub fn new<Writer>(span_formatter: SF, marker_formatter: MF, number_formatter: NF) -> Self
    where
        Writer: fmt::Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        Self {
            span_formatter,
            marker_formatter,
            number_formatter,
        }
    }
    fn display_snippet_single_line<Writer>(
        mut self,
        f: &mut Writer,
        index_digit: usize,
        line: Partition2<'_>,
    ) -> fmt::Result
    where
        Writer: fmt::Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        let spacing = " ".repeat(index_digit);
        write!(f, "{} ", spacing)?;
        (self.number_formatter)("|", f)?;
        writeln!(f)?;

        let number = format!("{:w$}", line.line + 1, w = index_digit);
        (self.number_formatter)(&number, f)?;
        write!(f, " ")?;
        (self.number_formatter)("|", f)?;
        write!(f, " {}", line.former)?;
        (self.span_formatter)(&line.middle, f)?;
        write!(f, "{}", line.latter)?;
        writeln!(f)?;

        write!(f, "{} ", spacing)?;
        (self.number_formatter)("|", f)?;
        write!(
            f,
            " {}",
            " ".repeat(UnicodeWidthStr::width_cjk(line.former.as_str())),
        )?;
        (self.marker_formatter)(
            &"^".repeat(UnicodeWidthStr::width_cjk(line.middle.as_str())),
            f,
        )?;
        writeln!(f)?;

        Ok(())
    }
    fn display_full_covered_snippet<Writer>(
        &mut self,
        f: &mut Writer,
        index_digit: usize,
        line: usize,
        line_content: &str,
    ) -> fmt::Result
    where
        Writer: fmt::Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        let number = format!("{:w$}", line, w = index_digit);
        (self.number_formatter)(&number, f)?;
        write!(f, " ")?;
        (self.number_formatter)("|", f)?;
        write!(f, " ")?;
        (self.span_formatter)(&line_content, f)?;
        writeln!(f)?;
        Ok(())
    }
    fn display_snippet_multi_line<Writer>(
        mut self,
        f: &mut Writer,
        index_digit: usize,
        start: Partition<'_>,
        end: Partition<'_>,
        // 100
        // 101
        // 111
        // 101
        inner: (Option<&str>, Option<&str>, bool, Option<&str>),
    ) -> fmt::Result
    where
        Writer: fmt::Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        let spacing = " ".repeat(index_digit);
        write!(f, "{} ", spacing)?;
        (self.number_formatter)("|", f)?;
        write!(
            f,
            " {}",
            " ".repeat(UnicodeWidthStr::width_cjk(start.former.as_str()))
        )?;
        (self.marker_formatter)("v", f)?;
        writeln!(f)?;

        let number = format!("{:w$}", start.line + 1, w = index_digit);
        (self.number_formatter)(&number, f)?;
        write!(f, " ")?;
        (self.number_formatter)("|", f)?;
        write!(f, " {}", start.former)?;
        (self.span_formatter)(&start.latter, f)?;
        writeln!(f)?;

        if let Some(line) = inner.0 {
            self.display_full_covered_snippet(f, index_digit, start.line + 2, line)?;
        }

        if let Some(line) = inner.1 {
            self.display_full_covered_snippet(f, index_digit, start.line + 3, line)?;
        } else if inner.2 {
            write!(f, "{} ", spacing)?;
            (self.number_formatter)("|", f)?;
            writeln!(f, " ...")?;
        }

        if let Some(line) = inner.3 {
            self.display_full_covered_snippet(f, index_digit, end.line, line)?;
        }

        let number = format!("{:w$}", end.line + 1, w = index_digit);
        (self.number_formatter)(&number, f)?;
        write!(f, " ")?;
        (self.number_formatter)("|", f)?;
        write!(f, " ")?;
        (self.span_formatter)(&end.former, f)?;
        writeln!(f, "{}", end.latter)?;

        write!(f, "{} ", spacing)?;
        (self.number_formatter)("|", f)?;
        write!(
            f,
            " {}",
            " ".repeat(UnicodeWidthStr::width_cjk(end.former.as_str()).saturating_sub(1))
        )?;
        (self.marker_formatter)("^", f)?;
        writeln!(f)?;

        Ok(())
    }
    pub(crate) fn display_snippet<'i, Writer>(self, span: &Span<'i>, f: &mut Writer) -> fmt::Result
    where
        Writer: fmt::Write,
        SF: FnMut(&str, &mut Writer) -> fmt::Result,
        MF: FnMut(&str, &mut Writer) -> fmt::Result,
        NF: FnMut(&str, &mut Writer) -> fmt::Result,
    {
        let mut start = None;
        let mut end = None;
        let mut pos = 0usize;
        let input = Span::new(span.get_input(), 0, span.get_input().len()).unwrap();
        let mut iter = input.lines().enumerate().peekable();
        while let Some((index, line)) = iter.peek() {
            if pos + line.len() >= span.start() {
                start = Some(Pos {
                    line: index.clone(),
                    col: span.start() - pos,
                });
                break;
            }
            pos += line.len();
            iter.next();
        }
        for (index, line) in iter {
            if pos + line.len() >= span.end() {
                end = Some(Pos {
                    line: index,
                    col: span.end() - pos,
                });
                break;
            }
            pos += line.len();
        }
        let start = start.unwrap();
        let end = end.unwrap();
        let mut lines = input
            .lines()
            .skip(start.line)
            .take(end.line - start.line + 1)
            .peekable();
        let index_digit = {
            let mut digit = 1usize;
            let mut i = end.line + 1;
            while i >= 10 {
                digit += 1;
                i /= 10;
            }
            digit
        };
        if start.line == end.line {
            let cur_line = lines.next().unwrap();
            let line = Partition2::new(start.line, &cur_line, start.col, end.col);
            self.display_snippet_single_line(f, index_digit, line)?;
        } else {
            let lines: Vec<_> = lines.collect();
            let start_line = lines.first().unwrap();
            let end_line = lines.last().unwrap();
            let start = Partition::new(start.line, &start_line, start.col);
            let end = Partition::new(end.line, &end_line, end.col);
            let inner_first = if lines.len() >= 3 {
                Some(visualize_white_space(lines[1]))
            } else {
                None
            };
            let inner_mid = if lines.len() > 5 {
                (None, true)
            } else if lines.len() == 5 {
                (Some(visualize_white_space(lines[2])), false)
            } else {
                (None, false)
            };
            let inner_last = if lines.len() >= 4 {
                Some(visualize_white_space(lines[lines.len() - 2]))
            } else {
                None
            };
            let inner = (
                inner_first.as_deref(),
                inner_mid.0.as_deref(),
                inner_mid.1,
                inner_last.as_deref(),
            );
            self.display_snippet_multi_line(f, index_digit, start, end, inner)?;
        }
        Ok(())
    }
}
