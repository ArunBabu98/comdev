pub struct Node{
    id:u32,
    data: String,
}

impl Node {
    pub fn new(id:u32, data:String) -> Node {
        Node { id, data }
    }
}