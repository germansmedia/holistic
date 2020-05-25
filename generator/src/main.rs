// Social Robotics Platform 04
// Desmond Germans, Ph.D
// generate huge set of faces for deep learning experiments

extern crate rand;
use rand::Rng;
use rand::seq::SliceRandom;

use std::fs;
use std::io::prelude::*;

mod math;
use math::*;

mod pixel;
use pixel::*;

mod image;
use image::*;

mod image_formats;
use image_formats::*;

mod context3d_xcb_glx_opengl45;
use context3d_xcb_glx_opengl45::*;

mod opengl45;
use opengl45::*;

mod face;
use face::*;

struct Context {
    _ctx: Context3D,
    framebuffer: Framebuffer<ARGB8>,
    skin: Skin,
    eye: Eye,
}

impl Context {
    pub fn new(size: usizev2) -> Context {
        let ctx = Context3D::new().expect("Unable to create 3D context.");
        let framebuffer = Framebuffer::<ARGB8>::new(4 * size).expect("Unable to create framebuffer object.");
        framebuffer.bind();
        Context {
            _ctx: ctx,
            framebuffer: framebuffer,
            skin: Skin::new(),
            eye: Eye::new(),
        }
    }
}

fn downsample4(image: Image<ARGB8>) -> Image<ARGB8> {
    let mut dst = Image::<ARGB8>::new(*image.size() / 4);
    for y in 0..image.size().y / 4 {
        for x in 0..image.size().x / 4 {
            let mut r = 0usize;
            let mut g = 0usize;
            let mut b = 0usize;
            for i in 0..4 {
                for k in 0..4 {
                    let pix = image.pixel(usizev2::new(x * 4 + k,y * 4 + i));
                    r += pix.r() as usize;
                    g += pix.g() as usize;
                    b += pix.b() as usize;
                }
            }
            r /= 16;
            g /= 16;
            b /= 16;
            *dst.pixel_mut(usizev2::new(x,y)) = ARGB8::new_rgb(r as u8,g as u8,b as u8);
        }
    }
    dst
}

struct Space {
    size: usizev2,
    projection_matrix: f32m4x4,
    pos: (f32v3,f32v3),
    head: (f32e,f32e),
    left_eye: (f32e,f32e),
    right_eye: (f32e,f32e),
    light: (f32e,f32e),
    background_color: (f32k,f32k),
    light_color: (f32k,f32k),
    ambient_color: f32k,
    skin_color: (f32k,f32k),
    sclera_color: f32k,
    iris_color: (f32k,f32k),
}

struct Instance {
    projection_matrix: f32m4x4,
    pos: f32v3,
    head: f32e,
    left_eye: f32e,
    right_eye: f32e,
    light: f32e,
    background_color: f32k,
    light_color: f32k,
    ambient_color: f32k,
    skin_color: f32k,
    sclera_color: f32k,
    iris_color: f32k,
}

const LEFT_EYE_POS: f32v3 = f32v3 { x: -0.031,y: 0.026,z: 0.023, };
const RIGHT_EYE_POS: f32v3 = f32v3 { x: 0.031,y: 0.026,z: 0.023, };
const EYE_SIZE: f32v3 = f32v3 { x: 0.0115,y: 0.0115,z: 0.0115, };

