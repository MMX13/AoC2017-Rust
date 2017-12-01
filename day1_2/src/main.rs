use std::io;

fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	let mut seq: Vec<i32> = vec![];
	for i in input.trim().chars(){
		seq.push(i.to_digit(10).unwrap() as i32)
	}
    let mut total = 0;

    for (i, &val) in seq.iter().enumerate(){
    	let next = (i + seq.len()/2) % seq.len();
    	if val == seq[next]{
    		total = total + val;
    	}
    }
    println!("{:?}", total);
}

