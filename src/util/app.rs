use clap::{ArgMatches, App, Arg};
use anyhow::Context;

fn create_app() -> ArgMatches {
    App::new("packet capture application")
        .version("1.0.0")
        .author("sabaniki")
        .about("this is packet capture application written in Rust.")
        .arg(
            Arg::new("interface_name")
                //.about("select target interface")
                .value_name("interface_name")
                .index(1)
                .required(true)
        )
        .get_matches()
}

pub fn get_arg() -> Result<String, anyhow::Error> {
    let app = create_app();
    let interface_name = app.value_of("interface_name")
        .with_context(||"could not get the arg [interface_name]")?;
    Ok(interface_name.to_string())
}