fn render_full(rng: &mut rand::rngs::ThreadRng,ctx: &Context,backgrounds: &Vec<Image<ARGB8>>,instance: &Instance) -> Image<ARGB8> {

    let light_matrix = f32m3x3::yaw(instance.light.y) * f32m3x3::pitch(instance.light.p) * f32m3x3::roll(instance.light.b);
    let light_dir = light_matrix * f32v3::new(0.0,1.0,0.0);
    let head_matrix = f32m4x4::translate(instance.pos) * f32m4x4::yaw(instance.head.y) * f32m4x4::pitch(instance.head.p);
    let left_eye_matrix = f32m4x4::translate(LEFT_EYE_POS) * f32m4x4::yaw(instance.left_eye.y) * f32m4x4::pitch(instance.left_eye.p) * f32m4x4::scale(EYE_SIZE);
    let right_eye_matrix = f32m4x4::translate(RIGHT_EYE_POS) * f32m4x4::yaw(instance.right_eye.y) * f32m4x4::pitch(instance.right_eye.p) * f32m4x4::scale(EYE_SIZE);

    ctx.framebuffer.bind();
    unsafe {
        gl::ClearDepth(1.0);
        gl::Clear(gl::DEPTH_BUFFER_BIT);
    }
    ctx.framebuffer.unbind();

    let mut background = backgrounds.choose(rng).expect("huh?");
    while (background.size().x < ctx.framebuffer.size.x) || (background.size().y < ctx.framebuffer.size.y) {
        background = backgrounds.choose(rng).expect("huh?");
    }
    let cropspace = *background.size() - ctx.framebuffer.size;
    let pos = usizev2 { x: (rng.gen::<f32>() * (cropspace.x as f32)) as usize,y: (rng.gen::<f32>() * (cropspace.y as f32)) as usize, };
    ctx.framebuffer.set(&background.crop_upside_down(usizer { o: pos,s: ctx.framebuffer.size, }));

    ctx.framebuffer.bind();
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        ctx.eye.render_full(instance.projection_matrix,head_matrix * left_eye_matrix,light_dir,instance.light_color,instance.ambient_color,instance.sclera_color,instance.iris_color);
        ctx.eye.render_full(instance.projection_matrix,head_matrix * right_eye_matrix,light_dir,instance.light_color,instance.ambient_color,instance.sclera_color,instance.iris_color);
        ctx.skin.render_full(instance.projection_matrix,head_matrix,light_dir,instance.light_color,instance.ambient_color,instance.skin_color);
        gl::Disable(gl::DEPTH_TEST);
        gl::Finish();
        gl::Flush();
    }
    ctx.framebuffer.unbind();

    ctx.framebuffer.grab()
}

