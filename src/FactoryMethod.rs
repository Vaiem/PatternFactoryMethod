pub mod FactoryMethod{
    use::std;

    pub struct Car;
    pub struct Track;
    pub struct CarWorkShop;
    pub struct TruckWorkShop;

    pub trait Production {
        fn release(&self);
    }

    pub trait WorkShop{
        fn create(&self) -> Box<dyn Production>;
    }

    impl Production for Car{

         fn release(&self){ 
            println!("release car");
         }

    }

    impl Production for Track{

        fn release(&self){
            println!("release Track");
        }

    }
     
    impl  WorkShop for CarWorkShop{
         fn create(&self) -> Box<dyn Production> {
            Box::new(Car)
         }
    }
    
    impl  WorkShop for TruckWorkShop {
        fn create(&self) -> Box<dyn Production>{
           Box::new(Track)
        }
    }

}