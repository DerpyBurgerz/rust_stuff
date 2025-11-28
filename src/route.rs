use rand::prelude::*;
trait Route<T: HasAddress> {
    fn get_random_address(&self, rng: &mut ThreadRng) -> &T; // not sure if this should be Address, or T?
    fn add_address(&mut self, address: T, new_address: T);
    fn remove_address(&mut self, address: T);

}
type Address = u16;
trait HasAddress{
    fn address(&self) -> Address;
}
struct Node {
    address: Address,
    index: usize,
    prev: usize,
    next: usize,
}
impl HasAddress for Node {
    fn address(&self) -> Address {
        self.address
    }
}
pub struct ArrayRoute {
    start_address: Address,
    route: Vec<Node>,
    head: Node,
}

impl Route<Node> for ArrayRoute{
    fn get_random_address(&self, rng:&mut ThreadRng) -> &Node{
        let length = self.route.len();
        let index = rng.random_range(0..length);
        &self.route[index]
    }
    fn add_address(&mut self, mut address: Node, mut new_address: Node) {
        // we will add the new address at the end of the list
        // the index will thus be len+1
        let new_address_index = self.route.len() + 1;
        new_address.index = new_address_index;

        // update the address in the new address
        new_address.next = self.route[address.next].index;
        new_address.prev = address.index;

        // update nodes in front and behind our new node
        self.route[address.next].prev = new_address_index;
        address.next = new_address_index;

        // add the address to the back of the list
        self.route.push(new_address);
    }

    fn remove_address(&mut self, address: Node) {
        // update address in front and behind
        self.route[address.prev].next = address.next;
        self.route[address.next].prev = address.next;

        self.route[address.index] = self.route.pop().unwrap();
    }
}