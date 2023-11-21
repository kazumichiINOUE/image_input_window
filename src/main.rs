use image::{GenericImageView};
use minifb::{Key, Window, WindowOptions};

fn main() {
    // 画像の読み込み
    let occ_map_path = "occMap.png";
    let img = image::open(occ_map_path).expect("画像のオープンに失敗しました");
    let (width, height) = img.dimensions();

    // 表示する画像をバッファとして作成する
    // 色並びは右の通り ARGB
    let argb: u32 = 255 << 24 | 125 << 16 | 125 << 8 | 125;
    let mut buffer: Vec<u32> = vec![argb; width as usize * height as usize];
    // 画像データをバッファにコピー
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgba = (pixel[3] as u32) << 24 | (pixel[2] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[0] as u32);
            buffer[(y * width + x) as usize] = rgba;
        }
    }

    // 画像を表示するウィンドウを作成
    let mut window = Window::new(
        "Image with Plots",
        width as usize,
        height as usize,
        WindowOptions::default(),
        )
        .expect("ウィンドウの作成に失敗しました");

    // ウィンドウの表示
    // ESCが押されるまで待機
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("ウィンドウの更新に失敗しました");
    }

    println!("Bye!");
}

