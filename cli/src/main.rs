use clap::clap_app;

fn main() {
    let matches = clap_app!(jargonoc =>
        (version: "0.1.0")
        (@arg input: +required "Sets the input file to use")
    )
    .get_matches();

    // $ jargono myfile.jo
    let file = matches.value_of("input").unwrap();

    let contents = std::fs::read_to_string(file).expect(&*format!(
        "Unable to read {}: does it exist and do you have access to it?",
        file
    ));

    jargonoc::build(contents, "out.ll".to_owned());
    println!("{} compiled successfully.", file);
}
