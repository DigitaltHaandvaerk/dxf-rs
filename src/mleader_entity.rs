use crate::CodePair;

use crate::entities::*;
use crate::enums::*;
use crate::helper_functions::*;

impl Entity {
    pub(crate) fn add_custom_code_pairs_mleader(
        pairs: &mut Vec<CodePair>,
        mleader: &MLeader,
        version: AcadVersion,
    ) -> bool {
        if version >= AcadVersion::R13 {
            pairs.push(CodePair::new_str(100, "AcDbMLeader"));
        }

        add_mleader_common_properties(pairs, mleader);
        add_mleader_context_data(pairs, mleader);
        add_mleader_leader_data(pairs, mleader);

        true
    }
}

fn add_mleader_common_properties(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_i16(270, 2));
    if !mleader.leader_style_id.is_empty() {
        pairs.push(CodePair::new_str(340, &mleader.leader_style_id));
    }
    pairs.push(CodePair::new_i32(90, mleader.property_override_flag));
    pairs.push(CodePair::new_i16(170, mleader.leader_line_type as i16));
    pairs.push(CodePair::new_i32(91, mleader.leader_line_color));

    if !mleader.leader_line_type_id.is_empty() {
        pairs.push(CodePair::new_str(341, &mleader.leader_line_type_id));
    }

    pairs.push(CodePair::new_i16(171, mleader.leader_line_weight));
    pairs.push(CodePair::new_i16(290, as_i16(mleader.enable_landing)));
    pairs.push(CodePair::new_i16(291, as_i16(mleader.enable_dogleg)));
    pairs.push(CodePair::new_f64(41, mleader.dogleg_length));

    if !mleader.arrowhead_id.is_empty() {
        pairs.push(CodePair::new_str(342, &mleader.arrowhead_id));
    }

    pairs.push(CodePair::new_f64(42, mleader.arrowhead_size));
    pairs.push(CodePair::new_i16(172, mleader.content_type as i16));

    if !mleader.text_style_id.is_empty() {
        pairs.push(CodePair::new_str(343, &mleader.text_style_id));
    }

    pairs.push(CodePair::new_i16(173, mleader.text_left_attachment_type));
    pairs.push(CodePair::new_i16(95, mleader.text_right_attachment_type));
    pairs.push(CodePair::new_i16(174, mleader.text_angle_type));
    pairs.push(CodePair::new_i16(175, mleader.text_alignment_type));
    pairs.push(CodePair::new_i32(92, mleader.text_color));
    pairs.push(CodePair::new_i16(292, as_i16(mleader.enable_frame_text)));
    // if !mleader.block_content_id.is_empty() {
    //     pairs.push(CodePair::new_str(344, &mleader.block_content_id));
    // }
    // pairs.push(CodePair::new_i32(93, mleader.block_content_color));
    // pairs.push(CodePair::new_f64(10, mleader.block_content_scale));
    // pairs.push(CodePair::new_f64(43, mleader.block_content_rotation));
    // pairs.push(CodePair::new_i16(
    // 176,
    // mleader.block_content_connection_type,
    // ));
    pairs.push(CodePair::new_i16(
        293,
        as_i16(mleader.enable_annotation_scale),
    ));
    pairs.push(CodePair::new_i32(94, mleader.arrowhead_index));

    if !mleader.arrowhead_id_ref.is_empty() {
        pairs.push(CodePair::new_str(345, &mleader.arrowhead_id_ref));
    }

    // if !mleader.block_attribute_id.is_empty() {
    // pairs.push(CodePair::new_str(330, &mleader.block_attribute_id));
    // }
    // pairs.push(CodePair::new_i16(177, mleader.block_attribute_index));
    // pairs.push(CodePair::new_f64(44, mleader.block_attribute_width));
    // if !mleader.block_attribute_text_string.is_empty() {
    // pairs.push(CodePair::new_str(302, &mleader.block_attribute_text_string));
    // }
    pairs.push(CodePair::new_i16(
        294,
        as_i16(mleader.text_direction_negative),
    ));
    pairs.push(CodePair::new_i16(178, mleader.text_align_in_ipe));
    pairs.push(CodePair::new_i16(179, mleader.text_attachment_point));
    pairs.push(CodePair::new_i16(271, mleader.text_attachment_direction));
    pairs.push(CodePair::new_i16(
        272,
        mleader.bottom_text_attachment_direction,
    ));
    pairs.push(CodePair::new_i16(
        273,
        mleader.top_text_attachment_direction,
    ));
}

