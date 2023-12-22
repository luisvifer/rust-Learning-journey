use std::{collections::LinkedList, usize};



/** Estructura conformada por una lista enlazada de pares y el número de elementos. 
 * Los pares son elementes genéricos K representa la clave de búsqueda y V el valor. 
 */
pub struct MyHashTable<K,V> {
    buckets:Vec<LinkedList<(K,V)>>,
    count: usize
}

/**
 * Definimos un trait para la función Hash que dadot un elemento Hashable devolverá 
 * una posición dentro de nuestra lista enlazada. 
 * Por tanto necesitaremos que el elemeneto K implemente este trait
 */
pub trait Hashable{
    fn hash(&self) -> usize;
}

/** Implementamos la estructura */
impl <K: Hashable + std::cmp::PartialEq + Clone,V: Clone> MyHashTable<K,V>{

    /**
     * Constructor que recibe el tamaño inicial de la estructura
     */
    pub fn new (initial_len: usize) -> MyHashTable<K,V> {
       
        MyHashTable {
            buckets: MyHashTable::create_linked_list_vector(initial_len),
             count: 0
            }
        
    }

   
/**
 *  Inserta pares clave-valor
 *   El método toma el control de ambos parámetros
 */
    pub fn insert(&mut self, key: K, value: V){

        let buffer_len = (self.buckets.len()  * 2 / 3) as f64;
        if self.count + 1 >= buffer_len.ceil() as usize {
            self.resize();
        }   
         let new_index = key.hash()% self.buckets.len();
        self.buckets[new_index].push_back((key, value)); 
        self.count += 1;
    }

    /**
     *  Inserta pares clave-valor
     *  El método toma el control de ambos parámetros
     */
    pub fn put(&mut self, key: K, value: V){
        
        let index = key.hash()% self.buckets.len();
        if  self.find(&key).is_some(){
            if let Some(list) = self.buckets.get_mut(index) {
                MyHashTable::update_element_from_linked_list(list, key, value);
            }  

        }else {
            self.insert(key, value)
            
        }
   
    }

    fn update_element_from_linked_list(list: &mut LinkedList<(K, V)>, key: K, new_value:V)  {
        
        // Crear una nueva lista enlazada, moviendo los elementos excepto el que se quiere eliminar
        let mut new_list = LinkedList::new();
      
        
    
       
            while let Some(pair) = list.pop_front() {
                if pair.0 == key{
                    new_list.push_back((pair.0,new_value.clone()));
                }else {
                new_list.push_back(pair);
                }
            }
        
    
        // Reemplazar la lista antigua con la nueva
        *list = new_list; 

    }
   

    /**
     * Comprueba que la clave se encuentra en la estructura
     */
    pub fn contains_key(&self, key:&K) -> bool {
        self.find(key).is_some()
    }
    /**
     * Busca la clave en la estructura
     */
    pub fn find(&self, key: &K) -> Option<&V> {
        let index = key.hash() % self.buckets.len();
        self.buckets[index]
        .iter()
        .find(|(k,_)| *k == *key)
        .map(|( _ ,v)| v)
    }

    /*
        *
     */
    pub fn remove(&mut self, key: K) -> Option<(K,V)> {
        let index = key.hash() % self.buckets.len();
        if let Some(list) = self.buckets.get_mut(index) {
            // Si se encontró un índice, elimina el elemento de la lista
            let result   =  MyHashTable::remove_element_from_linked_list(list, key);
                if let  Some((_,_)) = result {
                    self.count -= 1;
                }
                result
        }else {
    
        None
        }
    }
    fn remove_element_from_linked_list(list: &mut LinkedList<(K, V)>, key: K) -> Option<(K, V)> {
        let mut found = None;
    
        // Crear una nueva lista enlazada, moviendo los elementos excepto el que se quiere eliminar
        let mut new_list = LinkedList::new();
        while let Some((k, v)) = list.pop_front() {
            if k == key {
                found = Some((k, v));
                break;
            } else {
                new_list.push_back((k, v));
            }
        }
        
    
        // Mover los elementos restantes a la nueva lista
        if found.is_some() {
            while let Some(pair) = list.pop_front() {
                new_list.push_back(pair);
            }
        }
    
        // Reemplazar la lista antigua con la nueva
        *list = new_list; 
        found
    }


