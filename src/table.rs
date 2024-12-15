use crate::enums::*;
use crate::helper_functions::*;
use crate::tables::*;
use crate::{CodePair, Color, Drawing, DxfError, DxfResult, Handle};

//------------------------------------------------------------------------------
//                                                                         LineTypeElement
//------------------------------------------------------------------------------
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LineTypeElement {
    pub dash_dot_space_length: f64,
    pub complex_line_type_element_type: i16,
    pub shape_number: Option<i16>,
    #[doc(hidden)]
    pub __styles_handle: Option<Handle>,
    pub rotation_angle: Option<f64>,
    pub text_string: Option<String>,
    pub scale_value: Option<f64>,
    pub x_offset: Option<f64>,
    pub y_offset: Option<f64>,
}

impl Default for LineTypeElement {
    fn default() -> Self {
        Self {
            dash_dot_space_length: 0.0,
            complex_line_type_element_type: 0,
            shape_number: None,
            __styles_handle: None,
            rotation_angle: None,
            text_string: None,
            scale_value: None,
            x_offset: None,
            y_offset: None,
        }
    }
}

impl LineType {
    pub fn add_line_type_pattern(&mut self, drawing: &mut Drawing, pattern: &str) -> DxfResult<()> {
        if !pattern.starts_with('A') {
            return Err(DxfError::MalformedString);
        }

        let elements =
            self.parse_elements(pattern.strip_prefix("A,").expect("A, prefix not found"));
        let mut line_type_element = LineTypeElement::default();
        let length = elements.len();
        for (i, element) in elements.iter().enumerate() {
            if element.starts_with('[') {
                let nested_elements = element.split(',');

                // TODO: If the first element does not start with quotation marks
                // return error as we do not support SHAPE line types yet
                for (j, nested_element) in nested_elements.enumerate() {
                    if j == 0 && !nested_element.starts_with("[\"") {
                        return Err(DxfError::MalformedString);
                    } else if j == 0 {
                        line_type_element.text_string =
                            Some(nested_element.replace("[", "").replace('"', ""));
                    }

                    line_type_element.complex_line_type_element_type = 2;
                    line_type_element.shape_number = Some(0);
                    line_type_element.__styles_handle = Some(
                        self.add_text_style(
                            drawing,
                            &line_type_element
                                .text_string
                                .as_ref()
                                .unwrap()
                                .replace("/", "forwardslash"),
                        )?,
                    );

                    let nested_element = nested_element.replace("[", "").replace("]", "");

                    if nested_element == "STANDARD" {
                        continue;
                    } else if let Some(value) = nested_element.strip_prefix("S=") {
                        line_type_element.scale_value = Some(parse_f64(value.to_string(), 0)?);
                    } else if let Some(value) = nested_element.strip_prefix("R=") {
                        line_type_element.rotation_angle = Some(parse_f64(value.to_string(), 0)?);
                    } else if let Some(value) = nested_element.strip_prefix("X=") {
                        line_type_element.x_offset = Some(parse_f64(value.to_string(), 0)?);
                    } else if let Some(value) = nested_element.strip_prefix("Y=") {
                        line_type_element.y_offset = Some(parse_f64(value.to_string(), 0)?);
                    }
                }
            } else {
                if i > 0 {
                    self.element_count += 1;
                    self.line_elements.push(line_type_element.clone());

                    // Create a new line type element
                    line_type_element = LineTypeElement::default();
                }

                line_type_element.dash_dot_space_length = parse_f64(element.to_string(), 0)?;
            }

            if i == length - 1 {
                self.element_count += 1;
                self.line_elements.push(line_type_element.clone());
            }
        }

        // Set total_length_pattern
        self.total_pattern_length = self
            .line_elements
            .iter()
            .map(|e| e.dash_dot_space_length.abs())
            .sum();

        Ok(())
    }

    fn add_text_style(
        &mut self,
        drawing: &mut Drawing,
        text_style: &str,
    ) -> Result<Handle, DxfError> {
        for style in drawing.styles() {
            if style.name == text_style {
                return Ok(style.handle);
            }
        }

        let style = Style {
            name: text_style.to_string(),
            ..Default::default()
        };

        let style_with_handle = drawing.add_style(style);

        Ok(style_with_handle.handle)
    }

