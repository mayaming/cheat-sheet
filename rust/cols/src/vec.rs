pub fn vec_2d() {
    println!("**********start of vec baisc demo*********");
    // 创建一个4行6列的vec，元素初始化为0
    let mut v = vec![vec![0; 6]; 4];
    // 更改其中一个元素
    v[0][0] = 1;
    // [[1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]]
    println!("{:?}", v);

    let v = &vec![vec![1,3], vec![0], vec![3], vec![0,2]];
    println!("{:?}", v);

    for i in v {
        for j in i {
            if j == 0 {
                println!("{}", j);
            }
        }
    }
    println!("**********start of vec baisc demo*********");
}