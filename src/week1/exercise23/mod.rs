pub fn capture_surrounded_regions(input: &mut Vec<Vec<char>>) {
   if !input.is_empty() {
      let down_border = input.len() - 1;
      let right_border = input[0].len() - 1;

      for y in 0..=down_border {
         for x in 0..=right_border {
            if y == 0 || y == down_border || x == 0 || x == right_border {
               check_adjacent_regions(input, y, x);
            }
         }
      }

      input.iter_mut().for_each(|row| {
         row.iter_mut().for_each(|region| match region {
            '-' => *region = 'O',
            _ => *region = 'X',
         });
      });
   }
}

fn check_adjacent_regions(input: &mut Vec<Vec<char>>, y: usize, x: usize) {
   if input[y][x] == 'O' {
      input[y][x] = '-';

      if y > 0 && input[y - 1][x] == 'O' {
         check_adjacent_regions(input, y - 1, x);
      }
      if x < input[y].len() - 1 && input[y][x + 1] == 'O' {
         check_adjacent_regions(input, y, x + 1);
      }
      if y < input.len() - 1 && input[y + 1][x] == 'O' {
         check_adjacent_regions(input, y + 1, x);
      }
      if x > 1 && input[y][x - 1] == 'O' {
         check_adjacent_regions(input, y, x - 1);
      }
   }
}
