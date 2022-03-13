struct Operations {
    vect: Vec<i32>,
}

impl Operations {
    fn new() -> Operations {
        Operations { vect: vec![] }
    }

    fn add(&mut self, n: i32) {
        let mut pos = 0;
        for i in 0..self.vect.len() {
            if self.vect[i] < n {
                pos = i;
            }
        }

        self.vect.insert(pos, n);
        self.vect.sort();
    }

    fn remove(&mut self, m: i32){
        let mut poz = 0;
        for i in 0..self.vect.len(){
            if self.vect[i] == m{
                poz = i;
            }
        }
        self.vect.remove(poz);
        self.vect.sort();

    }

    fn display(&self){
        println!("{:?}", self.vect);
    }
}

fn main() {
    let mut vec = Operations::new();
    vec.add(4);
    vec.add(10);
    vec.add(35);
    vec.add(5);
    vec.remove(5);
    vec.display();
}

// fn operations(){
//     let mut vec = Vec::new();
//     vec.push(21);
//     vec.push(13);
//     vec.push(9);
//     vec.remove(1);
//     vec.sort();
//     println!("{:?}", vec);
// }
