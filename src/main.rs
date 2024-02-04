use std::
{
    fs::remove_file,
    path::Path,
};

use image::
{
    GenericImageView,
    ImageBuffer,
    Pixel,
    Rgba,
};

use rand::
{
    thread_rng,
    Rng,
};

fn main()
{
    for i in 1..106 //LOOP TROUGH FRAMES
    {
        let filename = format!("thumb{:04}.png", i);
        println!("{}", i);

        colorize_img(&format!("./coze/{}", filename), &format!("./out/{}", filename)); //COLORIZE
    }
}

pub fn colorize_img(path_in: &str, path_out: &str)
{
    if Path::new(path_out).exists() { remove_file(path_out).expect("File deletion failed!"); }
    let mut img = image::open(path_in).expect("File not found!");

    let (w, h) = img.dimensions();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    let mut negative: bool;

    img = img.blur(1.5);

    for (x, y, pixel) in img.pixels()
    {
        let Rgba([red, green, blue, alpha]) = pixel;

        negative = thread_rng().gen_range(0..(img.width() * img.height() / 480)) == 69;

        let px = Rgba([((distort(red, negative) as i32 - 19).abs() / 2) as u8, ((distort(green, negative) as i32 - 70).abs() / 2) as u8, ((distort(blue, negative) as i32 - 22).abs() / 2) as u8, alpha]);

        output.put_pixel
        (
            x,
            y,
            px.map(|p| 255 - ((p.saturating_pow(4) as f64) / 2.) as u8),
        );
    }

    output.save(path_out).expect("Saving failed!");
}

fn distort(p: u8, negative: bool) -> u8
{
    let mut returning = if p != 0
    {
        (thread_rng().gen_range((p as f64 * 70.)..(p as f64 * 140. + 1.)) / 100.).floor() as u8 % 255
    } else { 0 };

    returning = (returning as f32 * 1./4.) as u8;

    if negative
    {
        returning = 255 - returning;
    }

    return returning;
}