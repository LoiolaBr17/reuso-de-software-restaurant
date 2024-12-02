use restaurant::constants::PI;
use restaurant::back_of_house::take_care_trash;
use restaurant::front_of_house::hosting::add_to_wait_list;
use restaurant::front_of_house::serving::serve_order;

fn main() {
    add_to_wait_list();
    serve_order();
    println!("The value o PI is: {}", PI);
    take_care_trash();
}