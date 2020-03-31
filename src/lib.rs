pub fn format(cpf: &str) -> String { 

    let cpf = cpf.matches(char::is_numeric)
        .collect::<Vec<_>>()
        .concat();

    if cpf.len() > 3 && cpf.len() <= 6 {
        format!("{}.{}", &cpf[0..3], &cpf[3..cpf.len()])
    }
    else if cpf.len() >= 7 && cpf.len() <= 9 {
        format!("{}.{}.{}", &cpf[0..3], &cpf[3..6], &cpf[6..cpf.len()])
    }
    else if cpf.len() >= 10 && cpf.len() <= 11 {
        format!("{}.{}.{}-{}", &cpf[0..3], &cpf[3..6], &cpf[6..9], &cpf[9..cpf.len()])
    }
    else 
    {
        String::from(cpf)
    }
}
