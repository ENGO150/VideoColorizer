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
    if Path::new(path_out).exists() { remove_file(path_out).expect("File deletion failed!"); } //REMOVE DUPLICATE FRAMES
    let mut img = image::open(path_in).expect("File not found!"); //LOAD ORIGINAL

    let (w, h) = img.dimensions();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    let mut buffer: u8;

    img = img.blur(1.5); //BLUR THAT

    for (x, y, pixel) in img.pixels() //LOOP TROUGH PIXELS
    {
        let Rgba([red, green, blue, alpha]) = pixel;

        buffer = if thread_rng().gen_range(0..(img.width() * img.height() / 480)) == 69 { 255 } else { 0 };

        let px = Rgba([(((buffer - distort(red) )as i32 - 19).abs() / 2) as u8, (((buffer - distort(green)) as i32 - 70).abs() / 2) as u8, (((buffer - distort(blue)) as i32 - 22).abs() / 2) as u8, alpha]); //COMPLETELY FUCK THE COLORS

        output.put_pixel
        (
            x,
            y,
            px.map(|p| 255 - ((p.saturating_pow(4) as f64) / 2.) as u8),
        ); //ADD TO OUTPUT
    }

    output.save(path_out).expect("Saving failed!"); //SAVE OUTPUT
}

fn distort(p: u8) -> u8 //FUCK THE IMAGE
{
    let mut returning = if p != 0
    {
        (thread_rng().gen_range((p as f64 * 70.)..(p as f64 * 140. + 1.)) / 100.).floor() as u8 % 255 //GENERATE RANDOM COLOR AROUND THE p
    } else { 0 };

    returning = (returning as f32 * 1./4.) as u8;

    return returning;
}