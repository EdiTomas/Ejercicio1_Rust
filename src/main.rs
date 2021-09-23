


fn main() {
   //println!("Hello, world!");
   let mut person =Persona::new(String::from("Edi"),"Tomas".to_string(),200,String::from("45678912"),"2015".to_string()); 
   println!("Nombre :{}",person.get_nombre());
   println!("Apellido: {}",person.get_apellido());
   println!("Telefono: {}",person.get_telefono());
   println!("Edad: {}",person.get_edad());
   person.set_nombre(String::from("Yovani"));
   println!("Modificar Nombre: {}",person.get_nombre());
   println!("Year: {}",person.get_year());
   
   
}





pub struct Persona{
     nombre:String,
     apellido:String,
     edad: u64,
     telefono: String,
     ano:String,     
}

impl Persona{
    /*
    pub fn new(nombre:String,apellido:String,edad: u64,telefono: String,ano: String)->Self{
       Self{
           nombre,
           apellido,
           edad,
           telefono,
           ano,
       }
   }

   pub fn new(nombre:String,apellido:String,edad: u64,telefono: String,ano: String)->Self{
      Persona{
              nombre,
              apellido,
              edad,
              telefono,
              ano,
            }
   }
   */

   pub fn new(name:String,lastname:String,age: u64,phone: String,year: String)->Persona{
           Persona {nombre:name,
            apellido:lastname,
            edad:age,
            telefono:phone,
            ano:year,}
          }
   pub fn set_nombre(& mut self, nombre:String){
          self.nombre = nombre;
   } 
   pub fn get_year(&self)->String {
          String::from(self.ano.to_string())
   } 

   pub fn get_nombre(&self) -> String {
          return format!("{}",self.nombre);     
   } 
   pub fn get_apellido(&self) -> String {
          return format!("{}",self.apellido);     
   }
  
   pub fn get_edad(&self) -> u64 {
          return self.edad*2;        
   }
  
   pub fn get_telefono(&self) -> String {
          return format!("{}",self.telefono);     
   }
 


}




