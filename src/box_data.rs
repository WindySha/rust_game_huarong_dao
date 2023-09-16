use crate::enums::Direction;
use rand::Rng;

pub struct BoxData {
    pub data: Vec<i32>,     // 1 - n*n
    pub block_size: usize,  // dimension of the block: n
    pub total_count: usize, // n * n
    pub empty_pos_x: usize, // the empty block x position
    pub empty_pos_y: usize, // the empty block y position
}

impl BoxData {
    pub fn new(block_size: usize) -> Self {
        BoxData {
            data: vec![0; block_size * block_size],
            block_size: block_size,
            total_count: block_size * block_size,
            empty_pos_x: 0,
            empty_pos_y: 0,
        }
    }

    pub fn reset(&mut self, dimensions: usize) {
        self.block_size = dimensions;
        self.total_count = dimensions * dimensions;

        self.data.resize(self.total_count, 0);
        self.generate_random_data();
    }

    fn generate_random_data(&mut self) {
        self.reset_ordered_data();

        let mut rng = rand::thread_rng();
        let max_randon_count = rng.gen_range(200..400);
        let mut curr_moved_times = 0;

        loop {
            let rand_num = rng.gen_range(0..4);

            let direction = match rand_num {
                0 => Direction::DOWN,
                1 => Direction::UP,
                2 => Direction::LEFT,
                3 => Direction::RIGHT,
                _ => Direction::UP,
            };
            if self.move_box(direction) {
                curr_moved_times += 1;
            }
            if curr_moved_times >= max_randon_count {
                break;
            }
        }
    }

    fn reset_ordered_data(&mut self) {
        for i in 1..self.total_count + 1 {
            self.data[i - 1] = i as i32;
        }
        self.empty_pos_x = self.block_size - 1;
        self.empty_pos_y = self.block_size - 1;
    }

    /// move one step of the box
    pub fn move_box(&mut self, direction: Direction) -> bool {
        let empty_box_pos = self.empty_pos_x + (self.block_size * self.empty_pos_y);
        let empty_box_value = self.data[empty_box_pos];

        match direction {
            Direction::UP => {
                if self.empty_pos_y >= self.block_size - 1 {
                    return false;
                }
                self.empty_pos_y += 1;
            }
            Direction::DOWN => {
                if self.empty_pos_y <= 0 {
                    return false;
                }
                self.empty_pos_y -= 1;
            }
            Direction::LEFT => {
                if self.empty_pos_x >= self.block_size - 1 {
                    return false;
                }
                self.empty_pos_x += 1;
            }
            Direction::RIGHT => {
                if self.empty_pos_x <= 0 {
                    return false;
                }
                self.empty_pos_x -= 1;
            }
        }

        let target_box_vect_pos = self.empty_pos_x + (self.block_size * self.empty_pos_y);
        self.data[empty_box_pos] = self.data[target_box_vect_pos];
        self.data[target_box_vect_pos] = empty_box_value;
        true
    }

    pub fn check_succeed(&self) -> bool {
        let empty_box_pos = self.empty_pos_x + (self.block_size * self.empty_pos_y);
        let empty_box_value = self.data[empty_box_pos];
        if empty_box_value != self.total_count as i32 {
            return false;
        }

        let count = self.data.len();
        for i in 0..count {
            if i == 0 {
                continue;
            }
            let num: i32 = self.data[i];
            let last_num: i32 = self.data[i - 1];
            if last_num > num {
                return false;
            }
        }
        true
    }
}
