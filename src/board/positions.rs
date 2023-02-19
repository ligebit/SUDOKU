//! Types for positions in the sudoku
//!
//! A sudoku consists of 81 cells, arranged into 9 rows, 9 columns and 9 blocks.
#![allow(unused, missing_docs)]

use crate::bitset::{Set, SetElement};
use crate::board::Digit;
use crate::consts::*;
use crate::helper::Unsolvable;
use std::num::NonZeroU8;

#[inline(always)]
pub(crate) fn row(cell: u8) -> u8 {
    cell / 9
}
#[inline(always)]
pub(crate) fn col(cell: u8) -> u8 {
    cell % 9
}
#[inline(always)]
pub(crate) fn block(cell: u8) -> u8 {
    BLOCK[cell as usize]
}

fn band(cell: u8) -> u8 {
    cell / 27
}

fn stack(cell: u8) -> u8 {
    col(cell) / 3
}

#[rustfmt::skip]
static BLOCK: [u8; N_CELLS] = [
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    0, 0, 0, 1, 1, 1, 2, 2, 2,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    3, 3, 3, 4, 4, 4, 5, 5, 5,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
    6, 6, 6, 7, 7, 7, 8, 8, 8,
];

#[rustfmt::skip]
static CELLS_BY_HOUSE: [[u8; 9]; 27] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],

    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],

    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

