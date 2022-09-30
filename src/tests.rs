#[cfg(test)]
mod tests {
    use crate::rules::*;

    #[test]
    fn test_space_before_bracket() {
        let input = r#"Text{"#;
        let expected_output = r#"Text {"#;
        assert_eq!(space_before_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }
    #[test]
    fn test_if_movement() {
        let input = r#"
    if(abc)
    bcd
"#;
        let expected_output = r#"
    if (abc)
        bcd
"#;
        assert_eq!(if_movement(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }
    #[test]
    fn test_remove_empty_line_before_close_bracket() {
        let input = r#"Text {
text: "TODO"

}"#;
        let expected_output = r#"Text {
text: "TODO"
}"#;
        assert_eq!(remove_empty_line_before_close_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_connect_multiple_empty_lines_into_one() {
        let input = r#"
Text {}






Text {}
"#;
        let expected_output = r#"
Text {}

Text {}
"#;
        assert_eq!(connect_multiple_empty_lines_into_one(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_move_single_open_bracket() {
        let input = r#"Text
{
}"#;
        let expected_output = r#"Text {
}"#;
        assert_eq!(move_single_open_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"Text

{
}"#;
        let expected_output = r#"Text {
}"#;
        assert_eq!(move_single_open_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_remove_useless_spaces_around_colon() {
        let input = r#"property var roman   :    ABCD"#;
        let expected_output = r#"property var roman: ABCD"#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"property var roman   :    "ABCD    :    ABCD""#;
        let expected_output = r#"property var roman: "ABCD    :    ABCD""#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"text: "ABCD \" \" \", \":   \" \" \" \" \" \"""#;
        let expected_output = r#"text: "ABCD \" \" \", \":   \" \" \" \" \" \"""#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"property var roman:ABCD"#;
        let expected_output = r#"property var roman:ABCD"#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_move_elements_inside() {
        let input = r#"
Text {
Label {
Text {}
}
}
"#;
        let expected_output = r#"
Text {
    Label {
        Text {}
    }
}
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
Service(
Ser(
Serr()
)
)
"#;
        let expected_output = r#"
Service(
    Ser(
        Serr()
    )
)
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
Service(
)
"#;
        let expected_output = r#"
Service(
)
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
Service[
]
"#;
        let expected_output = r#"
Service[
]
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
Service[([{[
]}])]
"#;
        let expected_output = r#"
Service[([{[
]}])]
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
Service[([{[
]}])])]}]]}}]}
"#;
        let expected_output = r#"
Service[([{[
]}])])]}]]}}]}
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_remove_empty_space_on_end_of_line() {
        let input = r#"
        
Text {}         
                 
"#;
        let expected_output = r#"

Text {}

"#;
        assert_eq!(remove_empty_space_on_end_of_line(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_skip_start_end_empty_lines() {
        let input = r#"

Text {}

"#;
        let expected_output = r#"Text {}"#;
        assert_eq!(skip_start_end_empty_lines(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_reorganize_space_in_models() {
        let input = r#"
model: [{ "name": qsTr("Keksz"), "data": "Keksz" },
    { "name": qsTr("Static"), "data": "Static" },
    { "name": qsTr("Bs"), "data": "Bs" }]
abc
"#;
        let expected_output = r#"
model: [{ "name": qsTr("Keksz"), "data": "Keksz" },
        { "name": qsTr("Static"), "data": "Static" },
        { "name": qsTr("Bs"), "data": "Bs" }]
abc
"#;
        assert_eq!(reorganize_space_in_models(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
model: [modelData, BaseData[a] + " " + BaseData[b]] //TODO
Text {
}
"#;
        let expected_output = r#"
model: [modelData, BaseData[a] + " " + BaseData[b]] //TODO
Text {
}
"#;
        assert_eq!(reorganize_space_in_models(split_text_to_vector(input)), split_text_to_vector(expected_output));
        let input = r#"
states: [
    State {
    name:readyS
    }
]
"#;
        let expected_output = r#"
states: [
    State {
    name:readyS
    }
]
"#;
        assert_eq!(reorganize_space_in_models(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    // include tests generated by `build.rs`
    include!(concat!(env!("OUT_DIR"), "/test_conversion.rs"));
}
