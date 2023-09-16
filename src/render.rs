use crate::box_data::BoxData;
use crate::constants::*;
use crate::enums::Stage;
use crate::event::*;
use bracket_lib::prelude::*;

pub struct State {
    pub data: BoxData,
    pub size: usize,
    pub succeed: bool,
    pub stage: Stage,
    pub quit_game: bool,
}

impl State {
    pub fn new(dimen: usize) -> Self {
        let mut s = State {
            data: BoxData::new(dimen),
            size: dimen,
            succeed: false,
            stage: Stage::HOME,
            quit_game: false,
        };
        s.reset(dimen);
        s
    }

    pub fn reset(&mut self, dimen: usize) {
        self.succeed = false;
        self.size = dimen;
        self.data.reset(self.size);
    }

    fn render_home_view(&self, ctx: &mut BTerm, center_y: i32) {
        let home_bg_color: (u8, u8, u8) = (82, 132, 144);
        let text_color: (u8, u8, u8) = (230, 230, 230);
        let text_bg_color: (u8, u8, u8) = home_bg_color;

        let title_text_color: (u8, u8, u8) = (255, 255, 255);

        ctx.cls_bg(home_bg_color);

        let title_text = "Welcome To Play Huarong Dao (Klotski)";
        ctx.print_color_centered(center_y - 15, text_color, text_bg_color, title_text);

        let tips_content_y = center_y - 10;
        ctx.print_color_centered(
            tips_content_y,
            title_text_color,
            text_bg_color,
            "HOW TO PLAY",
        );

        ctx.print_color_centered(
            tips_content_y + 2,
            text_color,
            text_bg_color,
            "(2)(3)(4)(5)(6): choose dimension.",
        );
        ctx.print_color_centered(
            tips_content_y + 4,
            text_color,
            text_bg_color,
            "Enter: enter game.",
        );
        ctx.print_color_centered(
            tips_content_y + 6,
            text_color,
            text_bg_color,
            "B: back to menu.",
        );
        ctx.print_color_centered(
            tips_content_y + 8,
            text_color,
            text_bg_color,
            "R: restart game.",
        );
        ctx.print_color_centered(
            tips_content_y + 10,
            text_color,
            text_bg_color,
            "Q: quit game.",
        );

        ctx.print_color_centered(
            tips_content_y + 12,
            text_color,
            text_bg_color,
            "Left Right Up Down (W)(A)(S)(D): move the box.",
        );
        ctx.print_color_centered(
            tips_content_y + 14,
            text_color,
            text_bg_color,
            "Game Purpose: numbers arranged in order.",
        );
    }

    fn render_game_view(&self, ctx: &mut BTerm, center_x: i32, center_y: i32) {
        let bg_color = NUMBER_BACKGROUND_COLOR;
        let text_color = NUMBER_COLOR;
        let gap_distance: usize = 4;

        let x_start_drawing_pos = center_x - (gap_distance * self.size / 2) as i32;
        let y_start_drawing_pos = center_y - (gap_distance * self.size / 2) as i32 - 2;

        let start_x = x_start_drawing_pos;
        let end_x = x_start_drawing_pos + (gap_distance * self.size) as i32;
        let start_y = y_start_drawing_pos;
        let end_y = y_start_drawing_pos + (gap_distance * self.size) as i32;

        let border = '#';
        for x in start_x..end_x + 3 {
            ctx.set(
                x - 2,
                start_y - 2,
                BORDER_COLOR,
                BACKGROUND_COLOR,
                to_cp437(border),
            );

            ctx.set(
                x - 2,
                end_y,
                BORDER_COLOR,
                BACKGROUND_COLOR,
                to_cp437(border),
            );
        }

        for y in start_y..end_y + 1 {
            ctx.set(
                start_x - 2,
                y - 1,
                BORDER_COLOR,
                BACKGROUND_COLOR,
                to_cp437(border),
            );

            ctx.set(
                end_x,
                y - 1,
                BORDER_COLOR,
                BACKGROUND_COLOR,
                to_cp437(border),
            );
        }

        let count = self.data.data.len();
        for i in 0..count {
            let num: i32 = self.data.data[i];

            if num as usize == self.data.total_count {
                continue;
            }

            let x_position = i % self.data.block_size;
            let y_position = i / self.data.block_size;

            let x_box = x_start_drawing_pos + (x_position * gap_distance) as i32;
            let y_box = y_start_drawing_pos + (y_position * gap_distance) as i32;

            ctx.draw_box(
                x_box,
                y_box,
                2,
                2,
                RGB::named(bg_color),
                RGB::named(bg_color),
            );
            let mut x_diff: i32 = 1;
            if num > 9 {
                x_diff = 0;
            }
            ctx.print_color(x_box + x_diff, y_box + 1, text_color, bg_color, num);
        }

        if self.succeed {
            let text = "Game Completed !";
            ctx.print_color_centered(center_y - 15, RED, BACKGROUND_COLOR, text);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if self.quit_game {
            ctx.quit();
        }

        ctx.cls_bg(BACKGROUND_COLOR);
        ctx.set_active_console(0);
        ctx.set_active_font(1, true);

        let screeen_center_pos_x = ((SCREEN_WIDTH / 2) as u32 - 16) as i32;
        let screeen_center_pos_y = (SCREEN_HEIGHT / 2 - 17) as i32;

        ctx.set_scale(1.8, screeen_center_pos_x, screeen_center_pos_y);

        on_key_pressed(ctx.key, self);

        match self.stage {
            Stage::HOME => {
                self.render_home_view(ctx, screeen_center_pos_y);
            }
            Stage::GAMING => {
                self.render_game_view(ctx, screeen_center_pos_x, screeen_center_pos_y);
            }
        }
    }
}