// list of cells that share a row, col or field for a given cell
// sorted low to high
#[rustfmt::skip]
static HOUSE_NEIGHBORS_OF_CELL: [[u8; 20]; 81] = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 27, 36, 45, 54, 63, 72],
    [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 28, 37, 46, 55, 64, 73],
    [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 18, 19, 20, 29, 38, 47, 56, 65, 74],
    [0, 1, 2, 4, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 30, 39, 48, 57, 66, 75],
    [0, 1, 2, 3, 5, 6, 7, 8, 12, 13, 14, 21, 22, 23, 31, 40, 49, 58, 67, 76],
    [0, 1, 2, 3, 4, 6, 7, 8, 12, 13, 14, 21, 22, 23, 32, 41, 50, 59, 68, 77],
    [0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78],
    [0, 1, 2, 3, 4, 5, 6, 8, 15, 16, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79],
    [0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 24, 25, 26, 35, 44, 53, 62, 71, 80],
    [0, 1, 2, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 27, 36, 45, 54, 63, 72],
    [0, 1, 2, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 28, 37, 46, 55, 64, 73],
    [0, 1, 2, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20, 29, 38, 47, 56, 65, 74],
    [3, 4, 5, 9, 10, 11, 13, 14, 15, 16, 17, 21, 22, 23, 30, 39, 48, 57, 66, 75],
    [3, 4, 5, 9, 10, 11, 12, 14, 15, 16, 17, 21, 22, 23, 31, 40, 49, 58, 67, 76],
    [3, 4, 5, 9, 10, 11, 12, 13, 15, 16, 17, 21, 22, 23, 32, 41, 50, 59, 68, 77],
    [6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 24, 25, 26, 33, 42, 51, 60, 69, 78],
    [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 24, 25, 26, 34, 43, 52, 61, 70, 79],
    [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 24, 25, 26, 35, 44, 53, 62, 71, 80],
    [0, 1, 2, 9, 10, 11, 19, 20, 21, 22, 23, 24, 25, 26, 27, 36, 45, 54, 63, 72],
    [0, 1, 2, 9, 10, 11, 18, 20, 21, 22, 23, 24, 25, 26, 28, 37, 46, 55, 64, 73],
    [0, 1, 2, 9, 10, 11, 18, 19, 21, 22, 23, 24, 25, 26, 29, 38, 47, 56, 65, 74],
    [3, 4, 5, 12, 13, 14, 18, 19, 20, 22, 23, 24, 25, 26, 30, 39, 48, 57, 66, 75],
    [3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 23, 24, 25, 26, 31, 40, 49, 58, 67, 76],
    [3, 4, 5, 12, 13, 14, 18, 19, 20, 21, 22, 24, 25, 26, 32, 41, 50, 59, 68, 77],
    [6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 33, 42, 51, 60, 69, 78],
    [6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 26, 34, 43, 52, 61, 70, 79],
    [6, 7, 8, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 35, 44, 53, 62, 71, 80],
    [0, 9, 18, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 54, 63, 72],
    [1, 10, 19, 27, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 55, 64, 73],
    [2, 11, 20, 27, 28, 30, 31, 32, 33, 34, 35, 36, 37, 38, 45, 46, 47, 56, 65, 74],
    [3, 12, 21, 27, 28, 29, 31, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 57, 66, 75],
    [4, 13, 22, 27, 28, 29, 30, 32, 33, 34, 35, 39, 40, 41, 48, 49, 50, 58, 67, 76],
    [5, 14, 23, 27, 28, 29, 30, 31, 33, 34, 35, 39, 40, 41, 48, 49, 50, 59, 68, 77],
    [6, 15, 24, 27, 28, 29, 30, 31, 32, 34, 35, 42, 43, 44, 51, 52, 53, 60, 69, 78],
    [7, 16, 25, 27, 28, 29, 30, 31, 32, 33, 35, 42, 43, 44, 51, 52, 53, 61, 70, 79],
    [8, 17, 26, 27, 28, 29, 30, 31, 32, 33, 34, 42, 43, 44, 51, 52, 53, 62, 71, 80],
    [0, 9, 18, 27, 28, 29, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 54, 63, 72],
    [1, 10, 19, 27, 28, 29, 36, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 55, 64, 73],
    [2, 11, 20, 27, 28, 29, 36, 37, 39, 40, 41, 42, 43, 44, 45, 46, 47, 56, 65, 74],
    [3, 12, 21, 30, 31, 32, 36, 37, 38, 40, 41, 42, 43, 44, 48, 49, 50, 57, 66, 75],
    [4, 13, 22, 30, 31, 32, 36, 37, 38, 39, 41, 42, 43, 44, 48, 49, 50, 58, 67, 76],
    [5, 14, 23, 30, 31, 32, 36, 37, 38, 39, 40, 42, 43, 44, 48, 49, 50, 59, 68, 77],
    [6, 15, 24, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 51, 52, 53, 60, 69, 78],
    [7, 16, 25, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 44, 51, 52, 53, 61, 70, 79],
    [8, 17, 26, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 51, 52, 53, 62, 71, 80],
    [0, 9, 18, 27, 28, 29, 36, 37, 38, 46, 47, 48, 49, 50, 51, 52, 53, 54, 63, 72],
    [1, 10, 19, 27, 28, 29, 36, 37, 38, 45, 47, 48, 49, 50, 51, 52, 53, 55, 64, 73],
    [2, 11, 20, 27, 28, 29, 36, 37, 38, 45, 46, 48, 49, 50, 51, 52, 53, 56, 65, 74],
    [3, 12, 21, 30, 31, 32, 39, 40, 41, 45, 46, 47, 49, 50, 51, 52, 53, 57, 66, 75],
    [4, 13, 22, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 50, 51, 52, 53, 58, 67, 76],
    [5, 14, 23, 30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 49, 51, 52, 53, 59, 68, 77],
    [6, 15, 24, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 52, 53, 60, 69, 78],
    [7, 16, 25, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 53, 61, 70, 79],
    [8, 17, 26, 33, 34, 35, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 62, 71, 80],
    [0, 9, 18, 27, 36, 45, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74],
    [1, 10, 19, 28, 37, 46, 54, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74],
    [2, 11, 20, 29, 38, 47, 54, 55, 57, 58, 59, 60, 61, 62, 63, 64, 65, 72, 73, 74],
    [3, 12, 21, 30, 39, 48, 54, 55, 56, 58, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77],
    [4, 13, 22, 31, 40, 49, 54, 55, 56, 57, 59, 60, 61, 62, 66, 67, 68, 75, 76, 77],
    [5, 14, 23, 32, 41, 50, 54, 55, 56, 57, 58, 60, 61, 62, 66, 67, 68, 75, 76, 77],
    [6, 15, 24, 33, 42, 51, 54, 55, 56, 57, 58, 59, 61, 62, 69, 70, 71, 78, 79, 80],
    [7, 16, 25, 34, 43, 52, 54, 55, 56, 57, 58, 59, 60, 62, 69, 70, 71, 78, 79, 80],
    [8, 17, 26, 35, 44, 53, 54, 55, 56, 57, 58, 59, 60, 61, 69, 70, 71, 78, 79, 80],
    [0, 9, 18, 27, 36, 45, 54, 55, 56, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74],
    [1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74],
    [2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 66, 67, 68, 69, 70, 71, 72, 73, 74],
    [3, 12, 21, 30, 39, 48, 57, 58, 59, 63, 64, 65, 67, 68, 69, 70, 71, 75, 76, 77],
    [4, 13, 22, 31, 40, 49, 57, 58, 59, 63, 64, 65, 66, 68, 69, 70, 71, 75, 76, 77],
    [5, 14, 23, 32, 41, 50, 57, 58, 59, 63, 64, 65, 66, 67, 69, 70, 71, 75, 76, 77],
    [6, 15, 24, 33, 42, 51, 60, 61, 62, 63, 64, 65, 66, 67, 68, 70, 71, 78, 79, 80],
    [7, 16, 25, 34, 43, 52, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 71, 78, 79, 80],
    [8, 17, 26, 35, 44, 53, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 78, 79, 80],
    [0, 9, 18, 27, 36, 45, 54, 55, 56, 63, 64, 65, 73, 74, 75, 76, 77, 78, 79, 80],
    [1, 10, 19, 28, 37, 46, 54, 55, 56, 63, 64, 65, 72, 74, 75, 76, 77, 78, 79, 80],
    [2, 11, 20, 29, 38, 47, 54, 55, 56, 63, 64, 65, 72, 73, 75, 76, 77, 78, 79, 80],
    [3, 12, 21, 30, 39, 48, 57, 58, 59, 66, 67, 68, 72, 73, 74, 76, 77, 78, 79, 80],
    [4, 13, 22, 31, 40, 49, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 77, 78, 79, 80],
    [5, 14, 23, 32, 41, 50, 57, 58, 59, 66, 67, 68, 72, 73, 74, 75, 76, 78, 79, 80],
    [6, 15, 24, 33, 42, 51, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 79, 80],
    [7, 16, 25, 34, 43, 52, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 80],
    [8, 17, 26, 35, 44, 53, 60, 61, 62, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79],
];

