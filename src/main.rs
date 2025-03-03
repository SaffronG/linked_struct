use std::env;
#[derive(PartialEq, Copy, Clone)]
struct Node {                       // SIZE 2.4mb
    qlast: u16,
    qnext: u16,
}

struct Plist {                      // SIZE 3.6mb
    processes: [Option<Node>; 100], // SIZE 3.2mb [4bytes * 100]
    size: u16,                      // SIZE 2 bytes
    count: u16,                     // SIZE 2 bytes
}

fn main() -> Result<(), &'static str> {
    let mut processes = Plist::init();
    let args: Vec<String> = env::args().collect(); // ARG [1] is function, args[2] is uid
    if args.len() < 3 {
        return Err("ERROR: NOT ENOUGH ARGUMENTS")
    }
    if args.len() > 3 {
        return Err("ERROR: TOO MANY ARGUMENTS")
    }
    let uid: u16 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("ERROR: UID IS NOT A NUMBER"),
    };
    if uid > 99 {
        return Err("ERROR: UID IS OUT OF BOUNDS");
    }
    match args[1].as_str() {
        "create" => {
            processes.create(uid)?;
        }
        "kill" => {
            processes.kill(uid)?;
        }
        "running" => {
            processes.running();
        }
        "get" => {
            let node = processes.get(uid)?;
            println!("PROCESS {} -> qlast: {}, qnext: {}", uid, node.qlast, node.qnext);
        }
        _ => {
            return Err("ERROR: INVALID FUNCTION");
        }
    }
    Ok(())
}

impl Node {
    fn new(last: u16, next: u16) -> Self {
        Self {
            qlast: last,
            qnext: next,
        }
    }
}

impl Plist {
   fn init() -> Self {
        Self {
            processes: [None; 100],
            size: 100,
            count: 0,
        }
   }
   fn create(&mut self, uid: u16) -> Result<(), &'static str> {
        let mut last: Option<u16> = None;
        let mut next: Option<u16> = None;

        if uid >= self.size {
            return Err("ERROR: UID IS OUT OF BOUNDS");
        }
        if self.processes[uid as usize].is_some() {
            return Err("ERROR: PROCESS WITH THAT PID ALREADY EXISTS");
        }
        if self.count >= self.size {
            return Err("ERROR: PROCESS LIST IS FULL");
        }

        for (i, pid) in self.processes.iter().enumerate() {
            if let Some(_) = pid {
                if i < uid as usize {
                    if last.is_none() || (i as u32) > last.unwrap() as u32 {
                        last = Some(i as u16);
                    }
                } else if i > uid as usize {
                    if next.is_none() || (i as u32) < next.unwrap() as u32 {
                        next = Some(i as u16);
                    }
                }
            }
        }

        if let Some(last_idx) = last {
            if let Some(ref mut node) = self.processes[last_idx as usize] {
                node.qnext = uid as u16
            }
        }
        if let Some(next_idx) = next {
            if let Some(ref mut node) = self.processes[next_idx as usize] {
                node.qlast = uid as u16
            }
        }
        self.processes[uid as usize] = Some(Node::new(
            last.unwrap_or(uid as u16),
            next.unwrap_or(uid as u16),
        ));
        self.count += 1;
        Ok(())
   }
   fn kill(&mut self, uid: u16) -> Result<(), &'static str> {
        if self.count == 0 {
            return Err("ERROR: PROCESS LIST IS EMPTY");
        }
        let (left, right) = match self.processes[uid as usize] {
            Some(ref node) => (node.qlast, node.qnext),
            _ => return Err("ERROR: PROCESS WITH THAT PID DOES NOT EXIST"),
        };
        match self.processes[left as usize] {
            Some(ref mut node) => {
                node.qnext = right;
            }
            _ => return Err("ERROR: LEFT NODE DOES NOT EXIST"),
        }
        match self.processes[right as usize] {
            Some(ref mut node) => {
                node.qlast = left;
            }
            _ => return Err("ERROR: RIGHT NODE DOES NOT EXIST"),
            
        }
        self.processes[uid as usize] = None;
        self.count -= 1;
        Ok(())
   }
   fn get(&self, uid: u16) -> Result<Node, &'static str> {
        match self.processes[uid as usize] {
            Some(ref node) => Ok(*node),
            _ => Err("ERROR: PROCESS WITH THAT PID DOES NOT EXIST"),
        }
   }
   fn running(&self) {
        let mut i = 0;
        for pid in self.processes {
            match pid {
                Some(node) => {
                    println!("PROCESS {i} -> qlast: {}, qnext: {}", node.qlast, node.qnext);
                }
                None => {
                }
            }
            i += 1;
        }
   }
}
