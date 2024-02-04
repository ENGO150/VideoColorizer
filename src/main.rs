use std::
{
    ffi::OsStr, fs::
    {
        copy, read_dir, remove_file
    }, io::
    {
        stdin, stdout, Write
    }, path::Path, process::Command, thread::sleep, time::Duration
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
    let mut original_path = String::new();

    loop //ASK FOR ORIGINAL
    {
        print!("Please enter full path to your MP4 video.\n>>> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut original_path).expect("Reading failed.");
        original_path = original_path.trim().to_string();

        if original_path.ends_with(".mp4") && Path::new(&original_path).exists() { break; }

        original_path.clear();
    }

    copy(original_path, "./out/original.mp4").expect("Copying file failed."); //COPY lmao

    println!("\nGET READY FOR SHITLOAD OF OUTPUT IN 5 SECS!\n"); //NEWLINE

    sleep(Duration::from_secs(5));

    Command::new("ffmpeg") //RUN FFMPEG
        .arg("-i")
        .arg("./out/original.mp4")
        .arg("-vf")
        .arg("fps=30")
        .arg("./out/frames_original/%10d.png")
        .spawn()
        .expect("Getting frames failed.")
        .wait()
        .unwrap();

    println!("\nFrames successfully extracted. Starting distortion in 5 secs.\n");
    sleep(Duration::from_secs(5));

    return;

    //COLORIZE FRAMES
    let mut n = 0;
    for path in read_dir("./out/frames_original").unwrap() //COUNT FRAMES
    {
        if path.unwrap().path().extension() == Some(OsStr::new("png")) { n += 1; } //THIS MAY BE DUMB AF BUT WELL
    }

    for i in 1..(n + 1u128) //LOOP TROUGH FRAMES
    {
        let filename = format!("{:10}.png", i);
        println!("Processing frame {:10}/{:10}", i, n);

        colorize_img(&format!("./out/frames_original/{}", filename), &format!("./out/frames_new/{}", filename)); //COLORIZE
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

        let px = Rgba([(((buffer - distort(red) ) as i32 - 19).abs() / 2) as u8, (((buffer - distort(green)) as i32 - 70).abs() / 2) as u8, (((buffer - distort(blue)) as i32 - 22).abs() / 2) as u8, alpha]); //COMPLETELY FUCK THE COLORS

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