macro_rules! define_types(
    ($( $name:ident : $limit:expr ),* $(,)*) => {
        $(
            define_types!(@internal $name $limit , concat!("`0..", stringify!($limit), "`"));
        )*
    };
    (@internal $name:ident $limit:expr , $limit_doc:expr) => {


        impl $name {
            /// Construct a new instance of this type. `num` needs to be inside
            #[doc = $limit_doc]
            ///
            /// # Panics
            /// Panics, if the argument is outside the allowed range.
            pub fn new(num: u8) -> Self {
                debug_assert!(num < $limit);
                $name(num)
            }

            /// Construct a new instance of this type. If `num` is outside the allowed range, then `None` is returned.
            pub fn new_checked(num: u8) -> Option<Self> {
                if num < $limit {
                    Some($name(num))
                } else {
                    None
                }
            }

            /// Returns the number contained within.
            pub fn get(self) -> u8 {
                self.0
            }

            /// Returns the number contained within as `usize`. Guarantees that the numbering starts from `0`.
            /// For position structs such as this one, it's equivalent to `.get() as usize`.
            pub fn as_index(self) -> usize {
                self.0 as _
            }

            /// Iterator over all positions of this type.
            pub fn all() -> impl Iterator<Item = Self> {
                (0..$limit).map(Self::new)
            }
        }
    };
);

/// One of the 81 cells of the sudoku
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Cell(u8);

/// Set of 9 cells in a horizontal line
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Row(u8);

