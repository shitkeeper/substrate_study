// 使用枚举类型

enum TrafficLight {
    Red(u32),
    Green(u32),  
    Yellow(u32), 
}

// 定义trait，返回时间
// 使用 u32 类型返回
trait Last {
    fn last(&self) -> u32;
}

// 为 TrafficLight 枚举类型实现 trait
// impl 是作为方法类型实现 trait 的关键字
// *为取地址上的值，类似指针所指的值
impl Last for TrafficLight {
    fn last(&self) -> u32 {
        match self {
            TrafficLight::Red(time) => *time,  
            TrafficLight::Green(time) => *time,
            TrafficLight::Yellow(time) => *time,
        }
    }
}

// 函数的输入参数类型为 & 取地址类型，返回值 option 
fn sum_u32(numbers: &[u32]) -> Option<u32> {
    //初始化值，为可变
    let mut sum = 0u32;
    //循环遍历
    for &num in numbers {
        // checked 方法加入到总和
        if let Some(result) = sum.checked_add(num) {
            sum = result;
        } else {
        //考虑数据错误溢出的情况
            return None; 
        }
    }

    Some(sum) // 没有溢出，返回求和结果
}

// 定义一个 trait，表示可以计算面积的类型
trait Area {
    //考虑数据小数乘积，选用浮点型
    fn calculate_area(&self) -> f64;
    //计算的图形类型也需要知道是什么
    fn shape_name(&self) -> &'static str;
}

// 圆结构体，的面积数据>>半径
struct Circle {
    radius: f64,
}

//绑定 trait
//圆的面积计算 pi*r的平方
impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn shape_name(&self) -> &'static str {
        "Circle"
    }
}

// 三角形结构体，面积数据>>底和高
struct Triangle {
    base: f64,
    height: f64,
}

// 绑定实现，底 * 高 ➗ 2
// 为了前后计算保持数据的一致，2 需要写成浮点型 2.0
impl Area for Triangle {
    fn calculate_area(&self) -> f64 {
        self.base * self.height / 2.0
    }
    fn shape_name(&self) -> &'static str {
        "Triangle"
    }
}

// 正方形结构体，面积数据>>边
struct Square {
    side: f64,
}

// 正方形面积计算公式
impl Area for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
    fn shape_name(&self) -> &'static str {
        "Square"
    }
}

// 定义一个打印面积的函数，接收实现了 Area trait 的参数
fn print_area<T: Area>(shape: T) {
    let area = shape.calculate_area();
    let shape_name = shape.shape_name();
    println!("{}'s area is {:.2}", shape_name, area);
}


fn main() {

    //作业1初始化并调用、打印
    let red_light = TrafficLight::Red(50);   
    let green_light = TrafficLight::Green(60);  
    let yellow_light = TrafficLight::Yellow(3);

    println!("Red light last: {}sec", red_light.last());
    println!("Green light last: {}sec", green_light.last());
    println!("Yellow light last: {}sec", yellow_light.last());

    //作业2给到数据，算出和
    //考虑从1加到9，初始化数据
    let numbers1 = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    //为了考虑溢出的情况，初始化一个 u32 类型最大的数
    let numbers2 = &[u32::MAX, 1];

    //测试正常数据
    match sum_u32(numbers1) {
        Some(result) => println!("Sum: {}", result),
        None => println!("数据溢出了"),
    }

    //溢出测试
    match sum_u32(numbers2) {
        Some(result) => println!("Sum: {}", result),
        None => println!("数据溢出了"),
    }

    //作业三，计算面积
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 5.0, height: 6.0 };
    let square = Square { side: 1.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);

}

