#[cfg(test)]
mod tests {
    use crate::*;
    use voronoi::Point;
    use approx::assert_relative_eq;

    #[test]
    fn test_lerp() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(10.0, 10.0);

        // Test lerp at t = 0.5
        let result = lerp(a, b, 0.5);
        assert_relative_eq!(result.x(), 5.0);
        assert_relative_eq!(result.y(), 5.0);

        // Test lerp at t = 0.0
        let result = lerp(a, b, 0.0);
        assert_relative_eq!(result.x(), 0.0);
        assert_relative_eq!(result.y(), 0.0);

        // Test lerp at t = 1.0
        let result = lerp(a, b, 1.0);
        assert_relative_eq!(result.x(), 10.0);
        assert_relative_eq!(result.y(), 10.0);
    }

    #[test]
    fn test_window_conf() {
        let conf = window_conf();
        assert_eq!(conf.window_width, WINDOW_SIZE);
        assert_eq!(conf.window_height, WINDOW_SIZE);
    }
}
