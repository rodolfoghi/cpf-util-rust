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
    for cpf_number in cpf::reserved_numbers() {
        assert!(!cpf::is_valid(&cpf_number));
    }
}

#[test]
fn should_return_false_when_is_a_empty_string() {
    assert!(!cpf::is_valid(""));
}

#[test]
fn should_return_false_when_dont_match_with_cpf_length() {
    assert!(!cpf::is_valid("123456"));
}

#[test]
fn should_return_false_when_contains_only_letters_or_special_characters() {
    assert!(!cpf::is_valid("abcabcabcde"));
}

#[test]
fn should_return_false_when_is_a_cpf_invalid() {
    assert!(!cpf::is_valid("11257245286"));
}

#[test]
fn should_return_false_when_is_a_cpf_invalid_tes_numbers_with_letters() {
    assert!(!cpf::is_valid("foo391.838.38test0-66"));
}

#[test]
fn should_return_true_when_is_cpf_valid_without_mask() {
    assert!(cpf::is_valid("40364478829"));
}

#[test]
fn should_return_true_when_is_cpf_valid_with_mask() {
    assert!(cpf::is_valid("962.718.458-60"));
}