    fn parse_elements(&self, input: &str) -> Vec<String> {
        let mut elements = Vec::new();
        let mut current = String::new();
        let mut in_brackets = false;

        for c in input.chars() {
            match c {
                '[' => {
                    in_brackets = true;
                    current.push(c);
                }
                ']' => {
                    in_brackets = false;
                    current.push(c);
                }
                ',' => {
                    if in_brackets {
                        current.push(c);
                    } else {
                        elements.push(current.trim().to_string());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }

        // Push the last element if any
        if !current.is_empty() {
            elements.push(current.trim().to_string());
        }

        elements
    }
}

// Used as override in TableSpec.xmls
pub(crate) fn custom_line_type_add_code_pairs(
    item: &LineType,
    pairs: &mut Vec<CodePair>,
    drawing: &Drawing,
) {
    pairs.push(CodePair::new_string(3, &item.description));
    pairs.push(CodePair::new_i16(72, item.alignment_code as i16));
    pairs.push(CodePair::new_i16(73, item.element_count as i16));
    pairs.push(CodePair::new_f64(40, item.total_pattern_length));

    // Specification alignement LineType TableSpec.xml
    for line_element in &item.line_elements {
        pairs.push(CodePair::new_f64(49, line_element.dash_dot_space_length));
        if drawing.header.version >= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(
                74,
                line_element.complex_line_type_element_type,
            ));
            if let Some(shape_number) = line_element.shape_number {
                pairs.push(CodePair::new_i16(75, shape_number));
            }
            if let Some(styles_handle) = line_element.__styles_handle {
                pairs.push(CodePair::new_str(340, styles_handle.as_string().as_str()));
            }
            if let Some(scale_value) = line_element.scale_value {
                pairs.push(CodePair::new_f64(46, scale_value));
            }
            if let Some(rotation_angle) = line_element.rotation_angle {
                pairs.push(CodePair::new_f64(50, rotation_angle));
            }
            if let Some(x_offset) = line_element.x_offset {
                pairs.push(CodePair::new_f64(44, x_offset));
            }
            if let Some(y_offset) = line_element.y_offset {
                pairs.push(CodePair::new_f64(45, y_offset));
            }
            if let Some(text_string) = &line_element.text_string {
                pairs.push(CodePair::new_str(9, text_string));
            }
        }
    }
}

// Used as override in TableSpec.xmls
pub(crate) fn custom_read_line_type_code_pairs(
    pair: CodePair,
    line_type: &mut LineType,
) -> DxfResult<()> {
    match pair.code {
        49 => {
            let mut element = LineTypeElement::default();
            element.dash_dot_space_length = pair.assert_f64()?;
            line_type.line_elements.push(element);
        }
        74 => {
            let element = line_type.line_elements.last_mut().unwrap();
            element.complex_line_type_element_type = pair.assert_i16()?;
        }
        75 => {
            let element = line_type.line_elements.last_mut().unwrap();
            element.shape_number = Some(pair.assert_i16()?);
        }
        46 => {
            let element = line_type.line_elements.last_mut().unwrap();
            element.scale_value = Some(pair.assert_f64()?);
        }
        44 => {
            let element = line_type.line_elements.last_mut().unwrap();
            element.x_offset = Some(pair.assert_f64()?);
        }
        45 => {
            let element = line_type.line_elements.last_mut().unwrap();
            element.y_offset = Some(pair.assert_f64()?);
        }
        9 => {
            let element = line_type.line_elements.last_mut().unwrap();
            element.text_string = Some(pair.assert_string()?);
        }
        _ => (),
    }

    Ok(())
}

//------------------------------------------------------------------------------
//                                                                         Layer
//------------------------------------------------------------------------------
impl Layer {
    /// Ensure all values are valid.
    pub fn normalize(&mut self) {
        default_if_empty(&mut self.line_type_name, "CONTINUOUS");
        match self.color.raw_value() {
            0 | 256 => self.color = Color::from_raw_value(7), // BYLAYER and BYBLOCK aren't valid layer colors
            _ => (),
        }
    }
}

//------------------------------------------------------------------------------
//                                                                         Style
//------------------------------------------------------------------------------
impl Style {
    /// Ensure all values are valid.
    pub fn normalize(&mut self) {
        ensure_positive_or_default(&mut self.text_height, 0.0);
        ensure_positive_or_default(&mut self.width_factor, 1.0);
    }
}

//------------------------------------------------------------------------------
//                                                                          View
//------------------------------------------------------------------------------
impl View {
    /// Ensure all values are valid.
    pub fn normalize(&mut self) {
        ensure_positive_or_default(&mut self.view_height, 1.0);
        ensure_positive_or_default(&mut self.view_width, 1.0);
        ensure_positive_or_default(&mut self.lens_length, 1.0);
    }
}

//------------------------------------------------------------------------------
//                                                                      ViewPort
//------------------------------------------------------------------------------
impl ViewPort {
    /// Ensure all values are valid.
    pub fn normalize(&mut self) {
        ensure_positive_or_default(&mut self.snap_spacing.x, 1.0);
        ensure_positive_or_default(&mut self.snap_spacing.y, 1.0);
        ensure_positive_or_default(&mut self.snap_spacing.z, 1.0);
        ensure_positive_or_default(&mut self.grid_spacing.x, 1.0);
        ensure_positive_or_default(&mut self.grid_spacing.y, 1.0);
        ensure_positive_or_default(&mut self.grid_spacing.z, 1.0);
        ensure_positive_or_default(&mut self.view_height, 1.0);
        ensure_positive_or_default(&mut self.view_port_aspect_ratio, 1.0);
        ensure_positive_or_default(&mut self.lens_length, 50.0);
        ensure_positive_or_default_i16(&mut self.ucs_icon, 3);
        ensure_positive_or_default_i32(&mut self.circle_sides, 1000);
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::*;
    use crate::enums::*;
    use crate::helper_functions::tests::*;
    use crate::objects::*;
    use crate::tables::*;
    use crate::*;
    use float_cmp::approx_eq;

    // STD
    use std::io::Cursor;

    fn read_table(table_name: &str, value_pairs: Vec<CodePair>) -> Drawing {
        let mut pairs = vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "TABLES"),
            CodePair::new_str(0, "TABLE"),
            CodePair::new_str(2, table_name),
            CodePair::new_str(100, "AcDbSymbolTable"),
            CodePair::new_i16(70, 0),
        ];
        for pair in value_pairs {
            pairs.push(pair);
        }
        pairs.push(CodePair::new_str(0, "ENDTAB"));
        pairs.push(CodePair::new_str(0, "ENDSEC"));
        pairs.push(CodePair::new_str(0, "EOF"));
        drawing_from_pairs(pairs)
    }

