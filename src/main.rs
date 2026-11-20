
mod player;
use player::Player;
use macroquad::prelude::*;

const WINDOW_TITLE: &str = "Blank Window Template";
const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 800;

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let mut render_target_cam = Camera2D::from_display_rect(Rect::new(
        0.0,
        0.0,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
    ));
    render_target_cam.render_target = Some(render_target.clone());

    let mut player: Player = Player::new(276.0, 700.0, 48.0, 48.0, 400.0);

    loop {
        // do update logic here

        {
            player.update(get_frame_time());
        }

        let w = WINDOW_WIDTH as f32;
        let h = WINDOW_HEIGHT as f32;
        let scale = f32::min(screen_width() / w, screen_height() / h);

        set_camera(&render_target_cam);
        clear_background(Color::from_hex(0x8cbade));

        // start drawing here

        {
            player.draw(Color::from_hex(0x364996));
        }

        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (w * scale)) * 0.5,
            (screen_height() - (h * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w * scale, h * scale)),
                flip_y: true, // Required because render targets are flipped vertically
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
