use std::fmt;

struct TreeId(u32);

impl fmt::Display for TreeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

trait GardenTree {
    fn new(id: u32, age: u32, fruiting: bool) -> Self;
    fn should_transplant(&self) -> bool;
    fn get_id(&self) -> &TreeId;
    fn get_age(&self) -> u32;
    fn is_fruiting(&self) -> bool;
}

struct AppleTree {
    id: TreeId,
    age: u32,
    fruiting: bool,
}

impl GardenTree for AppleTree {
    fn new(id: u32, age: u32, fruiting: bool) -> AppleTree {
        AppleTree {
            id: TreeId(id),
            age,
            fruiting,
        }
    }

    fn should_transplant(&self) -> bool {
        self.age > 5 && !self.fruiting
    }

    fn get_id(&self) -> &TreeId {
        &self.id
    }

    fn get_age(&self) -> u32 {
        self.age
    }

    fn is_fruiting(&self) -> bool {
        self.fruiting
    }
}

struct CherryTree {
    id: TreeId,
    age: u32,
    fruiting: bool,
}

impl GardenTree for CherryTree {
    fn new(id: u32, age: u32, fruiting: bool) -> CherryTree {
        CherryTree {
            id: TreeId(id),
            age,
            fruiting,
        }
    }

    fn should_transplant(&self) -> bool {
        self.age > 3 && !self.fruiting
    }

    fn get_id(&self) -> &TreeId {
        &self.id
    }

    fn get_age(&self) -> u32 {
        self.age
    }

    fn is_fruiting(&self) -> bool {
        self.fruiting
    }
}

struct PearTree {
    id: TreeId,
    age: u32,
    fruiting: bool,
}

impl GardenTree for PearTree {
    fn new(id: u32, age: u32, fruiting: bool) -> PearTree {
        PearTree {
            id: TreeId(id),
            age,
            fruiting,
        }
    }

    fn should_transplant(&self) -> bool {
        self.age > 4 && !self.fruiting
    }

    fn get_id(&self) -> &TreeId {
        &self.id
    }

    fn get_age(&self) -> u32 {
        self.age
    }

    fn is_fruiting(&self) -> bool {
        self.fruiting
    }
}

fn main() {
    let apple_tree = AppleTree::new(1, 6, false);
    let cherry_tree = CherryTree::new(2, 4, true);
    let pear_tree = PearTree::new(3, 3, false);

    println!(
        "Apple Tree (ID: {}), Age: {}, Should Transplant: {}",
        apple_tree.get_id(),
        apple_tree.get_age(),
        apple_tree.should_transplant()
    );
    println!(
        "Cherry Tree (ID: {}), Age: {}, Should Transplant: {}",
        cherry_tree.get_id(),
        cherry_tree.get_age(),
        cherry_tree.should_transplant()
    );
    println!(
        "Pear Tree (ID: {}), Age: {}, Should Transplant: {}",
        pear_tree.get_id(),
        pear_tree.get_age(),
        pear_tree.should_transplant()
    );
}
