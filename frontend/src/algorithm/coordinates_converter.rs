use crate::Coordinates;

pub fn convert_device_to_webgl(width: f64, height: f64, x: f64, y: f64) -> (f64, f64) {
    let out_x = (x - width / 2.0) * (1.0 / (width / 2.0));
    let out_y = -(y - height / 2.0) * (1.0 / (height / 2.0));

    (out_x, out_y)
}

pub fn convert_figure_to_device(coordinates: &Coordinates, x: f64, y: f64) -> (f64, f64) {
    let out_x = (x * coordinates.zoom_rate) - coordinates.scroll_h_pos
        + (coordinates.center_x * coordinates.zoom_rate);
    let out_y = -1.0
        * ((y * coordinates.zoom_rate) + coordinates.scroll_v_pos
            - (coordinates.center_y * coordinates.zoom_rate));

    (out_x, out_y)
}

pub fn convert_device_to_figure(coordinates: &Coordinates, x: f64, y: f64) -> (f64, f64) {
    let out_x = (x + coordinates.scroll_h_pos - (coordinates.center_x * coordinates.zoom_rate))
        * 1.000
        / coordinates.zoom_rate;
    let out_y = -1.0
        * (y + coordinates.scroll_v_pos - (coordinates.center_y * coordinates.zoom_rate))
        * 1.000
        / coordinates.zoom_rate;

    (out_x, out_y)
}

pub fn convert_figure_to_webgl(
    coordinates: &Coordinates,
    width: f64,
    height: f64,
    x: f64,
    y: f64,
) -> (f64, f64) {
    let temp_x = (x * coordinates.zoom_rate) - coordinates.scroll_h_pos
        + (coordinates.center_x * coordinates.zoom_rate);
    let temp_y = -1.0
        * ((y * coordinates.zoom_rate) + coordinates.scroll_v_pos
            - (coordinates.center_y * coordinates.zoom_rate));

    convert_device_to_webgl(width, height, temp_x, temp_y)
}
