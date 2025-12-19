#![allow(unused)]
use colored::{ColoredString, Colorize};

use crate::AlixtError;

pub const TOP_LEFT: &str = "╭";
pub const TOP_RIGHT: &str = "╮";
pub const BOTTOM_LEFT: &str = "╰";
pub const BOTTOM_RIGHT: &str = "╯";
pub const HORIZONTAL: &str = "─";
pub const VERTICAL: &str = "│";
pub const CROSS: &str = "┼";
pub const TOP_T: &str = "┬";
pub const BOTTOM_T: &str = "┴";
pub const LEFT_T: &str = "├";
pub const RIGHT_T: &str = "┤";

pub const SPACES: &str = "                                                                                                                               ";
pub enum TableResult<const N: usize> {
    Table(Table<N>),
    TableError(AlixtError),
}

impl<const N: usize> TableResult<N> {
    /// Example:
    /// let Table::new().title("My table".red())
    pub fn title(self, title: ColoredString) -> Self {
        let has_bad_char = title.chars().any(|c| c.is_ascii_control() || !c.is_ascii());
        match self {
            Self::Table(_) if has_bad_char => Self::TableError(AlixtError::TableInputError(
                "title contians non-ascii character or ascii control character".to_string(),
            )),
            Self::Table(mut table) => {
                table.title = title;
                Self::Table(table)
            }
            Self::TableError(err) => Self::TableError(err),
        }
    }

    pub fn headers(self, headers: [ColoredString; N]) -> Self {
        match self {
            Self::Table(mut table) => {
                for (i, header) in headers.into_iter().enumerate() {
                    if header
                        .chars()
                        .any(|c| !c.is_ascii() || c.is_ascii_control())
                    {
                        return Self::TableError(AlixtError::TableInputError(format!(
                            "header contains invalid header, '{}', bad character",
                            header
                        )));
                    }
                    if header.len() > table.col_widths[i] {
                        table.col_widths[i] = header.len();
                    }
                    table.headers[i] = header;
                }
                Self::Table(table)
            }
            err => err,
        }
    }

    pub fn row(self, row: [ColoredString; N]) -> Self {
        match self {
            Self::Table(mut table) => {
                for (i, cell) in row.into_iter().enumerate() {
                    if cell.chars().any(|c| !c.is_ascii() || c.is_ascii_control()) {
                        return Self::TableError(AlixtError::TableInputError(format!(
                            "row contains invalid cell, '{cell}', bad character"
                        )));
                    }
                    if cell.len() > table.col_widths[i] {
                        table.col_widths[i] = cell.len();
                    }
                    table.cells.push(cell);
                }
                table.rows += 1;
                TableResult::Table(table)
            }
            err => err,
        }
    }

    pub fn collect(self) -> Result<Table<N>, AlixtError> {
        match self {
            Self::Table(table) => Ok(table),
            Self::TableError(err) => Err(err),
        }
    }
}

pub struct Table<const N: usize> {
    title: ColoredString,
    rows: usize,
    col_widths: [usize; N],
    headers: [ColoredString; N],
    cells: Vec<ColoredString>,
}

impl<const N: usize> Table<N> {
    /// Table is created with a fixed number of columns
    pub fn new() -> TableResult<N> {
        TableResult::Table(Self {
            title: "".white(),
            rows: 0,
            col_widths: [0; N],
            headers: std::array::from_fn(|_| "".white()),
            cells: Vec::new(),
        })
    }
    pub fn headers(&mut self, headers: [ColoredString; N]) -> Result<(), AlixtError> {
        for header in headers.iter() {
            if header
                .chars()
                .any(|c| !c.is_ascii() || c.is_ascii_control())
            {
                return Err(AlixtError::TableInputError(format!(
                    "Invalid header '{header}' contains non-ascii or control char"
                )));
            }
        }
        for (i, header) in headers.into_iter().enumerate() {
            if header.len() > self.col_widths[i] {
                self.col_widths[i] = header.len();
            }
            self.headers[i] = header;
        }
        Ok(())
    }

    pub fn push_row(&mut self, row: [ColoredString; N]) -> Result<(), AlixtError> {
        for cell in row.iter() {
            if cell.chars().any(|c| !c.is_ascii() || c.is_ascii_control()) {
                return Err(AlixtError::TableInputError(format!(
                    "Invalid row cell, '{cell}', contains non-ascii or control char"
                )));
            }
        }
        for (i, cell) in row.into_iter().enumerate() {
            if cell.len() > self.col_widths[i] {
                self.col_widths[i] = cell.len();
            }
            self.cells.push(cell);
        }
        self.rows += 1;
        Ok(())
    }