fn add_mleader_context_data(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_str(300, "CONTEXT_DATA{"));

    add_mleader_content_properties(pairs, mleader);
    add_mleader_text_properties(pairs, mleader);

    // pairs.push(CodePair::new_f64(10, mleader.vertex.x));
    // pairs.push(CodePair::new_f64(20, mleader.vertex.y));
    // pairs.push(CodePair::new_f64(30, mleader.vertex.z));
    pairs.push(CodePair::new_i32(90, mleader.break_point_index));
}

fn add_mleader_content_properties(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_f64(40, mleader.content_scale));
    pairs.push(CodePair::new_f64(10, mleader.content_base_point.x));
    pairs.push(CodePair::new_f64(20, mleader.content_base_point.y));
    pairs.push(CodePair::new_f64(30, mleader.content_base_point.z));
    pairs.push(CodePair::new_f64(41, mleader.text_height));
    pairs.push(CodePair::new_f64(140, mleader.arrow_head_size));
    pairs.push(CodePair::new_f64(145, mleader.landing_gap));
    pairs.push(CodePair::new_i16(290, as_i16(mleader.has_m_text)));
    pairs.push(CodePair::new_str(304, &mleader.default_text_contents));
    pairs.push(CodePair::new_f64(11, mleader.text_normal_direction.x));
    pairs.push(CodePair::new_f64(21, mleader.text_normal_direction.y));
    pairs.push(CodePair::new_f64(31, mleader.text_normal_direction.z));

    if !mleader.text_style_id_context.is_empty() {
        pairs.push(CodePair::new_str(340, &mleader.text_style_id_context));
    }
}

fn add_mleader_text_properties(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_f64(12, mleader.text_location.x));
    pairs.push(CodePair::new_f64(22, mleader.text_location.y));
    pairs.push(CodePair::new_f64(32, mleader.text_location.z));
    pairs.push(CodePair::new_f64(13, mleader.text_direction.x));
    pairs.push(CodePair::new_f64(23, mleader.text_direction.y));
    pairs.push(CodePair::new_f64(33, mleader.text_direction.z));
    pairs.push(CodePair::new_f64(42, mleader.text_rotation));
    pairs.push(CodePair::new_f64(43, mleader.text_width));
    pairs.push(CodePair::new_f64(44, mleader.text_height_context));
    pairs.push(CodePair::new_f64(45, mleader.text_line_spacing_factor));
    pairs.push(CodePair::new_i16(170, mleader.text_line_spacing_style));
    pairs.push(CodePair::new_i32(90, mleader.text_color_context));
    pairs.push(CodePair::new_i16(171, mleader.text_attachment));
    pairs.push(CodePair::new_i16(172, mleader.text_flow_direction));
    pairs.push(CodePair::new_i32(91, mleader.text_background_color));
    pairs.push(CodePair::new_f64(141, mleader.text_background_scale_factor));
    pairs.push(CodePair::new_i32(92, mleader.text_background_transparency));
    pairs.push(CodePair::new_i16(
        291,
        as_i16(mleader.is_text_background_color_on),
    ));
    pairs.push(CodePair::new_i16(
        292,
        as_i16(mleader.is_text_background_fill_on),
    ));
    pairs.push(CodePair::new_i16(173, mleader.text_column_type));
    pairs.push(CodePair::new_i16(293, as_i16(mleader.use_text_auto_height)));
    pairs.push(CodePair::new_f64(142, mleader.text_column_width));
    pairs.push(CodePair::new_f64(143, mleader.text_column_gutter_width));
    pairs.push(CodePair::new_i16(
        294,
        as_i16(mleader.text_column_flow_reversed),
    ));
    pairs.push(CodePair::new_f64(144, mleader.text_column_height));
    pairs.push(CodePair::new_i16(295, as_i16(mleader.text_use_word_break)));
    pairs.push(CodePair::new_i16(296, as_i16(mleader.has_block)));
    // if !mleader.block_content_id_context.is_empty() {
    // pairs.push(CodePair::new_str(341, &mleader.block_content_id_context));
    // }
    // pairs.push(CodePair::new_f64(
    // 14,
    // mleader.block_content_normal_direction.x,
    // ));
    // pairs.push(CodePair::new_f64(
    // 24,
    // mleader.block_content_normal_direction.y,
    // ));
    // pairs.push(CodePair::new_f64(
    // 34,
    // mleader.block_content_normal_direction.z,
    // ));
    // pairs.push(CodePair::new_f64(15, mleader.block_content_position.x));
    // pairs.push(CodePair::new_f64(25, mleader.block_content_position.y));
    // pairs.push(CodePair::new_f64(35, mleader.block_content_position.z));
    // pairs.push(CodePair::new_f64(16, mleader.block_content_scale_context));
    // pairs.push(CodePair::new_f64(
    // 46,
    // mleader.block_content_rotation_context,
    // ));
    // pairs.push(CodePair::new_i32(93, mleader.block_content_color_context));
    // pairs.push(CodePair::new_f64(47, mleader.block_transformation_matrix));

    // MLeader plane origin point
    // pairs.push(CodePair::new_f64(110, mleader.mleader_plane_origin_point.x));
    // pairs.push(CodePair::new_f64(
    //     120,
    //     mleader.mleader_plane_origin_point.y,
    // ));
    // pairs.push(CodePair::new_f64(130, mleader.mleader_plane_origin_point.z));

    // MLeader plane x and y axis direction
    // pairs.push(CodePair::new_f64(
    //     111,
    //     mleader.mleader_plane_x_axis_direction.x,
    // ));
    // pairs.push(CodePair::new_f64(
    //     112,
    //     mleader.mleader_plane_y_axis_direction.x,
    // ));
    // pairs.push(CodePair::new_i16(
    //     297,
    //     as_i16(mleader.mleader_plane_normal_reversed),
    // ));
}

