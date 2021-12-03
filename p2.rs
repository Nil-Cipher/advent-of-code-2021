use std::fs;


fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let lines = data.lines();
    let mut aim = 0;
    let mut hori = 0;
    let mut verti = 0;
    for l in lines{
        let mut iter = l.splitn(2, ' ');
        let direction = iter.next().unwrap();
        let value = iter.next().unwrap().parse::<i32>().unwrap();
        match direction{
            "forward" =>{
                hori += value;
                verti += value * aim;
            }, 
            "up" => aim -= value,
            "down" => aim += value,
            _ => println!("oops"),
        }
    }
    println!("{}", hori * verti);
}