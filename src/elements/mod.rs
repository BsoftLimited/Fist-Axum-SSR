mod header;
pub use header::Header;

mod home_body;
pub use home_body::HomeBody;

mod not_found_body;
pub use not_found_body::NotFoundBody;

mod about_body;
pub use about_body::AboutBody;

pub trait Element{
    fn style(&self)->Option<String>;
    fn layout(&self)->String;
    fn script(&self)->Option<String>;
}