pub fn count_islands(input: Vec<Vec<char>>) -> i32 {
   if input.is_empty() {
      return 0;
   }

   let mut trace_grid = input.clone();
   let mut land_count = 0;

   for (y, row) in input.iter().enumerate() {
      for (x, _) in row.iter().enumerate() {
         if trace_grid[y][x] == '1' {
            land_count += 1;
            check_adjacent_land(&mut trace_grid, x as i32, y as i32);
         }
      }
   }

   land_count
}

fn check_adjacent_land(trace_grid: &mut Vec<Vec<char>>, x: i32, y: i32) {
   let width = trace_grid[0].len() as i32;
   let height = trace_grid.len() as i32;

   if (x + 1) < width && trace_grid[y as usize][x as usize + 1] == '1' {
      trace_grid[y as usize][x as usize + 1] = '0';
      check_adjacent_land(trace_grid, x + 1, y);
   }

   if (x - 1) >= 0 && trace_grid[y as usize][x as usize - 1] == '1' {
      trace_grid[y as usize][x as usize - 1] = '0';
      check_adjacent_land(trace_grid, x - 1, y);
   }

   if (y + 1) < height && trace_grid[y as usize + 1][x as usize] == '1' {
      trace_grid[y as usize + 1][x as usize] = '0';
      check_adjacent_land(trace_grid, x, y + 1);;
   }

   if (y - 1) >= 0 && trace_grid[y as usize - 1][x as usize] == '1' {
      trace_grid[y as usize - 1][x as usize] = '0';
      check_adjacent_land(trace_grid, x, y - 1);;
   }
}