/// Set of 9 cells in a vertical line
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Col(u8);

/// Set of 9 cells in a 3x3 box shape
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Block(u8);

/// A [`Row`] or [`Col`]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Line(u8);

/// A [`Row`], [`Col`] or [`Block`]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct House(u8);

/// Intersection of a [`Block`] and a [`Row`], 3 cells in a row.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct MiniRow(u8);

/// Intersection of a [`Block`] and a [`Col`], 3 cells in a column.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct MiniCol(u8);

/// A [`MiniRow`] or [`MiniCol`]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct MiniLine(u8);

/// Set of 3 [`Row`]s and 3 [`Block`]s where each [`Row`] intersects each [`Block`]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Band(u8);

/// Set of 3 [`Col`]s and 3 [`Block`]s where each [`Col`] intersects each [`Block`]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Stack(u8);

/// A [`Band`] or [`Stack`]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Chute(u8);

define_types!(
    Cell: 81,
    Row: 9,
    Col: 9,
    Block: 9,
    Line: 18,
    House: 27,
    MiniRow: 27,
    MiniCol: 27,
    MiniLine: 54,
    Band: 3,
    Stack: 3,
    Chute: 6,
);

/// A [`Row`] or [`Col`]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum LineType {
    Row(Row),
    Col(Col),
}

impl Line {
    /// Determine whether this line is a [`Row`] or a [`Col`]
    pub fn categorize(self) -> LineType {
        debug_assert!(self.0 < BLOCK_OFFSET);
        match self.0 < COL_OFFSET {
            true => LineType::Row(Row::new(self.0)),
            false => LineType::Col(Col::new(self.0 - COL_OFFSET)),
        }
    }
}

/// A [`Row`], [`Col`] or [`Block`]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum HouseType {
    Row(Row),
    Col(Col),
    Block(Block),
}

impl House {
    pub(crate) const ALL_ROWS: Set<House> = Set(0o000_000_777);
    pub(crate) const ALL_COLS: Set<House> = Set(0o000_777_000);
    pub(crate) const ALL_BLOCKS: Set<House> = Set(0o777_000_000);

    /// Determine whether this house is a [`Row`], [`Col`] or [`Block`]
    #[rustfmt::skip]
    pub fn categorize(self) -> HouseType {
        debug_assert!(self.0 < 27);
        match self.0 {
            0..= 8 => HouseType::Row(Row::new(self.0)),
            9..=17 => HouseType::Col(Col::new(self.0 - COL_OFFSET)),
            _      => HouseType::Block(Block::new(self.0 - BLOCK_OFFSET)),
        }
    }
}

/// A [`Band`] or [`Stack`]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ChuteType {
    Band(Band),
    Stack(Stack),
}

impl Chute {
    /// Determine whether this chute is a [`Band`] or [`Stack`]
    pub fn categorize(self) -> ChuteType {
        debug_assert!(self.0 < 6);
        match self.0 < 3 {
            true => ChuteType::Band(Band::new(self.0)),
            false => ChuteType::Stack(Stack::new(self.0 - 3)),
        }
    }
}

/// A [`MiniRow`] or [`MiniCol`]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MiniLineType {
    MiniRow(MiniRow),
    MiniCol(MiniCol),
}

impl MiniLine {
    /// Determine whether this miniline is a [`MiniRow`] or [`MiniCol`]
    pub fn categorize(self) -> MiniLineType {
        debug_assert!(self.0 < 54);
        match self.0 < 27 {
            true => MiniLineType::MiniRow(MiniRow::new(self.0)),
            false => MiniLineType::MiniCol(MiniCol::new(self.0 - 27)),
        }
    }
}

/// Generic struct for a cell inside a given set of cells, like e.g. a [`House`]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Position<IN>(pub(crate) u8, std::marker::PhantomData<IN>);

impl<IN> Position<IN> {
    /// Construct a new instance of this type.
    pub fn new(pos: u8) -> Self {
        // TODO: make panic on invalid positions
        Position(pos, std::marker::PhantomData)
    }

