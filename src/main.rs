use rand::prelude::*;
use serde_json::{json, Result, Value};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::vec::Vec;

fn shuffle(v: &Value, n: usize) -> &Value {
    let mut rng = thread_rng();
    let seed = rng.gen_range(0, n);
    &v[seed]
}

fn main() -> Result<()> {
    let filename = "D:/CodeWorkSpace/rust/rust_BullshitGenerator/data.json";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let v: Value = serde_json::from_str(&contents)?;
    let sv = json!(&v);
    // println!("{:?}",sv);
    let famous = &sv["famous"];
    let before = &sv["before"];
    let after = &sv["after"];
    let bosh = &sv["bosh"];
    let mut default_title = String::new();
    // let mut input = String::new();
    println!("请输入主题:");
    match io::stdin().read_line(&mut default_title) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{} 生成完成", default_title);
        }
        Err(error) => println!("error: {}", error),
    }

    default_title = default_title.replace("\n", &"").replace("\r", &"");
    let mut a_bosh;
    let mut a_famous;
    let mut a_before;
    let mut a_after;

    let mut vec: Vec<String> = Vec::new();
    let mut tmp = String::from("");
    let mut rng = thread_rng();
    let mut count_len = 0;
    while tmp.len() < 18000 {
        // println!("{}", tmp.len());
        let seed = rng.gen_range(0, 100);
        if count_len % 20 == 0 {
            tmp += &"\n    ";
        } else if seed < 20 {
            a_famous = shuffle(famous, 105);
            a_before = shuffle(before, 5);
            a_after = shuffle(after, 8);
            let a_famous = a_famous
                .to_string()
                .replace("a", &a_before.to_string())
                .replace("b", &a_after.to_string())
                .replace("\"", &"")
                .replace("\n", &"");
            tmp += &a_famous;
        } else {
            a_bosh = shuffle(bosh, 32)
                .to_string()
                .replace("\"", &"")
                .replace("\n", &"");

            if !vec.contains(&a_bosh) {
                vec.push(a_bosh.clone());
                tmp += &a_bosh;
            };
        }
        count_len += 1;
    }
    let tmp = tmp.replace("x", &default_title);

    let mut output: File = File::create("bosh.txt").unwrap();
    write!(output, "{}", tmp);
    Ok(())
}
