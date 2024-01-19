// incomplete. will be updated as needed

#[derive(Debug)]
pub struct GeometryDashLevel {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

impl From<String> for GeometryDashLevel {
    fn from(value: String) -> Self {
        let mut id: u32 = 0;
        let mut name = Default::default();
        let mut description = None;

        println!("VALUE -> {}", value);
        for line in value.split('|').take(1) {
            println!("LINE -> {}", line);
            let mut split = line.split(':');

            println!("SPLIT -> {:?}", split);

            while let Some((key, value)) = split.next().zip(split.next()) {
                println!("KEY -> {:?}", key);
                println!("VALUE -> {:?}", value);
                match key {
                    "1" => {
                        id = value.parse().unwrap();
                    }
                    "2" => {
                        name = value.to_string();
                    }
                    "3" => {
                        description = Some(split.next().unwrap().to_string());
                    }
                    _ => {}
                }
            }
        }

        GeometryDashLevel {
            id,
            name,
            description,
        }
    }
}