    /// Returns the number contained within.
    pub fn get(self) -> u8 {
        self.0
    }

    /// Returns the number contained within as `usize`. Guarantees that the numbering starts from `0`.
    /// For position structs such as this one, it's equivalent to `.get() as usize`.
    pub fn as_index(self) -> usize {
        self.0 as _
    }
}

macro_rules! into_cells {
    ( $( $name:ident => |$arg:ident| $code:block );* $(;)* ) => {
        $(
            impl $name {
                /// Returns a bitset of the cells belonging to this grouping.
                pub fn cells(self) -> Set<Cell> {
                    let $arg = self;
                    Set($code)
                }
            }
        )*
    };
}

// the closures here aren't actually closures, they just introduce
// the variables to be used in the code blocks for macro hygiene reasons
into_cells!(
    Cell => |cell| { 1 << cell.0 };
    Row  => |row| { 0o777 << (9 * row.0) };
    Col  => |col| { 0o_001_001_001___001_001_001___001_001_001 << col.0 };
    Block  => |block| {
        let band = block.0 / 3;
        let stack = block.0 % 3;
        0o007_007_007 << (band * 27 + stack * 3)
    };
    Line => |line| {
        use self::LineType::*;
        match line.categorize() {
            Row(row) => row.cells().0,
            Col(col) => col.cells().0,
        }
    };
    House => |house| {
        use self::HouseType::*;
        match house.categorize() {
            Row(row) => row.cells().0,
            Col(col) => col.cells().0,
            Block(block) => block.cells().0,
        }
    };
    MiniRow => |mr| { 0o7 << 3 * mr.0 };
    //MiniCol => |mc| { 0o001_001_001 << mc / 9 * 27 + mc % 9 }; // old, different counting system
    MiniCol => |mc| {
        let band = mc.0 % 3;
        let col = mc.0 / 3;
        0o001_001_001 << band * 27 + col
    };
    MiniLine => |ml| {
        use self::MiniLineType::*;
        match ml.categorize() {
            MiniRow(mr)  => mr.cells().0,
            MiniCol(mc) => mc.cells().0,
        }
    };
    Band => |band| { 0o777_777_777 << 27 * band.0 };
    Stack => |stack| { 0o_007_007_007___007_007_007___007_007_007 << 3 * stack.0 };
    Chute => |chute| {
        use self::ChuteType::*;
        match chute.categorize() {
            Band(band) => band.cells().0,
            Stack(stack) => stack.cells().0,
        }
    };
);

///////////////////////////////////////////////////////////////////////////////////////////////
//                                  Conversions
///////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! define_conversion_shortcuts {
    (
        $(
            $type:ty : {
                $( $target_type:ty , $method_name:ident );* $(;)*
            }
        )*
    ) => {
        $(
            impl $type {
                $(
                    #[inline(always)]
                    pub fn $method_name(self) -> $target_type {
                        <$target_type>::from(self)
                    }
                )*
            }
        )*
    };
}

define_conversion_shortcuts!(
    Cell : {
        Row, row;
        Col, col;
        Block, block;
        //Position<Row>, row_pos;
        //Position<Col>, col_pos;
        //Position<Block>, block_pos;
    }
    /*
    Row : {
        Line, line;
        House, house;
    }
    Col : {
        Line, line;
        House, house;
    }
    Block : {
        House, house;
    }
    Position<Row> : {
        Position<Line>, line_pos;
        Position<House>, house_pos;
    }
    Position<Col> : {
        Position<Line>, line_pos;
        Position<House>, house_pos;
    }
    Position<Block> : {
        Position<House>, house_pos;
    }
    */
);

impl Cell {
    pub(crate) fn row_pos(self) -> Position<House> {
        Position::<Row>::from(self).into()
    }

    pub(crate) fn col_pos(self) -> Position<House> {
        Position::<Col>::from(self).into()
    }

