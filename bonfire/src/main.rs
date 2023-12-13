use raylib_ffi::{*, colors::WHITE};
use std::{ffi::c_void, ops::RangeFrom};
use tinyrand::*;

const WINDOWS_WIDTH:i32 = 600;
const WINDOW_HEIGHT:i32 = 480;
const PALETTE_SIZE: usize = 256;

fn main() {
    init_gui();

    let mut screen_buffer_data = [ Color{r:0,g:0,b:0,a:0xFF} ; (WINDOWS_WIDTH * WINDOW_HEIGHT) as usize];
    let screen_buffer = Image {
        data: screen_buffer_data.as_mut_ptr() as *mut c_void,
        width: WINDOWS_WIDTH,
        height: WINDOW_HEIGHT,
        format: enums::PixelFormat::R8g8b8a8 as i32,
        mipmaps: 1,
    };
    let screen_buffer_texture 
        = load_texture_from_image(screen_buffer);
    let palette = generate_palette();
    let mut fire_buffer = [ 0u8; (WINDOWS_WIDTH * WINDOW_HEIGHT) as usize ];
    let mut rng: Wyrand = StdRand::default();

    while ! window_should_close() {
        begin_drawing();

        draw_next_frame(&mut screen_buffer_data,  &mut fire_buffer,&palette, &mut rng);

        // Pasar a la GPU
        update_texture(
            screen_buffer_texture, 
            &screen_buffer_data
        );
        draw_texture(screen_buffer_texture, 0, 0, WHITE);
        end_drawing()
    }
    println!("Hello, world!");

    close_window();
}

fn generate_palette() -> [Color; PALETTE_SIZE] {
    let mut pal = [ Color{r:0,g:0,b:0,a:0xFF} ; PALETTE_SIZE];
    let palette_third = PALETTE_SIZE / 3;
   
    for  i in 0 .. PALETTE_SIZE {
        if i < palette_third {
            pal[i].r = (i * (0xFF / palette_third)) as u8;
        }else  if i < (2 * palette_third) {
            pal[i].r = 0xFF;
            pal[i].g = ((i-palette_third) * (0xFF / palette_third)) as u8;
        } else {
            pal[i].r = 0xFF;
            pal[i].g = 0xFF;
            pal[i].b = ((i-palette_third) * (0xFF / palette_third)) as u8;
        }
    }

    pal
}

fn draw_the_palette(screen: &mut [Color], pal: &[Color]) {
    for i in 0..pal.len() {
        let init = i* (WINDOWS_WIDTH) as usize  + 50;
        let pixeles = &mut screen[ init..(init+4) ];
        pixeles[0] = pal[i];
        pixeles[1] = pal[i];
        pixeles[2] = pal[i];
        pixeles[3] = pal[i];
    }
}




fn draw_next_frame(screen: &mut [Color],fire_buffer: &mut [u8], pal: &[Color],rng :&mut Wyrand) {
    draw_the_palette(screen, pal);
    fill_bottom_with_random_ashes(fire_buffer, rng);
    // calulate next frame
    calculate_next_fire_frame(fire_buffer);
    convert_fire_buffer_to_screen(fire_buffer, pal, screen);
 
}
fn fill_bottom_with_random_ashes(fire_buffer: &mut [u8],rng :&mut Wyrand){
    let init = ((WINDOWS_WIDTH * WINDOW_HEIGHT ) - WINDOWS_WIDTH) as usize;

    for i in init..fire_buffer.len() {
        fire_buffer[i] = rng.next_range(0..PALETTE_SIZE as u16) as u8
    }

}
fn calculate_next_fire_frame(fire_buffer: &mut[u8]){
    let mut old_fire_buf = [0u8; (WINDOWS_WIDTH*WINDOW_HEIGHT) as usize];
    old_fire_buf.clone_from_slice(fire_buffer);
    for y in 0..WINDOW_HEIGHT-1  {
        for x in 1..WINDOWS_WIDTH-1  {
             let index= (y*WINDOWS_WIDTH + x) as usize;
            
             fire_buffer[index] =  ((
                20* old_fire_buf[index] as u64
                + 10 * old_fire_buf[index -1] as u64
                + 10 * old_fire_buf[index +1] as u64
                + 320 * (old_fire_buf[index + WINDOWS_WIDTH as usize]) as u64
                + 160 * old_fire_buf[index -1  + WINDOWS_WIDTH as usize] as u64
                + 160 * old_fire_buf[index +1 + WINDOWS_WIDTH as usize ]  as u64 )/ 680) as u8 ;
                
        }
    }
}

fn convert_fire_buffer_to_screen(fire_buffer:  &[u8],pal: &[Color],screen: &mut [Color]){
    for i in 0..fire_buffer.len() {
        let heat = fire_buffer[i] as usize;
        screen[i] = pal[ heat ];
    }
}

fn update_texture(tex: Texture2D, data: &[Color]) {
    unsafe{ 
        UpdateTexture(tex, data.as_ptr() as *const c_void);
    }
}

fn draw_texture(tex: Texture2D, x: i32, y: i32, tint: Color) {
    unsafe {
        DrawTexture(tex, x, y, tint);
    }
}

fn load_texture_from_image(img: Image) -> Texture2D {
    unsafe {
        LoadTextureFromImage(img)
    }
}

fn init_gui() {
    unsafe {
        InitWindow(WINDOWS_WIDTH, WINDOW_HEIGHT, "Fire".as_ptr() as *const i8);
    }
}

fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }    
}

fn begin_drawing() {
    unsafe {
        BeginDrawing();
    }    
}

fn end_drawing() {
    unsafe {
        EndDrawing();
    }    
}

fn close_window() {
    unsafe {
        CloseWindow();   
    }
}

fn draw_pixel(x: i32, y: i32, color: Color) {
    unsafe {
        DrawPixel(x, y, color)
    }
}