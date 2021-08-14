pub mod input_and_output;
pub fn module_test(){

    println!("this is a module 1 implementation");
}
pub fn meter_to_km(  ) ->(f32, i32) {
    println!("Enter total meter that will converted into KM");
    let x=input_and_output::get_input_integer();    
        
             (x as f32/1000.0 ,x )
        

}
pub fn meter_to_inch() -> (f32,i32){
    println!("Enter total meter that will converted into inch");

    let x=input_and_output::get_input_integer();  

    (x as f32/ 0.0254  , x)
}
pub fn meter_to_cm() -> (f32,i32){
    println!("Enter total meter that will converted into Centimeter");

    let x=input_and_output::get_input_integer();  

    (x as f32* 100.0  , x)
}
pub fn centimeter_to_foot() -> (f32,i32){
    println!("Enter total Centimeter that will converted into Foot");

    let x=input_and_output::get_input_integer();  

    (x as f32/ 30.48  , x)
}
pub fn miles_to_km() -> (f32,i32){
    println!("Enter total miles that will converted into Kilometer");

    let x=input_and_output::get_input_integer();  

    (x as f32* 1.609 , x)
}