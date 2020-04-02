use cpf_util as cpf;

#[test]
fn should_format_cpf_empty() {
    assert_eq!(cpf::format(""), "");
}

#[test]
fn should_format_cpf_with_one_number() {
    assert_eq!(cpf::format("9"), "9");
}

#[test]
fn should_format_cpf_with_two_numbers() {
    assert_eq!(cpf::format("93"), "93");
}

#[test]
fn should_format_cpf_with_three_numbers() {
    assert_eq!(cpf::format("943"), "943");
}

#[test]
fn should_format_cpf_with_four_numbers() {
    assert_eq!(cpf::format("9438"), "943.8");
}

#[test]
fn should_format_cpf_with_five_numbers() {
    assert_eq!(cpf::format("94389"), "943.89");
}

#[test]
fn should_format_cpf_with_six_numbers() {
    assert_eq!(cpf::format("943895"), "943.895");
}

#[test]
fn should_format_cpf_with_seven_numbers() {
    assert_eq!(cpf::format("9438957"), "943.895.7");
}

#[test]
fn should_format_cpf_with_eight_numbers() {
    assert_eq!(cpf::format("94389575"), "943.895.75");
}

#[test]
fn should_format_cpf_with_nine_numbers() {
    assert_eq!(cpf::format("943895751"), "943.895.751");
}

#[test]
fn should_format_cpf_with_ten_numbers() {
    assert_eq!(cpf::format("9438957510"), "943.895.751-0");
}

#[test]
fn should_format_cpf_with_eleven_numbers() {
    assert_eq!(cpf::format("94389575104"), "943.895.751-04");
}

#[test]
fn should_not_add_digits_after_the_cpf_length() {
    assert_eq!(cpf::format("94389575104000000"), "943.895.751-04");
}

#[test]
fn should_remove_all_non_numeric_characters() {
    assert_eq!(cpf::format("943.?ABC895.751-04abc"), "943.895.751-04");
}

#[test]
fn should_return_false_when_it_is_on_the_reserved_words() {
    for cpf_number in cpf::RESERVED_NUMBERS {
        assert_eq!(cpf::is_valid(cpf_number), false);
    }
}