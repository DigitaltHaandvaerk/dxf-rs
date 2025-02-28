use dxf::entities::*;
use dxf::{Drawing, Point};

pub fn all() -> dxf::DxfResult<()> {
    apply_line_types_to_entities()?;
    Ok(())
}

fn apply_line_types_to_entities() -> dxf::DxfResult<()> {
    let mut drawing = Drawing::new();

    //
    // create a new line type named "dashed-lines"...
    //
    let mut line_type = dxf::tables::LineType {
        name: "dash-lines".to_string(),
        ..Default::default()
    };
    line_type.total_pattern_length = 1.0;
    // line pattern contains 2 elements; positive values draw a line, negative values draw a gap
    // the following draws 3/4 of a line with a 1/4 gap
    line_type.element_count = 2;

    let line_type_element = dxf::tables::LineTypeElement {
        dash_dot_space_length: 0.75,
        ..Default::default()
    };

    let line_type_element_2 = dxf::tables::LineTypeElement {
        dash_dot_space_length: -0.25,
        ..Default::default()
    };

    line_type.line_elements.push(line_type_element);
    line_type.line_elements.push(line_type_element_2);
    drawing.add_line_type(line_type);

    //
    // create a new line entity that applies the specified line type by name
    //
    let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(10.0, 10.0, 0.0));
    let mut line = Entity::new(EntityType::Line(line));
    line.common.line_type_name = String::from("dashed-lines");
    drawing.add_entity(line);

    drawing.save_file("apply_line_types_to_entities.dxf")?;
    Ok(())
}
