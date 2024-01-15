pub fn annotate(minefield: &[&str]) -> Vec<String> {
    /*
    for each grid in minefield:
      if grid is space:
        count all * in 8 adjacent grids and update
    */
    let mut annotated = Vec::new();
    for (i, &row) in minefield.iter().enumerate() {
      let mut annotated_row = String::new();
      for (j, &col) in row.as_bytes().iter().enumerate() {
        annotated_row.push(match col {
          b'*' => '*',
          _ => {
            let stars = count_surrounding_stars(minefield, i, j);
            if stars == 0 { 
              ' '
            } else { 
              char::from_digit(stars, 10).unwrap()
            }
          },
        });
      }
      annotated.push(annotated_row);
    }
    annotated
}

fn count_surrounding_stars(minefield: &[&str], row: usize, col: usize) -> u32 {
  let surroundings = [
    (row as isize - 1, col as isize - 1), 
    (row as isize - 1, col as isize), 
    (row as isize - 1, col as isize + 1), 
    (row as isize, col as isize - 1), 
    (row as isize, col as isize + 1), 
    (row as isize + 1, col as isize - 1), 
    (row as isize + 1, col as isize), 
    (row as isize + 1, col as isize + 1),
  ];

  let mut stars: u32 = 0;
  for &pair in surroundings.iter() {
    if 0 <= pair.0
    && pair.0 < minefield.len() as isize 
    && 0 <= pair.1 
    && pair.1 < minefield[0].len() as isize {
      if minefield[pair.0 as usize].as_bytes()[pair.1 as usize] == b'*' {
        stars += 1;
      }
    }
  }

  stars
}
