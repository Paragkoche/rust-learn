pub struct Stack {
    top: usize,
    size: i128,
    data: Vec<i128>,
}

impl Stack {
    pub fn new(size: i128) -> Stack {
        Stack {
            top: 0,
            size,
            data: vec![0; size as usize],
        }
    }

    pub fn push(&mut self, data: i128) -> Result<(), &str> {
        if self.top >= self.size.try_into().unwrap() {
            return Err("over flow");
        }
        self.data[self.top] = data;
        self.top = self.top + 1;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<i128, &str> {
        if self.top == 0 {
            return Err("under flow");
        }
        self.top = self.top - 1;
        let value = self.data[self.top];
        self.data[self.top] = 0;
        Ok(value)
    }

    pub fn get_data(&self) -> Vec<i128> {
        self.data.clone()
    }
}

#[derive(Clone)]
struct link_lis {
    head: Option<*mut u32>,
    data: i128,
}
impl link_lis {
    pub fn new() -> link_lis {
        link_lis {
            data: 0,
            head: None,
        }
    }
    pub fn add_data(&mut self, data: i128, head: Option<*mut u32>) {
        self.data = data;
        self.head = head;
    }
}
pub struct list {
    array: Vec<link_lis>,
}

impl list {
    pub fn new(size: usize) -> list {
        list {
            array: vec![link_lis::new(); size],
        }
    }
    pub fn add_data(&mut self, data: i128) {
        let mut link: link_lis = link_lis::new();
        if self.array[0].data == 0 {
            self.array.clear();
            link.add_data(data, None);
        } else {
            link.add_data(
                data,
                self.array.last().map(|link| link as *const _ as *mut u32),
            );
        }
        self.array.push(link);
    }

    pub fn remove_data(&mut self, head: u32) {
        println!("data :{:?}", head as *mut u32);

        for i in 0..self.array.len() {
            println!("data :{} ", self.array[i].data);

            if self.array[i].head == Some(head as *mut u32) {
                print!("data :{}", self.array[i].data);
                self.array[i].data = 0;
                self.array[i].head = None;
                if i + 1 < self.array.len() {
                    self.array[i + 1].head =
                        Some(&self.array[i - 1] as *const link_lis as *mut u32);
                }
            }
        }
    }
    pub fn get_hash(&self, index: usize) -> Option<*mut u32> {
        let a = &self.array[index];
        println!("data :{:?}", a.head);
        a.head
    }

    pub fn show_data(&self) {
        for i in 0..self.array.len() {
            println!("{:?}:{}", self.array[i].head, self.array[i].data)
        }
    }
}