fn add_mleader_leader_data(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_str(302, "LEADER{"));

    add_mleader_leader_node(pairs, mleader);
    add_mleader_leader_lines(pairs, mleader);

    // Close sections
    pairs.push(CodePair::new_str(305, "}"));
    pairs.push(CodePair::new_str(303, "}"));
    pairs.push(CodePair::new_str(301, "}"));
}

fn add_mleader_leader_node(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_i16(
        290,
        as_i16(mleader.has_set_last_leader_line_point),
    ));
    pairs.push(CodePair::new_i16(
        291,
        as_i16(mleader.has_set_dogleg_vector),
    ));
    pairs.push(CodePair::new_f64(10, mleader.last_leader_line_point.x));
    pairs.push(CodePair::new_f64(20, mleader.last_leader_line_point.y));
    pairs.push(CodePair::new_f64(30, mleader.last_leader_line_point.z));
    pairs.push(CodePair::new_f64(11, mleader.dogleg_vector.x));
    pairs.push(CodePair::new_f64(21, mleader.dogleg_vector.y));
    pairs.push(CodePair::new_f64(31, mleader.dogleg_vector.z));
    // pairs.push(CodePair::new_f64(12, mleader.break_start_point.x));
    // pairs.push(CodePair::new_f64(22, mleader.break_start_point.y));
    // pairs.push(CodePair::new_f64(32, mleader.break_start_point.z));
    // pairs.push(CodePair::new_f64(13, mleader.break_end_point.x));
    // pairs.push(CodePair::new_f64(23, mleader.break_end_point.y));
    // pairs.push(CodePair::new_f64(33, mleader.break_end_point.z));
    pairs.push(CodePair::new_i32(90, mleader.leader_branch_index));
    pairs.push(CodePair::new_f64(40, mleader.dogleg_length_leader));
}

fn add_mleader_leader_lines(pairs: &mut Vec<CodePair>, mleader: &MLeader) {
    pairs.push(CodePair::new_str(304, "LEADER_LINE{"));

    for vertex in &mleader.vertices {
        pairs.push(CodePair::new_f64(10, vertex.x));
        pairs.push(CodePair::new_f64(20, vertex.y));
        pairs.push(CodePair::new_f64(30, vertex.z));
    }
    // pairs.push(CodePair::new_i32(90, mleader.break_point_index_line));
    // pairs.push(CodePair::new_f64(11, mleader.break_start_point_line.x));
    // pairs.push(CodePair::new_f64(21, mleader.break_start_point_line.y));
    // pairs.push(CodePair::new_f64(31, mleader.break_start_point_line.z));
    // pairs.push(CodePair::new_f64(12, mleader.break_end_point_line.x));
    // pairs.push(CodePair::new_f64(22, mleader.break_end_point_line.y));
    // pairs.push(CodePair::new_f64(32, mleader.break_end_point_line.z));

    pairs.push(CodePair::new_i32(91, mleader.leader_line_index));
}