    fn create_linked_list_vector(initial_len: usize) -> Vec<LinkedList<(K, V)>> {
        let mut elements = Vec::with_capacity(initial_len);
        for _ in 0..initial_len {
            elements.push(LinkedList::new());
        }
        elements
    }
    fn resize (&mut self){
        let new_size = self.buckets.len() * 2;
        let mut new_buckets = MyHashTable::create_linked_list_vector(new_size);
        for elements in self.buckets.drain(..){
            for (key,value) in elements {
                let new_index = key.hash()%new_size;
                new_buckets[new_index].push_back((key,value)); 
            }
        }

        self.buckets = new_buckets;
       

    } 

   
}




    /** 
            let mut to_remove: Option<usize> = list
            .iter()
            .enumerate()
            .find(|(i,(k,_))| *k == key)
            .map(|(i,_)| i);
            */




/** Implementamos el trait Default para nuestro tipo MyHasTable
 * Nuestro tipo K tiene que ser Hashable y ordenable a través de PartialEq
  */
impl <K: Hashable + std::cmp::PartialEq + Clone,V: Clone> Default for MyHashTable<K,V> {
    fn default() -> Self {
        Self::new(2000)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq,Eq, Clone)]
    struct TestKey(usize);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
            self.0
        }
     }

     #[test]
     fn test_insert_and_find() {
        let mut my_hash_table = MyHashTable::new(2000);
        let key = TestKey(1); 
        let key_to_check = key.clone();
        let value: String = String::from("Hola mundo");
        let value_to_check = value.clone();
        my_hash_table.insert(key, value); 
        assert_eq!(my_hash_table.count, 1);
        let result = my_hash_table.find(&key_to_check); 
        assert_eq!(result, Some(&value_to_check)); 
    }



    #[test]
    fn test_find_non_exist(){
        let mut my_hash_table = MyHashTable::new(2000);
        let key = TestKey(1);
        let key_to_check =  TestKey(2); 
        let value: String = String::from("Hola mundo");
        my_hash_table.insert(key, value); 
        assert_eq!(my_hash_table.count, 1);
        let result = my_hash_table.find(&key_to_check); 
        assert_eq!(result, None); 
     

     
    }

    #[test]
    fn test_resize_needed(){
        let inital_size = 2;
        let mut my_hash_table = MyHashTable::new(inital_size);
        let key = TestKey(1);
        let value: String = String::from("Hola mundo");
        my_hash_table.insert(key, value);
        assert_eq!(my_hash_table.count, 1);
        assert_eq!(inital_size < my_hash_table.buckets.capacity(), true);
    }

    #[test]
    fn test_resize_no_needed(){
        let inital_size = 200;
        let mut my_hash_table = MyHashTable::new(inital_size);
        let key = TestKey(1); 
        let value: String = String::from("Hola mundo");
        my_hash_table.insert(key, value); 
        assert_eq!(my_hash_table.count, 1);
        assert_eq!(inital_size,  my_hash_table.buckets.capacity());
    }

    #[test]
    fn test_remove_exist(){
        let mut my_hash_table = MyHashTable::new(2000);
        let key = TestKey(1);
        let key_to_check = key.clone();
        let value: String = String::from("Hola mundo");
        let value_to_check = value.clone();
        my_hash_table.insert(key, value); 
        assert_eq!(my_hash_table.count, 1);
        let result = my_hash_table.remove(key_to_check.clone());
        assert_eq!(result, Some((key_to_check,value_to_check)));

      
        assert_eq!(my_hash_table.count, 0);
    }

    #[test]
    fn test_remove_non_exist(){
        let mut my_hash_table = MyHashTable::new(2000);
        let key = TestKey(1); 
        let key_to_check = TestKey(2);
        let value: String = String::from("Hola mundo");
        my_hash_table.insert(key, value); 
        assert_eq!(my_hash_table.count, 1);
        let result = my_hash_table.remove(key_to_check.clone());
        assert_eq!(result, None);

        assert_eq!(my_hash_table.count, 1);
    }

    #[test]
    fn test_put() {
        let mut my_hash_table = MyHashTable::new(2000);
        let key = TestKey(1); // Asegúrate de que TestKey implemente Hashable y PartialEq
        let value = "Hola mundo".to_string();

        // Insertar un nuevo valor y verificar que se insertó correctamente
        my_hash_table.put(key.clone(), value.clone());
        assert_eq!(my_hash_table.find(&key), Some(&value));
        assert_eq!(my_hash_table.count, 1);

        // Actualizar el valor existente y verificar que se actualizó
        let new_value = "Nuevo valor".to_string();
        my_hash_table.put(key.clone(), new_value.clone());
        assert_eq!(my_hash_table.find(&key), Some(&new_value));
        assert_eq!(my_hash_table.count, 1);
    }
    
}

