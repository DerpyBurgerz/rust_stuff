use linked_vector::LinkedVector;

pub fn test_dlv_list(){
    println!("Hello, world!");
    let mut dlv = dlv_list::VecList::new();
    let a  = dlv.push_back(0);
    dlv.push_back(1);
    dlv.push_back(2);
    dlv.push_back(3);
    dlv.push_back(4);

    let mut dlv2 = dlv_list::VecList::new();
    let index2 = dlv2.push_back(0);
    let val = dlv2.get(index2);

    println!("yay");
}
pub fn test_linked_vector(){
    println!("Hello, world!");
    let mut lv1 = LinkedVector::new();
    lv1.push_back(0);
    let h11 = lv1.push_back(1);
    lv1.push_back(2);
    lv1.push_back(3);
    lv1.push_back(4);
    let h1 = lv1.push_back(5);
    lv1.push_back(6);
    lv1.push_back(7);
    lv1.push_back(8);
    lv1.push_back(9);

    lv1.insert(h1, 15);
    lv1.remove(h1);
    let mut lv2 = LinkedVector::new();
    lv2.push_back(0);
    lv2.push_back(1);
    lv2.push_back(2);
    let h23 = lv2.push_back(3);
    let yay = lv2.get(h23);

    println!("yay");
}
