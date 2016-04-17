use std::cmp;
pub fn create_field(x_size:usize,y_size:usize)->Vec<Vec<i8>> {
    vec![vec![0;x_size];y_size]
}

pub fn create_cell(mut field:Vec<Vec<i8>>, x_index:usize, y_index:usize)->Vec<Vec<i8>> {
    field[x_index][y_index] = 1;
    field
}

pub fn count_neighbours(field:&Vec<Vec<i8>>, x_index:&usize, y_index:&usize)->i8 {
    let lowest_x = if *x_index>0 {*x_index-1} else {*x_index};
    let lowest_y = if *y_index>0 {*y_index-1} else {*y_index};
    let highest_x = cmp::min(field[0].len()-1 ,*x_index+1);
    let highest_y = cmp::min(field.len()-1 ,*y_index+1);

    let mut neighbours:i8 = 0;
    for x in lowest_x..highest_x+1 {
        for y in lowest_y..highest_y+1{
            neighbours+=field[y][x];
        }
    }

    neighbours - field[*y_index][*x_index]
}

pub fn do_step(field:&Vec<Vec<i8>>)->Vec<Vec<i8>>{
    let mut result = vec![vec![0;field[0].len()];field.len()];

    for row_index in 0..field.len(){
        for col_index in 0..field[row_index].len(){
            let neighbours = count_neighbours(field, &col_index, &row_index);
            if field[row_index][col_index] == 1 {
                if neighbours<=3&&neighbours>=2 {
                    result[row_index][col_index] = 1;
                }
            }else{
                if neighbours==3 {
                    result[row_index][col_index] = 1;
                }
            }

        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_field_test() {
        assert_eq!(vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]], create_field(3,3));
        assert_eq!(vec![vec![0,0],vec![0,0],vec![0,0]], create_field(2,3));
        assert_eq!(vec![vec![0,0],vec![0,0]], create_field(2,2));
    }

    #[test]
    fn create_cell_test() {
        let field = vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]];
        assert_eq!(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]], create_cell(field, 1,1));
    }

    #[test]
    fn count_neighbours_test() {
        //1 0 0
        //1 1 0
        //0 1 1
        let field = vec![vec![1,0,0],vec![1,1,0],vec![0,1,1]];
        assert_eq!(2, count_neighbours(&field, &0,&0));
        assert_eq!(4, count_neighbours(&field, &1,&1));
        assert_eq!(2, count_neighbours(&field, &2,&2))
    }

    #[test]
    fn do_step_test(){
        assert_eq!(vec![vec![0]],do_step(&vec![vec![0]]));
        assert_eq!(vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]],do_step(&vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
        assert_eq!(vec![vec![0,1,0],vec![0,1,0],vec![0,1,0]],do_step(&vec![vec![0,0,0],vec![1,1,1],vec![0,0,0]]));
        assert_eq!(vec![vec![0,0,0],vec![1,1,1],vec![0,0,0]],do_step(&vec![vec![0,1,0],vec![0,1,0],vec![0,1,0]]));
        assert_eq!(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]],do_step(&vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]]))
        }
}
