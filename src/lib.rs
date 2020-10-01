use std::cmp;

const CPF_LENGTH: usize = 11;

pub fn reserved_numbers() -> Vec<String> {
    vec![
        String::from("00000000000"),
        String::from("11111111111"),
        String::from("22222222222"),
        String::from("33333333333"),
        String::from("44444444444"),
        String::from("55555555555"),
        String::from("66666666666"),
        String::from("77777777777"),
        String::from("88888888888"),
        String::from("99999999999"),
    ]
}

pub fn format(cpf: &str) -> String {
    let cpf = cpf.matches(char::is_numeric).collect::<Vec<_>>().concat();

    if cpf.len() > 3 && cpf.len() <= 6 {
        format!("{}.{}", &cpf[0..3], &cpf[3..cpf.len()])
    } else if cpf.len() >= 7 && cpf.len() <= 9 {
        format!("{}.{}.{}", &cpf[0..3], &cpf[3..6], &cpf[6..cpf.len()])
    } else if cpf.len() >= 10 {
        let end_position = cmp::min(cpf.len(), CPF_LENGTH);
        format!(
            "{}.{}.{}-{}",
            &cpf[0..3],
            &cpf[3..6],
            &cpf[6..9],
            &cpf[9..end_position]
        )
    } else {
        cpf
    }
}

pub fn is_valid(cpf: &str) -> bool {
    if cpf.matches(char::is_lowercase).count() > 0 || cpf.matches(char::is_uppercase).count() > 0 {
        return false;
    }

    let cpf = cpf.matches(char::is_numeric).collect::<Vec<_>>().concat();

    let reserved_numbers = reserved_numbers();

    !cpf.is_empty() && cpf.len() == CPF_LENGTH && !reserved_numbers.contains(&cpf) && validate(cpf)
}

fn validate(cpf: String) -> bool {
    let cpf = cpf.matches(char::is_numeric).collect::<Vec<_>>();

    let sum = check_sum(&cpf, 10);

    let digit1: u32 = calc_digit(sum);

    let mut sum: u32 = check_sum(&cpf, 11);
    sum += digit1 * 2;
    let digit2: u32 = calc_digit(sum);

    cpf[9].parse::<u32>().unwrap() == digit1 && cpf[10].parse::<u32>().unwrap() == digit2
}

fn check_sum(cpf: &Vec<&str>, factor: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut factor: u32 = factor;
    for i in 0..9 {
        sum += cpf[i].parse::<u32>().unwrap() * factor;
        factor -= 1;
    }

    sum
}

fn calc_digit(sum: u32) -> u32 {
    let mod_sum1 = sum % 11;

    if mod_sum1 < 2 {
        0
    } else {
        11 - mod_sum1
    }
}
