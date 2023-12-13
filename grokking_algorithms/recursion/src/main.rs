use rand::Rng; // Importar Rng para generar números aleatorios
use std::*;


#[derive(Clone,Debug)]
struct MyBox {
    key: bool,
    nested_boxes: Vec<Box<MyBox>>,
   
}

trait KeyCheck {
    fn has_key(&self) -> bool;
}

impl KeyCheck for MyBox {
    fn has_key(&self) -> bool {
        self.key
    }
}
trait BoxManagement {
    fn add_box(&mut self, box_to_add: Box<MyBox>);
}

impl BoxManagement for MyBox {
    fn add_box(&mut self, box_to_add: Box<MyBox>) {
        self.nested_boxes.push(box_to_add);
    }
}
trait DisplayStructure {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result;
}
impl DisplayStructure for MyBox {
    fn display(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let indentation = " ".repeat(depth * 2);
        writeln!(f, "{}MyBox(Key: {}, Nested Boxes: {})", indentation, self.key, self.nested_boxes.len())?;

        for nested_box in &self.nested_boxes {
            nested_box.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl fmt::Display for MyBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        self.display(f, 0)
    }
}

// Función para crear una estructura de cajas anidadas aleatoria
fn create_random_boxes(max_depth: usize, current_depth: usize, key_placed: &mut bool) -> MyBox {
    let mut rng = rand::thread_rng();
    let has_key = if !*key_placed && (current_depth == max_depth || rng.gen_bool(1.0 / (max_depth as f64))) {
        *key_placed = true;
        true
    } else {
        false
    };

    let mut current_box = MyBox { nested_boxes: Vec::new(), key: has_key };

    if current_depth < max_depth {
        let num_boxes = rng.gen_range(0..=3);
        for _ in 0..num_boxes {
            let nested_box = create_random_boxes(max_depth, current_depth + 1, key_placed);
            current_box.add_box(Box::new(nested_box));
        }
    }

    current_box
}

#[derive(Debug)]
struct MyBoxIterator {
    stack: Vec<Box<MyBox>>,
}
impl IntoIterator for MyBox {
    type Item = Box<MyBox>;
    type IntoIter = MyBoxIterator;

    fn into_iter(self) -> Self::IntoIter {
        MyBoxIterator { stack: vec![Box::new(self)] }
    }
}

impl Iterator for MyBoxIterator {
    type Item = Box<MyBox>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|current_box| {
            self.stack.extend(current_box.nested_boxes.iter().cloned());
            current_box
        })
    }
}




fn main() {
    let mut key_placed = false;
    let random_box_structure = create_random_boxes(3, 0, &mut key_placed);
    println!("Caja creada con {} cajas anidadas", random_box_structure.nested_boxes.len());
    println!("Estructura de MyBox:\n{}", random_box_structure);
    println!("Estructura flatten de MyBox:\n");
    for element in random_box_structure.clone().into_iter(){
         println!("{:?}", element);
    }

    match  find_key_iter(random_box_structure.clone().into_iter()) {
        Ok(num) => println!("Se encontró la caja en la iteración {}", num),
        Err(num) => println!("No se encontró la caja en las {} iteración", num),
    }

    let  structure = Box::new(random_box_structure.clone());
    match  find_key_with_recursion(structure,1) {
        Ok(num) => println!("Se encontró la caja en la iteración {}", num),
        Err(num) => println!("No se encontró la caja en las {} iteración", num),
    }
    println!("Cuenta atrás");
    count_down_recursive(10);
    println!("Factorial");
    println!("-> {}",factorial(10));
    

  
}


fn find_key_iter (into_iter: MyBoxIterator)-> Result<i32, i32> {
    let mut count:i32 =0;
    for element in into_iter{
        count = count +1;
        if element.has_key(){
            return Ok(count);
        }
       
        
    }
    Err(count)

}

fn find_key_with_recursion( current_box :Box<MyBox>, mut count :i32) -> Result<i32,i32> {
    if current_box.has_key() {
       return  Ok(count);
    } 
    for nested_box in current_box.nested_boxes {
        match find_key_with_recursion(nested_box, count + 1 ) {
            Ok(pos) => return Ok(pos),
            Err(pos) => { count = pos;
            continue }
        }
    }
    
    return Err(count);
    
}

fn count_down_recursive (counter_down: i32) {
    if counter_down > 0 {
        print!("{},",counter_down);
        count_down_recursive(counter_down-1)
    }
    else {
        println!("{}.",counter_down)
    }
}

fn factorial (number:i128)-> i128{
    if number == 0{
        print!("{}",1);
        return 1;
    }
    else {
        print!("{} * ",number);
        return number * factorial(number - 1);
    }

}