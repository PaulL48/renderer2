pub struct Uniform {
    staging_buffer: Buffer,
    buffer: Buffer,
}

pub trait UniformGroupSource {
    fn name(&self) -> &'static str;
    fn uniform_sources(&self) -> Iter<&[u8]>;
}

pub struct UniformGroup {

}

impl Uniform {

}
