let ref_table = [
    [0,0,0,0,0,0,0,0,0,0,0,-1,0,-1,0,0,],
    [0,0,0,0,0,0,-1,0,0,-1,0,0,-1,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,-1,0,],
    [0,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,-1,0,],
    [0,0,0,0,0,0,0,-1,0,0,0,0,-1,0,0,-1,],
    [-1,0,0,0,0,0,0,0,-1,0,-1,0,0,-1,0,0,],
    [-1,0,-1,0,-1,0,0,0,0,-1,-1,0,0,0,0,0,],
    [0,-1,-1,0,-1,0,-1,0,0,0,0,-1,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,-1,-1,0,-1,0,],
    [0,0,0,0,-1,0,0,0,0,0,0,0,0,0,0,0,]
    ,[-1,-1,0,0,-1,0,-1,-1,-1,-1,-1,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,-1,0,0,0,0,0,],
    [0,0,0,-1,0,0,0,-1,0,0,0,-1,0,-1,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,-1,0,-1,0,0,],
    [0,0,0,-1,-1,0,-1,0,-1,0,0,0,0,0,0,0,]];
let cell = [1,4];
let table_size = 16;
//a cell can be called multiple times
function count_mines(ref_table, coor) {
    if (coor[0] < 0 || coor[1] < 0) return;
    if (ref_table[coor[0]][coor[1]] != 0) return;
    let count = 0;
    if (coor[0] - 1 >= 0 && coor[1] - 1 >= 0) {
        if (ref_table[coor[0] - 1][coor[1] - 1] == -1) {
            count += 1;
        }
    }
    if (coor[0] - 1 >= 0) {
        if (ref_table[coor[0] - 1][coor[1]] == -1) {
            count += 1;
        }
    }
    if (coor[0] - 1 >= 0 && coor[1] + 1 < table_size) {
        if (ref_table[coor[0] - 1][coor[1] + 1] == -1) {
            count += 1;
        }
    }
    if (coor[1] - 1 >= 0) {
        if (ref_table[coor[0]][coor[1] - 1] == -1) {
            count += 1;
        }
    }
    if (coor[1] + 1 < table_size) {
        if (ref_table[coor[0]][coor[1] + 1] == -1) {
            count += 1;
        }
    }
    if (coor[0] + 1 < table_size && coor[1] - 1 >= 0) {
        if (ref_table[coor[0] + 1][coor[1] - 1] == -1) {
            count += 1;
        }
    }
    if (coor[0] + 1 < table_size) {
        if (ref_table[coor[0] + 1][coor[1]] == -1) {
            count += 1;
        }
    }
    if (coor[0] + 1 < table_size && coor[1] + 1 < table_size) {
        if (ref_table[coor[0] + 1][coor[1] + 1] == -1) {
            count += 1;
        }
    }
    if (count != 0) {
        ref_table[coor[0]][coor[1]] = count;
        console.table(ref_table);
    } else {
        ref_table[coor[0]][coor[1]] = -2; //add this line
        console.log("Next");
        count_mines(ref_table, [coor[0] - 1, coor[1] - 1]);
        count_mines(ref_table, [coor[0] - 1, coor[1]]);
        count_mines(ref_table, [coor[0] - 1, coor[1] + 1]);
        count_mines(ref_table, [coor[0], coor[1] - 1]);
        count_mines(ref_table, [coor[0], coor[1] + 1]);
        count_mines(ref_table, [coor[0] + 1, coor[1] - 1]);
        count_mines(ref_table, [coor[0] + 1, coor[1]]);
        count_mines(ref_table, [coor[0] + 1, coor[1] + 1]);
    }
}
count_mines(ref_table, cell);