fn render_spec(ctx: &Context,instance: &Instance) -> Image<ARGB8> {

    let head_matrix = f32m4x4::translate(instance.pos) * f32m4x4::yaw(instance.head.y) * f32m4x4::pitch(instance.head.p);
    let left_eye_matrix = f32m4x4::translate(LEFT_EYE_POS) * f32m4x4::yaw(instance.left_eye.y) * f32m4x4::pitch(instance.left_eye.p) * f32m4x4::scale(EYE_SIZE);
    let right_eye_matrix = f32m4x4::translate(RIGHT_EYE_POS) * f32m4x4::yaw(instance.right_eye.y) * f32m4x4::pitch(instance.right_eye.p) * f32m4x4::scale(EYE_SIZE);

    ctx.framebuffer.bind();
    unsafe {
        gl::ClearColor(0.0,0.0,0.0,1.0);
        gl::ClearDepth(1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::Enable(gl::DEPTH_TEST);
        ctx.eye.render_spec(instance.projection_matrix,head_matrix * left_eye_matrix);
        ctx.eye.render_spec(instance.projection_matrix,head_matrix * right_eye_matrix);
        ctx.skin.render_spec(instance.projection_matrix,head_matrix,f32k { r: 0.5,g: 0.5,b: 0.5, });
        gl::Disable(gl::DEPTH_TEST);
        gl::Finish();
        gl::Flush();
    }
    ctx.framebuffer.unbind();

    ctx.framebuffer.grab()
}

fn save_image(image: Image<ARGB8>,name: &str) {
    let data = bmp::encode(&image).expect("Unable to encode image as BMP.");
    let mut file = fs::File::create(name).expect("Unable to create file.");
    file.write_all(&data).expect("Unable to write BMP.");
}

fn random_f32v3(rng: &mut rand::rngs::ThreadRng,lo: f32v3,hi: f32v3) -> f32v3 {
    f32v3::new(
        lo.x + (hi.x - lo.x) * rng.gen::<f32>(),
        lo.y + (hi.y - lo.y) * rng.gen::<f32>(),
        lo.z + (hi.z - lo.z) * rng.gen::<f32>(),
    )
}

fn random_f32e(rng: &mut rand::rngs::ThreadRng,lo: f32e,hi: f32e) -> f32e {
    f32e::new(
        lo.y + (hi.y - lo.y) * rng.gen::<f32>(),
        lo.p + (hi.p - lo.p) * rng.gen::<f32>(),
        lo.b + (hi.b - lo.b) * rng.gen::<f32>(),
    )
}

fn random_f32k(rng: &mut rand::rngs::ThreadRng,lo: f32k,hi: f32k) -> f32k {
    f32k::new(
        lo.r + (hi.r - lo.r) * rng.gen::<f32>(),
        lo.g + (hi.g - lo.g) * rng.gen::<f32>(),
        lo.b + (hi.b - lo.b) * rng.gen::<f32>(),
    )
}

fn process(rng: &mut rand::rngs::ThreadRng,ctx: &Context,backgrounds: &Vec<Image<ARGB8>>,space: &Space,dir_name: &str,csv: &mut fs::File,num: usize) {
    let image_name = format!("{:05}.bmp",num);
    let full_name = format!("{}/{}",dir_name,image_name);
    let mut instance = Instance {
        projection_matrix: space.projection_matrix,
        pos: random_f32v3(rng,space.pos.0,space.pos.1),
        head: random_f32e(rng,space.head.0,space.head.1),
        left_eye: random_f32e(rng,space.left_eye.0,space.left_eye.1),
        right_eye: random_f32e(rng,space.right_eye.0,space.right_eye.1),
        light: random_f32e(rng,space.light.0,space.light.1),
        background_color: random_f32k(rng,space.background_color.0,space.background_color.1),
        light_color: random_f32k(rng,space.light_color.0,space.light_color.1),
        ambient_color: space.ambient_color,
        skin_color: random_f32k(rng,space.skin_color.0,space.skin_color.1),
        sclera_color: space.sclera_color,
        iris_color: random_f32k(rng,space.iris_color.0,space.iris_color.1),
    };
    let mut visible = false;
    while !visible {
        loop {
            instance.pos = random_f32v3(rng,space.pos.0,space.pos.1);
            let pos = instance.projection_matrix * f32v4 { x: instance.pos.x,y: instance.pos.y,z: instance.pos.z,w: 1.0, };
            if (pos.x > -pos.w) && (pos.x < pos.w) && (pos.y > -pos.w) && (pos.y < pos.w) {
                break;
            }
        }
        let spec_image = render_spec(&ctx,&instance);
        for i in 0..spec_image.size().y {
            for k in 0..spec_image.size().x {
                let r = (*spec_image.pixel(usizev2 { x: k,y: i })).r();
                let g = (*spec_image.pixel(usizev2 { x: k,y: i })).g();
                let b = (*spec_image.pixel(usizev2 { x: k,y: i })).b();
                if (r >= 0x70) && (r < 0x90) && (g >= 0x70) && (g < 0x90) && (b >= 0x70) && (b < 0x90) {
                    visible = true;
                    break;
                }
            }
            if visible {
                break;
            }
        }
    }
    save_image(downsample4(render_full(rng,&ctx,&backgrounds,&instance)),&full_name);
    let pos = instance.projection_matrix * f32v4 { x: instance.pos.x,y: instance.pos.y,z: instance.pos.z,w: 1.0, };
    let ndc = f32v3 {
        x: pos.x / pos.w,
        y: pos.y / pos.w,
        z: pos.z / pos.w,
    };
    let screen = f32v2 {
        x: 0.5 * (1.0 + ndc.x) * (space.size.x as f32),
        y: 0.5 * (1.0 - ndc.y) * (space.size.y as f32),
    };
    let line = format!("\"{}\", {},{},{}, {},{}, {},{},{}, {},{}, {},{}, {},{},{}, {},{},{}, {},{},{}, {},{},{}\n",image_name,
        instance.pos.x,instance.pos.y,instance.pos.z,
        instance.head.y,instance.head.p,
        ndc.x,ndc.y,ndc.z,
        screen.x,screen.y,
        instance.light.y,instance.light.p,
        instance.background_color.r,instance.background_color.g,instance.background_color.b,
        instance.light_color.r,instance.light_color.g,instance.light_color.b,
        instance.ambient_color.r,instance.ambient_color.g,instance.ambient_color.b,
        instance.skin_color.r,instance.skin_color.g,instance.skin_color.b,
    );
    csv.write_all(line.as_bytes()).expect("Unable to write to CSV file.");
}

fn main() {

    let mut rng = rand::thread_rng();

    println!("loading backgrounds...");
    let mut backgrounds: Vec<Image<ARGB8>> = Vec::new();
    for entry in fs::read_dir("backgrounds").expect("unable to read from backgrounds directory") {
        let entry = entry.expect("invalid entry").file_name().into_string().expect("unable to convert");
        let mut file = fs::File::open(format!("backgrounds/{}",entry)).expect("cannot open file");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("unable to read file");
        let image = decode::<ARGB8>(&buffer).expect("unable to decode");
        backgrounds.push(image);
    }

    let space = Space {
        size: usizev2 { x: 256,y: 192, },
        projection_matrix: f32m4x4::perspective(30.0,256.0 / 192.0,0.1,100.0),
        pos: (f32v3 { x: -0.1,y: -0.1,z: -1.0, },f32v3 { x: 0.1,y: 0.1,z: 0.0, }),
        head: (f32e { y: -0.8,p: -0.8,b: 0.0, },f32e { y: 0.8,p: 0.8,b: 0.0, }),
        left_eye: (f32e { y: 0.0,p: 0.0,b: 0.0, },f32e { y: 0.0,p: 0.0,b: 0.0, }),
        right_eye: (f32e { y: 0.0,p: 0.0,b: 0.0, },f32e { y: 0.0,p: 0.0,b: 0.0, }),
        light: (f32e { y: -1.0,p: 0.0,b: 0.0, },f32e { y: 1.0,p: 1.5,b: 0.0, }),
        background_color: (f32k { r: 0.0,g: 0.0,b: 0.0, },f32k { r: 0.0,g: 0.0,b: 0.0, }),
        light_color: (f32k { r: 0.7,g: 0.7,b: 0.7, },f32k { r: 0.7,g: 0.7,b: 0.7, }),
        ambient_color: f32k { r: 0.2,g: 0.2,b: 0.2, },
        skin_color: (f32k { r: 0.3,g: 0.3,b: 0.3, },f32k { r: 0.9,g: 0.9,b: 0.9, }),
        sclera_color: f32k { r: 0.8,g: 0.8,b: 0.8, },
        iris_color: (f32k { r: 0.14,g: 0.14,b: 0.30, },f32k { r: 0.14,g: 0.14,b: 0.30, }),
    };

    let ctx = Context::new(space.size);

    println!("generating data0...");
    match fs::remove_dir_all("data0") {
        _ => { },
    };
    fs::create_dir("data0").expect("Unable to create directory.");
    let mut data_csv = fs::File::create(format!("data0/files.csv")).expect("Unable to create CSV file.");
    for i in 0..32768 {
        process(&mut rng,&ctx,&backgrounds,&space,"data0",&mut data_csv,i);
        println!("{} / 32768",i);
    }

    println!("generating test0...");
    match fs::remove_dir_all("test0") {
        _ => { },
    };
    fs::create_dir("test0").expect("Unable to create directory.");
    let mut test_csv = fs::File::create(format!("test0/files.csv")).expect("Unable to create CSV file.");
    for i in 0..256 {
        process(&mut rng,&ctx,&backgrounds,&space,"test0",&mut test_csv,i);
        println!("{} / 256",i);
    }

    let matrix = space.projection_matrix;

    println!("projection matrix:");
    println!("    {},{},{},{}",matrix.x.x,matrix.x.y,matrix.x.z,matrix.x.w);
    println!("    {},{},{},{}",matrix.y.x,matrix.y.y,matrix.y.z,matrix.y.w);
    println!("    {},{},{},{}",matrix.z.x,matrix.z.y,matrix.z.z,matrix.z.w);
    println!("    {},{},{},{}",matrix.w.x,matrix.w.y,matrix.w.z,matrix.w.w);
}
