use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, Color, ScatterView, LineSeriesView};

pub fn save_final_chart(data: &Vec<(f64, f64)>, theta_0: f64, theta_1: f64, labels: &(String, String)) {
	let width = 1000;
	let height = 700;
	let (top, right, bottom, left) = (50, 40, 50, 60);
	let mut parsed: Vec<(f32, f32)> = Vec::new();
    for el in data {
        parsed.push((el.0 as f32, el.1 as f32));
    }

    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    for el in &parsed {
		if el.0 > max_x {
			max_x = el.0;
		}
    if el.1 > max_y {
			max_y = el.1;
		}
	}

	let x = ScaleLinear::new()
		.set_domain(vec![0 as f32, max_x * 1.1])
		.set_range(vec![0, width - left - right]);
	let y = ScaleLinear::new()
		.set_domain(vec![0 as f32, max_y * 1.5])
		.set_range(vec![height - top - bottom, 0]);
	
	// Create Scatter series view that is going to represent the data.
	let scatter_view = ScatterView::new()
		.set_x_scale(&x)
		.set_y_scale(&y)
		.set_label_position(PointLabelPosition::E)
		.set_marker_type(MarkerType::Square)
		.set_colors(Color::from_vec_of_hex_strings(vec!["#409EFF"]))
		.set_label_visibility(false)
		.load_data(&parsed).unwrap();
	
	let start_curve = theta_0;
	let end_curve = theta_0 + (theta_1 * max_x as f64 * 1.2 as f64);
    let curve = vec![(0 as f32, start_curve as f32), (max_x * 1.2 as f32, end_curve as f32)];

    // Create Line series view that is going to represent the data.
    let line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#F56C6C"]))
		.set_label_visibility(false)
        .load_data(&curve).unwrap();

	Chart::new()
		.set_width(width)
		.set_height(height)
		.set_margins(top, right, bottom, left)
		.add_title(String::from("Linear Regression 42"))
		.add_view(&scatter_view)
        .add_view(&line_view)
		.add_axis_bottom(&x)
		.add_axis_left(&y)
		.add_bottom_axis_label(&labels.0)
		.add_left_axis_label(&labels.1)
		.save("final_chart.svg").unwrap();
}

pub fn save_line_chart(data: &Vec<(f64, f64)>, labels: &(String, String)) {
	let width = 1000;
	let height = 700;
	let (top, right, bottom, left) = (90, 40, 50, 60);
	let mut parsed: Vec<(f32, f32)> = Vec::new();
    for el in data {
        parsed.push((el.0 as f32, el.1 as f32));
    }

    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
	let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    for el in &parsed {
		if el.0 > max_x {
			max_x = el.0;
		}
		if el.0 < min_x {
			min_x = el.0;
		}
        if el.1 > max_y {
			max_y = el.1;
		}
		if el.1 < min_y {
			min_y = el.1;
		}
	}

	let x = ScaleLinear::new()
		.set_domain(vec![min_x as f32, max_x * 1.1])
		.set_range(vec![0, width - left - right]);
	let y = ScaleLinear::new()
		.set_domain(vec![min_y as f32, max_y * 1.5])
		.set_range(vec![height - top - bottom, 0]);
	
    // Create Line series view that is going to represent the data.
    let line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#F56C6C"]))
		.set_label_visibility(false)
        .load_data(&parsed).unwrap();

	let mut pute = Chart::new();

	pute.set_width(width)
		.set_height(height)
		.set_margins(top, right, bottom, left)
		.add_title(String::from("Learning curve"))
        .add_view(&line_view)
		.add_axis_bottom(&x)
		.add_axis_left(&y)
		.add_bottom_axis_label(&labels.0)
		.add_left_axis_label(&labels.1)
		.save("chart_learning_curve.svg").unwrap();

}