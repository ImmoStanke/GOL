use std::cmp;
pub fn create_field(x_size:usize,y_size:usize)->Vec<Vec<i8>> {
    vec![vec![0;x_size];y_size]
}

pub fn create_cell(mut field:Vec<Vec<i8>>, x_index:usize, y_index:usize)->Vec<Vec<i8>> {
    field[x_index][y_index] = 1;
    field
}

pub fn count_neighbours(field:Vec<Vec<i8>>, x_index:usize, y_index:usize)->i8 {
    let lowest_x = if x_index>0 {x_index-1}else{x_index};
    let lowest_y = if y_index>0 {y_index-1}else{y_index};
    let highest_x = x_index+1;
    let highest_y =  y_index+1;


    println!("x{}y{}x{}y{}",lowest_x,lowest_y,highest_x,highest_y);
    let mut neighbours:i8 = 0;
    for x in lowest_x..highest_x+1 {
        for y in lowest_y..highest_y+1{
            if !((x==x_index)&&(y==y_index)){
            neighbours+=field[y][x];}
        }
    }
    neighbours
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
        assert_eq!(2, count_neighbours(field, 0,0))
    }
}
