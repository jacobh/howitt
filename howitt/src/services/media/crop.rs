#[derive(Debug)]
pub struct CropDimensions {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub fn calculate_square_center_crop((width, height): (usize, usize)) -> CropDimensions {
    if width > height {
        let x = (width - height) / 2;
        CropDimensions {
            x,
            y: 0,
            width: height,
            height,
        }
    } else {
        let y = (height - width) / 2;
        CropDimensions {
            x: 0,
            y,
            width,
            height: width,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_square_center_crop_wider_than_tall() {
        let input_dimensions = (800, 600);

        let crop = calculate_square_center_crop(input_dimensions);

        assert_eq!(crop.x, 100); // (800 - 600) / 2
        assert_eq!(crop.y, 0);
        assert_eq!(crop.width, 600);
        assert_eq!(crop.height, 600);
    }

    #[test]
    fn test_calculate_square_center_crop_taller_than_wide() {
        let input_dimensions = (600, 800);

        let crop = calculate_square_center_crop(input_dimensions);

        assert_eq!(crop.x, 0);
        assert_eq!(crop.y, 100); // (800 - 600) / 2
        assert_eq!(crop.width, 600);
        assert_eq!(crop.height, 600);
    }
}
