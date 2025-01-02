pub struct Config {
    pub func: String,
    pub input_filename: String,
    pub output_filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("NOT ENOUGH ARGUMENTS");
        }

        let func = args[1].clone();
        let filename = args[2].clone();

        let mut input_filename = String::new();
        let mut output_filename = String::new();
        if func == "pngrle" {
            input_filename = format!("test_images/{0}", filename);
            output_filename = format!("output_images/{0}.pnle", &filename[..filename.len() - 4]);
        }
        else if func == "depngrle" {
            input_filename = format!("output_images/{0}.pnle", &filename[..filename.len() - 4]);
            output_filename = format!("output_images/{0}.png", &filename[..filename.len() - 4]);
        }

        println!("Input filename is {0}", input_filename);
        println!("Output filename is {0}", output_filename);

        Ok(
            Config {
                func: func,
                input_filename: input_filename,
                output_filename: output_filename,
            }
        )
    }
}