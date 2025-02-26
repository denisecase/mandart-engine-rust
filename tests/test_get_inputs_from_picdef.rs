#[cfg(test)]
mod tests {
    use mandart_core::get_inputs_from_picdef_string::get_shape_inputs_from_picdef_string;

    #[test]
    fn test_shape_inputs_parsing() {
        let test_json = r#"
        {
            "imageHeight": 600,
            "imageWidth": 800,
            "iterationsMax": 1000,
            "scale": 2.5,
            "xCenter": -0.5,
            "yCenter": 0.0,
            "theta": 45,
            "dFIterMin": 0.1,
            "rSqLimit": 4.0,
            "mandPowerReal": 2
        }"#;

        let shape_inputs = get_shape_inputs_from_picdef_string(test_json)
            .expect("Failed to parse shape inputs");

        assert_eq!(shape_inputs.image_width, 800);
        assert_eq!(shape_inputs.iterations_max, 1000.0);
        assert_eq!(shape_inputs.mand_power_real, 2);
    }
}
