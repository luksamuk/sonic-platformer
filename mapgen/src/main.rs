use anyhow::{bail, Result};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::env;

fn coord_to_tile(x: u32, y: u32, max_x: u32) -> u32 {
    (y * max_x) + x
}

fn main() -> Result<()> {
    // How to use:
    // mapgen small.png smallsize big.png bigsize
    // mapgen small.png big.png mode
    //    where mode = "8-16" or "16-128"
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        bail!("Not enough parameters");
    }

    let mode = &args[3];

    let smaller_tilesize;
    let bigger_tilesize;

    if mode == "8_16" {
        smaller_tilesize = 8;
        bigger_tilesize = 16;
    } else if mode == "16_128" {
        smaller_tilesize = 16;
        bigger_tilesize = 128;
    } else {
        bail!("Unknown mode {}", mode);
    }

    let smaller_image = &args[1];
    let bigger_image = &args[2];

    /*println!("Working dir: {:10?}", env::current_dir()?);

    println!(
        "Small: {} ({}x{})",
        smaller_image, smaller_tilesize, smaller_tilesize
    );
    println!(
        "Big: {} ({}x{})",
        bigger_image, bigger_tilesize, bigger_tilesize
    );*/

    // Load images
    let smaller_image = ImageReader::open(smaller_image)?.decode()?;
    let bigger_image = ImageReader::open(bigger_image)?.decode()?;

    let smaller_size = {
        let width = smaller_image.width();
        let height = smaller_image.height();
        (width / smaller_tilesize, height / smaller_tilesize)
    };

    let bigger_size = {
        let width = bigger_image.width();
        let height = bigger_image.height();
        (width / bigger_tilesize, height / bigger_tilesize)
    };

    let small_tiles_in_bigger = bigger_tilesize / smaller_tilesize;

    println!("[");
    for j_big in 0..bigger_size.1 {
        for i_big in 0..bigger_size.0 {
            println!("    {{");
            print!("        \"tiles\": [");
            // Bigger tile
            let big_view = bigger_image.view(
                i_big * bigger_tilesize,
                j_big * bigger_tilesize,
                bigger_tilesize,
                bigger_tilesize,
            );
            let big_view = big_view.to_image();
            // Iterate over smaller views in bigger tile
            for j_big_s in 0..small_tiles_in_bigger {
                for i_big_s in 0..small_tiles_in_bigger {
                    let curr_tile = big_view.view(
                        i_big_s * smaller_tilesize,
                        j_big_s * smaller_tilesize,
                        smaller_tilesize,
                        smaller_tilesize,
                    );
                    let curr_tile = curr_tile.to_image();
                    let mut found = false;
                    //  // Iterate over views in smaller tiles
                    'small_loop: for j_small in 0..smaller_size.1 {
                        for i_small in 0..smaller_size.0 {
                            let small_view = smaller_image.view(
                                i_small * smaller_tilesize,
                                j_small * smaller_tilesize,
                                smaller_tilesize,
                                smaller_tilesize,
                            );
                            let small_view = small_view.to_image();
                            //let small_view_h = image::imageops::flip_horizontal(&small_view);
                            //let small_view_v = image::imageops::flip_vertical(&small_view);
                            //let small_view_hv = image::imageops::flip_vertical(&small_view_h);

                            if curr_tile == small_view {
                                print!("{}", coord_to_tile(i_small, j_small, smaller_size.0));

                                if !((j_big_s == small_tiles_in_bigger - 1)
                                    && (i_big_s == small_tiles_in_bigger - 1))
                                {
                                    print!(", ");
                                }

                                found = true;
                                break 'small_loop;
                            } /*else if curr_tile == small_view_h {
                                  print!("{}(h) ", coord_to_tile(i_small, j_small, smaller_size.0));
                                  found = true;
                                  break 'small_loop;
                              } else if curr_tile == small_view_v {
                                  print!("{}(v) ", coord_to_tile(i_small, j_small, smaller_size.0));
                                  found = true;
                                  break 'small_loop;
                              } else if curr_tile == small_view_hv {
                                  print!("{}(hv) ", coord_to_tile(i_small, j_small, smaller_size.0));
                                  found = true;
                                  break 'small_loop;
                              }*/
                        }
                    }
                    if !found {
                        panic!(
                            "Tile not found! Big piece index: {}",
                            coord_to_tile(i_big, j_big, small_tiles_in_bigger)
                        );
                    }
                }
            }
            println!("],");

            if mode == "8_16" {
                println!("        \"heightmask\": 0,");
                println!("        \"angle\": 0.0");
            }

            print!("    }}");
            if !((j_big == bigger_size.1 - 1) && (i_big == bigger_size.0 - 1)) {
                print!(",");
            }
            println!();
        }
    }

    println!("]");

    Ok(())
}
