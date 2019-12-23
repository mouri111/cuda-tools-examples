use clap::{App, Arg};

fn main() {
    let matches = App::new("ray-tracing")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ray-per-pixel")
                .short("r")
                .long("ray-per-pixel")
                .takes_value(true),
        )
        .get_matches();
    let default_seed = 0;
    let seed = matches
        .value_of("seed")
        .and_then(|seed| seed.parse::<u32>().ok())
        .unwrap_or(default_seed);
    let default_height = 400;
    let height = matches
        .value_of("height")
        .and_then(|seed| seed.parse::<usize>().ok())
        .unwrap_or(default_height);
    let default_width = 600;
    let width = matches
        .value_of("width")
        .and_then(|seed| seed.parse::<usize>().ok())
        .unwrap_or(default_width);
    let default_ray_per_pixel = 128;
    let ray_per_pixel = matches
        .value_of("ray-per-pixel")
        .and_then(|seed| seed.parse::<usize>().ok())
        .unwrap_or(default_ray_per_pixel);
    ray_tracing::run(seed, height, width, ray_per_pixel);
}