    #[test]
    fn read_unsupported_table() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "TABLES"),
            CodePair::new_str(0, "TABLE"),
            CodePair::new_str(2, "UNSUPPORTED"),
            CodePair::new_str(0, "UNSUPPORTED"),
            CodePair::new_str(2, "unsupported-name"),
            CodePair::new_str(0, "ENDTAB"),
            CodePair::new_str(0, "TABLE"),
            CodePair::new_str(2, "LAYER"),
            CodePair::new_str(0, "LAYER"),
            CodePair::new_str(0, "ENDTAB"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        assert_eq!(1, drawing.layers().count());
    }

    #[test]
    fn read_single_layer() {
        let drawing = read_table(
            "LAYER",
            vec![
                CodePair::new_str(0, "LAYER"),
                CodePair::new_str(2, "layer-name"),
            ],
        );
        let layers = drawing.layers().collect::<Vec<_>>();
        assert_eq!(1, layers.len());
        assert_eq!("layer-name", layers[0].name);
    }

    #[test]
    fn read_variable_table_items() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "TABLES"),
            CodePair::new_str(0, "TABLE"), // no app ids
            CodePair::new_str(2, "APPID"),
            CodePair::new_str(0, "ENDTAB"),
            CodePair::new_str(0, "TABLE"), // 1 layer
            CodePair::new_str(2, "LAYER"),
            CodePair::new_str(0, "LAYER"),
            CodePair::new_str(2, "layer-name"),
            CodePair::new_str(0, "ENDTAB"),
            CodePair::new_str(0, "TABLE"), // 2 styles
            CodePair::new_str(2, "STYLE"),
            CodePair::new_str(0, "STYLE"),
            CodePair::new_f64(40, 1.1),
            CodePair::new_str(0, "STYLE"),
            CodePair::new_f64(40, 2.2),
            CodePair::new_str(0, "ENDTAB"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        assert_eq!(0, drawing.block_records().count()); // not listed in file, but make sure there are still 0
        assert_eq!(0, drawing.app_ids().count());
        let layers = drawing.layers().collect::<Vec<_>>();
        assert_eq!(1, layers.len());
        assert_eq!("layer-name", layers[0].name);
        let styles = drawing.styles().collect::<Vec<_>>();
        assert_eq!(2, styles.len());
        assert!(approx_eq!(f64, 1.1, styles[0].text_height));
        assert!(approx_eq!(f64, 2.2, styles[1].text_height));
    }

    #[test]
    fn read_layer_color_and_layer_is_on() {
        let drawing = read_table(
            "LAYER",
            vec![CodePair::new_str(0, "LAYER"), CodePair::new_i16(62, 5)],
        );
        let layers = drawing.layers().collect::<Vec<_>>();
        let layer = layers[0];
        assert_eq!(Some(5), layer.color.index());
        assert!(layer.is_layer_on);
    }

    #[test]
    fn read_layer_color_and_layer_is_off() {
        let drawing = read_table(
            "LAYER",
            vec![CodePair::new_str(0, "LAYER"), CodePair::new_i16(62, -5)],
        );
        let layers = drawing.layers().collect::<Vec<_>>();
        let layer = layers[0];
        assert_eq!(Some(5), layer.color.index());
        assert!(!layer.is_layer_on);
    }

    #[test]
    fn write_layer() {
        let mut drawing = Drawing::new();
        let layer = Layer {
            name: String::from("layer-name"),
            color: Color::from_index(3),
            ..Default::default()
        };

        drawing.add_layer(layer);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "LAYER"),
                CodePair::new_str(5, "1E"),
                CodePair::new_str(100, "AcDbSymbolTableRecord"),
                CodePair::new_str(100, "AcDbLayerTableRecord"),
                CodePair::new_str(2, "layer-name"),
                CodePair::new_i16(70, 0),
                CodePair::new_i16(62, 3),
                CodePair::new_str(6, "CONTINUOUS"),
            ],
        );
    }

    #[test]
    fn normalize_layer() {
        let mut layer = Layer {
            name: String::from("layer-name"),
            color: Color::by_layer(), // value 256 not valid; normalized to 7
            line_type_name: String::from(""), // empty string not valid; normalized to CONTINUOUS
            ..Default::default()
        };
        layer.normalize();
        assert_eq!(Some(7), layer.color.index());
        assert_eq!("CONTINUOUS", layer.line_type_name);
    }

    #[test]
    fn normalize_view() {
        let mut view = View {
            view_height: 0.0,  // invalid; normalized to 1.0
            view_width: -1.0,  // invalid; normalized to 1.0
            lens_length: 42.0, // valid
            ..Default::default()
        };
        view.normalize();
        assert!(approx_eq!(f64, 1.0, view.view_height));
        assert!(approx_eq!(f64, 1.0, view.view_width));
        assert!(approx_eq!(f64, 42.0, view.lens_length));
    }

    #[test]
    fn read_table_item_with_extended_data() {
        let drawing = read_table(
            "LAYER",
            vec![
                CodePair::new_str(0, "LAYER"),
                CodePair::new_str(102, "{IXMILIA"),
                CodePair::new_str(1, "some string"),
                CodePair::new_str(102, "}"),
            ],
        );
        let layers = drawing.layers().collect::<Vec<_>>();
        let layer = layers[0];
        assert_eq!(1, layer.extension_data_groups.len());
        let group = &layer.extension_data_groups[0];
        assert_eq!("IXMILIA", group.application_name);
        assert_eq!(1, group.items.len());
        match group.items[0] {
            ExtensionGroupItem::CodePair(ref p) => {
                assert_eq!(&CodePair::new_str(1, "some string"), p);
            }
            _ => panic!("expected a code pair"),
        }
    }

    #[test]
    fn write_table_item_with_extended_data() {
        let layer = Layer {
            extension_data_groups: vec![ExtensionGroup {
                application_name: String::from("IXMILIA"),
                items: vec![ExtensionGroupItem::CodePair(CodePair::new_str(
                    1,
                    "some string",
                ))],
            }],
            ..Default::default()
        };
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14;
        drawing.add_layer(layer);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(102, "{IXMILIA"),
                CodePair::new_str(1, "some string"),
                CodePair::new_str(102, "}"),
            ],
        );
    }

    #[test]
    fn read_table_item_with_x_data() {
        let drawing = read_table(
            "LAYER",
            vec![
                CodePair::new_str(0, "LAYER"),
                CodePair::new_str(1001, "IXMILIA"),
                CodePair::new_f64(1040, 1.1),
            ],
        );
        let layers = drawing.layers().collect::<Vec<_>>();
        let layer = layers[0];
        assert_eq!(1, layer.x_data.len());
        let x = &layer.x_data[0];
        assert_eq!("IXMILIA", x.application_name);
        assert_eq!(1, x.items.len());
        match x.items[0] {
            XDataItem::Real(r) => assert!(approx_eq!(f64, 1.1, r)),
            _ => panic!("expected a code pair"),
        }
    }

    #[test]
    fn write_table_item_with_x_data() {
        let layer = Layer {
            x_data: vec![XData {
                application_name: String::from("IXMILIA"),
                items: vec![XDataItem::Real(1.1)],
            }],
            ..Default::default()
        };
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_layer(layer);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(1001, "IXMILIA"),
                CodePair::new_f64(1040, 1.1),
            ],
        );
    }

    #[test]
    fn normalize_layers() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.layers().count());
        file.header.current_layer = String::from("current layer");
        file.normalize();
        let layers = file.layers().collect::<Vec<_>>();
        assert_eq!(2, layers.len());
        assert_eq!("0", layers[0].name);
        assert_eq!("current layer", layers[1].name);
    }

    #[test]
    fn normalize_line_types() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.line_types().count());
        file.add_entity(Entity {
            common: EntityCommon {
                line_type_name: String::from("line type"),
                ..Default::default()
            },
            specific: EntityType::Line(Default::default()),
        });
        file.normalize();
        let line_types = file.line_types().collect::<Vec<_>>();
        assert_eq!(4, line_types.len());
        assert_eq!("BYBLOCK", line_types[0].name);
        assert_eq!("BYLAYER", line_types[1].name);
        assert_eq!("CONTINUOUS", line_types[2].name);
        assert_eq!("line type", line_types[3].name);
    }

    #[test]
    fn test_line_types_writing_order() {
        let mut file = Drawing::new();
        file.header.version = AcadVersion::R13;
        file.clear();
        assert_eq!(0, file.line_types().count());
        file.normalize();
        let line_types = file.line_types().collect::<Vec<_>>();
        assert_eq!(3, line_types.len());

        // Add custom line type now
        let mut line_type = LineType::default();
        line_type.name = "custom-dash-line".to_string();
        line_type.description = "Dash ---".to_string();

        line_type.element_count = 2;

        let line_type_element = LineTypeElement {
            dash_dot_space_length: 0.75,
            ..Default::default()
        };

        let line_type_element_2 = LineTypeElement {
            dash_dot_space_length: -0.25,
            ..Default::default()
        };

        line_type.line_elements.push(line_type_element);
        line_type.line_elements.push(line_type_element_2);

        file.add_line_type(line_type);
        assert_eq!(4, file.line_types().count());

        assert_contains_pairs(
            &file,
            vec![
                CodePair::new_str(2, "custom-dash-line"),
                CodePair::new_i16(70, 0),
                CodePair::new_str(3, "Dash ---"),
                CodePair::new_i16(72, 65),
                CodePair::new_i16(73, 2),
                CodePair::new_f64(40, 0.0),
                CodePair::new_f64(49, 0.75),
                CodePair::new_i16(74, 0),
                CodePair::new_f64(49, -0.25),
                CodePair::new_i16(74, 0),
                CodePair::new_str(0, "ENDTAB"),
            ],
        );
    }

    // TODO: This did not work in the past, so I made a test for it to ensure we
    // can fix it later
    #[test]
    #[ignore]
    fn test_read_line_types() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        drawing.clear();
        assert_eq!(0, drawing.line_types().count());
        drawing.normalize();
        let line_types = drawing.line_types().collect::<Vec<_>>();
        assert_eq!(3, line_types.len());

        // Add custom line type now
        let mut line_type = LineType::default();
        line_type.name = "custom-dash-line".to_string();
        line_type.description = "Dash ---".to_string();
        line_type.element_count = 2;

        let line_type_element = LineTypeElement {
            dash_dot_space_length: 0.75,
            ..Default::default()
        };

        let line_type_element_2 = LineTypeElement {
            dash_dot_space_length: -0.25,
            ..Default::default()
        };

        line_type.line_elements.push(line_type_element);
        line_type.line_elements.push(line_type_element_2);

        drawing.add_line_type(line_type);

        // Write drawing to cursor
        let mut buf = Cursor::new(vec![]);
        drawing.save(&mut buf).unwrap();

        // Print drawing text
        let text = String::from_utf8(buf.get_ref().to_vec()).unwrap();
        println!("{}", text);

        // Read drawing from cursor
        let read_draw = Drawing::load(&mut buf).unwrap();

        let line_types = read_draw.line_types().collect::<Vec<_>>();
        assert_eq!(line_types.len(), 4);
    }

    #[test]
    fn normalize_text_styles() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.styles().count());
        file.add_entity(Entity::new(EntityType::Attribute(Attribute {
            text_style_name: String::from("text style"),
            ..Default::default()
        })));
        file.normalize();
        let styles = file.styles().collect::<Vec<_>>();
        assert_eq!(3, styles.len());
        assert_eq!("ANNOTATIVE", styles[0].name);
        assert_eq!("STANDARD", styles[1].name);
        assert_eq!("text style", styles[2].name);
    }

    #[test]
    fn normalize_view_ports() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.view_ports().count());
        file.normalize();
        let view_ports = file.view_ports().collect::<Vec<_>>();
        assert_eq!(1, view_ports.len());
        assert_eq!("*ACTIVE", view_ports[0].name);
    }

    #[test]
    fn normalize_views() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.views().count());
        file.add_object(Object::new(ObjectType::PlotSettings(PlotSettings {
            plot_view_name: String::from("some view"),
            ..Default::default()
        })));
        file.normalize();
        let views = file.views().collect::<Vec<_>>();
        assert_eq!(1, views.len());
        assert_eq!("some view", views[0].name);
    }

    #[test]
    fn normalize_ucs() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.ucss().count());
        file.header.ucs_name = String::from("primary ucs");
        file.normalize();
        let ucss = file.ucss().collect::<Vec<_>>();
        assert_eq!(1, ucss.len());
        assert_eq!("primary ucs", ucss[0].name);
    }

    #[test]
    fn block_record_table_not_written_on_r12() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R12;
        assert_not_contains_pairs(&drawing, vec![CodePair::new_str(0, "BLOCK_RECORD")]);
    }

    #[test]
    fn block_record_table_is_written_on_r13() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "BLOCK_RECORD")]);
    }

    #[test]
    fn test_line_type_pattern_parsing() {
        let mut drawing = Drawing::new();

        let mut line_type = LineType::default();
        line_type
            .add_line_type_pattern(&mut drawing, "A,0.35,-1.05")
            .unwrap();
        assert_eq!(line_type.line_elements.len(), 2);
        assert_eq!(line_type.element_count, 2);
        assert_eq!(line_type.total_pattern_length, 1.4);
        assert_eq!(line_type.line_elements[0].dash_dot_space_length, 0.35);
        assert_eq!(line_type.line_elements[1].dash_dot_space_length, -1.05);

        let mut line_type = LineType::default();
        line_type
            .add_line_type_pattern(&mut drawing, "A,0.35,-1.05,0.35,-1.05")
            .unwrap();
        assert_eq!(line_type.line_elements.len(), 4);
        assert_eq!(line_type.element_count, 4);
        assert_eq!(line_type.total_pattern_length, 2.8);
        assert_eq!(line_type.line_elements[0].dash_dot_space_length, 0.35);
        assert_eq!(line_type.line_elements[1].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[2].dash_dot_space_length, 0.35,);
        assert_eq!(line_type.line_elements[3].dash_dot_space_length, -1.05);

        let mut line_type = LineType::default();
        line_type
            .add_line_type_pattern(&mut drawing, "A,2.8,0,0,-1.05,0.7,-1.05")
            .unwrap();
        assert_eq!(line_type.line_elements.len(), 6);
        assert_eq!(line_type.element_count, 6);
        assert_eq!(line_type.total_pattern_length, 3.85 + 0.7 + 1.05);
        assert_eq!(line_type.line_elements[0].dash_dot_space_length, 2.8);
        assert_eq!(line_type.line_elements[1].dash_dot_space_length, 0.0);
        assert_eq!(line_type.line_elements[2].dash_dot_space_length, 0.0);
        assert_eq!(line_type.line_elements[3].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[4].dash_dot_space_length, 0.7);
        assert_eq!(line_type.line_elements[5].dash_dot_space_length, -1.05);
    }

    #[test]
    fn test_line_type_pattern_parsing_complex() {
        let mut drawing = Drawing::new();

        let initial_styles_count = drawing.styles().count();

        let mut line_type = LineType::default();
        line_type
            .add_line_type_pattern(
                &mut drawing,
                "A,0.35,-1.05,[\"X\",STANDARD,S=1.50,R=0.0,X=-0.25,Y=-.75],0.35,-1.05,0.35,-1.05",
            )
            .unwrap();

        let styles_count = drawing.styles().count();

        assert_eq!(styles_count, initial_styles_count + 1);

        assert_eq!(line_type.line_elements.len(), 6);
        assert_eq!(line_type.element_count, 6);
        assert_eq!(
            line_type.total_pattern_length,
            0.35 + 1.05 + 0.35 + 1.05 + 0.35 + 1.05
        );
        assert_eq!(line_type.line_elements[0].dash_dot_space_length, 0.35);
        assert_eq!(line_type.line_elements[0].shape_number, None);

        // Complex element two
        assert_eq!(line_type.line_elements[1].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[1].shape_number, Some(0));
        assert_eq!(line_type.line_elements[1].scale_value, Some(1.50));
        assert_eq!(line_type.line_elements[1].rotation_angle, Some(0.0));
        assert_eq!(line_type.line_elements[1].x_offset, Some(-0.25));
        assert_eq!(line_type.line_elements[1].y_offset, Some(-0.75));
        assert_eq!(line_type.line_elements[1].complex_line_type_element_type, 2);
        assert_eq!(
            line_type.line_elements[1].text_string,
            Some("X".to_string())
        );

        assert_eq!(line_type.line_elements[2].dash_dot_space_length, 0.35);
        assert_eq!(line_type.line_elements[2].shape_number, None);
        assert_eq!(line_type.line_elements[3].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[3].shape_number, None);
        assert_eq!(line_type.line_elements[4].dash_dot_space_length, 0.35);
        assert_eq!(line_type.line_elements[4].shape_number, None);
        assert_eq!(line_type.line_elements[5].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[5].shape_number, None);
    }

    #[test]
    fn test_line_type_pattern_parsing_multiple_complex() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2018;

        let initial_styles_count = drawing.styles().count();

        let mut line_type = LineType::default();
        line_type.name = "custom-dash-line".to_string();
        line_type.description = "Dash ---".to_string();
        line_type
            .add_line_type_pattern(
                &mut drawing,
                "A,0.35,-1.05,[\"X\",STANDARD,S=1.50,R=0.0,X=-0.25,Y=-.75],0.35,-1.05,[\"X\",STANDARD,S=1.50,R=0.0,X=-0.25,Y=-.75],0.35,-1.05",
            )
            .unwrap();

        let styles_count = drawing.styles().count();

        assert_eq!(styles_count, initial_styles_count + 1);

        assert_eq!(line_type.line_elements.len(), 6);
        assert_eq!(line_type.element_count, 6);
        assert_eq!(
            line_type.total_pattern_length,
            0.35 + 1.05 + 0.35 + 1.05 + 0.35 + 1.05
        );
        assert_eq!(line_type.line_elements[0].dash_dot_space_length, 0.35);

        // Complex element two
        assert_eq!(line_type.line_elements[1].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[1].shape_number, Some(0));
        assert_eq!(line_type.line_elements[1].scale_value, Some(1.50));
        assert_eq!(line_type.line_elements[1].rotation_angle, Some(0.0));
        assert_eq!(line_type.line_elements[1].x_offset, Some(-0.25));
        assert_eq!(line_type.line_elements[1].y_offset, Some(-0.75));
        assert_eq!(line_type.line_elements[1].complex_line_type_element_type, 2);
        assert_eq!(
            line_type.line_elements[1].text_string,
            Some("X".to_string())
        );

        assert_eq!(line_type.line_elements[2].dash_dot_space_length, 0.35);

        // Complex element fourth
        assert_eq!(line_type.line_elements[3].dash_dot_space_length, -1.05);
        assert_eq!(line_type.line_elements[3].shape_number, Some(0));
        assert_eq!(line_type.line_elements[3].scale_value, Some(1.50));
        assert_eq!(line_type.line_elements[3].rotation_angle, Some(0.0));
        assert_eq!(line_type.line_elements[3].x_offset, Some(-0.25));
        assert_eq!(line_type.line_elements[3].y_offset, Some(-0.75));
        assert_eq!(line_type.line_elements[3].complex_line_type_element_type, 2);
        assert_eq!(
            line_type.line_elements[3].text_string,
            Some("X".to_string())
        );

        assert_eq!(line_type.line_elements[4].dash_dot_space_length, 0.35);
        assert_eq!(line_type.line_elements[5].dash_dot_space_length, -1.05);

        drawing.add_line_type(line_type);

        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(2, "custom-dash-line"),
                CodePair::new_i16(70, 0),
                CodePair::new_str(3, "Dash ---"),
                CodePair::new_i16(72, 65),
                CodePair::new_i16(73, 6),
                CodePair::new_f64(40, 4.2),
                CodePair::new_f64(49, 0.35),
                CodePair::new_i16(74, 0),
                CodePair::new_f64(49, -1.05),
                CodePair::new_i16(74, 2),
                CodePair::new_i16(75, 0),
                CodePair::new_str(340, "1E"),
                CodePair::new_f64(46, 1.5),
                CodePair::new_f64(50, 0.0),
                CodePair::new_f64(44, -0.25),
                CodePair::new_f64(45, -0.75),
                CodePair::new_str(9, "X"),
                CodePair::new_f64(49, 0.35),
                CodePair::new_i16(74, 0),
                CodePair::new_f64(49, -1.05),
                CodePair::new_i16(74, 2),
                CodePair::new_i16(75, 0),
                CodePair::new_str(340, "1E"),
                CodePair::new_f64(46, 1.5),
                CodePair::new_f64(50, 0.0),
                CodePair::new_f64(44, -0.25),
                CodePair::new_f64(45, -0.75),
                CodePair::new_str(9, "X"),
                CodePair::new_f64(49, 0.35),
                CodePair::new_i16(74, 0),
                CodePair::new_f64(49, -1.05),
                CodePair::new_i16(74, 0),
                CodePair::new_str(0, "ENDTAB"),
            ],
        );
    }
}
