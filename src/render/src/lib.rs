pub mod sprite_shader;
pub mod window_instance;
pub mod window;
pub mod texture;
pub mod material;
mod shader;
mod gl_error;
mod vbo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
