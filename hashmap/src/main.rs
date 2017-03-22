// hashmap
// https://rustcc.gitbooks.io/rustprimer/content/collections/hashmap.html
use std::collections::HashMap;

fn main() {
    // 声明 hasmap
    let mut come_from = HashMap::new();
    // 插入
    come_from.insert("quxl", "🇨🇳");
    come_from.insert("wangergou", "🇯🇵");
    come_from.insert("Sey", "🇺🇸");
    // 查找 key
    if !come_from.contains_key("Elton"){
        println!("Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
    }

    // 删除元素
    come_from.remove("wangergou");

    let who = ["quxl", "wan"];
    for person in &who{
        match come_from.get(person){
            Some(location) => println!("{}来自于{}", person, location),
            None => println!("{}无家可归", person),
        }
    }

    // 那么所有人呢
    for (person, location) in &come_from {
        println!("{}来自于{}", person, location);
    }

    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars(){
        let counter = letters.entry(ch).or_insert(0);
        *counter+=1;
    }
    for (key, value) in &letters {
        println!("{}:{}", key, value);
    }

}