    pub(crate) fn block_pos(self) -> Position<House> {
        Position::<Block>::from(self).into()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

impl Cell {
    /// Returns an array of the row, column and block this cell belongs to, in that order.
    pub(crate) fn houses(self) -> [House; 3] {
        [self.row().house(), self.col().house(), self.block().house()]
    }

    /// Returns an iterator over the 20 cells that share a house with this one. The iteration
    /// order is unspecified.
    #[inline(always)]
    pub(crate) fn neighbors(self) -> impl IntoIterator<Item = Cell> {
        HOUSE_NEIGHBORS_OF_CELL[self.as_index()]
            .iter()
            .cloned()
            .map(Cell::new)
    }

    /// Returns a set of the 20 cells that share a house with this one.
    #[inline(always)]
    #[rustfmt::skip]
    pub(crate) fn neighbors_set(self) -> Set<Cell> {
        (self.row().cells() | self.col().cells() | self.block().cells())
            ^ self
    }
}

impl Chute {
    /// Returns an array of all minilines in this chute, ordered first by lines, then by block.
    /// This means that minirows are given from left to right, then top to bottom and minicols
    /// the other way around.
    pub(crate) fn minilines(self) -> [MiniLine; 9] {
        let mut slices = [MiniLine(0); 9];
        for (i, slice) in (0..9).zip(slices.iter_mut()) {
            *slice = MiniLine(self.0 * 9 + i);
        }
        slices
    }
}

impl Line {
    pub(crate) const ALL_ROWS: Set<Line> = Set(0o000_777);
    pub(crate) const ALL_COLS: Set<Line> = Set(0o777_000);
}

impl MiniLine {
    /// Returns the line and block neighbors of this miniline. The neighbors within the same
    /// line are given in the first array.
    pub(crate) fn neighbors(self) -> ([MiniLine; 2], [MiniLine; 2]) {
        // line neighbor, block neighbor
        let (ln, bn) = MINILINE_NEIGHBORS[self.as_index()];
        (
            [MiniLine(ln[0]), MiniLine(ln[1])],
            [MiniLine(bn[0]), MiniLine(bn[1])],
        )
    }

    // TODO: refactor to be part of the define_conversion_shortcuts macro
    pub(crate) fn chute(self) -> Chute {
        Chute::new(self.0 / 9)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

/// Trait for cell groupings that have all of their cells enumerated
pub trait CellAt: Sized {
    /// Return the cell at the given position in this cell grouping
    fn cell_at(self, pos: Position<Self>) -> Cell;

    /// Return the set of cells at the given positions in this cell grouping
    fn cells_at(self, positions: Set<Position<Self>>) -> Set<Cell>
    where
        Position<Self>: crate::bitset::SetElement,
        Set<Position<Self>>: IntoIterator<Item = Position<Self>>,
        Self: Copy,
    {
        // could be sped up via look up tables
        // for 9 bit positions
        let mut cells = Set::NONE;
        for pos in positions {
            cells |= self.cell_at(pos);
        }
        cells
    }
}

impl CellAt for Row {
    fn cell_at(self, pos: Position<Row>) -> Cell {
        Cell::new(self.0 * 9 + pos.0)
    }
}

impl CellAt for Col {
    fn cell_at(self, pos: Position<Col>) -> Cell {
        Cell::new(pos.0 * 9 + self.0)
    }
}

impl CellAt for Block {
    fn cell_at(self, pos: Position<Block>) -> Cell {
        // TODO: use (implement) methods for getting band of block and such
        let band = self.0 / 3;
        let stack = self.0 % 3;
        let row_in_band = pos.0 / 3;
        let col_in_stack = pos.0 % 3;
        let row = band * 3 + row_in_band;
        let col = stack * 3 + col_in_stack;
        Cell::new(row * 9 + col)
    }
}

impl CellAt for Line {
    fn cell_at(self, pos: Position<Line>) -> Cell {
        match self.categorize() {
            LineType::Row(row) => row.cell_at(Position::new(pos.0)),
            LineType::Col(col) => col.cell_at(Position::new(pos.0)),
        }
    }
}

impl CellAt for House {
    fn cell_at(self, pos: Position<House>) -> Cell {
        match self.categorize() {
            HouseType::Row(row) => row.cell_at(Position::new(pos.0)),
            HouseType::Col(col) => col.cell_at(Position::new(pos.0)),
            HouseType::Block(block) => block.cell_at(Position::new(pos.0)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////
macro_rules! impl_from {
    ( $( $from:ty, $to:ty, |$arg:ident| $code:block ),* $(,)* ) => {
        $(
            impl From<$from> for $to {
                fn from($arg: $from) -> $to {
                    let $arg = $arg.0;
                    <$to>::new($code)
                }
            }
        )*
    };
}

// same as above, but doesn't take the inner element before
// executing $code
macro_rules! impl_from_raw {
    ( $( $from:ty, $to:ty, |$arg:ident| $code:block ),* $(,)* ) => {
        $(
            impl From<$from> for $to {
                fn from($arg: $from) -> $to {
                    $code
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_from!(
    Row, Line, |r| { r },
    Col, Line, |c| { c + COL_OFFSET },
    Row, House, |r| { r },
    Col, House, |c| { c + COL_OFFSET },
    Line, House, |l| { l },
    Block, House, |b| { b + BLOCK_OFFSET },
    Band, Chute, |b| { b },
    Stack, Chute, |s| { s + 3 },
);

// non-equivalent conversions
// the first type is the container of the second
#[rustfmt::skip]
impl_from!(
    Cell, Row, |c| { row(c) },
    Cell, Col, |c| { col(c) },
    Cell, Block, |c| { 3 * band(c) + stack(c) },
    Cell, Band, |c| { band(c) },
    Cell, Stack, |c| { stack(c) },
    Cell, Position<Row>, |c| { col(c) },
    Cell, Position<Col>, |c| { row(c) },
    Position<Row>, Position<Line>, |pos| { pos },
    Position<Col>, Position<Line>, |pos| { pos },
    Position<Row>, Position<House>, |pos| { pos },
    Position<Col>, Position<House>, |pos| { pos },
    Position<Line>, Position<House>, |pos| { pos },
    Position<Block>, Position<House>, |pos| { pos },
    Cell, Position<Block>, |c| { row(c) % 3 * 3 + col(c) % 3 },
    Cell, Position<Band>, |c| { row(c) % 3 * 9 + col(c) },
    Position<Chute>, Position<Band>, |pos| { pos },
    //Cell, Position<Stack>, |c| { row(c) % 3 * 3 + col(c) % 3 }, // not sure how to lay this out, yet
);

/*impl_from_raw!(
    Position<Row>, Set<Position<Line>>, |pos| { Position::<Line>::from(pos).as_set() },
    Position<Col>, Set<Position<Line>>, |pos| { Position::<Line>::from(pos).as_set() },
    Position<Row>, Set<Position<House>>, |pos| { Position::<House>::from(pos).as_set() },
    Position<Col>, Set<Position<House>>, |pos| { Position::<House>::from(pos).as_set() },
    Position<Line>, Set<Position<House>>, |pos| { Position::<House>::from(pos).as_set() },
    Position<Block>, Set<Position<House>>, |pos| { Position::<House>::from(pos).as_set() },
);*/

// TODO: generalize
impl Set<Position<House>> {
    pub fn as_line_set(self) -> Set<Position<Line>> {
        debug_assert!(self.0 <= Set::<Position<Line>>::ALL.0);
        Set::from_bits(self.0)
    }
}

impl<T: SetElement> From<T> for Set<T> {
    fn from(element: T) -> Self {
        element.as_set()
    }
}

pub(crate) trait IntoHouse: Into<House> {
    /// Return the [`House`] corresponding to this value. Equivalent to `House::from(self)`.
    #[inline(always)]
    fn house(self) -> House {
        self.into()
    }
}

impl<T: Into<House>> IntoHouse for T {}

#[rustfmt::skip]
static MINILINE_NEIGHBORS: [([u8; 2], [u8; 2]); 54] = [
        ([1, 2], [3, 6]),
        ([2, 0], [4, 7]),
        ([0, 1], [5, 8]),

        ([4, 5], [6, 0]),
        ([5, 3], [7, 1]),
        ([3, 4], [8, 2]),

        ([7, 8], [0, 3]),
        ([8, 6], [1, 4]),
        ([6, 7], [2, 5]),


        ([10, 11], [12, 15]),
        ([11, 9], [13, 16]),
        ([9, 10], [14, 17]),

        ([13, 14], [15, 9]),
        ([14, 12], [16, 10]),
        ([12, 13], [17, 11]),

        ([16, 17], [9, 12]),
        ([17, 15], [10, 13]),
        ([15, 16], [11, 14]),


        ([19, 20], [21, 24]),
        ([20, 18], [22, 25]),
        ([18, 19], [23, 26]),

        ([22, 23], [24, 18]),
        ([23, 21], [25, 19]),
        ([21, 22], [26, 20]),

        ([25, 26], [18, 21]),
        ([26, 24], [19, 22]),
        ([24, 25], [20, 23]),

        // above: rows, below: cols

        ([28, 29], [30, 33]),
        ([29, 27], [31, 34]),
        ([27, 28], [32, 35]),

        ([31, 32], [33, 27]),
        ([32, 30], [34, 28]),       // stack 0
        ([30, 31], [35, 29]),

        ([34, 35], [27, 30]),
        ([35, 33], [28, 31]),
        ([33, 34], [29, 32]),


        ([37, 38], [39, 42]),
        ([38, 36], [40, 43]),
        ([36, 37], [41, 44]),

        ([40, 41], [42, 36]),
        ([41, 39], [43, 37]),       // stack 1
        ([39, 40], [44, 38]),

        ([43, 44], [36, 39]),
        ([44, 42], [37, 40]),
        ([42, 43], [38, 41]),


        ([46, 47], [48, 51]),
        ([47, 45], [49, 52]),
        ([45, 46], [50, 53]),

        ([49, 50], [51, 45]),
        ([50, 48], [52, 46]),       // stack 2
        ([48, 49], [53, 47]),

        ([52, 53], [45, 48]),
        ([53, 51], [46, 49]),
        ([51, 52], [47, 50]),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unique() {
        for digit in <Set<Digit>>::ALL {
            assert_eq!(digit.as_set().into_iter().count(), 1);
        }
    }

    #[test]
    fn row_cells() {
        for (raw_row, row) in (0..9).map(|r| (r, Row::new(r))) {
            let first_cell = raw_row * 9;

            let iter1 = row.cells().into_iter();
            let iter2 = (first_cell..first_cell + 9).map(Cell::new);
            assert!(iter1.eq(iter2));
        }
    }

    #[test]
    fn col_cells() {
        for (raw_col, col) in (0..9).map(|c| (c, Col::new(c))) {
            let iter1 = col.cells().into_iter();
            let iter2 = (raw_col..81).step_by(9).map(Cell::new);
            assert!(iter1.eq(iter2));
        }
    }

    /*
    #[test]
    fn block_cells() {
        for (raw_block, block) in (0..9).map(|b| (b, Block::new(b))) {
            let iter1 = block.cells().into_iter();


            let iter2 = (raw_block..81).step_by(9).map(Cell::new);
            assert!( iter1.eq(iter2) );
        }
    }
    */

    #[test]
    fn band_cells() {
        for (raw_band, band) in (0..3).map(|b| (b, Band::new(b))) {
            let first_cell = raw_band * 27;

            let iter1 = band.cells().into_iter();
            let iter2 = (first_cell..first_cell + 27).map(Cell::new);
            assert!(iter1.eq(iter2));
        }
    }

    #[test]
    fn stack_cells() {
        for (raw_stack, stack) in (0..3).map(|s| (s, Stack::new(s))) {
            let first_col = raw_stack * 3;

            let iter1 = stack.cells().into_iter();

            let iter2 = (0..9)
                .flat_map(|row| {
                    let cell = row * 9 + first_col;
                    cell..cell + 3
                })
                .map(Cell::new);

            assert!(iter1.eq(iter2));
        }
    }
}
