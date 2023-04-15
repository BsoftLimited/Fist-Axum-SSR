mod header;
pub use header::Header;

mod  body;
pub use body::Body;

/*macro_rules! layout {
    ($($arg:tt)*) => {
        
    };
}*/

pub trait Element{
    fn style(&self)->Option<String>;
    fn layout(&self)->String;
    fn script(&self)->Option<String>;
}