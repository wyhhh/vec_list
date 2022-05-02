use vec_list::*;
fn main() {
    // let mut l = vec_list![];
    //  let mut l = VecList::new();
    let mut l = vec_list![1, 2, 3];
    //  let mut l = vec_list!["abc".to_string(); 4];
    let a = l.push_back(1);
    let b = l.push_back(2);
    let c = l.push_back(3);
    let a = l.push_front(1);
    let b = l.push_front(2);
    let c = l.push_front(3);
    let x = l.pop_back();
    println!("{:?}", x);
    let x = l.pop_back();
    println!("{:?}", x);
    let x = l.pop_back();
    println!("{:?}", x);
    let x = l.pop_back();
    println!("{:?}", x);
    let x = l.pop_front();
    println!("{:?}", x);
    let x = l.pop_front();
    println!("{:?}", x);
    let x = l.pop_front();
    println!("{:?}", x);
    let x = l.pop_front();
    println!("{:?}", x);
    l.delete(a);
    l.delete(b);
    let c = l.push_back(4);
    let d = l.push_back(5);
    let e = l.push_back(6);
    l.delete(e);
    l.delete(d);

    for x in l.iter().rev() {
        println!("{:?}", x);
    }
    println!("{:?}", l.front());
    println!("{:?}", l.back());
    println!("{:#?}", l);
    println!("{}", l);
}
