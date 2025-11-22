use colored::{ColoredString, Colorize};

use crate::AlixtError;

pub enum TableResult<const N: usize> {
    Table(Table<N>),
    TableError(AlixtError),
}

impl<const N: usize> TableResult<N> {
    /// Example:
    /// let Table::new().title("My table".red())
    pub fn title(self, title: ColoredString) -> Self {
        let is_safe = title.chars().any(|c| c.is_ascii() && !c.is_ascii_control());
        match self {
            Self::Table(_) if !is_safe => Self::TableError(AlixtError::TableInputError(
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
                        return Self::TableError(AlixtError::TableInputError(format!("row contains invalid cell, '{cell}', bad character")));
                    }
                    if cell.len() > table.col_widths[i] {
                        table.col_widths[i] = cell.len();
                    }
                    table.cells.push(cell);
                }
                table.rows += 1;
                TableResult::Table(table)
            },
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
            if header.chars().any(|c| !c.is_ascii() || c.is_ascii_control()) {
                return Err(AlixtError::TableInputError(format!("Invalid header '{header}' contains non-ascii or control char")));
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
                return Err(AlixtError::TableInputError(format!("Invalid row cell, '{cell}', contains non-ascii or control char")));
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

    pub fn get_row(&self, index: usize) -> Option<&[ColoredString]> {
        if index >= self.rows {
            return None;
        }

        Some(&self.cells[index * N..(index * N) + N])
    }
}
