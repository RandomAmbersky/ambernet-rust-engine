mod array2d;
mod cell_type;
mod error;
mod pos2d;
mod size2d;
mod unsigned_num;

pub use cell_type::CellType;
pub use unsigned_num::UnsignedNum;

pub use array2d::Array2D;
pub use pos2d::Pos2D;
pub use size2d::Size2D;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