    pub fn get_title(&self) -> Option<&ColoredString> {
        if self.title.is_empty() {
            None
        } else {
            Some(&self.title)
        }
    }

    pub fn get_row_count(&self) -> usize {
        self.rows
    }

    pub fn get_column_count(&self) -> usize {
        N
    }

    pub fn get_headers(&self) -> &[ColoredString] {
        &self.headers
    }

    pub fn get_col_widths(&self) -> &[usize] {
        &self.col_widths
    }

    pub fn get_row(&self, index: usize) -> Option<&[ColoredString]> {
        if index >= self.rows {
            return None;
        }
        Some(&self.cells[index * N..(index * N) + N])
    }

    pub fn padding(&self, spacing: usize) -> &str {
        &SPACES[..std::cmp::min(spacing, SPACES.len())]
    }

    pub fn content_width(&self) -> usize {
        self.get_col_widths().iter().sum()
    }

    fn _render_top<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let table_width = self.content_width() + self.get_column_count() - 1;

        writeln!(
            w,
            "\n{}{}{}",
            TOP_LEFT.blue(),
            HORIZONTAL.repeat(table_width).blue(),
            TOP_RIGHT.blue()
        )?;

        Ok(())
    }

    fn _render_title<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let table_width = self.content_width() + self.get_column_count() - 1;

        if let Some(title) = self.get_title() {
            write!(w, "{}", VERTICAL.blue())?;
            // if title is longer than table width, truncate to be 5 chars shorter
            // than the table, to account for the border chars and the '...'
            if title.len() > table_width - 2 {
                writeln!(
                    w,
                    "{}...{}",
                    self.title[..table_width - 3]
                        .to_string()
                        .color(self.title.fgcolor.unwrap()),
                    VERTICAL.blue(),
                )?;
            } else {
                let pad_l = self.padding((table_width - title.len()) / 2);
                let pad_r =
                    self.padding((table_width - title.len()) - (table_width - title.len()) / 2);

                writeln!(w, "{}{}{}{}", pad_l, self.title, pad_r, VERTICAL.blue(),)?;
            }
        }
        Ok(())
    }

    fn _render_headers<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(
            w,
            "{}{}",
            LEFT_T.blue(),
            HORIZONTAL.repeat(self.get_col_widths()[0]).blue()
        )?;
        for i in 1..self.get_column_count() {
            write!(
                w,
                "{}{}",
                TOP_T.blue(),
                HORIZONTAL.repeat(self.get_col_widths()[i]).blue()
            )?;
        }
        writeln!(w, "{}", RIGHT_T.blue())?;
        write!(w, "{}", VERTICAL.blue())?;
        for (i, header) in self.get_headers().iter().enumerate() {
            write!(
                w,
                "{}{}{}",
                header,
                self.padding(self.get_col_widths()[i] - header.len()),
                VERTICAL.blue()
            )?;
        }
        writeln!(w, "");
        Ok(())
    }

    fn _render_row<W: std::io::Write>(&self, w: &mut W, row: usize) -> std::io::Result<()> {
        write!(
            w,
            "{}{}",
            LEFT_T.blue(),
            HORIZONTAL.repeat(self.get_col_widths()[0]).blue()
        )?;
        for i in 1..self.get_column_count() {
            write!(
                w,
                "{}{}",
                CROSS.blue(),
                HORIZONTAL.repeat(self.get_col_widths()[i]).blue()
            )?;
        }
        writeln!(w, "{}", RIGHT_T.blue())?;
        write!(w, "{}", VERTICAL.blue())?;
        for (i, cell) in self.get_row(row).unwrap().iter().enumerate() {
            write!(
                w,
                "{}{}{}",
                cell,
                self.padding(self.get_col_widths()[i] - cell.len()),
                VERTICAL.blue()
            )?;
        }
        writeln!(w, "")?;
        Ok(())
    }

    fn _render_bottom<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        write!(
            w,
            "{}{}",
            BOTTOM_LEFT.blue(),
            HORIZONTAL.repeat(self.get_col_widths()[0]).blue()
        )?;
        for i in 1..self.get_column_count() {
            write!(
                w,
                "{}{}",
                BOTTOM_T.blue(),
                HORIZONTAL.repeat(self.get_col_widths()[i]).blue()
            )?;
        }
        writeln!(w, "{}", BOTTOM_RIGHT.blue())?;
        Ok(())
    }

    pub fn render<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let table_width = self.content_width() + self.get_column_count() - 1;

        self._render_top(w)?;
        self._render_title(w)?;
        self._render_headers(w)?;
        for row in 0..self.get_row_count() {
            self._render_row(w, row)?;
        }
        self._render_bottom(w)?;
        Ok(())
    }
}
