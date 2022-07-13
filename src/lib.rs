
mod generator;

pub use generator::*;

mod options_and_defaults;

pub use options_and_defaults::*;

#[cfg(test)]
mod tests {
    use crate::generator::Generator;

    #[test]
    fn rudimentary_test()
    {

        let mut gen = Generator::new();

        let name1 = gen.generate();

        let name2 = gen.generate();

        assert_ne!(name1, name2);

    }
}
