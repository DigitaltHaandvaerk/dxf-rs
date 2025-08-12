// other implementation is in `generated/entities.rs`

use enum_primitive::FromPrimitive;

use crate::{CodePair, Color, DxfError, DxfResult, Handle, Point, Vector};

use crate::code_pair_put_back::CodePairPutBack;
use crate::entities::*;
use crate::enums::*;
use crate::helper_functions::*;
use crate::Drawing;

//------------------------------------------------------------------------------
//                                                                           Arc
//------------------------------------------------------------------------------
impl Arc {
    pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Self {
        Arc {
            center,
            radius,
            start_angle: start,
            end_angle: end,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Circle
//------------------------------------------------------------------------------
impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle {
            center,
            radius,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                 DimensionBase
//------------------------------------------------------------------------------
impl DimensionBase {
    fn set_dimension_type(&mut self, val: i16) -> DxfResult<()> {
        self.is_block_reference_referenced_by_this_block_only = (val & 32) == 32;
        self.is_ordinate_x_type = (val & 64) == 64;
        self.is_at_user_defined_location = (val & 128) == 128;
        self.dimension_type = enum_from_number!(DimensionType, Aligned, from_i16, val & 0x0F); // only take the lower 4 bits
        Ok(())
    }
    pub(crate) fn dimension_type(&self) -> i16 {
        let mut val = self.dimension_type as i16;
        if self.is_block_reference_referenced_by_this_block_only {
            val |= 32;
        }
        if self.is_ordinate_x_type {
            val |= 64;
        }
        if self.is_at_user_defined_location {
            val |= 128;
        }
        val
    }
}

//------------------------------------------------------------------------------
//                                                                        Face3D
//------------------------------------------------------------------------------
impl Face3D {
    pub fn new(
        first_corner: Point,
        second_corner: Point,
        third_corner: Point,
        fourth_corner: Point,
    ) -> Self {
        Face3D {
            first_corner,
            second_corner,
            third_corner,
            fourth_corner,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Insert
//------------------------------------------------------------------------------
impl Insert {
    pub fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.__attributes_and_handles.iter().map(|a| &a.0)
    }
    pub fn attributes_mut(&mut self) -> impl Iterator<Item = &mut Attribute> {
        self.__attributes_and_handles.iter_mut().map(|a| &mut a.0)
    }
    pub fn add_attribute(&mut self, drawing: &mut Drawing, att: Attribute) {
        let att_handle = drawing.next_handle();
        self.__attributes_and_handles.push((att, att_handle));
    }
}

//------------------------------------------------------------------------------
//                                                                          Line
//------------------------------------------------------------------------------
impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line {
            p1,
            p2,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                              LwPolylineVertex
//------------------------------------------------------------------------------
/// Represents a single vertex of a `LwPolyline`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LwPolylineVertex {
    pub x: f64,
    pub y: f64,
    pub id: i32,
    pub starting_width: f64,
    pub ending_width: f64,
    pub bulge: f64,
}

//------------------------------------------------------------------------------
//                                                                       MLeader
//------------------------------------------------------------------------------

impl MLeader {
    pub fn get_vertices(&self) -> Vec<Point> {
        if self.vertices.is_empty() {
            // If there are no vertices, return an empty vector
            return vec![];
        }

        if self.enable_dogleg {
            // Insert before the first coordinate the dogleg coordinate
            let dogleg_vertice = match self.calculate_dogleg_vertice() {
                Some(dogleg) => dogleg,
                None => return vec![], // If no dogleg can be calculated, return empty
            };
            return vec![dogleg_vertice]
                .into_iter()
                .chain(self.vertices.clone().into_iter()) // <-- Use self.vertices directly, not self.vertices()
                .collect();
        }
        self.vertices.clone()
    }

    fn calculate_dogleg_vertice(&self) -> Option<Point> {
        // Calculate from the first coordinate of the vertices
        let first_vertex = match self.vertices.first() {
            Some(first_vertex) => first_vertex,
            None => return None, // No vertices available
        };
        // Calculate the new vertex with cos and sin of the 3D coordinate
        Some(Point {
            x: first_vertex.x + self.dogleg_vector.x * self.dogleg_length,
            y: first_vertex.y + self.dogleg_vector.y * self.dogleg_length,
            z: first_vertex.z + self.dogleg_vector.z * self.dogleg_length,
        })
    }
}

//------------------------------------------------------------------------------
//                                                                    ModelPoint
//------------------------------------------------------------------------------
impl ModelPoint {
    pub fn new(p: Point) -> Self {
        ModelPoint {
            location: p,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                      Polyline
//------------------------------------------------------------------------------
impl Polyline {
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex> {
        self.__vertices_and_handles.iter().map(|v| &v.0)
    }
    pub fn vertices_mut(&mut self) -> impl Iterator<Item = &mut Vertex> {
        self.__vertices_and_handles.iter_mut().map(|v| &mut v.0)
    }
    pub fn add_vertex(&mut self, drawing: &mut Drawing, vertex: Vertex) {
        let vertex_handle = drawing.next_handle();
        self.__vertices_and_handles.push((vertex, vertex_handle));
    }
}

//------------------------------------------------------------------------------
//                                                                   ProxyEntity
//------------------------------------------------------------------------------
impl ProxyEntity {
    // lower word
    pub fn object_drawing_format_version(&self) -> i32 {
        (self.__object_drawing_format & 0xFFFF) as i32
    }
    pub fn set_object_drawing_format_version(&mut self, version: i32) {
        self.__object_drawing_format |= version as u32 & 0xFFFF;
    }
    // upper word
    pub fn object_maintenance_release_version(&self) -> i32 {
        self.__object_drawing_format as i32 >> 4
    }
    pub fn set_object_mainenance_release_version(&mut self, version: i32) {
        self.__object_drawing_format =
            (version << 4) as u32 + (self.__object_drawing_format & 0xFFFF);
    }
}

//------------------------------------------------------------------------------
//                                                                         Solid
//------------------------------------------------------------------------------
impl Solid {
    pub fn new(
        first_corner: Point,
        second_corner: Point,
        third_corner: Point,
        fourth_corner: Point,
    ) -> Self {
        Solid {
            first_corner,
            second_corner,
            third_corner,
            fourth_corner,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                         Trace
//------------------------------------------------------------------------------
impl Trace {
    pub fn new(
        first_corner: Point,
        second_corner: Point,
        third_corner: Point,
        fourth_corner: Point,
    ) -> Self {
        Trace {
            first_corner,
            second_corner,
            third_corner,
            fourth_corner,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Vertex
//------------------------------------------------------------------------------
impl Vertex {
    pub fn new(location: Point) -> Self {
        Vertex {
            location,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                    EntityType
//------------------------------------------------------------------------------
impl EntityType {
    fn apply_dimension_code_pair(&mut self, pair: &CodePair) -> DxfResult<bool> {
        match *self {
            EntityType::RotatedDimension(ref mut dim) => match pair.code {
                12 => {
                    dim.insertion_point.x = pair.assert_f64()?;
                }
                22 => {
                    dim.insertion_point.y = pair.assert_f64()?;
                }
                32 => {
                    dim.insertion_point.z = pair.assert_f64()?;
                }
                13 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                23 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                33 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                14 => {
                    dim.definition_point_3.x = pair.assert_f64()?;
                }
                24 => {
                    dim.definition_point_3.y = pair.assert_f64()?;
                }
                34 => {
                    dim.definition_point_3.z = pair.assert_f64()?;
                }
                50 => {
                    dim.rotation_angle = pair.assert_f64()?;
                }
                52 => {
                    dim.extension_line_angle = pair.assert_f64()?;
                }
                _ => {
                    return Ok(false);
                }
            },
            EntityType::RadialDimension(ref mut dim) => match pair.code {
                15 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                25 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                35 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                40 => {
                    dim.leader_length = pair.assert_f64()?;
                }
                _ => {
                    return Ok(false);
                }
            },
            EntityType::DiameterDimension(ref mut dim) => match pair.code {
                15 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                25 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                35 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                40 => {
                    dim.leader_length = pair.assert_f64()?;
                }
                _ => {
                    return Ok(false);
                }
            },
            EntityType::AngularThreePointDimension(ref mut dim) => match pair.code {
                13 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                23 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                33 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                14 => {
                    dim.definition_point_3.x = pair.assert_f64()?;
                }
                24 => {
                    dim.definition_point_3.y = pair.assert_f64()?;
                }
                34 => {
                    dim.definition_point_3.z = pair.assert_f64()?;
                }
                15 => {
                    dim.definition_point_4.x = pair.assert_f64()?;
                }
                25 => {
                    dim.definition_point_4.y = pair.assert_f64()?;
                }
                35 => {
                    dim.definition_point_4.z = pair.assert_f64()?;
                }
                16 => {
                    dim.definition_point_5.x = pair.assert_f64()?;
                }
                26 => {
                    dim.definition_point_5.y = pair.assert_f64()?;
                }
                36 => {
                    dim.definition_point_5.z = pair.assert_f64()?;
                }
                _ => {
                    return Ok(false);
                }
            },
            EntityType::OrdinateDimension(ref mut dim) => match pair.code {
                13 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                23 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                33 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                14 => {
                    dim.definition_point_3.x = pair.assert_f64()?;
                }
                24 => {
                    dim.definition_point_3.y = pair.assert_f64()?;
                }
                34 => {
                    dim.definition_point_3.z = pair.assert_f64()?;
                }
                _ => {
                    return Ok(false);
                }
            },
            _ => {
                return Err(DxfError::UnexpectedEnumValue(pair.offset));
            }
        }
        Ok(true)
    }
}

//------------------------------------------------------------------------------
//                                                                  EntityCommon
//------------------------------------------------------------------------------
impl EntityCommon {
    /// Ensures all values are valid.
    pub fn normalize(&mut self) {
        default_if_empty(&mut self.layer, "0");
    }
}

//------------------------------------------------------------------------------
//                                                                        Entity
//------------------------------------------------------------------------------
impl Entity {
    /// Creates a new `Entity` with the default common values.
    pub fn new(specific: EntityType) -> Self {
        Entity {
            common: Default::default(),
            specific,
        }
    }
    /// Ensures all entity values are valid.
    pub fn normalize(&mut self) {
        self.common.normalize();
        // no entity-specific values to set
    }
    pub(crate) fn read(iter: &mut CodePairPutBack) -> DxfResult<Option<Entity>> {
        'new_entity: loop {
            match iter.next() {
                // first code pair must be 0/entity-type
                Some(Ok(pair @ CodePair { code: 0, .. })) => {
                    let type_string = pair.assert_string()?;
                    if type_string == "ENDSEC" || type_string == "ENDBLK" {
                        iter.put_back(Ok(pair));
                        return Ok(None);
                    }

                    match &*type_string {
                        "DIMENSION" => {
                            // dimensions require special handling
                            let mut common = EntityCommon::default();
                            let mut dimension_entity: Option<EntityType> = None;
                            let mut dimension_base = DimensionBase::default();
                            loop {
                                match iter.next() {
                                    Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                        // new entity or ENDSEC
                                        iter.put_back(Ok(pair));
                                        break;
                                    }
                                    Some(Ok(pair)) => {
                                        match dimension_entity {
                                            Some(ref mut dim) => {
                                                if !dim.apply_dimension_code_pair(&pair)? {
                                                    common.apply_individual_pair(&pair, iter)?;
                                                }
                                            }
                                            None => {
                                                match pair.code {
                                                    1 => {
                                                        dimension_base.text =
                                                            pair.assert_string()?;
                                                    }
                                                    2 => {
                                                        dimension_base.block_name =
                                                            pair.assert_string()?;
                                                    }
                                                    3 => {
                                                        dimension_base.dimension_style_name =
                                                            pair.assert_string()?;
                                                    }
                                                    10 => {
                                                        dimension_base.definition_point_1.x =
                                                            pair.assert_f64()?;
                                                    }
                                                    20 => {
                                                        dimension_base.definition_point_1.y =
                                                            pair.assert_f64()?;
                                                    }
                                                    30 => {
                                                        dimension_base.definition_point_1.z =
                                                            pair.assert_f64()?;
                                                    }
                                                    11 => {
                                                        dimension_base.text_mid_point.x =
                                                            pair.assert_f64()?;
                                                    }
                                                    21 => {
                                                        dimension_base.text_mid_point.y =
                                                            pair.assert_f64()?;
                                                    }
                                                    31 => {
                                                        dimension_base.text_mid_point.z =
                                                            pair.assert_f64()?;
                                                    }
                                                    41 => {
                                                        dimension_base.text_line_spacing_factor =
                                                            pair.assert_f64()?;
                                                    }
                                                    42 => {
                                                        dimension_base.actual_measurement =
                                                            pair.assert_f64()?;
                                                    }
                                                    51 => {
                                                        dimension_base.horizontal_direction_angle =
                                                            pair.assert_f64()?;
                                                    }
                                                    53 => {
                                                        dimension_base.text_rotation_angle =
                                                            pair.assert_f64()?;
                                                    }
                                                    70 => {
                                                        dimension_base.set_dimension_type(
                                                            pair.assert_i16()?,
                                                        )?;
                                                    }
                                                    71 => {
                                                        dimension_base.attachment_point = enum_from_number!(
                                                            AttachmentPoint,
                                                            TopLeft,
                                                            from_i16,
                                                            pair.assert_i16()?
                                                        );
                                                    }
                                                    72 => {
                                                        dimension_base.text_line_spacing_style = enum_from_number!(
                                                            TextLineSpacingStyle,
                                                            AtLeast,
                                                            from_i16,
                                                            pair.assert_i16()?
                                                        );
                                                    }
                                                    210 => {
                                                        dimension_base.normal.x =
                                                            pair.assert_f64()?;
                                                    }
                                                    220 => {
                                                        dimension_base.normal.y =
                                                            pair.assert_f64()?;
                                                    }
                                                    230 => {
                                                        dimension_base.normal.z =
                                                            pair.assert_f64()?;
                                                    }
                                                    280 => {
                                                        dimension_base.version = enum_from_number!(
                                                            Version,
                                                            R2010,
                                                            from_i16,
                                                            pair.assert_i16()?
                                                        );
                                                    }
                                                    100 => {
                                                        match &*pair.assert_string()? {
                                                            "AcDbAlignedDimension" => {
                                                                dimension_entity = Some(
                                                                    EntityType::RotatedDimension(
                                                                        RotatedDimension {
                                                                            dimension_base:
                                                                                dimension_base
                                                                                    .clone(),
                                                                            ..Default::default()
                                                                        },
                                                                    ),
                                                                );
                                                            }
                                                            "AcDbRadialDimension" => {
                                                                dimension_entity = Some(
                                                                    EntityType::RadialDimension(
                                                                        RadialDimension {
                                                                            dimension_base:
                                                                                dimension_base
                                                                                    .clone(),
                                                                            ..Default::default()
                                                                        },
                                                                    ),
                                                                );
                                                            }
                                                            "AcDbDiametricDimension" => {
                                                                dimension_entity = Some(
                                                                    EntityType::DiameterDimension(
                                                                        DiameterDimension {
                                                                            dimension_base:
                                                                                dimension_base
                                                                                    .clone(),
                                                                            ..Default::default()
                                                                        },
                                                                    ),
                                                                );
                                                            }
                                                            "AcDb3PointAngularDimension" => {
                                                                dimension_entity = Some(EntityType::AngularThreePointDimension(AngularThreePointDimension { dimension_base: dimension_base.clone(), .. Default::default() }));
                                                            }
                                                            "AcDbOrdinateDimension" => {
                                                                dimension_entity = Some(
                                                                    EntityType::OrdinateDimension(
                                                                        OrdinateDimension {
                                                                            dimension_base:
                                                                                dimension_base
                                                                                    .clone(),
                                                                            ..Default::default()
                                                                        },
                                                                    ),
                                                                );
                                                            }
                                                            _ => {} // unexpected dimension type
                                                        }
                                                    }
                                                    _ => {
                                                        common
                                                            .apply_individual_pair(&pair, iter)?;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(Err(e)) => return Err(e),
                                    None => return Err(DxfError::UnexpectedEndOfInput),
                                }
                            }

                            match dimension_entity {
                                Some(dim) => {
                                    return Ok(Some(Entity {
                                        common,
                                        specific: dim,
                                    }));
                                }
                                None => {
                                    continue 'new_entity;
                                } // unsuccessful dimension match
                            }
                        }
                        _ => {
                            match EntityType::from_type_string(&type_string) {
                                Some(e) => {
                                    let mut entity = Entity::new(e);
                                    if !entity.apply_custom_reader(iter)? {
                                        // no custom reader, use the auto-generated one
                                        loop {
                                            match iter.next() {
                                                Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                                    // new entity or ENDSEC
                                                    iter.put_back(Ok(pair));
                                                    break;
                                                }
                                                Some(Ok(pair)) => {
                                                    entity.apply_code_pair(&pair, iter)?
                                                }
                                                Some(Err(e)) => return Err(e),
                                                None => return Err(DxfError::UnexpectedEndOfInput),
                                            }
                                        }

                                        entity.post_parse()?;
                                    }

                                    return Ok(Some(entity));
                                }
                                None => {
                                    // swallow unsupported entity
                                    loop {
                                        match iter.next() {
                                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                                // found another entity or ENDSEC
                                                iter.put_back(Ok(pair));
                                                break;
                                            }
                                            Some(Ok(_)) => (), // part of the unsupported entity
                                            Some(Err(e)) => return Err(e),
                                            None => return Err(DxfError::UnexpectedEndOfInput),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Ok(pair)) => {
                    return Err(DxfError::UnexpectedCodePair(
                        pair,
                        String::from("expected 0/entity-type or 0/ENDSEC"),
                    ))
                }
                Some(Err(e)) => return Err(e),
                None => return Err(DxfError::UnexpectedEndOfInput),
            }
        }
    }
    fn apply_code_pair(&mut self, pair: &CodePair, iter: &mut CodePairPutBack) -> DxfResult<()> {
        if !self.specific.try_apply_code_pair(pair)? {
            self.common.apply_individual_pair(pair, iter)?;
        }
        Ok(())
    }
    fn post_parse(&mut self) -> DxfResult<()> {
        match self.specific {
            EntityType::Image(ref mut image) => {
                combine_points_2(
                    &mut image.__clipping_vertices_x,
                    &mut image.__clipping_vertices_y,
                    &mut image.clipping_vertices,
                    Point::new,
                );
            }
            EntityType::Leader(ref mut leader) => {
                combine_points_3(
                    &mut leader.__vertices_x,
                    &mut leader.__vertices_y,
                    &mut leader.__vertices_z,
                    &mut leader.vertices,
                    Point::new,
                );
            }
            EntityType::MLine(ref mut mline) => {
                combine_points_3(
                    &mut mline.__vertices_x,
                    &mut mline.__vertices_y,
                    &mut mline.__vertices_z,
                    &mut mline.vertices,
                    Point::new,
                );
                combine_points_3(
                    &mut mline.__segment_direction_x,
                    &mut mline.__segment_direction_y,
                    &mut mline.__segment_direction_z,
                    &mut mline.segment_directions,
                    Vector::new,
                );
                combine_points_3(
                    &mut mline.__miter_direction_x,
                    &mut mline.__miter_direction_y,
                    &mut mline.__miter_direction_z,
                    &mut mline.miter_directions,
                    Vector::new,
                );
            }
            EntityType::Section(ref mut section) => {
                combine_points_3(
                    &mut section.__vertices_x,
                    &mut section.__vertices_y,
                    &mut section.__vertices_z,
                    &mut section.vertices,
                    Point::new,
                );
                combine_points_3(
                    &mut section.__back_line_vertices_x,
                    &mut section.__back_line_vertices_y,
                    &mut section.__back_line_vertices_z,
                    &mut section.back_line_vertices,
                    Point::new,
                );
            }
            EntityType::Spline(ref mut spline) => {
                combine_points_3(
                    &mut spline.__control_point_x,
                    &mut spline.__control_point_y,
                    &mut spline.__control_point_z,
                    &mut spline.control_points,
                    Point::new,
                );
                combine_points_3(
                    &mut spline.__fit_point_x,
                    &mut spline.__fit_point_y,
                    &mut spline.__fit_point_z,
                    &mut spline.fit_points,
                    Point::new,
                );
            }
            EntityType::DgnUnderlay(ref mut underlay) => {
                combine_points_2(
                    &mut underlay.__point_x,
                    &mut underlay.__point_y,
                    &mut underlay.points,
                    Point::new,
                );
            }
            EntityType::DwfUnderlay(ref mut underlay) => {
                combine_points_2(
                    &mut underlay.__point_x,
                    &mut underlay.__point_y,
                    &mut underlay.points,
                    Point::new,
                );
            }
            EntityType::PdfUnderlay(ref mut underlay) => {
                combine_points_2(
                    &mut underlay.__point_x,
                    &mut underlay.__point_y,
                    &mut underlay.points,
                    Point::new,
                );
            }
            EntityType::Wipeout(ref mut wo) => {
                combine_points_2(
                    &mut wo.__clipping_vertices_x,
                    &mut wo.__clipping_vertices_y,
                    &mut wo.clipping_vertices,
                    Point::new,
                );
            }
            _ => (),
        }

        Ok(())
    }
    fn apply_custom_reader(&mut self, iter: &mut CodePairPutBack) -> DxfResult<bool> {
        match self.specific {
            EntityType::Attribute(ref mut att) => {
                Entity::apply_custom_reader_attribute(&mut self.common, att, iter)
            }
            EntityType::AttributeDefinition(ref mut att) => {
                Entity::apply_custom_reader_attributedefinition(&mut self.common, att, iter)
            }
            EntityType::LwPolyline(ref mut poly) => {
                Entity::apply_custom_reader_lwpolyline(&mut self.common, poly, iter)
            }
            EntityType::MText(ref mut mtext) => {
                Entity::apply_custom_reader_mtext(&mut self.common, mtext, iter)
            }
            EntityType::MLeader(ref mut mleader) => {
                Entity::apply_custom_reader_mleader(&mut self.common, mleader, iter)
            }
            _ => Ok(false), // no custom reader
        }
    }
    fn apply_custom_reader_attribute(
        common: &mut EntityCommon,
        att: &mut Attribute,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        let xrecord_text = "AcDbXrecord";
        let mut last_subclass_marker = String::new();
        let mut is_version_set = false;
        let mut xrec_code_70_count = 0;
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                100 => {
                    last_subclass_marker = pair.assert_string()?;
                }
                1 => {
                    att.value = pair.assert_string()?;
                }
                2 => {
                    if last_subclass_marker == xrecord_text {
                        att.x_record_tag = pair.assert_string()?;
                    } else {
                        att.attribute_tag = pair.assert_string()?;
                    }
                }
                7 => {
                    att.text_style_name = pair.assert_string()?;
                }
                10 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.x = pair.assert_f64()?;
                    } else {
                        att.location.x = pair.assert_f64()?;
                    }
                }
                20 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.y = pair.assert_f64()?;
                    } else {
                        att.location.y = pair.assert_f64()?;
                    }
                }
                30 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.z = pair.assert_f64()?;
                    } else {
                        att.location.z = pair.assert_f64()?;
                    }
                }
                11 => {
                    att.second_alignment_point.x = pair.assert_f64()?;
                }
                21 => {
                    att.second_alignment_point.y = pair.assert_f64()?;
                }
                31 => {
                    att.second_alignment_point.z = pair.assert_f64()?;
                }
                39 => {
                    att.thickness = pair.assert_f64()?;
                }
                40 => {
                    if last_subclass_marker == xrecord_text {
                        att.annotation_scale = pair.assert_f64()?;
                    } else {
                        att.text_height = pair.assert_f64()?;
                    }
                }
                41 => {
                    att.relative_x_scale_factor = pair.assert_f64()?;
                }
                50 => {
                    att.rotation = pair.assert_f64()?;
                }
                51 => {
                    att.oblique_angle = pair.assert_f64()?;
                }
                70 => {
                    if last_subclass_marker == xrecord_text {
                        match xrec_code_70_count {
                            0 => {
                                att.m_text_flag = enum_from_number!(
                                    MTextFlag,
                                    MultilineAttribute,
                                    from_i16,
                                    pair.assert_i16()?
                                )
                            }
                            1 => att.is_really_locked = as_bool(pair.assert_i16()?),
                            2 => att.__secondary_attribute_count = i32::from(pair.assert_i16()?),
                            _ => return Err(DxfError::UnexpectedCodePair(pair, String::new())),
                        }
                        xrec_code_70_count += 1;
                    } else {
                        att.flags = i32::from(pair.assert_i16()?);
                    }
                }
                71 => {
                    att.text_generation_flags = i32::from(pair.assert_i16()?);
                }
                72 => {
                    att.horizontal_text_justification = enum_from_number!(
                        HorizontalTextJustification,
                        Left,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                73 => {
                    att.field_length = pair.assert_i16()?;
                }
                74 => {
                    att.vertical_text_justification = enum_from_number!(
                        VerticalTextJustification,
                        Baseline,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                210 => {
                    att.normal.x = pair.assert_f64()?;
                }
                220 => {
                    att.normal.y = pair.assert_f64()?;
                }
                230 => {
                    att.normal.z = pair.assert_f64()?;
                }
                280 => {
                    if last_subclass_marker == xrecord_text {
                        att.keep_duplicate_records = as_bool(pair.assert_i16()?);
                    } else if !is_version_set {
                        att.version =
                            enum_from_number!(Version, R2010, from_i16, pair.assert_i16()?);
                        is_version_set = true;
                    } else {
                        att.is_locked_in_block = as_bool(pair.assert_i16()?);
                    }
                }
                340 => {
                    att.__secondary_attributes_handle.push(pair.as_handle()?);
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    fn apply_custom_reader_attributedefinition(
        common: &mut EntityCommon,
        att: &mut AttributeDefinition,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        let xrecord_text = "AcDbXrecord";
        let mut last_subclass_marker = String::new();
        let mut is_version_set = false;
        let mut xrec_code_70_count = 0;
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                100 => {
                    last_subclass_marker = pair.assert_string()?;
                }
                1 => {
                    att.value = pair.assert_string()?;
                }
                2 => {
                    if last_subclass_marker == xrecord_text {
                        att.x_record_tag = pair.assert_string()?;
                    } else {
                        att.text_tag = pair.assert_string()?;
                    }
                }
                3 => {
                    att.prompt = pair.assert_string()?;
                }
                7 => {
                    att.text_style_name = pair.assert_string()?;
                }
                10 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.x = pair.assert_f64()?;
                    } else {
                        att.location.x = pair.assert_f64()?;
                    }
                }
                20 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.y = pair.assert_f64()?;
                    } else {
                        att.location.y = pair.assert_f64()?;
                    }
                }
                30 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.z = pair.assert_f64()?;
                    } else {
                        att.location.z = pair.assert_f64()?;
                    }
                }
                11 => {
                    att.second_alignment_point.x = pair.assert_f64()?;
                }
                21 => {
                    att.second_alignment_point.y = pair.assert_f64()?;
                }
                31 => {
                    att.second_alignment_point.z = pair.assert_f64()?;
                }
                39 => {
                    att.thickness = pair.assert_f64()?;
                }
                40 => {
                    if last_subclass_marker == xrecord_text {
                        att.annotation_scale = pair.assert_f64()?;
                    } else {
                        att.text_height = pair.assert_f64()?;
                    }
                }
                41 => {
                    att.relative_x_scale_factor = pair.assert_f64()?;
                }
                50 => {
                    att.rotation = pair.assert_f64()?;
                }
                51 => {
                    att.oblique_angle = pair.assert_f64()?;
                }
                70 => {
                    if last_subclass_marker == xrecord_text {
                        match xrec_code_70_count {
                            0 => {
                                att.m_text_flag = enum_from_number!(
                                    MTextFlag,
                                    MultilineAttribute,
                                    from_i16,
                                    pair.assert_i16()?
                                )
                            }
                            1 => att.is_really_locked = as_bool(pair.assert_i16()?),
                            2 => att.__secondary_attribute_count = i32::from(pair.assert_i16()?),
                            _ => return Err(DxfError::UnexpectedCodePair(pair, String::new())),
                        }
                        xrec_code_70_count += 1;
                    } else {
                        att.flags = i32::from(pair.assert_i16()?);
                    }
                }
                71 => {
                    att.text_generation_flags = i32::from(pair.assert_i16()?);
                }
                72 => {
                    att.horizontal_text_justification = enum_from_number!(
                        HorizontalTextJustification,
                        Left,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                73 => {
                    att.field_length = pair.assert_i16()?;
                }
                74 => {
                    att.vertical_text_justification = enum_from_number!(
                        VerticalTextJustification,
                        Baseline,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                210 => {
                    att.normal.x = pair.assert_f64()?;
                }
                220 => {
                    att.normal.y = pair.assert_f64()?;
                }
                230 => {
                    att.normal.z = pair.assert_f64()?;
                }
                280 => {
                    if last_subclass_marker == xrecord_text {
                        att.keep_duplicate_records = as_bool(pair.assert_i16()?);
                    } else if !is_version_set {
                        att.version =
                            enum_from_number!(Version, R2010, from_i16, pair.assert_i16()?);
                        is_version_set = true;
                    } else {
                        att.is_locked_in_block = as_bool(pair.assert_i16()?);
                    }
                }
                340 => {
                    att.__secondary_attributes_handle.push(pair.as_handle()?);
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    fn apply_custom_reader_lwpolyline(
        common: &mut EntityCommon,
        poly: &mut LwPolyline,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                // vertex-specific pairs
                10 => {
                    // start a new vertex
                    poly.vertices.push(LwPolylineVertex::default());
                    vec_last!(poly.vertices).x = pair.assert_f64()?;
                }
                20 => {
                    vec_last!(poly.vertices).y = pair.assert_f64()?;
                }
                40 => {
                    vec_last!(poly.vertices).starting_width = pair.assert_f64()?;
                }
                41 => {
                    vec_last!(poly.vertices).ending_width = pair.assert_f64()?;
                }
                42 => {
                    vec_last!(poly.vertices).bulge = pair.assert_f64()?;
                }
                91 => {
                    vec_last!(poly.vertices).id = pair.assert_i32()?;
                }
                // other pairs
                39 => {
                    poly.thickness = pair.assert_f64()?;
                }
                43 => {
                    poly.constant_width = pair.assert_f64()?;
                }
                70 => {
                    poly.flags = i32::from(pair.assert_i16()?);
                }
                210 => {
                    poly.extrusion_direction.x = pair.assert_f64()?;
                }
                220 => {
                    poly.extrusion_direction.y = pair.assert_f64()?;
                }
                230 => {
                    poly.extrusion_direction.z = pair.assert_f64()?;
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    fn apply_custom_reader_mtext(
        common: &mut EntityCommon,
        mtext: &mut MText,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        let mut reading_column_data = false;
        let mut read_column_count = false;
        let mut reading_embedded_object = false;
        loop {
            let pair = next_pair!(iter);
            // The embedded object logic has been deducted from https://github.com/mozman/ezdxf/blob/master/src/ezdxf/entities/mtext.py. It does not seem documented great in DXF spec.
            if pair.code == 101 {
                reading_embedded_object = true;
                continue;
            }

            if reading_embedded_object {
                match pair.code {
                    10 => {
                        if mtext.x_axis_direction.x == 0.0 {
                            mtext.x_axis_direction.x = pair.assert_f64()?;
                        }
                    }
                    20 => {
                        if mtext.x_axis_direction.y == 0.0 {
                            mtext.x_axis_direction.y = pair.assert_f64()?;
                        }
                    }
                    30 => {
                        if mtext.x_axis_direction.z == 0.0 {
                            mtext.x_axis_direction.z = pair.assert_f64()?;
                        }
                    }
                    11 => {
                        if mtext.insertion_point.x == 0.0 {
                            mtext.insertion_point.x = pair.assert_f64()?;
                        }
                    }
                    21 => {
                        if mtext.insertion_point.y == 0.0 {
                            mtext.insertion_point.y = pair.assert_f64()?;
                        }
                    }
                    31 => {
                        if mtext.insertion_point.z == 0.0 {
                            mtext.insertion_point.z = pair.assert_f64()?;
                        }
                    }
                    40 => {
                        if mtext.reference_rectangle_width == 0.0 {
                            mtext.reference_rectangle_width = pair.assert_f64()?;
                        }
                    }
                    41 => {
                        if mtext.initial_text_height == 0.0 {
                            mtext.initial_text_height = pair.assert_f64()?;
                        }
                    }
                    42 => {
                        mtext.horizontal_width = pair.assert_f64()?;
                    }
                    43 => {
                        mtext.vertical_height = pair.assert_f64()?;
                    }
                    44 => {
                        mtext.reference_rectangle_width = pair.assert_f64()?;
                    }
                    45 => {
                        mtext.column_gutter = pair.assert_f64()?;
                    }
                    46 => {
                        // Not implemented
                    }
                    71 => {
                        mtext.column_type = pair.assert_i16()?;
                    }
                    72 => {
                        mtext.column_count = pair.assert_i16()? as i32;
                    }
                    73 => {
                        mtext.is_column_auto_height = as_bool(pair.assert_i16()?);
                    }
                    74 => {
                        mtext.is_column_flow_reversed = as_bool(pair.assert_i16()?);
                    }
                    _ => {
                        common.apply_individual_pair(&pair, iter)?;
                    }
                }
            } else {
                match pair.code {
                    10 => {
                        mtext.insertion_point.x = pair.assert_f64()?;
                    }
                    20 => {
                        mtext.insertion_point.y = pair.assert_f64()?;
                    }
                    30 => {
                        mtext.insertion_point.z = pair.assert_f64()?;
                    }
                    40 => {
                        mtext.initial_text_height = pair.assert_f64()?;
                    }
                    41 => {
                        mtext.reference_rectangle_width = pair.assert_f64()?;
                    }
                    71 => {
                        if !reading_column_data {
                            mtext.attachment_point = enum_from_number!(
                                AttachmentPoint,
                                TopLeft,
                                from_i16,
                                pair.assert_i16()?
                            );
                        }
                    }
                    72 => {
                        if !reading_column_data {
                            mtext.drawing_direction = enum_from_number!(
                                DrawingDirection,
                                LeftToRight,
                                from_i16,
                                pair.assert_i16()?
                            );
                        }
                    }
                    3 => {
                        mtext.extended_text.push(pair.assert_string()?);
                    }
                    1 => {
                        mtext.text = pair.assert_string()?;
                    }
                    7 => {
                        mtext.text_style_name = pair.assert_string()?;
                    }
                    210 => {
                        mtext.extrusion_direction.x = pair.assert_f64()?;
                    }
                    220 => {
                        mtext.extrusion_direction.y = pair.assert_f64()?;
                    }
                    230 => {
                        mtext.extrusion_direction.z = pair.assert_f64()?;
                    }
                    11 => {
                        mtext.x_axis_direction.x = pair.assert_f64()?;
                    }
                    21 => {
                        mtext.x_axis_direction.y = pair.assert_f64()?;
                    }
                    31 => {
                        mtext.x_axis_direction.z = pair.assert_f64()?;
                    }
                    42 => {
                        mtext.horizontal_width = pair.assert_f64()?;
                    }
                    43 => {
                        mtext.vertical_height = pair.assert_f64()?;
                    }
                    50 => {
                        if reading_column_data {
                            if read_column_count {
                                mtext.column_heights.push(pair.assert_f64()?);
                            } else {
                                mtext.column_count = pair.assert_f64()? as i32;
                                read_column_count = true;
                            }
                        } else {
                            mtext.rotation_angle = pair.assert_f64()?;
                        }
                    }
                    73 => {
                        mtext.line_spacing_style = enum_from_number!(
                            MTextLineSpacingStyle,
                            AtLeast,
                            from_i16,
                            pair.assert_i16()?
                        );
                    }
                    44 => {
                        if !reading_column_data {
                            mtext.line_spacing_factor = pair.assert_f64()?;
                        }
                    }
                    90 => {
                        mtext.background_fill_setting = enum_from_number!(
                            BackgroundFillSetting,
                            Off,
                            from_i32,
                            pair.assert_i32()?
                        );
                    }
                    420 => {
                        mtext.background_color_rgb = pair.assert_i32()?;
                    }
                    430 => {
                        mtext.background_color_name = pair.assert_string()?;
                    }
                    45 => {
                        mtext.fill_box_scale = pair.assert_f64()?;
                    }
                    63 => {
                        mtext.background_fill_color = Color::from_raw_value(pair.assert_i16()?);
                    }
                    441 => {
                        mtext.background_fill_color_transparency = pair.assert_i32()?;
                    }
                    75 => {
                        mtext.column_type = pair.assert_i16()?;
                        reading_column_data = true;
                    }
                    76 => {
                        mtext.column_count = i32::from(pair.assert_i16()?);
                    }
                    78 => {
                        mtext.is_column_flow_reversed = as_bool(pair.assert_i16()?);
                    }
                    79 => {
                        mtext.is_column_auto_height = as_bool(pair.assert_i16()?);
                    }
                    48 => {
                        mtext.column_width = pair.assert_f64()?;
                    }
                    49 => {
                        mtext.column_gutter = pair.assert_f64()?;
                    }
                    _ => {
                        common.apply_individual_pair(&pair, iter)?;
                    }
                }
            }
        }
    }
    fn apply_custom_reader_mleader(
        common: &mut EntityCommon,
        mleader: &mut MLeader,
        iter: &mut CodePairPutBack,
    ) -> DxfResult<bool> {
        #[derive(PartialEq)]
        enum ReadingState {
            TopLevel,
            ContextData,
            Leader,
            LeaderLine,
        }

        let mut reading_state = ReadingState::TopLevel;

        const CONTEXT_DATA_START: i32 = 300;
        const CONTEXT_DATA_END: i32 = 301;
        const LEADER_START: i32 = 302;
        const LEADER_END: i32 = 303;
        const LEADER_LINE_START: i32 = 304;
        const LEADER_LINE_END: i32 = 305;

        loop {
            let pair = next_pair!(iter);

            // Check for state changes first
            // The state changes are based on embedded states where context_data includes leader and leader includes leader_line data.
            match pair.code {
                CONTEXT_DATA_START => {
                    reading_state = ReadingState::ContextData;
                    continue;
                }
                CONTEXT_DATA_END if reading_state == ReadingState::ContextData => {
                    reading_state = ReadingState::TopLevel;
                    continue;
                }
                LEADER_START if reading_state == ReadingState::ContextData => {
                    reading_state = ReadingState::Leader;
                    continue;
                }
                LEADER_END if reading_state == ReadingState::Leader => {
                    reading_state = ReadingState::ContextData;
                    continue;
                }
                LEADER_LINE_START if reading_state == ReadingState::Leader => {
                    reading_state = ReadingState::LeaderLine;
                    continue;
                }
                LEADER_LINE_END if reading_state == ReadingState::LeaderLine => {
                    reading_state = ReadingState::Leader;
                    continue;
                }
                _ => {}
            }

            // Process data based on current state
            match reading_state {
                ReadingState::TopLevel => match pair.code {
                    290 => {
                        mleader.enable_landing = as_bool(pair.assert_i16()?);
                    }
                    291 => {
                        mleader.enable_dogleg = as_bool(pair.assert_i16()?);
                    }
                    172 => {
                        mleader.content_type = enum_from_number!(
                            MLeaderContentType,
                            None,
                            from_i16,
                            pair.assert_i16()?
                        )
                    }
                    _ => {
                        common.apply_individual_pair(&pair, iter)?;
                    }
                },
                ReadingState::ContextData => match pair.code {
                    304 => {
                        mleader.default_text_contents = pair.assert_string()?;
                    }
                    10 => {
                        // Loop through the next two to get code 20 and 30 break and error out if those are not found
                        let x_coord = pair.assert_f64()?;
                        // Next iter
                        let pair = next_pair!(iter);
                        if pair.code != 20 {
                            return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                        }
                        let y_coord = pair.assert_f64()?;
                        // Next iter
                        let pair = next_pair!(iter);
                        if pair.code != 30 {
                            return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                        }
                        let z_coord = pair.assert_f64()?;
                        mleader.content_base_point = Point {
                            x: x_coord,
                            y: y_coord,
                            z: z_coord,
                        };
                    }
                    11 => {
                        // Text normal direction (11,21,31)
                        let x_coord = pair.assert_f64()?;
                        let pair = next_pair!(iter);
                        if pair.code != 21 {
                            return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                        }
                        let y_coord = pair.assert_f64()?;
                        let pair = next_pair!(iter);
                        if pair.code != 31 {
                            return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                        }
                        let z_coord = pair.assert_f64()?;
                        mleader.text_normal_direction = Vector {
                            x: x_coord,
                            y: y_coord,
                            z: z_coord,
                        };
                    }
                    12 => {
                        // Text location (12,22,32)
                        let x_coord = pair.assert_f64()?;
                        let pair = next_pair!(iter);
                        if pair.code != 22 {
                            return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                        }
                        let y_coord = pair.assert_f64()?;
                        let pair = next_pair!(iter);
                        if pair.code != 32 {
                            return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                        }
                        let z_coord = pair.assert_f64()?;
                        mleader.text_location = Point {
                            x: x_coord,
                            y: y_coord,
                            z: z_coord,
                        };
                    }
                    41 => {
                        mleader.text_height = pair.assert_f64()?;
                    }
                    42 => {
                        mleader.text_rotation = pair.assert_f64()?;
                    }
                    43 => {
                        mleader.text_width = pair.assert_f64()?;
                    }
                    171 => {
                        mleader.text_attachment = pair.assert_i16()?;
                    }
                    _ => common.apply_individual_pair(&pair, iter)?,
                },
                ReadingState::Leader => {
                    match pair.code {
                        10 => {
                            // Loop through the next two to get code 20 and 30 break and error out if those are not found
                            let x_coord = pair.assert_f64()?;
                            // Next iter
                            let pair = next_pair!(iter);
                            if pair.code != 20 {
                                return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                            }
                            let y_coord = pair.assert_f64()?;
                            // Next iter
                            let pair = next_pair!(iter);
                            if pair.code != 30 {
                                return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                            }
                            let z_coord = pair.assert_f64()?;
                            mleader.vertices.push(Point {
                                x: x_coord,
                                y: y_coord,
                                z: z_coord,
                            });
                        }
                        40 => {
                            mleader.dogleg_length = pair.assert_f64()?;
                        }
                        11 => {
                            mleader.dogleg_vector.x = pair.assert_f64()?;
                        }
                        21 => {
                            mleader.dogleg_vector.y = pair.assert_f64()?;
                        }
                        31 => {
                            mleader.dogleg_vector.z = pair.assert_f64()?;
                        }
                        _ => {
                            common.apply_individual_pair(&pair, iter)?;
                        }
                    }
                }
                ReadingState::LeaderLine => {
                    match pair.code {
                        10 => {
                            // Loop through the next two to get code 20 and 30 break and error out if those are not found
                            let x_coord = pair.assert_f64()?;
                            // Next iter
                            let pair = next_pair!(iter);
                            if pair.code != 20 {
                                return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                            }
                            let y_coord = pair.assert_f64()?;
                            // Next iter
                            let pair = next_pair!(iter);
                            if pair.code != 30 {
                                return Err(DxfError::UnexpectedCodePair(pair, String::new()));
                            }
                            let z_coord = pair.assert_f64()?;
                            mleader.vertices.push(Point {
                                x: x_coord,
                                y: y_coord,
                                z: z_coord,
                            });
                        }
                        _ => {
                            common.apply_individual_pair(&pair, iter)?;
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn add_code_pairs(
        &self,
        pairs: &mut Vec<CodePair>,
        version: AcadVersion,
        write_handles: bool,
    ) {
        if self.specific.is_supported_on_version(version) {
            pairs.push(CodePair::new_str(0, self.specific.to_type_string()));
            self.common.add_code_pairs(pairs, version, write_handles);
            if !self.add_custom_code_pairs(pairs, version) {
                self.specific.add_code_pairs(pairs, &self.common, version);
            }

            self.add_post_code_pairs(pairs, version, write_handles);
            for x in &self.common.x_data {
                x.add_code_pairs(pairs, version);
            }
        }
    }
    fn add_custom_code_pairs(&self, pairs: &mut Vec<CodePair>, version: AcadVersion) -> bool {
        match self.specific {
            EntityType::RotatedDimension(ref dim) => {
                Entity::add_custom_code_pairs_rotateddimension(pairs, dim, version);
            }
            EntityType::RadialDimension(ref dim) => {
                Entity::add_custom_code_pairs_radialdimension(pairs, dim, version);
            }
            EntityType::DiameterDimension(ref dim) => {
                Entity::add_custom_code_pairs_diameterdimension(pairs, dim, version);
            }
            EntityType::AngularThreePointDimension(ref dim) => {
                Entity::add_custom_code_pairs_angularthreepointdimension(pairs, dim, version);
            }
            EntityType::OrdinateDimension(ref dim) => {
                Entity::add_custom_code_pairs_ordinatedimension(pairs, dim, version);
            }
            EntityType::Polyline(ref poly) => {
                Entity::add_custom_code_pairs_polyline(pairs, poly, version);
            }
            EntityType::Vertex(ref v) => {
                Entity::add_custom_code_pairs_vertex(pairs, v, version);
            }
            EntityType::MLeader(ref leader) => {
                Entity::add_custom_code_pairs_mleader(pairs, leader, version);
            }
            _ => return false, // no custom code pairs
        }

        true
    }
    fn add_custom_code_pairs_rotateddimension(
        pairs: &mut Vec<CodePair>,
        dim: &RotatedDimension,
        version: AcadVersion,
    ) -> bool {
        dim.dimension_base.add_code_pairs(pairs, version);
        if version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(100, "AcDbAlignedDimension"));
        }
        pairs.push(CodePair::new_f64(12, dim.insertion_point.x));
        pairs.push(CodePair::new_f64(22, dim.insertion_point.y));
        pairs.push(CodePair::new_f64(32, dim.insertion_point.z));
        pairs.push(CodePair::new_f64(13, dim.definition_point_2.x));
        pairs.push(CodePair::new_f64(23, dim.definition_point_2.y));
        pairs.push(CodePair::new_f64(33, dim.definition_point_2.z));
        pairs.push(CodePair::new_f64(14, dim.definition_point_3.x));
        pairs.push(CodePair::new_f64(24, dim.definition_point_3.y));
        pairs.push(CodePair::new_f64(34, dim.definition_point_3.z));
        pairs.push(CodePair::new_f64(50, dim.rotation_angle));
        pairs.push(CodePair::new_f64(52, dim.extension_line_angle));
        if version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(100, "AcDbRotatedDimension"));
        }
        true
    }
    fn add_custom_code_pairs_radialdimension(
        pairs: &mut Vec<CodePair>,
        dim: &RadialDimension,
        version: AcadVersion,
    ) -> bool {
        dim.dimension_base.add_code_pairs(pairs, version);
        pairs.push(CodePair::new_str(100, "AcDbRadialDimension"));
        pairs.push(CodePair::new_f64(15, dim.definition_point_2.x));
        pairs.push(CodePair::new_f64(25, dim.definition_point_2.y));
        pairs.push(CodePair::new_f64(35, dim.definition_point_2.z));
        pairs.push(CodePair::new_f64(40, dim.leader_length));
        true
    }
    fn add_custom_code_pairs_diameterdimension(
        pairs: &mut Vec<CodePair>,
        dim: &DiameterDimension,
        version: AcadVersion,
    ) -> bool {
        dim.dimension_base.add_code_pairs(pairs, version);
        pairs.push(CodePair::new_str(100, "AcDbDiametricDimension"));
        pairs.push(CodePair::new_f64(15, dim.definition_point_2.x));
        pairs.push(CodePair::new_f64(25, dim.definition_point_2.y));
        pairs.push(CodePair::new_f64(35, dim.definition_point_2.z));
        pairs.push(CodePair::new_f64(40, dim.leader_length));
        true
    }
    fn add_custom_code_pairs_angularthreepointdimension(
        pairs: &mut Vec<CodePair>,
        dim: &AngularThreePointDimension,
        version: AcadVersion,
    ) -> bool {
        dim.dimension_base.add_code_pairs(pairs, version);
        pairs.push(CodePair::new_str(100, "AcDb3PointAngularDimension"));
        pairs.push(CodePair::new_f64(13, dim.definition_point_2.x));
        pairs.push(CodePair::new_f64(23, dim.definition_point_2.y));
        pairs.push(CodePair::new_f64(33, dim.definition_point_2.z));
        pairs.push(CodePair::new_f64(14, dim.definition_point_3.x));
        pairs.push(CodePair::new_f64(24, dim.definition_point_3.y));
        pairs.push(CodePair::new_f64(34, dim.definition_point_3.z));
        pairs.push(CodePair::new_f64(15, dim.definition_point_4.x));
        pairs.push(CodePair::new_f64(25, dim.definition_point_4.y));
        pairs.push(CodePair::new_f64(35, dim.definition_point_4.z));
        pairs.push(CodePair::new_f64(16, dim.definition_point_5.x));
        pairs.push(CodePair::new_f64(26, dim.definition_point_5.y));
        pairs.push(CodePair::new_f64(36, dim.definition_point_5.z));
        true
    }
    fn add_custom_code_pairs_ordinatedimension(
        pairs: &mut Vec<CodePair>,
        dim: &OrdinateDimension,
        version: AcadVersion,
    ) -> bool {
        dim.dimension_base.add_code_pairs(pairs, version);
        pairs.push(CodePair::new_str(100, "AcDbOrdinateDimension"));
        pairs.push(CodePair::new_f64(13, dim.definition_point_2.x));
        pairs.push(CodePair::new_f64(23, dim.definition_point_2.y));
        pairs.push(CodePair::new_f64(33, dim.definition_point_2.z));
        pairs.push(CodePair::new_f64(14, dim.definition_point_3.x));
        pairs.push(CodePair::new_f64(24, dim.definition_point_3.y));
        pairs.push(CodePair::new_f64(34, dim.definition_point_3.z));
        true
    }
    fn add_custom_code_pairs_polyline(
        pairs: &mut Vec<CodePair>,
        poly: &Polyline,
        version: AcadVersion,
    ) -> bool {
        let subclass_marker = if poly.is_3d_polyline() || poly.is_3d_polygon_mesh() {
            "AcDb3dPolyline"
        } else {
            "AcDb2dPolyline"
        };
        if version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(100, subclass_marker));
        }
        if version <= AcadVersion::R13 {
            pairs.push(CodePair::new_i16(66, as_i16(poly.contains_vertices)));
        }
        if version >= AcadVersion::R12 {
            pairs.push(CodePair::new_f64(10, poly.location.x));
            pairs.push(CodePair::new_f64(20, poly.location.y));
            pairs.push(CodePair::new_f64(30, poly.location.z));
        }
        if poly.thickness != 0.0 {
            pairs.push(CodePair::new_f64(39, poly.thickness));
        }
        if poly.flags != 0 {
            pairs.push(CodePair::new_i16(70, poly.flags as i16));
        }
        if poly.default_starting_width != 0.0 {
            pairs.push(CodePair::new_f64(40, poly.default_starting_width));
        }
        if poly.default_ending_width != 0.0 {
            pairs.push(CodePair::new_f64(41, poly.default_ending_width));
        }
        if poly.polygon_mesh_m_vertex_count != 0 {
            pairs.push(CodePair::new_i16(
                71,
                poly.polygon_mesh_m_vertex_count as i16,
            ));
        }
        if poly.polygon_mesh_n_vertex_count != 0 {
            pairs.push(CodePair::new_i16(
                72,
                poly.polygon_mesh_n_vertex_count as i16,
            ));
        }
        if poly.smooth_surface_m_density != 0 {
            pairs.push(CodePair::new_i16(73, poly.smooth_surface_m_density as i16));
        }
        if poly.smooth_surface_n_density != 0 {
            pairs.push(CodePair::new_i16(74, poly.smooth_surface_n_density as i16));
        }
        if poly.surface_type != PolylineCurvedAndSmoothSurfaceType::None {
            pairs.push(CodePair::new_i16(75, poly.surface_type as i16));
        }
        if poly.normal != Vector::z_axis() {
            pairs.push(CodePair::new_f64(210, poly.normal.x));
            pairs.push(CodePair::new_f64(220, poly.normal.y));
            pairs.push(CodePair::new_f64(230, poly.normal.z));
        }
        true
    }
    fn add_custom_code_pairs_vertex(
        pairs: &mut Vec<CodePair>,
        v: &Vertex,
        version: AcadVersion,
    ) -> bool {
        pairs.push(CodePair::new_str(100, "AcDbVertex"));
        let subclass_marker = if v.is_3d_polyline_vertex() || v.is_3d_polygon_mesh() {
            "AcDb3dPolylineVertex"
        } else {
            "AcDb2dVertex"
        };
        if version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(100, subclass_marker));
        }
        pairs.push(CodePair::new_f64(10, v.location.x));
        pairs.push(CodePair::new_f64(20, v.location.y));
        pairs.push(CodePair::new_f64(30, v.location.z));
        if v.starting_width != 0.0 {
            pairs.push(CodePair::new_f64(40, v.starting_width));
        }
        if v.ending_width != 0.0 {
            pairs.push(CodePair::new_f64(41, v.ending_width));
        }
        if v.bulge != 0.0 {
            pairs.push(CodePair::new_f64(42, v.bulge));
        }
        pairs.push(CodePair::new_i16(70, v.flags as i16));
        pairs.push(CodePair::new_f64(50, v.curve_fit_tangent_direction));
        if version >= AcadVersion::R12 {
            if v.polyface_mesh_vertex_index1 != 0 {
                pairs.push(CodePair::new_i16(71, v.polyface_mesh_vertex_index1 as i16));
            }
            if v.polyface_mesh_vertex_index2 != 0 {
                pairs.push(CodePair::new_i16(72, v.polyface_mesh_vertex_index2 as i16));
            }
            if v.polyface_mesh_vertex_index3 != 0 {
                pairs.push(CodePair::new_i16(73, v.polyface_mesh_vertex_index3 as i16));
            }
            if v.polyface_mesh_vertex_index4 != 0 {
                pairs.push(CodePair::new_i16(74, v.polyface_mesh_vertex_index4 as i16));
            }
        }

        // Even though the identifier is part of the spec https://help.autodesk.com/view/OARX/2025/ENU/?guid=GUID-0741E831-599E-4CBF-91E1-8ADBCFD6556D AutoCAD can't handle 91 group code. So we do not write it for R2018 and later.
        if version >= AcadVersion::R2010 && !v.is_3d_polyline_vertex() {
            pairs.push(CodePair::new_i32(91, v.identifier));
        }
        true
    }
    // fn add_custom_code_pairs_mleader(
    //     pairs: &mut Vec<CodePair>,
    //     mleader: &MLeader,
    //     version: AcadVersion,
    // ) -> bool {
    //     if version >= AcadVersion::R13 {
    //         pairs.push(CodePair::new_str(100, "AcDbMLeader"));
    //     }

    //     // TopLevel - Version number (always 2 for MLEADER)
    //     pairs.push(CodePair::new_i16(270, 2));

    //     // Common MLeader Group Codes (before context data)
    //     if !mleader.leader_style_id.is_empty() {
    //         pairs.push(CodePair::new_str(340, &mleader.leader_style_id));
    //     }
    //     pairs.push(CodePair::new_i32(90, mleader.property_override_flag));
    //     pairs.push(CodePair::new_i16(170, mleader.leader_line_type as i16));
    //     pairs.push(CodePair::new_i32(91, mleader.leader_line_color));
    //     if !mleader.leader_line_type_id.is_empty() {
    //         pairs.push(CodePair::new_str(341, &mleader.leader_line_type_id));
    //     }
    //     pairs.push(CodePair::new_i16(171, mleader.leader_line_weight));
    //     pairs.push(CodePair::new_i16(290, as_i16(mleader.enable_landing)));
    //     pairs.push(CodePair::new_i16(291, as_i16(mleader.enable_dogleg)));
    //     pairs.push(CodePair::new_f64(41, mleader.dogleg_length));
    //     if !mleader.arrowhead_id.is_empty() {
    //         pairs.push(CodePair::new_str(342, &mleader.arrowhead_id));
    //     }
    //     pairs.push(CodePair::new_f64(42, mleader.arrowhead_size));
    //     pairs.push(CodePair::new_i16(172, mleader.content_type as i16));
    //     if !mleader.text_style_id.is_empty() {
    //         pairs.push(CodePair::new_str(343, &mleader.text_style_id));
    //     }
    //     pairs.push(CodePair::new_i16(173, mleader.text_left_attachment_type));
    //     pairs.push(CodePair::new_i16(95, mleader.text_right_attachment_type));
    //     pairs.push(CodePair::new_i16(174, mleader.text_angle_type));
    //     pairs.push(CodePair::new_i16(175, mleader.text_alignment_type));
    //     pairs.push(CodePair::new_i32(92, mleader.text_color));
    //     pairs.push(CodePair::new_i16(292, as_i16(mleader.enable_frame_text)));
    //     // if !mleader.block_content_id.is_empty() {
    //     //     pairs.push(CodePair::new_str(344, &mleader.block_content_id));
    //     // }
    //     // pairs.push(CodePair::new_i32(93, mleader.block_content_color));
    //     // pairs.push(CodePair::new_f64(10, mleader.block_content_scale));
    //     // pairs.push(CodePair::new_f64(43, mleader.block_content_rotation));
    //     // pairs.push(CodePair::new_i16(
    //         // 176,
    //         // mleader.block_content_connection_type,
    //     // ));
    //     pairs.push(CodePair::new_i16(
    //         293,
    //         as_i16(mleader.enable_annotation_scale),
    //     ));
    //     pairs.push(CodePair::new_i32(94, mleader.arrowhead_index));
    //     if !mleader.arrowhead_id_ref.is_empty() {
    //         pairs.push(CodePair::new_str(345, &mleader.arrowhead_id_ref));
    //     }
    //     // if !mleader.block_attribute_id.is_empty() {
    //         // pairs.push(CodePair::new_str(330, &mleader.block_attribute_id));
    //     // }
    //     // pairs.push(CodePair::new_i16(177, mleader.block_attribute_index));
    //     // pairs.push(CodePair::new_f64(44, mleader.block_attribute_width));
    //     // if !mleader.block_attribute_text_string.is_empty() {
    //         // pairs.push(CodePair::new_str(302, &mleader.block_attribute_text_string));
    //     // }
    //     pairs.push(CodePair::new_i16(
    //         294,
    //         as_i16(mleader.text_direction_negative),
    //     ));
    //     pairs.push(CodePair::new_i16(178, mleader.text_align_in_ipe));
    //     pairs.push(CodePair::new_i16(179, mleader.text_attachment_point));
    //     pairs.push(CodePair::new_i16(271, mleader.text_attachment_direction));
    //     pairs.push(CodePair::new_i16(
    //         272,
    //         mleader.bottom_text_attachment_direction,
    //     ));
    //     pairs.push(CodePair::new_i16(
    //         273,
    //         mleader.top_text_attachment_direction,
    //     ));

    //     // Context Data Section
    //     pairs.push(CodePair::new_str(300, "CONTEXT_DATA{"));

    //     // MLeader Context Data Group Codes
    //     pairs.push(CodePair::new_f64(40, mleader.content_scale));
    //     pairs.push(CodePair::new_f64(10, mleader.content_base_point.x));
    //     pairs.push(CodePair::new_f64(20, mleader.content_base_point.y));
    //     pairs.push(CodePair::new_f64(30, mleader.content_base_point.z));
    //     pairs.push(CodePair::new_f64(41, mleader.text_height));
    //     pairs.push(CodePair::new_f64(140, mleader.arrow_head_size));
    //     pairs.push(CodePair::new_f64(145, mleader.landing_gap));
    //     pairs.push(CodePair::new_i16(290, as_i16(mleader.has_m_text)));
    //     pairs.push(CodePair::new_str(304, &mleader.default_text_contents));
    //     pairs.push(CodePair::new_f64(11, mleader.text_normal_direction.x));
    //     pairs.push(CodePair::new_f64(21, mleader.text_normal_direction.y));
    //     pairs.push(CodePair::new_f64(31, mleader.text_normal_direction.z));
    //     if !mleader.text_style_id_context.is_empty() {
    //         pairs.push(CodePair::new_str(340, &mleader.text_style_id_context));
    //     }
    //     pairs.push(CodePair::new_f64(12, mleader.text_location.x));
    //     pairs.push(CodePair::new_f64(22, mleader.text_location.y));
    //     pairs.push(CodePair::new_f64(32, mleader.text_location.z));
    //     pairs.push(CodePair::new_f64(13, mleader.text_direction.x));
    //     pairs.push(CodePair::new_f64(23, mleader.text_direction.y));
    //     pairs.push(CodePair::new_f64(33, mleader.text_direction.z));
    //     pairs.push(CodePair::new_f64(42, mleader.text_rotation));
    //     pairs.push(CodePair::new_f64(43, mleader.text_width));
    //     pairs.push(CodePair::new_f64(44, mleader.text_height_context));
    //     pairs.push(CodePair::new_f64(45, mleader.text_line_spacing_factor));
    //     pairs.push(CodePair::new_i16(170, mleader.text_line_spacing_style));
    //     pairs.push(CodePair::new_i32(90, mleader.text_color_context));
    //     pairs.push(CodePair::new_i16(171, mleader.text_attachment));
    //     pairs.push(CodePair::new_i16(172, mleader.text_flow_direction));
    //     pairs.push(CodePair::new_i32(91, mleader.text_background_color));
    //     pairs.push(CodePair::new_f64(141, mleader.text_background_scale_factor));
    //     pairs.push(CodePair::new_i32(92, mleader.text_background_transparency));
    //     pairs.push(CodePair::new_i16(
    //         291,
    //         as_i16(mleader.is_text_background_color_on),
    //     ));
    //     pairs.push(CodePair::new_i16(
    //         292,
    //         as_i16(mleader.is_text_background_fill_on),
    //     ));
    //     pairs.push(CodePair::new_i16(173, mleader.text_column_type));
    //     pairs.push(CodePair::new_i16(293, as_i16(mleader.use_text_auto_height)));
    //     pairs.push(CodePair::new_f64(142, mleader.text_column_width));
    //     pairs.push(CodePair::new_f64(143, mleader.text_column_gutter_width));
    //     pairs.push(CodePair::new_i16(
    //         294,
    //         as_i16(mleader.text_column_flow_reversed),
    //     ));
    //     pairs.push(CodePair::new_f64(144, mleader.text_column_height));
    //     pairs.push(CodePair::new_i16(295, as_i16(mleader.text_use_word_break)));
    //     pairs.push(CodePair::new_i16(296, as_i16(mleader.has_block)));
    //     // if !mleader.block_content_id_context.is_empty() {
    //         // pairs.push(CodePair::new_str(341, &mleader.block_content_id_context));
    //     // }
    //     // pairs.push(CodePair::new_f64(
    //         // 14,
    //         // mleader.block_content_normal_direction.x,
    //     // ));
    //     // pairs.push(CodePair::new_f64(
    //         // 24,
    //         // mleader.block_content_normal_direction.y,
    //     // ));
    //     // pairs.push(CodePair::new_f64(
    //         // 34,
    //         // mleader.block_content_normal_direction.z,
    //     // ));
    //     // pairs.push(CodePair::new_f64(15, mleader.block_content_position.x));
    //     // pairs.push(CodePair::new_f64(25, mleader.block_content_position.y));
    //     // pairs.push(CodePair::new_f64(35, mleader.block_content_position.z));
    //     // pairs.push(CodePair::new_f64(16, mleader.block_content_scale_context));
    //     // pairs.push(CodePair::new_f64(
    //         // 46,
    //         // mleader.block_content_rotation_context,
    //     // ));
    //     // pairs.push(CodePair::new_i32(93, mleader.block_content_color_context));
    //     // pairs.push(CodePair::new_f64(47, mleader.block_transformation_matrix));

    //     // MLeader plane origin point
    //     // pairs.push(CodePair::new_f64(110, mleader.mleader_plane_origin_point.x));
    //     // pairs.push(CodePair::new_f64(
    //     //     120,
    //     //     mleader.mleader_plane_origin_point.y,
    //     // ));
    //     // pairs.push(CodePair::new_f64(130, mleader.mleader_plane_origin_point.z));

    //     // MLeader plane x and y axis direction
    //     // pairs.push(CodePair::new_f64(
    //     //     111,
    //     //     mleader.mleader_plane_x_axis_direction.x,
    //     // ));
    //     // pairs.push(CodePair::new_f64(
    //     //     112,
    //     //     mleader.mleader_plane_y_axis_direction.x,
    //     // ));
    //     // pairs.push(CodePair::new_i16(
    //     //     297,
    //     //     as_i16(mleader.mleader_plane_normal_reversed),
    //     // ));
    //     pairs.push(CodePair::new_f64(10, mleader.vertex.x));
    //     pairs.push(CodePair::new_f64(20, mleader.vertex.y));
    //     pairs.push(CodePair::new_f64(30, mleader.vertex.z));
    //     pairs.push(CodePair::new_i32(90, mleader.break_point_index));

    //     // Leader Section
    //     pairs.push(CodePair::new_str(302, "LEADER{"));

    //     // MLeader Leader Node Group Codes
    //     pairs.push(CodePair::new_i16(
    //         290,
    //         as_i16(mleader.has_set_last_leader_line_point),
    //     ));
    //     pairs.push(CodePair::new_i16(
    //         291,
    //         as_i16(mleader.has_set_dogleg_vector),
    //     ));
    //     pairs.push(CodePair::new_f64(10, mleader.last_leader_line_point.x));
    //     pairs.push(CodePair::new_f64(20, mleader.last_leader_line_point.y));
    //     pairs.push(CodePair::new_f64(30, mleader.last_leader_line_point.z));
    //     pairs.push(CodePair::new_f64(11, mleader.dogleg_vector.x));
    //     pairs.push(CodePair::new_f64(21, mleader.dogleg_vector.y));
    //     pairs.push(CodePair::new_f64(31, mleader.dogleg_vector.z));
    //     // pairs.push(CodePair::new_f64(12, mleader.break_start_point.x));
    //     // pairs.push(CodePair::new_f64(22, mleader.break_start_point.y));
    //     // pairs.push(CodePair::new_f64(32, mleader.break_start_point.z));
    //     // pairs.push(CodePair::new_f64(13, mleader.break_end_point.x));
    //     // pairs.push(CodePair::new_f64(23, mleader.break_end_point.y));
    //     // pairs.push(CodePair::new_f64(33, mleader.break_end_point.z));
    //     pairs.push(CodePair::new_i32(90, mleader.leader_branch_index));
    //     pairs.push(CodePair::new_f64(40, mleader.dogleg_length_leader));

    //     // Leader Line Section
    //     pairs.push(CodePair::new_str(304, "LEADER_LINE{"));

    //     // MLeader Leader Line Group Codes
    //     for vertex in &mleader.vertices {
    //         pairs.push(CodePair::new_f64(10, vertex.x));
    //         pairs.push(CodePair::new_f64(20, vertex.y));
    //         pairs.push(CodePair::new_f64(30, vertex.z));
    //     }
    //     // pairs.push(CodePair::new_i32(90, mleader.break_point_index_line));
    //     // pairs.push(CodePair::new_f64(11, mleader.break_start_point_line.x));
    //     // pairs.push(CodePair::new_f64(21, mleader.break_start_point_line.y));
    //     // pairs.push(CodePair::new_f64(31, mleader.break_start_point_line.z));
    //     // pairs.push(CodePair::new_f64(12, mleader.break_end_point_line.x));
    //     // pairs.push(CodePair::new_f64(22, mleader.break_end_point_line.y));
    //     // pairs.push(CodePair::new_f64(32, mleader.break_end_point_line.z));
    //     pairs.push(CodePair::new_i32(91, mleader.leader_line_index));

    //     // Close sections
    //     pairs.push(CodePair::new_str(305, "}"));
    //     pairs.push(CodePair::new_str(303, "}"));
    //     pairs.push(CodePair::new_str(301, "}"));

    //     true
    // }
    fn add_post_code_pairs(
        &self,
        pairs: &mut Vec<CodePair>,
        version: AcadVersion,
        write_handles: bool,
    ) {
        match self.specific {
            EntityType::Attribute(ref att) => self.add_code_pairs_attribute_m_text(
                pairs,
                att.m_text.clone(),
                version,
                write_handles,
            ),
            EntityType::AttributeDefinition(ref att) => self.add_code_pairs_attribute_m_text(
                pairs,
                att.m_text.clone(),
                version,
                write_handles,
            ),
            EntityType::Insert(ref ins) => {
                for (a, att_handle) in &ins.__attributes_and_handles {
                    let a = Entity {
                        common: EntityCommon {
                            handle: *att_handle,
                            ..Default::default()
                        },
                        specific: EntityType::Attribute(a.clone()),
                    };
                    a.add_code_pairs(pairs, version, write_handles);
                }
                if !ins.__attributes_and_handles.is_empty() {
                    Entity::add_code_pairs_seqend(pairs, &ins.__seqend_handle, write_handles);
                }
            }
            EntityType::Polyline(ref poly) => {
                for (v, vertex_handle) in &poly.__vertices_and_handles {
                    let mut v = v.clone();
                    v.set_is_3d_polyline_vertex(poly.is_3d_polyline());
                    if v.polyface_mesh_vertex_index1 == 0
                        && v.polyface_mesh_vertex_index2 == 0
                        && v.polyface_mesh_vertex_index3 == 0
                        && v.polyface_mesh_vertex_index4 == 0
                    {
                        v.set_is_3d_polygon_mesh(poly.is_3d_polygon_mesh());
                    }
                    let v = Entity {
                        common: EntityCommon {
                            handle: *vertex_handle,
                            ..Default::default()
                        },
                        specific: EntityType::Vertex(v),
                    };
                    v.add_code_pairs(pairs, version, write_handles);
                }
                Entity::add_code_pairs_seqend(pairs, &poly.__seqend_handle, write_handles);
            }
            _ => (),
        }
    }
    fn add_code_pairs_attribute_m_text(
        &self,
        pairs: &mut Vec<CodePair>,
        m_text: MText,
        version: AcadVersion,
        write_handles: bool,
    ) {
        let m_text_common = EntityCommon {
            handle: Handle::empty(), // TODO: set handle
            __owner_handle: self.common.handle,
            is_in_paper_space: self.common.is_in_paper_space,
            layer: self.common.layer.clone(),
            ..Default::default()
        };
        let m_text = Entity {
            common: m_text_common,
            specific: EntityType::MText(m_text),
        };
        m_text.add_code_pairs(pairs, version, write_handles);
    }
    fn add_code_pairs_seqend(pairs: &mut Vec<CodePair>, handle: &Handle, write_handles: bool) {
        pairs.push(CodePair::new_str(0, "SEQEND"));
        if write_handles {
            pairs.push(CodePair::new_string(5, &handle.as_string()));
            pairs.push(CodePair::new_string(100, "AcDbEntity"));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::*;
    use crate::enums::*;
    use crate::helper_functions::tests::*;
    use crate::objects::*;
    use crate::*;
    use float_cmp::approx_eq;

    fn read_entity(entity_type: &str, body: Vec<CodePair>) -> Entity {
        let mut pairs = vec![CodePair::new_str(0, entity_type)];
        for pair in body {
            pairs.push(pair);
        }
        let drawing = from_section("ENTITIES", pairs);
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        entities[0].clone()
    }

    #[test]
    fn read_empty_entities_section() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "ENTITIES"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        let entities = drawing.entities();
        assert_eq!(0, entities.count());
    }

    #[test]
    fn read_unsupported_entity() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "ENTITIES"),
            CodePair::new_str(0, "UNSUPPORTED_ENTITY"),
            CodePair::new_str(1, "unsupported string"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        let entities = drawing.entities();
        assert_eq!(0, entities.count());
    }

    #[test]
    fn read_unsupported_entity_between_supported_entities() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "ENTITIES"),
            CodePair::new_str(0, "LINE"),
            CodePair::new_str(0, "UNSUPPORTED_ENTITY"),
            CodePair::new_str(1, "unsupported string"),
            CodePair::new_str(0, "CIRCLE"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
        match entities[1].specific {
            EntityType::Circle(_) => (),
            _ => panic!("expected a circle"),
        }
    }

    #[test]
    fn read_entity_with_no_values() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "ENTITIES"),
            CodePair::new_str(0, "LINE"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn read_common_entity_fields() {
        let ent = read_entity("LINE", vec![CodePair::new_str(8, "layer")]);
        assert_eq!("layer", ent.common.layer);
    }

    #[test]
    fn read_line() {
        let ent = read_entity(
            "LINE",
            vec![
                CodePair::new_f64(10, 1.1), // p1
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.3),
                CodePair::new_f64(11, 4.4), // p2
                CodePair::new_f64(21, 5.5),
                CodePair::new_f64(31, 6.6),
            ],
        );
        match ent.specific {
            EntityType::Line(ref line) => {
                assert_eq!(Point::new(1.1, 2.2, 3.3), line.p1);
                assert_eq!(Point::new(4.4, 5.5, 6.6), line.p2);
            }
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn write_common_entity_fields_r12() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R12;
        let mut ent = Entity {
            common: Default::default(),
            specific: EntityType::Line(Default::default()),
        };
        "some-layer".clone_into(&mut ent.common.layer);
        drawing.add_entity(ent);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "LINE"),
                CodePair::new_str(5, "1E"),
                CodePair::new_str(8, "some-layer"),
                CodePair::new_f64(10, 0.0),
            ],
        );
    }

    #[test]
    fn write_common_entity_fields_r13() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        let mut ent = Entity {
            common: Default::default(),
            specific: EntityType::Line(Default::default()),
        };
        "some-layer".clone_into(&mut ent.common.layer);
        drawing.add_entity(ent);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "LINE"),
                CodePair::new_str(5, "1E"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "some-layer"),
                CodePair::new_str(100, "AcDbLine"),
                CodePair::new_f64(10, 0.0),
            ],
        );
    }

    #[test]
    fn write_specific_entity_fields() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        let line = Line {
            p1: Point::new(1.1, 2.2, 3.3),
            p2: Point::new(4.4, 5.5, 6.6),
            ..Default::default()
        };
        drawing.add_entity(Entity::new(EntityType::Line(line)));
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(100, "AcDbLine"),
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.3),
                CodePair::new_f64(11, 4.4),
                CodePair::new_f64(21, 5.5),
                CodePair::new_f64(31, 6.6),
            ],
        );
    }

    #[test]
    fn read_multiple_entities() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "CIRCLE"),
                CodePair::new_f64(10, 1.1), // center
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.3),
                CodePair::new_f64(40, 4.4), // radius
                CodePair::new_str(0, "LINE"),
                CodePair::new_f64(10, 5.5), // p1
                CodePair::new_f64(20, 6.6),
                CodePair::new_f64(30, 7.7),
                CodePair::new_f64(11, 8.8), // p2
                CodePair::new_f64(21, 9.9),
                CodePair::new_f64(31, 10.1),
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());

        // verify circle
        match entities[0].specific {
            EntityType::Circle(ref circle) => {
                assert_eq!(Point::new(1.1, 2.2, 3.3), circle.center);
                assert!(approx_eq!(f64, 4.4, circle.radius));
            }
            _ => panic!("expected a line"),
        }

        // verify line
        match entities[1].specific {
            EntityType::Line(ref line) => {
                assert_eq!(Point::new(5.5, 6.6, 7.7), line.p1);
                assert_eq!(Point::new(8.8, 9.9, 10.1), line.p2);
            }
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn read_field_with_multiples_common() {
        let ent = read_entity(
            "LINE",
            vec![
                CodePair::new_binary(310, vec![0x01, 0x02]),
                CodePair::new_binary(310, vec![0x03, 0x04]),
            ],
        );
        assert_eq!(
            vec![vec![0x01, 0x02], vec![0x03, 0x04]],
            ent.common.preview_image_data
        );
    }

    #[test]
    fn write_field_with_multiples_common() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_entity(Entity {
            common: EntityCommon {
                preview_image_data: vec![vec![0x01, 0x02], vec![0x03, 0x04]],
                ..Default::default()
            },
            specific: EntityType::Line(Default::default()),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_binary(310, vec![0x01, 0x02]),
                CodePair::new_binary(310, vec![0x03, 0x04]),
            ],
        );
    }

    #[test]
    fn read_field_with_multiples_specific() {
        let ent = read_entity(
            "3DSOLID",
            vec![
                CodePair::new_str(1, "one-1"),
                CodePair::new_str(1, "one-2"),
                CodePair::new_str(3, "three-1"),
                CodePair::new_str(3, "three-2"),
            ],
        );
        match ent.specific {
            EntityType::Solid3D(ref solid3d) => {
                assert_eq!(vec!["one-1", "one-2"], solid3d.custom_data);
                assert_eq!(vec!["three-1", "three-2"], solid3d.custom_data2);
            }
            _ => panic!("expected a 3DSOLID"),
        }
    }

    #[test]
    fn write_field_with_multiples_specific() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13; // 3DSOLID is only supported on R13+
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Solid3D(Solid3D {
                custom_data: vec![String::from("one-1"), String::from("one-2")],
                custom_data2: vec![String::from("three-1"), String::from("three-2")],
                ..Default::default()
            }),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(1, "one-1"),
                CodePair::new_str(1, "one-2"),
                CodePair::new_str(3, "three-1"),
                CodePair::new_str(3, "three-2"),
            ],
        );
    }

    #[test]
    fn read_entity_with_post_parse() {
        let ent = read_entity(
            "IMAGE",
            vec![
                CodePair::new_f64(14, 1.1), // clipping vertices[0]
                CodePair::new_f64(24, 2.2),
                CodePair::new_f64(14, 3.3), // clipping vertices[1]
                CodePair::new_f64(24, 4.4),
                CodePair::new_f64(14, 5.5), // clipping vertices[2]
                CodePair::new_f64(24, 6.6),
            ],
        );
        match ent.specific {
            EntityType::Image(ref image) => {
                assert_eq!(3, image.clipping_vertices.len());
                assert_eq!(Point::new(1.1, 2.2, 0.0), image.clipping_vertices[0]);
                assert_eq!(Point::new(3.3, 4.4, 0.0), image.clipping_vertices[1]);
                assert_eq!(Point::new(5.5, 6.6, 0.0), image.clipping_vertices[2]);
            }
            _ => panic!("expected an IMAGE"),
        }
    }

    #[test]
    fn write_entity_with_write_order() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14; // IMAGE is only supported on R14+
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Image(Image {
                clipping_vertices: vec![
                    Point::new(1.1, 2.2, 0.0),
                    Point::new(3.3, 4.4, 0.0),
                    Point::new(5.5, 6.6, 0.0),
                ],
                ..Default::default()
            }),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_i32(91, 3),
                CodePair::new_f64(14, 1.1),
                CodePair::new_f64(24, 2.2),
                CodePair::new_f64(14, 3.3),
                CodePair::new_f64(24, 4.4),
                CodePair::new_f64(14, 5.5),
                CodePair::new_f64(24, 6.6),
            ],
        );
    }

    #[test]
    fn read_entity_with_custom_reader_mtext() {
        let ent = read_entity(
            "MTEXT",
            vec![
                CodePair::new_f64(50, 1.1),  // rotation angle
                CodePair::new_i16(75, 7),    // column type
                CodePair::new_f64(50, 3.0),  // column count
                CodePair::new_f64(50, 10.0), // column values
                CodePair::new_f64(50, 20.0),
                CodePair::new_f64(50, 30.0),
            ],
        );
        match ent.specific {
            EntityType::MText(ref mtext) => {
                assert!(approx_eq!(f64, 1.1, mtext.rotation_angle));
                assert_eq!(7, mtext.column_type);
                assert_eq!(3, mtext.column_count);
                assert_eq!(3, mtext.column_heights.len());
                assert!(approx_eq!(f64, 10.0, mtext.column_heights[0]));
                assert!(approx_eq!(f64, 20.0, mtext.column_heights[1]));
                assert!(approx_eq!(f64, 30.0, mtext.column_heights[2]));
            }
            _ => panic!("expected an MTEXT"),
        }
    }

    #[test]
    fn read_entity_after_entity_with_custom_reader() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "MTEXT"), // has a custom reader
                CodePair::new_str(0, "LINE"),  // uses the auto-generated reader
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::MText(_) => {}
            _ => panic!("expected an mtext"),
        }
        match entities[1].specific {
            EntityType::Line(_) => {}
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn read_entity_mtext_with_embedded_object() {
        // Arrange
        let entity = read_entity(
            "MTEXT",
            vec![
                // CodePair::new_i16(5, 764),
                CodePair::new_str(102, "{ACAD_XDICTIONARY"),
                CodePair::new_i32(360, 765),
                CodePair::new_str(102, "}"),
                // CodePair::new_i16(330, 31),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "Proj_ledninger$0$TF_K_---_Tekst-"),
                CodePair::new_str(100, "AcDbMText"),
                CodePair::new_f64(10, 676538913.4250469),
                CodePair::new_f64(20, 6147397473.123696),
                CodePair::new_f64(30, -152.5233843860332),
                CodePair::new_f64(40, 400.0),
                CodePair::new_f64(41, 6126.179808862732),
                CodePair::new_f64(46, 0.0),
                CodePair::new_i16(71, 1),
                CodePair::new_i16(72, 5),
                CodePair::new_str(1, "SHARH80\\PØ1250 Br.\\PDK: 48.25\\PBK: 45.22"),
                CodePair::new_str(7, "Proj_ledninger$0$Arial"),
                CodePair::new_f64(11, 0.2644180576221719),
                CodePair::new_f64(21, 0.9644081557117392),
                CodePair::new_f64(31, 0.0),
                CodePair::new_i16(73, 1),
                CodePair::new_f64(44, 1.0),
                // Embedded random object data
                CodePair::new_str(101, "Embedded Object"),
                CodePair::new_i16(70, 1),
                CodePair::new_f64(10, 0.2644180576221719),
                CodePair::new_f64(20, 0.9644081557117392),
                CodePair::new_f64(30, 0.0),
                CodePair::new_f64(11, 676538913.4250469),
                CodePair::new_f64(21, 6147397473.123696),
                CodePair::new_f64(31, -152.5233843860332),
                CodePair::new_f64(40, 400.0),
                CodePair::new_f64(41, 0.0),
                CodePair::new_f64(42, 2525.784447476126),
                CodePair::new_f64(43, 2431.650750341065),
                CodePair::new_i16(71, 2),
                CodePair::new_i16(72, 1),
                CodePair::new_f64(44, 6126.179808862732),
                CodePair::new_f64(45, 5000.0),
                CodePair::new_i16(73, 0),
                CodePair::new_i16(74, 0),
                CodePair::new_f64(46, 0.0),
                CodePair::new_str(1001, "AcadAnnotative"),
                CodePair::new_str(1000, "AnnotativeData"),
                CodePair::new_str(1002, "{"),
                CodePair::new_i16(1070, 1),
                CodePair::new_i16(1070, 1),
                CodePair::new_str(1002, "}"),
            ],
        );

        // Assert
        match entity.specific {
            EntityType::MText(ref mtext) => {
                assert_eq!(
                    Point::new(676538913.4250469, 6147397473.123696, -152.5233843860332),
                    mtext.insertion_point
                );
                assert_eq!(400.0, mtext.initial_text_height);
                assert_eq!(6126.179808862732, mtext.reference_rectangle_width);
                assert_eq!(0.0, mtext.rotation_angle);
                assert_eq!("SHARH80\\PØ1250 Br.\\PDK: 48.25\\PBK: 45.22", mtext.text);
                assert_eq!("Proj_ledninger$0$Arial", mtext.text_style_name);
                assert_eq!(0.2644180576221719, mtext.x_axis_direction.x);
                assert_eq!(0.9644081557117392, mtext.x_axis_direction.y);
                assert_eq!(0.0, mtext.x_axis_direction.z);
                assert_eq!(1.0, mtext.line_spacing_factor);
            }
            _ => panic!("expected an MTEXT"),
        }
    }

    #[test]
    fn read_multi_leader() {
        // Arrange
        let entity = read_entity(
            "MULTILEADER",
            vec![
                CodePair::new_str(102, "{ACAD_XDICTIONARY"),
                CodePair::new_i32(360, 301),
                CodePair::new_str(102, "}"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "Proj_ledninger$0$TF_K_---_Leader-"),
                CodePair::new_i16(160, 452),
                CodePair::new_str(100, "AcDbMLeader"),
                CodePair::new_i16(270, 2),
                CodePair::new_str(300, "CONTEXT_DATA{"), // start of CONTEXT_DATA
                CodePair::new_f64(40, 200.0),
                CodePair::new_f64(10, 676536270.82997),
                CodePair::new_f64(20, 6147398029.870995),
                CodePair::new_f64(30, 41.18369923159479),
                CodePair::new_f64(41, 500.0),
                CodePair::new_f64(140, 500.0),
                CodePair::new_f64(145, 400.0),
                CodePair::new_i16(174, 1),
                CodePair::new_i16(175, 1),
                CodePair::new_i16(176, 0),
                CodePair::new_i16(177, 0),
                CodePair::new_i16(290, 1),
                CodePair::new_str(304, "Great MLEADER {\\fArial;content}"),
                CodePair::new_f64(11, 0.0),
                CodePair::new_f64(21, 0.0),
                CodePair::new_f64(31, 1.0),
                CodePair::new_i16(340, 715), // 2CB in hex is 715
                CodePair::new_f64(12, 676536406.164786),
                CodePair::new_f64(22, 6147397578.003218),
                CodePair::new_f64(32, 41.18369923159479),
                CodePair::new_f64(13, -0.2644180576221713),
                CodePair::new_f64(23, -0.9644081557117392),
                CodePair::new_f64(33, 0.0),
                CodePair::new_f64(42, 4.444788527204954),
                CodePair::new_f64(43, 0.0),
                CodePair::new_f64(44, 0.0),
                CodePair::new_f64(45, 1.0),
                CodePair::new_i16(170, 1),
                CodePair::new_i32(90, -1028521984),
                CodePair::new_i16(171, 1),
                CodePair::new_i16(172, 5),
                CodePair::new_i32(91, -1073741824),
                CodePair::new_f64(141, 0.0),
                CodePair::new_i32(92, 0),
                CodePair::new_i16(291, 0),
                CodePair::new_i16(292, 0),
                CodePair::new_i16(173, 0),
                CodePair::new_i16(293, 0),
                CodePair::new_f64(142, 0.0),
                CodePair::new_f64(143, 0.0),
                CodePair::new_i16(294, 0),
                CodePair::new_i16(295, 0),
                CodePair::new_i16(296, 0),
                CodePair::new_f64(110, 676538419.6360937),
                CodePair::new_f64(120, 6147411946.366201),
                CodePair::new_f64(130, 41.18369923159479),
                CodePair::new_f64(111, -0.2644180576221713),
                CodePair::new_f64(121, -0.9644081557117392),
                CodePair::new_f64(131, 0.0),
                CodePair::new_f64(112, 0.9644081557117392),
                CodePair::new_f64(122, -0.2644180576221713),
                CodePair::new_f64(132, 0.0),
                CodePair::new_i16(297, 0),
                CodePair::new_str(302, "LEADER{"), // start of LEADER
                CodePair::new_i16(290, 1),
                CodePair::new_i16(291, 1),
                CodePair::new_f64(10, 676537064.180925),
                CodePair::new_f64(20, 6147400923.448454),
                CodePair::new_f64(30, 41.18369923159479),
                CodePair::new_f64(11, -0.2644180576221713), // Dogleg direction vector x
                CodePair::new_f64(21, -0.9644081557117392), // Dogleg direction vector y
                CodePair::new_f64(31, 0.0),                 // Dogleg direction vector z
                CodePair::new_i32(90, 0),
                CodePair::new_f64(40, 3000.366019534463), // dogleg length
                CodePair::new_str(304, "LEADER_LINE{"),   // start of LEADER_LINE
                CodePair::new_f64(10, 676548478.6502683),
                CodePair::new_f64(20, 6147400741.812067),
                CodePair::new_f64(30, 41.18369923159479),
                CodePair::new_i32(91, 0),
                CodePair::new_str(305, "}"), // end of LEADER_LINE
                CodePair::new_i16(271, 0),
                CodePair::new_str(303, "}"), // end of LEADER
                CodePair::new_i16(272, 9),
                CodePair::new_i16(273, 9),
                CodePair::new_str(301, "}"), // end of CONTEXT_DATA
                CodePair::new_i16(340, 725), // 2D5 in hex is 725
                CodePair::new_i32(90, 279744),
                CodePair::new_i16(170, 1),
                CodePair::new_i32(91, -1073741824),
                CodePair::new_i16(341, 20), // 14 in hex is 20
                CodePair::new_i16(171, -2),
                CodePair::new_i16(290, 1),
                CodePair::new_i16(291, 1),
                CodePair::new_f64(41, 15.0018300976723),
                CodePair::new_f64(42, 2.5),
                CodePair::new_i16(172, 2),
                CodePair::new_i16(343, 715), // 2CB in hex is 715
                CodePair::new_i16(173, 1),
                CodePair::new_i32(95, 1),
                CodePair::new_i16(174, 1),
                CodePair::new_i16(175, 0),
                CodePair::new_i32(92, -1023410175),
                CodePair::new_i16(292, 0),
                CodePair::new_i32(93, -1056964608),
                CodePair::new_f64(10, 1.0),
                CodePair::new_f64(20, 1.0),
                CodePair::new_f64(30, 1.0),
                CodePair::new_f64(43, 0.0),
                CodePair::new_i16(176, 0),
                CodePair::new_i16(293, 1),
                CodePair::new_i16(294, 0),
                CodePair::new_i16(178, 0),
                CodePair::new_i16(179, 1),
                CodePair::new_f64(45, 1.0),
                CodePair::new_i16(271, 0),
                CodePair::new_i16(272, 9),
                CodePair::new_i16(273, 9),
                CodePair::new_i16(295, 1),
            ],
        );

        // Assert
        match entity.specific {
            EntityType::MLeader(ref mleader) => {
                assert_eq!(mleader.enable_dogleg, true);
                assert_eq!(mleader.enable_dogleg, true);
                assert_eq!(mleader.dogleg_length, 3000.366019534463);
                assert_eq!(
                    mleader.dogleg_vector,
                    Vector::new(-0.2644180576221713, -0.9644081557117392, 0.0)
                );
                assert_eq!(mleader.vertices.len(), 2);

                // Text content
                assert_eq!(mleader.content_type, MLeaderContentType::MTextContent);
                assert_eq!(
                    mleader.default_text_contents,
                    "Great MLEADER {\\fArial;content}"
                );

                // base point
                assert_eq!(
                    mleader.content_base_point,
                    Point {
                        x: 676536270.82997,
                        y: 6147398029.870995,
                        z: 41.18369923159479
                    }
                );

                // Dogleg included vertices
                assert_eq!(
                    mleader.get_vertices(),
                    vec![
                        Point {
                            x: 676536270.82997,
                            y: 6147398029.870995,
                            z: 41.18369923159479,
                        },
                        Point {
                            x: 676537064.180925,
                            y: 6147400923.448454,
                            z: 41.18369923159479,
                        },
                        Point {
                            x: 676548478.6502683,
                            y: 6147400741.812067,
                            z: 41.18369923159479,
                        }
                    ]
                );

                // Text normal direction (11,21,31)
                assert_eq!(mleader.text_normal_direction, Vector::new(0.0, 0.0, 1.0));

                // Text location (12,22,32)
                assert_eq!(
                    mleader.text_location,
                    Point {
                        x: 676536406.164786,
                        y: 6147397578.003218,
                        z: 41.18369923159479
                    }
                );

                // Text properties
                assert_eq!(mleader.text_height, 500.0);
                assert_eq!(mleader.text_rotation, 4.444788527204954);
                assert_eq!(mleader.text_width, 0.0);
                assert_eq!(mleader.text_attachment, 1);
            }
            _ => panic!("expected a MLeader"),
        }

        assert_eq!(entity.common.layer, "Proj_ledninger$0$TF_K_---_Leader-");
    }

    #[test]
    fn read_multi_leader_danish_text() {
        // Arrange
        let entity = read_entity(
            "MULTILEADER",
            vec![
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "L 50- - SÅLEDES UDFØRT Tekst"),
                CodePair::new_i16(160, 2492),
                CodePair::new_str(100, "AcDbMLeader"),
                CodePair::new_i16(270, 2),
                CodePair::new_str(300, "CONTEXT_DATA{"), // start of CONTEXT_DATA
                CodePair::new_f64(40, 1.0),
                CodePair::new_f64(10, 648648030.3511316),
                CodePair::new_f64(20, 1174316399.052236),
                CodePair::new_f64(30, 0.0),
                CodePair::new_f64(41, 300.0),
                CodePair::new_f64(140, 200.0),
                CodePair::new_f64(145, 100.0),
                CodePair::new_i16(174, 1),
                CodePair::new_i16(175, 1),
                CodePair::new_i16(176, 0),
                CodePair::new_i16(177, 0),
                CodePair::new_i16(290, 1),
                CodePair::new_str(304, "{\\fArial|b1|i0|c0|p34;FV01B17\\PDK }8.50\\P{\\fArial|b1|i0|c0|p34;IK }6.64^J{\\fArial|b1|i0|c0|p34;UK }6.27\\P{\\fArial|b1|i0|c0|p34;BK }5.60"),
                CodePair::new_f64(11, 0.0),
                CodePair::new_f64(21, 0.0),
                CodePair::new_f64(31, 1.0),
                CodePair::new_i16(340, 17), // 11 in hex is 17
                CodePair::new_f64(12, 648648188.7703227),
                CodePair::new_f64(22, 1174316489.480647),
                CodePair::new_f64(32, 0.0),
                CodePair::new_f64(13, 0.8907119816332103),
                CodePair::new_f64(23, -0.4545681090607213),
                CodePair::new_f64(33, 0.0),
                CodePair::new_f64(42, 5.811298035900279),
                CodePair::new_f64(43, 0.0),
                CodePair::new_f64(44, 0.0),
                CodePair::new_f64(45, 1.0),
                CodePair::new_i16(170, 1),
                CodePair::new_i32(90, -1073741824),
                CodePair::new_i16(171, 1),
                CodePair::new_i16(172, 5),
                CodePair::new_i32(91, -1073741824),
                CodePair::new_f64(141, 1.199999999999999),
                CodePair::new_i32(92, 0),
                CodePair::new_i16(291, 1),
                CodePair::new_i16(292, 1),
                CodePair::new_i16(173, 0),
                CodePair::new_i16(293, 0),
                CodePair::new_f64(142, 0.0),
                CodePair::new_f64(143, 0.0),
                CodePair::new_i16(294, 0),
                CodePair::new_i16(295, 0),
                CodePair::new_i16(296, 0),
                CodePair::new_f64(110, 648652444.1320328),
                CodePair::new_f64(120, 1174314908.43074),
                CodePair::new_f64(130, 0.0),
                CodePair::new_f64(111, 0.8907119816332103),
                CodePair::new_f64(121, -0.4545681090607213),
                CodePair::new_f64(131, 0.0),
                CodePair::new_f64(112, 0.4545681090607213),
                CodePair::new_f64(122, 0.8907119816332103),
                CodePair::new_f64(132, 0.0),
                CodePair::new_i16(297, 0),
                CodePair::new_str(302, "LEADER{"), // start of LEADER
                CodePair::new_i16(290, 1),
                CodePair::new_i16(291, 1),
                CodePair::new_f64(10, 648650186.7733452),
                CodePair::new_f64(20, 1174315298.538503),
                CodePair::new_f64(30, 0.0),
                CodePair::new_f64(11, -0.8907119816332102), // Dogleg direction vector x
                CodePair::new_f64(21, 0.4545681090607215),  // Dogleg direction vector y
                CodePair::new_f64(31, 0.0),                 // Dogleg direction vector z
                CodePair::new_i32(90, 0),
                CodePair::new_f64(40, 500.0000000000002), // dogleg length
                CodePair::new_str(304, "LEADER_LINE{"),   // start of LEADER_LINE
                CodePair::new_f64(10, 648652198.1563798),
                CodePair::new_f64(20, 1174315906.356325),
                CodePair::new_f64(30, 0.0),
                CodePair::new_i32(91, 0),
                CodePair::new_str(305, "}"), // end of LEADER_LINE
                CodePair::new_i16(271, 0),
                CodePair::new_str(303, "}"), // end of LEADER
                CodePair::new_i16(272, 9),
                CodePair::new_i16(273, 9),
                CodePair::new_str(301, "}"), // end of CONTEXT_DATA
                CodePair::new_i16(340, 302), // 12E in hex is 302
                CodePair::new_i32(90, 67585696),
                CodePair::new_i16(170, 1),
                CodePair::new_i32(91, -1056964608),
                CodePair::new_i16(341, 20), // 14 in hex is 20
                CodePair::new_i16(171, -2),
                CodePair::new_i16(290, 1),
                CodePair::new_i16(291, 1),
                CodePair::new_f64(41, 500.0),
                CodePair::new_f64(42, 200.0),
                CodePair::new_i16(172, 2),
                CodePair::new_i16(343, 17), // 11 in hex is 17
                CodePair::new_i16(173, 1),
                CodePair::new_i32(95, 1),
                CodePair::new_i16(174, 1),
                CodePair::new_i16(175, 0),
                CodePair::new_i32(92, -1056964608),
                CodePair::new_i16(292, 1),
                CodePair::new_i32(93, -1056964608),
                CodePair::new_f64(10, 1.0),
                CodePair::new_f64(20, 1.0),
                CodePair::new_f64(30, 1.0),
                CodePair::new_f64(43, 0.0),
                CodePair::new_i16(176, 0),
                CodePair::new_i16(293, 0),
                CodePair::new_i16(294, 0),
                CodePair::new_i16(178, 1),
                CodePair::new_i16(179, 1),
                CodePair::new_f64(45, 1.0),
                CodePair::new_i16(271, 0),
                CodePair::new_i16(272, 9),
                CodePair::new_i16(273, 9),
                CodePair::new_i16(295, 0),
            ],
        );

        // Assert
        match entity.specific {
            EntityType::MLeader(ref mleader) => {
                assert_eq!(mleader.enable_dogleg, true);
                assert_eq!(mleader.dogleg_length, 500.0000000000002);
                assert_eq!(
                    mleader.dogleg_vector,
                    Vector::new(-0.8907119816332102, 0.4545681090607215, 0.0)
                );
                assert_eq!(mleader.vertices.len(), 2);

                // Text content - Danish utility planning text with formatting
                assert_eq!(mleader.content_type, MLeaderContentType::MTextContent);
                assert_eq!(mleader.default_text_contents, "{\\fArial|b1|i0|c0|p34;FV01B17\\PDK }8.50\\P{\\fArial|b1|i0|c0|p34;IK }6.64^J{\\fArial|b1|i0|c0|p34;UK }6.27\\P{\\fArial|b1|i0|c0|p34;BK }5.60");

                // Base point
                assert_eq!(
                    mleader.content_base_point,
                    Point {
                        x: 648648030.3511316,
                        y: 1174316399.052236,
                        z: 0.0
                    }
                );

                // Dogleg included vertices
                assert_eq!(
                    mleader.get_vertices(),
                    vec![
                        Point {
                            x: 648649741.4173545,
                            y: 1174315525.8225574,
                            z: 0.0,
                        },
                        Point {
                            x: 648650186.7733452,
                            y: 1174315298.538503,
                            z: 0.0,
                        },
                        Point {
                            x: 648652198.1563798,
                            y: 1174315906.356325,
                            z: 0.0,
                        }
                    ]
                );

                // Text location for Danish test
                assert_eq!(
                    mleader.text_location,
                    Point {
                        x: 648648188.7703227,
                        y: 1174316489.480647,
                        z: 0.0
                    }
                );

                assert_eq!(mleader.text_normal_direction, Vector::new(0.0, 0.0, 1.0));

                // Text properties for Danish test
                assert_eq!(mleader.text_height, 300.0);
                assert_eq!(mleader.text_rotation, 5.811298035900279);
            }
            _ => panic!("expected a MLeader"),
        }

        assert_eq!(entity.common.layer, "L 50- - SÅLEDES UDFØRT Tekst");
    }

    #[test]
    fn write_multi_leader_basic() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2018;
        let mut mleader = MLeader::default();
        mleader.content_type = MLeaderContentType::MTextContent;
        mleader.enable_landing = true;

        // Text content
        mleader.default_text_contents = "Great MLEADER content\nTest".to_string();
        mleader.has_m_text = true;
        mleader.enable_frame_text = true;
        mleader.text_height = 0.18;
        mleader.text_direction = Vector::new(1.0, 0.0, 0.0); // text reading direction
        mleader.text_normal_direction = Vector::new(0.0, 0.0, 1.0); // text plane normal
        mleader.text_attachment = 1;
        mleader.vertices = vec![
            Point::new(0.0, 0.0, 0.0), // ONLY the arrow start point
        ];

        // Set the vertex point (from working DXF - this appears before LEADER{ section)
        mleader.vertex = Point::new(0.0, 0.0, 0.0);
        mleader.break_point_index = 0;

        // Last leader line point - where the dogleg starts (NOT same as vertices)
        mleader.last_leader_line_point = Point::new(2.5, 1.0, 0.0);
        mleader.has_set_last_leader_line_point = true;

        // Dogleg settings
        mleader.enable_dogleg = true;
        mleader.dogleg_length = 0.3;
        mleader.dogleg_length_leader = 0.3;
        mleader.dogleg_vector = Vector::new(1.0, 0.0, 0.0); // points right toward text
        mleader.has_set_dogleg_vector = true;

        // Content positioning (matches working DXF values)
        mleader.content_base_point = Point::new(2.8, 1.0, 0.0);
        mleader.text_location = Point::new(2.94, 1.0, 0.0);

        // Context data
        let landing_gap = 0.14;
        mleader.landing_gap = landing_gap;
        mleader.arrow_head_size = 0.18;
        mleader.content_scale = 1.0;

        // Additional properties from working DXF
        mleader.break_point_index_line = 0;

        drawing.add_entity(Entity {
            common: EntityCommon {
                layer: "Proj_ledninger$0$TF_K_---_Leader-".to_string(),
                ..Default::default()
            },
            specific: EntityType::MLeader(mleader),
        });

        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "MULTILEADER"),
                CodePair::new_str(5, "1E"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "Proj_ledninger$0$TF_K_---_Leader-"),
                CodePair::new_str(347, "0"), // sentinel for DXF version
                CodePair::new_i16(370, 0),   // sentinel for DXF version
                CodePair::new_str(430, ""),
                CodePair::new_i32(440, 0),   // sentinel for DX
                CodePair::new_str(390, "0"), // sentinel for DXF version
                CodePair::new_i16(284, 0),   // sentinel for DXF version
                CodePair::new_str(100, "AcDbMLeader"),
                // Version number
                CodePair::new_i16(270, 2),
                // Common MLeader Group Codes (before context data)
                CodePair::new_i32(90, 0),    // property_override_flag
                CodePair::new_i16(170, 1),   // leader_line_type (Straight)
                CodePair::new_i32(91, 256),  // leader_line_color
                CodePair::new_i16(171, -1),  // leader_line_weight
                CodePair::new_i16(290, 1),   // enable_landing
                CodePair::new_i16(291, 1),   // enable_dogleg
                CodePair::new_f64(41, 0.3),  // dogleg_length
                CodePair::new_f64(42, 0.18), // arrowhead_size
                CodePair::new_i16(172, 2),   // content_type (MTextContent)
                CodePair::new_i16(173, 1),   // text_left_attachment_type
                CodePair::new_i16(95, 1),    // text_right_attachment_type
                CodePair::new_i16(174, 0),   // text_angle_type
                CodePair::new_i16(175, 0),   // text_alignment_type
                CodePair::new_i32(92, 256),  // text_color
                CodePair::new_i16(292, 1),   // enable_frame_text
                // CodePair::new_i32(93, 256), // block_content_color
                // CodePair::new_f64(10, 1.0), // block_content_scale
                // CodePair::new_f64(43, 0.0), // block_content_rotation
                // CodePair::new_i16(176, 0),  // block_content_connection_type
                CodePair::new_i16(293, 0), // enable_annotation_scale
                CodePair::new_i32(94, 0),  // arrowhead_index
                // CodePair::new_i16(177, 0),  // block_attribute_index
                // CodePair::new_f64(44, 0.0), // block_attribute_width
                CodePair::new_i16(294, 0), // text_direction_negative
                CodePair::new_i16(178, 0), // text_align_in_ipe
                CodePair::new_i16(179, 1), // text_attachment_point
                CodePair::new_i16(271, 0), // text_attachment_direction
                CodePair::new_i16(272, 9), // bottom_text_attachment_direction
                CodePair::new_i16(273, 9), // top_text_attachment_direction
                // CONTEXT_DATA Section
                CodePair::new_str(300, "CONTEXT_DATA{"),
                // MLeader Context Data Group Codes
                CodePair::new_f64(40, 1.0),   // content_scale
                CodePair::new_f64(10, 2.8),   // content_base_point.x
                CodePair::new_f64(20, 1.0),   // content_base_point.y
                CodePair::new_f64(30, 0.0),   // content_base_point.z
                CodePair::new_f64(41, 0.18),  // text_height
                CodePair::new_f64(140, 0.18), // arrow_head_size
                CodePair::new_f64(145, 0.14), // landing_gap
                CodePair::new_i16(290, 1),    // has_m_text
                CodePair::new_str(304, "Great MLEADER content\nTest"), // default_text_contents
                CodePair::new_f64(11, 0.0),   // text_normal_direction.x
                CodePair::new_f64(21, 0.0),   // text_normal_direction.y
                CodePair::new_f64(31, 1.0),   // text_normal_direction.z
                CodePair::new_f64(12, 2.94),  // text_location.x
                CodePair::new_f64(22, 1.0),   // text_location.y
                CodePair::new_f64(32, 0.0),   // text_location.z
                CodePair::new_f64(13, 1.0),   // text_direction.x
                CodePair::new_f64(23, 0.0),   // text_direction.y
                CodePair::new_f64(33, 0.0),   // text_direction.z
                CodePair::new_f64(42, 0.0),   // text_rotation
                CodePair::new_f64(43, 0.0),   // text_width
                CodePair::new_f64(44, 0.0),   // text_height_context
                CodePair::new_f64(45, 1.0),   // text_line_spacing_factor
                CodePair::new_i16(170, 1),    // text_line_spacing_style
                CodePair::new_i32(90, 256),   // text_color_context
                CodePair::new_i16(171, 1),    // text_attachment
                CodePair::new_i16(172, 1),    // text_flow_direction
                CodePair::new_i32(91, 256),   // text_background_color
                CodePair::new_f64(141, 1.5),  // text_background_scale_factor
                CodePair::new_i32(92, 0),     // text_background_transparency
                CodePair::new_i16(291, 0),    // is_text_background_color_on
                CodePair::new_i16(292, 0),    // is_text_background_fill_on
                CodePair::new_i16(173, 0),    // text_column_type
                CodePair::new_i16(293, 0),    // use_text_auto_height
                CodePair::new_f64(142, 0.0),  // text_column_width
                CodePair::new_f64(143, 0.0),  // text_column_gutter_width
                CodePair::new_i16(294, 0),    // text_column_flow_reversed
                CodePair::new_f64(144, 0.0),  // text_column_height
                CodePair::new_i16(295, 0),    // text_use_word_break
                CodePair::new_i16(296, 0),    // has_block
                // CodePair::new_f64(14, 0.0),   // block_content_normal_direction.x
                // CodePair::new_f64(24, 0.0),   // block_content_normal_direction.y
                // CodePair::new_f64(34, 0.0),   // block_content_normal_direction.z
                // CodePair::new_f64(15, 0.0),   // block_content_position.x
                // CodePair::new_f64(25, 0.0),   // block_content_position.y
                // CodePair::new_f64(35, 0.0),   // block_content_position.z
                // CodePair::new_f64(16, 1.0),   // block_content_scale_context
                // CodePair::new_f64(46, 0.0),   // block_content_rotation_context
                // CodePair::new_i32(93, 256),   // block_content_color_context
                // CodePair::new_f64(47, 0.0),   // block_transformation_matrix
                // CodePair::new_f64(110, 0.0),  // mleader_plane_origin_point.x
                // CodePair::new_f64(111, 1.0),  // mleader_plane_x_axis_direction.x
                // CodePair::new_f64(112, 0.0),  // mleader_plane_y_axis_direction.x
                // CodePair::new_i16(297, 0),    // mleader_plane_normal_reversed
                // CodePair::new_f64(10, 0.0),   // vertex.x
                // CodePair::new_f64(20, 0.0),   // vertex.y
                // CodePair::new_f64(30, 0.0),   // vertex.z
                CodePair::new_i32(90, 0), // break_point_index
                // LEADER Section
                CodePair::new_str(302, "LEADER{"),
                // MLeader Leader Node Group Codes
                CodePair::new_i16(290, 1), // has_set_last_leader_line_point
                CodePair::new_i16(291, 1), // has_set_dogleg_vector
                CodePair::new_f64(10, 2.5), // last_leader_line_point.x
                CodePair::new_f64(20, 1.0), // last_leader_line_point.y
                CodePair::new_f64(30, 0.0), // last_leader_line_point.z
                CodePair::new_f64(11, 1.0), // dogleg_vector.x
                CodePair::new_f64(21, 0.0), // dogleg_vector.y
                CodePair::new_f64(31, 0.0), // dogleg_vector.z
                // CodePair::new_f64(12, 0.0), // break_start_point.x
                // CodePair::new_f64(22, 0.0), // break_start_point.y
                // CodePair::new_f64(32, 0.0), // break_start_point.z
                // CodePair::new_f64(13, 0.0), // break_end_point.x
                // CodePair::new_f64(23, 0.0), // break_end_point.y
                // CodePair::new_f64(33, 0.0), // break_end_point.z
                CodePair::new_i32(90, 0),   // leader_branch_index
                CodePair::new_f64(40, 0.3), // dogleg_length_leader
                // LEADER_LINE Section
                CodePair::new_str(304, "LEADER_LINE{"),
                // MLeader Leader Line Group Codes (vertices)
                CodePair::new_f64(10, 0.0), // vertex 0 x
                CodePair::new_f64(20, 0.0), // vertex 0 y
                CodePair::new_f64(30, 0.0), // vertex 0 z
                // CodePair::new_i32(90, 0),                // break_point_index_line
                // CodePair::new_f64(11, 0.0),              // break_start_point_line.x
                // CodePair::new_f64(21, 0.0),              // break_start_point_line.y
                // CodePair::new_f64(31, 0.0),              // break_start_point_line.z
                // CodePair::new_f64(12, 0.0),              // break_end_point_line.x
                // CodePair::new_f64(22, 0.0),              // break_end_point_line.y
                // CodePair::new_f64(32, 0.0),              // break_end_point_line.z
                CodePair::new_i32(91, 0), // leader_line_index
                // Close sections
                CodePair::new_str(305, "}"), // LEADER_LINE_END
                CodePair::new_str(303, "}"), // LEADER_END
                CodePair::new_str(301, "}"), // CONTEXT_DATA_END
            ],
        );

        // Output file
        let output_folder = "test-output";
        if !std::path::Path::new(output_folder).exists() {
            std::fs::create_dir(output_folder).unwrap();
        }
        let output_path = format!("{}/mleader_basic.dxf", output_folder);
        drawing.save_file(&output_path).unwrap();
        println!("MLeader written to {}", output_path);
    }

    #[test]
    fn read_entity_with_flags() {
        let ent = read_entity("IMAGE", vec![CodePair::new_i16(70, 5)]);
        match ent.specific {
            EntityType::Image(ref image) => {
                assert!(image.show_image());
                assert!(image.use_clipping_boundary());
            }
            _ => panic!("expected an IMAGE"),
        }
    }

    #[test]
    fn write_entity_with_flags() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14; // IMAGE is only supported on R14+
        let mut image = Image::default();
        assert_eq!(0, image.display_options_flags);
        image.set_show_image(true);
        image.set_use_clipping_boundary(true);
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Image(image),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_i16(70, 5),  // flags
                CodePair::new_i16(280, 1), // sentinels to make sure we're not reading a header value
                CodePair::new_i16(281, 50),
            ],
        );
    }

    #[test]
    fn read_entity_with_handle_and_pointer() {
        let ent = read_entity(
            "3DSOLID",
            vec![
                CodePair::new_str(5, "A1"),   // handle
                CodePair::new_str(330, "A2"), // owner handle
                CodePair::new_str(350, "A3"), // history_object pointer
            ],
        );
        assert_eq!(Handle(0xa1), ent.common.handle);
        assert_eq!(Handle(0xa2), ent.common.__owner_handle);
        match ent.specific {
            EntityType::Solid3D(ref solid) => {
                assert_eq!(Handle(0xa3), solid.__history_object_handle)
            }
            _ => panic!("expected a 3DSOLID entity"),
        }
    }

    #[test]
    fn write_entity_with_handle_and_pointer() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_entity(Entity {
            common: EntityCommon {
                __owner_handle: Handle(0xa2),
                ..Default::default()
            },
            specific: EntityType::Line(Default::default()),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "LINE"),
                CodePair::new_str(5, "1E"),
                CodePair::new_str(330, "A2"),
            ],
        );
    }

    #[test]
    fn write_version_specific_entity() {
        let mut drawing = Drawing::new();
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Solid3D(Default::default()),
        });

        // 3DSOLID not supported in R12 and below
        drawing.header.version = AcadVersion::R12;
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "SECTION"),
                CodePair::new_str(2, "ENTITIES"),
                CodePair::new_str(0, "ENDSEC"),
            ],
        );

        // but it is in R13 and above
        drawing.header.version = AcadVersion::R13;
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "SECTION"),
                CodePair::new_str(2, "ENTITIES"),
                CodePair::new_str(0, "3DSOLID"),
            ],
        );
    }

    #[test]
    fn read_polyline() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline sentinel
                CodePair::new_str(0, "VERTEX"),   // vertex 1
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(30, 3.1),
                CodePair::new_str(0, "VERTEX"), // vertex 2
                CodePair::new_f64(10, 1.2),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.2),
                CodePair::new_str(0, "VERTEX"), // vertex 3
                CodePair::new_f64(10, 1.3),
                CodePair::new_f64(20, 2.3),
                CodePair::new_f64(30, 3.3),
                CodePair::new_str(0, "SEQEND"), // end sequence
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_polyline_without_seqend() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline sentinel
                CodePair::new_str(0, "VERTEX"),   // vertex 1
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(30, 3.1),
                CodePair::new_str(0, "VERTEX"), // vertex 2
                CodePair::new_f64(10, 1.2),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.2),
                CodePair::new_str(0, "VERTEX"), // vertex 3
                CodePair::new_f64(10, 1.3),
                CodePair::new_f64(20, 2.3),
                CodePair::new_f64(30, 3.3),
                // no end sequence
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_empty_polyline() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"),
                CodePair::new_str(0, "SEQEND"),
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_empty_polyline_without_seqend() {
        let drawing = from_section("ENTITIES", vec![CodePair::new_str(0, "POLYLINE")]);
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_polyline_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline sentinel
                CodePair::new_str(0, "VERTEX"),   // vertex 1
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(30, 3.1),
                CodePair::new_str(0, "VERTEX"), // vertex 2
                CodePair::new_f64(10, 1.2),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.2),
                CodePair::new_str(0, "VERTEX"), // vertex 3
                CodePair::new_f64(10, 1.3),
                CodePair::new_f64(20, 2.3),
                CodePair::new_f64(30, 3.3),
                CodePair::new_str(0, "SEQEND"), // end sequence
                CodePair::new_str(0, "LINE"),   // trailing entity
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }

        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn read_polyline_without_seqend_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline sentinel
                CodePair::new_str(0, "VERTEX"),   // vertex 1
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(30, 3.1),
                CodePair::new_str(0, "VERTEX"), // vertex 2
                CodePair::new_f64(10, 1.2),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.2),
                CodePair::new_str(0, "VERTEX"), // vertex 3
                CodePair::new_f64(10, 1.3),
                CodePair::new_f64(20, 2.3),
                CodePair::new_f64(30, 3.3),
                // no end sequence
                CodePair::new_str(0, "LINE"), // trailing entity
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }

        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn read_empty_polyline_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline sentinel
                CodePair::new_str(0, "SEQEND"),   // end sequence
                CodePair::new_str(0, "LINE"),     // trailing entity
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn read_empty_polyline_without_seqend_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline sentinel
                CodePair::new_str(0, "LINE"),     // trailing entity
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn write_2d_polyline() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        let mut poly = Polyline::default();
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.1, 2.1, 3.1),
                ..Default::default()
            },
        );
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.2, 2.2, 3.2),
                ..Default::default()
            },
        );
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.3, 2.3, 3.3),
                ..Default::default()
            },
        );
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Polyline(poly),
        });
        // TODO
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline
                CodePair::new_str(5, "21"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "0"),
                CodePair::new_str(100, "AcDb2dPolyline"),
                CodePair::new_i16(66, 1),
                CodePair::new_f64(10, 0.0),
                CodePair::new_f64(20, 0.0),
                CodePair::new_f64(30, 0.0),
                CodePair::new_str(0, "VERTEX"), // vertex 1
                CodePair::new_str(5, "1E"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "0"),
                CodePair::new_str(100, "AcDbVertex"),
                CodePair::new_str(100, "AcDb2dVertex"),
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(30, 3.1),
                CodePair::new_i16(70, 0),
                CodePair::new_f64(50, 0.0),
                CodePair::new_str(0, "VERTEX"), // vertex 2
                CodePair::new_str(5, "1F"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "0"),
                CodePair::new_str(100, "AcDbVertex"),
                CodePair::new_str(100, "AcDb2dVertex"),
                CodePair::new_f64(10, 1.2),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.2),
                CodePair::new_i16(70, 0),
                CodePair::new_f64(50, 0.0),
                CodePair::new_str(0, "VERTEX"), // vertex 3
                CodePair::new_str(5, "20"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "0"),
                CodePair::new_str(100, "AcDbVertex"),
                CodePair::new_str(100, "AcDb2dVertex"),
                CodePair::new_f64(10, 1.3),
                CodePair::new_f64(20, 2.3),
                CodePair::new_f64(30, 3.3),
                CodePair::new_i16(70, 0),
                CodePair::new_f64(50, 0.0),
                CodePair::new_str(0, "SEQEND"), // end sequence
            ],
        );
    }

    #[test]
    fn write_3d_polyline() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        let mut poly = Polyline::default();
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.1, 2.1, 3.1),
                ..Default::default()
            },
        );
        poly.set_is_3d_polyline(true);
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Polyline(poly),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(0, "POLYLINE"), // polyline
                CodePair::new_str(5, "1F"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "0"),
                CodePair::new_str(100, "AcDb3dPolyline"), // 3d = true
                CodePair::new_i16(66, 1),
                CodePair::new_f64(10, 0.0),
                CodePair::new_f64(20, 0.0),
                CodePair::new_f64(30, 0.0),
                CodePair::new_i16(70, 8),       // 3d = true
                CodePair::new_str(0, "VERTEX"), // vertex 1
                CodePair::new_str(5, "1E"),
                CodePair::new_str(100, "AcDbEntity"),
                CodePair::new_str(8, "0"),
                CodePair::new_str(100, "AcDbVertex"),
                CodePair::new_str(100, "AcDb3dPolylineVertex"), // 3d = true
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(30, 3.1),
                CodePair::new_i16(70, 32), // 3d = true
            ],
        );
    }

    #[test]
    fn polyline_seqend_handle_is_assigned_when_added_to_drawing() {
        let mut drawing = Drawing::new();
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Polyline(Polyline::default()),
        });
        assert_not_contains_pairs(
            &drawing,
            vec![CodePair::new_str(0, "SEQEND"), CodePair::new_str(5, "0")],
        );
    }

    #[test]
    fn read_lw_polyline_with_elevation() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "LWPOLYLINE"),
                CodePair::new_f64(38, 42.0), // elevation
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        assert_eq!(42.0, entities[0].common.elevation);
    }

    #[test]
    fn read_lw_polyline_with_no_vertices() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "LWPOLYLINE"),
                CodePair::new_f64(43, 43.0),
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::LwPolyline(ref poly) => {
                assert!(approx_eq!(f64, 43.0, poly.constant_width));
                assert_eq!(0, poly.vertices.len());
            }
            _ => panic!("expected an LWPOLYLINE"),
        }
    }

    #[test]
    fn read_lw_polyline_with_one_vertex() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "LWPOLYLINE"),
                CodePair::new_f64(43, 43.0), // constant width
                CodePair::new_f64(10, 1.1),  // vertex 1
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(40, 40.1), // starting width
                CodePair::new_f64(41, 41.1), // ending width
                CodePair::new_f64(42, 42.1), // bulge
                CodePair::new_i32(91, 91),   // id
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::LwPolyline(ref poly) => {
                assert!(approx_eq!(f64, 43.0, poly.constant_width));
                assert_eq!(1, poly.vertices.len());
                // vertex 1
                assert!(approx_eq!(f64, 1.1, poly.vertices[0].x));
                assert!(approx_eq!(f64, 2.1, poly.vertices[0].y));
                assert!(approx_eq!(f64, 40.1, poly.vertices[0].starting_width));
                assert!(approx_eq!(f64, 41.1, poly.vertices[0].ending_width));
                assert!(approx_eq!(f64, 42.1, poly.vertices[0].bulge));
                assert_eq!(91, poly.vertices[0].id);
            }
            _ => panic!("expected an LWPOLYLINE"),
        }
    }

    #[test]
    fn read_lw_polyline_with_multiple_vertices() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "LWPOLYLINE"),
                CodePair::new_f64(43, 43.0), // constant width
                CodePair::new_f64(10, 1.1),  // vertex 1
                CodePair::new_f64(20, 2.1),
                CodePair::new_f64(40, 40.1), // starting width
                CodePair::new_f64(41, 41.1), // ending width
                CodePair::new_f64(42, 42.1), // bulge
                CodePair::new_i32(91, 91),   // id
                CodePair::new_f64(10, 1.2),  // vertex 1
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(40, 40.2), // starting width
                CodePair::new_f64(41, 41.2), // ending width
                CodePair::new_f64(42, 42.2), // bulge
                CodePair::new_i32(91, 92),   // id
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::LwPolyline(ref poly) => {
                assert!(approx_eq!(f64, 43.0, poly.constant_width));
                assert_eq!(2, poly.vertices.len());
                // vertex 1
                assert!(approx_eq!(f64, 1.1, poly.vertices[0].x));
                assert!(approx_eq!(f64, 2.1, poly.vertices[0].y));
                assert!(approx_eq!(f64, 40.1, poly.vertices[0].starting_width));
                assert!(approx_eq!(f64, 41.1, poly.vertices[0].ending_width));
                assert!(approx_eq!(f64, 42.1, poly.vertices[0].bulge));
                assert_eq!(91, poly.vertices[0].id);
                // vertex 2
                assert!(approx_eq!(f64, 1.2, poly.vertices[1].x));
                assert!(approx_eq!(f64, 2.2, poly.vertices[1].y));
                assert!(approx_eq!(f64, 40.2, poly.vertices[1].starting_width));
                assert!(approx_eq!(f64, 41.2, poly.vertices[1].ending_width));
                assert!(approx_eq!(f64, 42.2, poly.vertices[1].bulge));
                assert_eq!(92, poly.vertices[1].id);
            }
            _ => panic!("expected an LWPOLYLINE"),
        }
    }

    #[test]
    fn write_lw_polyline() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2013;
        let mut poly = LwPolyline {
            constant_width: 43.0,
            ..Default::default()
        };
        poly.vertices.push(LwPolylineVertex {
            x: 1.1,
            y: 2.1,
            ..Default::default()
        });
        poly.vertices.push(LwPolylineVertex {
            x: 1.2,
            y: 2.2,
            starting_width: 40.2,
            ending_width: 41.2,
            bulge: 42.2,
            id: 92,
        });
        let mut entity = Entity::new(EntityType::LwPolyline(poly));
        entity.common.elevation = 42.0;
        drawing.add_entity(entity);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(100, "AcDbPolyline"),
                CodePair::new_i32(90, 2), // number of vertices
                CodePair::new_i16(70, 0),
                CodePair::new_f64(43, 43.0), // constant width
                CodePair::new_f64(38, 42.0), // elevation
                CodePair::new_f64(10, 1.1),  // vertex 1
                CodePair::new_f64(20, 2.1),
                CodePair::new_i32(91, 0),
                CodePair::new_f64(10, 1.2), // vertex 2
                CodePair::new_f64(20, 2.2),
                CodePair::new_i32(91, 92),
                CodePair::new_f64(40, 40.2), // starting width
                CodePair::new_f64(41, 41.2), // ending width
                CodePair::new_f64(42, 42.2), // bulge
            ],
        );
    }

    #[test]
    fn read_dimension() {
        let ent = read_entity(
            "DIMENSION",
            vec![
                CodePair::new_str(1, "text"),
                CodePair::new_str(100, "AcDbOrdinateDimension"),
                CodePair::new_f64(13, 1.1), // definition_point_2
                CodePair::new_f64(23, 2.2),
                CodePair::new_f64(33, 3.3),
                CodePair::new_f64(14, 4.4), // definition_point_3
                CodePair::new_f64(24, 5.5),
                CodePair::new_f64(34, 6.6),
            ],
        );
        match ent.specific {
            EntityType::OrdinateDimension(ref dim) => {
                assert_eq!("text", dim.dimension_base.text);
                assert_eq!(Point::new(1.1, 2.2, 3.3), dim.definition_point_2);
                assert_eq!(Point::new(4.4, 5.5, 6.6), dim.definition_point_3);
            }
            _ => panic!("expected an ordinate dimension"),
        }
    }

    #[test]
    fn read_entity_after_unsupported_dimension() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "DIMENSION"),
                CodePair::new_str(1, "text"),
                CodePair::new_str(100, "AcDbSomeUnsupportedDimensionType"),
                CodePair::new_f64(10, 1.1),
                CodePair::new_f64(20, 2.2),
                CodePair::new_f64(30, 3.3),
                CodePair::new_str(0, "LINE"),
            ],
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => {}
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn write_arc() {
        let arc = Arc::new(Point::new(1.0, 2.0, 3.0), 4.0, 90.0, 180.0);
        let ent = Entity::new(EntityType::Arc(arc));
        let mut drawing = Drawing::new();
        drawing.add_entity(ent);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(100, "AcDbCircle"),
                CodePair::new_f64(10, 1.0),
                CodePair::new_f64(20, 2.0),
                CodePair::new_f64(30, 3.0),
                CodePair::new_f64(40, 4.0),
                CodePair::new_str(100, "AcDbArc"),
                CodePair::new_f64(50, 90.0),
                CodePair::new_f64(51, 180.0),
            ],
        );
    }

    #[test]
    fn write_dimension() {
        let dim = RadialDimension {
            dimension_base: DimensionBase {
                text: String::from("some-text"),
                ..Default::default()
            },
            definition_point_2: Point::new(1.1, 2.2, 3.3),
            ..Default::default()
        };
        let ent = Entity::new(EntityType::RadialDimension(dim));
        let mut drawing = Drawing::new();
        drawing.add_entity(ent);
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "DIMENSION")]);
        assert_contains_pairs(&drawing, vec![CodePair::new_str(1, "some-text")]);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(100, "AcDbRadialDimension"),
                CodePair::new_f64(15, 1.1), // definition_point_2
                CodePair::new_f64(25, 2.2),
                CodePair::new_f64(35, 3.3),
                CodePair::new_f64(40, 0.0), // leader_length
            ],
        );
    }

    #[test]
    fn read_insert_with_separate_attributes() {
        let file = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "INSERT"),
                CodePair::new_i16(66, 0),       // no attributes
                CodePair::new_str(0, "ATTRIB"), // this is a separate attribute, not tiee to the `INSERT` entity
                CodePair::new_str(0, "SEQEND"), // this is a separate `SEQEND` entity, not tiee to the `INSERT` entity
            ],
        );
        let entities = file.entities().collect::<Vec<_>>();
        assert_eq!(3, entities.len());
        match entities[0].specific {
            EntityType::Insert(_) => (),
            _ => panic!("expected an INSERT"),
        }
        match entities[1].specific {
            EntityType::Attribute(_) => (),
            _ => panic!("expected an ATTRIB"),
        }
        match entities[2].specific {
            EntityType::Seqend(_) => (),
            _ => panic!("expected a SEQEND"),
        }
    }

    #[test]
    fn read_insert_with_embedded_attributes() {
        let file = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "INSERT"),
                CodePair::new_i16(66, 1),       // includes attributes
                CodePair::new_str(0, "ATTRIB"), // these are embedded attributes tied to the `INSERT` entity
                CodePair::new_str(0, "ATTRIB"),
                CodePair::new_str(0, "SEQEND"), // this is an embedded `SEQEND` entity tied to the `INSERT` entity
            ],
        );
        let entities = file.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Insert(ref ins) => assert_eq!(2, ins.attributes().count()),
            _ => panic!("expected an INSERT"),
        }
    }

    #[test]
    fn write_insert_no_embedded_attributes() {
        let mut drawing = Drawing::new();
        let ins = Insert {
            name: "insert-name".to_string(),
            ..Default::default()
        };
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);
        assert_not_contains_pairs(
            &drawing,
            vec![
                CodePair::new_i16(66, 0),            // contains no attributes
                CodePair::new_str(2, "insert-name"), // sentinel to ensure we're reading at the correct location
            ],
        );
        assert_not_contains_pairs(&drawing, vec![CodePair::new_str(0, "SEQEND")]);
    }

    #[test]
    fn write_insert_with_embedded_attributes() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        let mut ins = Insert::default();
        ins.add_attribute(&mut drawing, Attribute::default());
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "INSERT")]);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(100, "AcDbBlockReference"),
                CodePair::new_i16(66, 1), // contains attributes
            ],
        );
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "ATTRIB")]);
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "SEQEND")]);
    }

    #[test]
    fn write_insert_no_extrusion_on_r11() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R11;
        let ins = Insert {
            extrusion_direction: Vector::new(1.0, 2.0, 3.0),
            ..Default::default()
        };
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);
        assert_not_contains_pairs(
            &drawing,
            vec![
                CodePair::new_f64(210, 1.0),
                CodePair::new_f64(220, 2.0),
                CodePair::new_f64(230, 3.0),
            ],
        );
    }

    #[test]
    fn write_insert_extrusion_on_r12() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R12;
        let ins = Insert {
            extrusion_direction: Vector::new(1.0, 2.0, 3.0),
            ..Default::default()
        };
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_f64(210, 1.0),
                CodePair::new_f64(220, 2.0),
                CodePair::new_f64(230, 3.0),
            ],
        );
    }

    #[test]
    fn write_insert_extrusion_on_r13() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13;
        let ins = Insert {
            extrusion_direction: Vector::new(1.0, 2.0, 3.0),
            ..Default::default()
        };
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_f64(210, 1.0),
                CodePair::new_f64(220, 2.0),
                CodePair::new_f64(230, 3.0),
            ],
        );
    }

    #[test]
    fn round_trip_insert_with_attributes() {
        let mut drawing = Drawing::new();
        let mut ins = Insert::default();
        ins.add_attribute(&mut drawing, Attribute::default());
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);

        let drawing = drawing_from_pairs(drawing.code_pairs().unwrap());

        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Insert(ref ins) => assert_eq!(1, ins.attributes().count()),
            _ => panic!("expected an INSERT"),
        }
    }

    #[test]
    fn read_attribute_with_attached_mtext() {
        let file = from_section(
            "ENTITIES",
            vec![
                CodePair::new_str(0, "ATTRIB"),
                CodePair::new_str(0, "MTEXT"),
                CodePair::new_str(1, "m_text"),
            ],
        );
        let entities = file.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Attribute(ref att) => assert_eq!("m_text", att.m_text.text),
            _ => panic!("expected an attribute"),
        }
    }

    #[test]
    fn write_attribute_with_attached_mtext() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13; // MTEXT is only written on R13+
        drawing.add_entity(Entity::new(EntityType::Attribute(Default::default())));
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "ATTRIB")]);
        assert_contains_pairs(&drawing, vec![CodePair::new_str(0, "MTEXT")]);
    }

    #[test]
    fn round_trip_attribute_with_attached_mtext() {
        let att = Attribute {
            m_text: MText {
                text: String::from("m_text"),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13; // MTEXT is only written on R13+
        drawing.add_entity(Entity::new(EntityType::Attribute(att)));

        let drawing = drawing_from_pairs(drawing.code_pairs().unwrap());

        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Attribute(ref att) => assert_eq!("m_text", att.m_text.text),
            _ => panic!("expected a attribute"),
        }
    }

    #[test]
    fn read_extension_data() {
        let ent = read_entity(
            "LINE",
            vec![
                CodePair::new_str(102, "{IXMILIA"),
                CodePair::new_str(1, "some string"),
                CodePair::new_str(102, "}"),
            ],
        );
        assert_eq!(1, ent.common.extension_data_groups.len());
        let group = &ent.common.extension_data_groups[0];
        assert_eq!("IXMILIA", group.application_name);
        match group.items[0] {
            ExtensionGroupItem::CodePair(ref p) => {
                assert_eq!(&CodePair::new_str(1, "some string"), p)
            }
            _ => panic!("expected a code pair"),
        }
    }

    #[test]
    fn write_extension_data() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14;
        drawing.add_entity(Entity {
            common: EntityCommon {
                extension_data_groups: vec![ExtensionGroup {
                    application_name: String::from("IXMILIA"),
                    items: vec![ExtensionGroupItem::CodePair(CodePair::new_str(
                        1,
                        "some string",
                    ))],
                }],
                ..Default::default()
            },
            specific: EntityType::Line(Line::default()),
        });
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
    fn read_x_data() {
        let ent = read_entity(
            "LINE",
            vec![
                CodePair::new_str(1001, "IXMILIA"),
                CodePair::new_str(1000, "some string"),
            ],
        );
        assert_eq!(1, ent.common.x_data.len());
        let x = &ent.common.x_data[0];
        assert_eq!("IXMILIA", x.application_name);
        match x.items[0] {
            XDataItem::Str(ref s) => assert_eq!("some string", s),
            _ => panic!("expected a string"),
        }
    }

    #[test]
    fn read_multiple_x_data() {
        let ent = read_entity(
            "POLYLINE",
            vec![
                CodePair::new_str(1001, "Alpha"),
                CodePair::new_str(1000, "a"),
                CodePair::new_str(1001, "Beta"),
                CodePair::new_str(1000, "b"),
                CodePair::new_str(1001, "Gamma"),
                CodePair::new_str(1000, "c"),
            ],
        );
        // dbg!(&ent);
        assert_eq!(ent.common.x_data.len(), 3);
        for (i, x) in ent.common.x_data.iter().enumerate() {
            let (name, val) = match i {
                0 => ("Alpha", "a"),
                1 => ("Beta", "b"),
                2 => ("Gamma", "c"),
                _ => panic!("should only have 3 items"),
            };

            assert_eq!(x.application_name, name);
            match x.items[0] {
                XDataItem::Str(ref a) => assert_eq!(a, val),
                _ => panic!("Expected a string"),
            }
        }
    }

    #[test]
    fn write_x_data() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_entity(Entity {
            common: EntityCommon {
                x_data: vec![XData {
                    application_name: String::from("IXMILIA"),
                    items: vec![XDataItem::Real(1.1)],
                }],
                ..Default::default()
            },
            specific: EntityType::Line(Line::default()),
        });
        assert_contains_pairs(
            &drawing,
            vec![
                CodePair::new_str(1001, "IXMILIA"),
                CodePair::new_f64(1040, 1.1),
                CodePair::new_str(0, "ENDSEC"), // xdata is written after all the entity's other code pairs
            ],
        );
    }

    #[test]
    fn read_entity_after_extension_data() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "ENTITIES"),
            CodePair::new_str(0, "LINE"),
            CodePair::new_str(102, "{IXMILIA"),
            CodePair::new_str(102, "}"),
            CodePair::new_str(0, "CIRCLE"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
        match entities[1].specific {
            EntityType::Circle(_) => (),
            _ => panic!("expected a circle"),
        }
    }

    #[test]
    fn read_entity_after_x_data() {
        let drawing = drawing_from_pairs(vec![
            CodePair::new_str(0, "SECTION"),
            CodePair::new_str(2, "ENTITIES"),
            CodePair::new_str(0, "LINE"),
            CodePair::new_str(1001, "IXMILIA"),
            CodePair::new_str(0, "CIRCLE"),
            CodePair::new_str(0, "ENDSEC"),
            CodePair::new_str(0, "EOF"),
        ]);
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
        match entities[1].specific {
            EntityType::Circle(_) => (),
            _ => panic!("expected a circle"),
        }
    }

    #[test]
    fn read_all_types() {
        for (type_string, subclass, expected_type, _) in all_types::all_entity_types() {
            println!("parsing {}/{}", type_string, subclass);
            let mut ent = read_entity(
                type_string,
                vec![
                    CodePair::new_str(100, subclass),
                    CodePair::new_str(102, "{IXMILIA"), // read extension data
                    CodePair::new_str(1, "some string"),
                    CodePair::new_str(102, "}"),
                    CodePair::new_str(1001, "IXMILIA"), // read x data
                    CodePair::new_f64(1040, 1.1),
                ],
            );

            // internal seqend handles might differ; patch them
            match ent.specific {
                EntityType::Insert(ref mut i) => {
                    i.__seqend_handle = Handle::empty();
                }
                EntityType::Polyline(ref mut p) => {
                    p.__seqend_handle = Handle::empty();
                }
                _ => {}
            }

            // validate specific
            assert_eq!(expected_type, ent.specific);

            // validate extension data
            assert_eq!(1, ent.common.extension_data_groups.len());
            assert_eq!(
                "IXMILIA",
                ent.common.extension_data_groups[0].application_name
            );
            assert_eq!(1, ent.common.extension_data_groups[0].items.len());
            assert_eq!(
                ExtensionGroupItem::CodePair(CodePair::new_str(1, "some string")),
                ent.common.extension_data_groups[0].items[0]
            );

            // validate x data
            assert_eq!(1, ent.common.x_data.len());
            assert_eq!("IXMILIA", ent.common.x_data[0].application_name);
            assert_eq!(1, ent.common.x_data[0].items.len());
            assert_eq!(XDataItem::Real(1.1), ent.common.x_data[0].items[0]);
        }
    }

    #[test]
    fn write_all_types() {
        for (type_string, _, expected_type, max_version) in all_types::all_entity_types() {
            println!("writing {}", type_string);
            let mut common = EntityCommon::default();
            common.extension_data_groups.push(ExtensionGroup {
                application_name: String::from("IXMILIA"),
                items: vec![ExtensionGroupItem::CodePair(CodePair::new_str(
                    1,
                    "some string",
                ))],
            });
            common.x_data.push(XData {
                application_name: String::from("IXMILIA"),
                items: vec![XDataItem::Real(1.1)],
            });
            let mut drawing = Drawing::new();
            drawing.header.version = max_version;
            drawing.add_entity(Entity {
                common,
                specific: expected_type,
            });
            // 3DLINE writes as a LINE
            let type_string = if type_string == "3DLINE" {
                "LINE"
            } else {
                type_string
            };
            assert_contains_pairs(&drawing, vec![CodePair::new_str(0, type_string)]);
            if max_version >= AcadVersion::R14 {
                // only written on R14+
                assert_contains_pairs(
                    &drawing,
                    vec![
                        CodePair::new_str(102, "{IXMILIA"),
                        CodePair::new_str(1, "some string"),
                        CodePair::new_str(102, "}"),
                    ],
                );
            }
            if max_version >= AcadVersion::R2000 {
                // only written on R2000+
                assert_contains_pairs(
                    &drawing,
                    vec![
                        CodePair::new_str(1001, "IXMILIA"),
                        CodePair::new_f64(1040, 1.1),
                    ],
                );
            }
        }
    }

    #[test]
    fn normalize_mline_styles() {
        let mut file = Drawing::new();
        file.clear();
        let objects = file.objects().collect::<Vec<_>>();
        assert_eq!(0, objects.len());
        let mline = MLine {
            style_name: "style name".to_string(),
            ..Default::default()
        };
        file.add_entity(Entity::new(EntityType::MLine(mline)));
        file.normalize();
        let objects = file.objects().collect::<Vec<_>>();
        assert_eq!(2, objects.len());
        match objects[0].specific {
            ObjectType::MLineStyle(ref ml) => assert_eq!("style name", ml.style_name),
            _ => panic!("expected an mline style"),
        }
    }

    #[test]
    fn normalize_dimension_styles() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.dim_styles().count());
        file.add_entity(Entity::new(EntityType::RadialDimension(RadialDimension {
            dimension_base: DimensionBase {
                dimension_style_name: String::from("style name"),
                ..Default::default()
            },
            ..Default::default()
        })));
        file.normalize();
        let dim_styles = file.dim_styles().collect::<Vec<_>>();
        assert_eq!(3, dim_styles.len());
        assert_eq!("ANNOTATIVE", dim_styles[0].name);
        assert_eq!("STANDARD", dim_styles[1].name);
        assert_eq!("style name", dim_styles[2].name);
    }
}
