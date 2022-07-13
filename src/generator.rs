
use rand::prelude::*;

use chrono::prelude::*;

use crate::options_and_defaults::*;

///The generator that generates the file names
pub struct Generator
{

    rng: ThreadRng

}

impl Generator
{

    ///Creates a new generator
    pub fn new() -> Self
    {

        Generator{ rng: thread_rng() }

    }

    ///Recreates the ThreadRng struct used by the generator
    pub fn refresh(&mut self)
    {

        self.rng = thread_rng();

    }

    ///Creates the alphabetic portion of the file name
    fn get_ablphabetic_char(&mut self)  -> char
    {

        let res: char; 

        if self.rng.gen()
        {

            res = self.rng.gen_range(65..90).into();

        }
        else
        {
        
            res = self.rng.gen_range(97..122).into();
            
        }
    
        res

    }

    ///Creates a file name
    fn internal_generate(&mut self, mut rnd_string: String) -> String
    {

        self.append_date(&mut rnd_string);

        rnd_string.push_str("_");

        self.generate_append_alphanumeric_sequence(&mut rnd_string, DEFAULT_RANDOM_SEGMENT_LENGTH);

        rnd_string

    }

    fn generate_append_alphanumeric_sequence(&mut self, rnd_string: &mut String, length: i32)
    {

        let mut current_count = 0;

        while current_count < length
        {
            
            if self.rng.gen()
            {

                let char_rnd: char = self.get_ablphabetic_char();

                rnd_string.push(char_rnd);

            }
            else
            {

                let u8_rnd: u8 = self.rng.gen_range(0..9);
            
                rnd_string.push_str(u8_rnd.to_string().as_str());
                
            }

            current_count += 1;

        }

    }

    fn append_date(&mut self, rnd_string: &mut String)
    {

        let now = Local::today();

        //Append date

        rnd_string.push_str(format!("{:02}", now.day()).as_str());

        rnd_string.push_str(format!("{:02}", now.month()).as_str());

        rnd_string.push_str(now.year().to_string().as_str());

        //and maby add the option to append time as well at some point

    }

    ///Creates a file name
    pub fn generate(&mut self) -> String
    {

        //probably shouldn't unwrap, but it's fine for now

        let rnd_string = String::with_capacity((DEFAULT_RANDOM_SEGMENT_LENGTH + 9) as usize); //.try_into().unwrap());

        self.internal_generate(rnd_string)

    }

    ///Creates a file name using a pre-existing string
    pub fn clear_generate(&mut self, mut current_string: String) -> String
    {

        current_string.clear();

        self.internal_generate(current_string)

    }

}


