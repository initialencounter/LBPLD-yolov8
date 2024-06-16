// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;
use image::{GenericImageView, imageops::FilterType};
use lazy_static::lazy_static;
use ndarray::{Array, IxDyn, s, Axis};
use ort::{inputs, Session, SessionOutputs};
use std::time::{Instant};
use base64::{Engine as _, engine::{general_purpose}};

lazy_static! {
    static ref MODEL: Session = {
        let model_data: &[u8] = include_bytes!("best.onnx");
        let model = Session::builder().unwrap().commit_from_memory(model_data)
        .map_err(|e| format!("Failed to run model: {:?}", e)).unwrap();
        model
    };
}

#[tauri::command]
fn detect_obj(file: &str) -> String {
    let buf = general_purpose::STANDARD.decode(file).unwrap();
    let start_time = Instant::now();
    let boxes = detect_objects_on_image(buf);
    let duration = start_time.elapsed();
    println!("{:?}",duration);
    return serde_json::to_string(&boxes).unwrap_or_default()
}

fn detect_objects_on_image(buf: Vec<u8>) -> Vec<Vec<String>> {
    let (input,img_width,img_height) = prepare_input(buf);
    let output = run_model(input);
    return process_output(output, img_width, img_height);
}

fn prepare_input(buf: Vec<u8>) -> (Array<f32,IxDyn>, u32, u32) {
    let img = image::load_from_memory(&buf).unwrap();
    let (img_width, img_height) = (img.width(), img.height());
    let img = img.resize_exact(640, 640, FilterType::CatmullRom);
    let mut input = Array::zeros((1, 3, 640, 640)).into_dyn();
    for pixel in img.pixels() {
        let x = pixel.0 as usize;
        let y = pixel.1 as usize;
        let [r,g,b,_] = pixel.2.0;
        input[[0, 0, y, x]] = (r as f32) / 255.0;
        input[[0, 1, y, x]] = (g as f32) / 255.0;
        input[[0, 2, y, x]] = (b as f32) / 255.0;
    };
    return (input, img_width, img_height);
}

fn run_model(input:Array<f32,IxDyn>) -> Array<f32,IxDyn> {
    let model = &*MODEL;
    // Run YOLOv8 inference
    let outputs: SessionOutputs = model.run(inputs!["images" => input.view()].unwrap()).unwrap();
    let output = outputs["output0"].try_extract_tensor::<f32>().unwrap().t().into_owned();
    return output;
}

fn process_output(output:Array<f32,IxDyn>,img_width: u32, img_height: u32) -> Vec<Vec<String>> {
    let mut boxes = Vec::new();
    let output = output.slice(s![..,..,0]);
    for row in output.axis_iter(Axis(0)) {
        let row:Vec<_> = row.iter().map(|x| *x).collect();
        let (class_id, prob) = row.iter().skip(4).enumerate()
            .map(|(index,value)| (index,*value))
            .reduce(|accum, row| if row.1>accum.1 { row } else {accum}).unwrap();
        if prob < 0.5 {
            continue
        }
        let label = YOLO_CLASSES[class_id];
        let xc = row[0]/640.0*(img_width as f32);
        let yc = row[1]/640.0*(img_height as f32);
        let w = row[2]/640.0*(img_width as f32);
        let h = row[3]/640.0*(img_height as f32);
        let x1 = xc - w/2.0;
        let x2 = xc + w/2.0;
        let y1 = yc - h/2.0;
        let y2 = yc + h/2.0;
        boxes.push((x1,y1,x2,y2,label,prob));
    }

    boxes.sort_by(|box1,box2| box2.5.total_cmp(&box1.5));
    let mut result = Vec::new();
    while boxes.len()>0 {
        result.push(boxes[0]);
        boxes = boxes.iter().filter(|box1| iou(&boxes[0],box1) < 0.7).map(|x| *x).collect()
    }
    let array_data: Vec<Vec<String>> = result.into_iter()
        .map(|(a, b, c, d, e, f)| vec![
            a.to_string(), 
            b.to_string(), 
            c.to_string(), 
            d.to_string(), 
            e.to_string(), 
            f.to_string()
        ])
        .collect();
    return array_data;
}


fn iou(box1: &(f32, f32, f32, f32, &'static str, f32), box2: &(f32, f32, f32, f32, &'static str, f32)) -> f32 {
    return intersection(box1, box2) / union(box1, box2);
}

fn union(box1: &(f32, f32, f32, f32, &'static str, f32), box2: &(f32, f32, f32, f32, &'static str, f32)) -> f32 {
    let (box1_x1,box1_y1,box1_x2,box1_y2,_,_) = *box1;
    let (box2_x1,box2_y1,box2_x2,box2_y2,_,_) = *box2;
    let box1_area = (box1_x2-box1_x1)*(box1_y2-box1_y1);
    let box2_area = (box2_x2-box2_x1)*(box2_y2-box2_y1);
    return box1_area + box2_area - intersection(box1, box2);
}

fn intersection(box1: &(f32, f32, f32, f32, &'static str, f32), box2: &(f32, f32, f32, f32, &'static str, f32)) -> f32 {
    let (box1_x1,box1_y1,box1_x2,box1_y2,_,_) = *box1;
    let (box2_x1,box2_y1,box2_x2,box2_y2,_,_) = *box2;
    let x1 = box1_x1.max(box2_x1);
    let y1 = box1_y1.max(box2_y1);
    let x2 = box1_x2.min(box2_x2);
    let y2 = box1_y2.min(box2_y2);
    return (x2-x1)*(y2-y1);
}

const YOLO_CLASSES:[&str;8] = [
    "9A","3480","CAO","3481","UN spec","Blur","RMD","3091"
];

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,detect_obj])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
