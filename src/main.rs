use std::
{
    ffi::OsStr,
    path::Path,
    process::Command,
    thread::sleep,
    time::Duration,

    fs::
    {
        copy,
        read_dir,
        remove_file
    },

    io::
    {
        stdin,
        stdout,
        Write
    },
};

use image::
{
    GenericImageView,
    ImageBuffer,
    Rgba,
};

use rand::
{
    thread_rng,
    Rng,
};

fn main()
{
    //CLEAN FRAMES DIRECTORIES
    clean_pngs("./out/frames_original".to_string());
    clean_pngs("./out/frames_new".to_string());

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

    //COLORIZE FRAMES
    let mut n = 0;
    for path in read_dir("./out/frames_original").unwrap() //COUNT FRAMES
    {
        if path.unwrap().path().extension() == Some(OsStr::new("png")) { n += 1; } //THIS MAY BE DUMB AF BUT WELL
    }

    for i in 1..(n + 1u128) //LOOP TROUGH FRAMES
    {
        let filename = format!("{:010}.png", i);
        println!("Processing frame {i}/{n}");

        let original_frame = format!("./out/frames_original/{}", filename);

        colorize_img(&original_frame, &format!("./out/frames_new/{}", filename)); //COLORIZE
        remove_file(original_frame).expect("File deletion failed!"); //DELETE NOW UNUSED FILE
    }

    Command::new("ffmpeg") //RUN FFMPEG
        .arg("-framerate")
        .arg("30")
        .arg("-i")
        .arg("./out/frames_new/%10d.png")
        .arg("-i")
        .arg("./out/original.mp4")
        .arg("-c:a")
        .arg("copy")
        .arg("-c:v")
        .arg("libx264")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("./out/new.mp4")
        .spawn()
        .expect("Getting frames failed.")
        .wait()
        .unwrap();
}

fn clean_pngs(dir_path: String)
{
    for path in read_dir(dir_path).unwrap() //ITER
    {
        if path.as_ref().unwrap().path().extension() == Some(OsStr::new("png")) //PNG FOUND
        {
            remove_file(&path.unwrap().path()).expect("File deletion failed!");
        }
    }
}

pub fn colorize_img(path_in: &str, path_out: &str)
{
    let mut img = image::open(path_in).expect("File not found!"); //LOAD ORIGINAL

    let (w, h) = img.dimensions();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);
    let mut buffer: u16;

    img = img.blur(1.5); //BLUR THAT

    for (x, y, pixel) in img.pixels() //LOOP TROUGH PIXELS
    {
        let Rgba([red, green, blue, alpha]) = pixel;

        buffer = if thread_rng().gen_range(0..(img.width() * img.height() / 480)) == 69 { 255 } else { 0 };

        let mut px = Rgba([((distort(red) as i32 - (19 + buffer) as i32).abs() / 2) as u8, ((distort(green) as i32 - (70 + buffer) as i32).abs() / 2) as u8, ((distort(blue) as i32 - (22 + buffer) as i32).abs() / 2) as u8, alpha]); //COMPLETELY FUCK THE COLORS

        //LAST EFFECT
        for i in 0..3
        {
            px[i] = ((255 - ((px[i].saturating_pow(4) as f64) / 2.) as u8) as f32 * 1000./1375.) as u8;
        }

        output.put_pixel
        (
            x,
            y,
            px,
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