pub mod input_and_output;
pub fn module_test(){

    println!("this is a module 1 implementation");
}
pub fn meter_to_km(  ) ->(f32, i32) {
    let x=input_and_output::get_input_integer();    
        
             (x as f32/1000.0 ,x )
        

}