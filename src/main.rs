//创建冒泡函数，元素数据类型固定为 i32
fn maopao_sort(arr: &mut [i32]) {
    //定义长度 n ，用作比较循环次数
    let n = arr.len();
    //进行循环比较，次数从 0 开始计算到数组长度 n 
    for i in 0..n {
        //每次比对时，如果前面数比后面一位数大，则交换，即小的数最后会在前面
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn main() {
    //定义一个数组
    let mut array = [17, 60, 83, 52, 3, 8, 6];
    
    println!("排序前: {:?}", array);
    
    maopao_sort(&mut array);
    
    println!("排序后: {:?}", array);
}
