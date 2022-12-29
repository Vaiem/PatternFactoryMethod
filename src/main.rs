use std;

use FactoryMethod::FactoryMethod::{CarWorkShop, WorkShop};
mod FactoryMethod;


fn main() {
   
    let workShop = FactoryMethod::FactoryMethod::CarWorkShop{};
    let mut product = workShop.create();
    product.release();
    
    let workShop = FactoryMethod::FactoryMethod::TruckWorkShop{};
    product = workShop.create();
    product.release();

    //println!("Hello, world!");
}
