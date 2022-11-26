use dirs::home_dir;
#[derive(Debug)]
struct Reading {
    url: String,
    preview_text: String,
    date_added: String,
}

#[derive(Debug)]
pub struct ReadingList {
    list: Vec<Reading>,
}

impl ReadingList {
    pub fn print_values(&self) -> String {
        let mut res = String::from("");
        for item in &self.list {
            res += &item.preview_text;
            res += "\n";
            res += &item.url;
            res += "\n";
            res += &item.date_added;
            res += "\n";
            res += "\n";
        }

        res
    }

    pub fn get_reading_list() -> Self {
        let input_file =
            home_dir().unwrap().to_str().unwrap().to_owned() + "/Library/Safari/Bookmarks.plist";
        let contents = plist::Value::from_file(&input_file).expect("Error reading file");
        let json = serde_json::to_value(&contents).unwrap();
        let children = json.get("Children").unwrap();

        // get reading list from apple generated file
        let mut list = Vec::new();
        for child in children.as_array().unwrap() {
            let child = child.as_object().unwrap();
            let title = child.get("Title").unwrap().as_str().unwrap();
            if title == "com.apple.ReadingList" {
                list = child.get("Children").unwrap().as_array().unwrap().to_vec();
                break;
            }
        }
        let mut res = ReadingList { list: Vec::new() };
        // iterate through reading list
        for item in list {
            let item = item.as_object().unwrap();
            let reading_list_data = item.get("ReadingList").unwrap().as_object().unwrap();

            let url = String::from(item.get("URLString").unwrap().as_str().unwrap());
            print!("{:?}", reading_list_data);
            let default_string = serde_json::Value::String("".to_string());
            let preview_text = reading_list_data
                .get("PreviewText")
                .unwrap_or(&default_string)
                .as_str()
                .unwrap_or("")
                .to_string();
            let date_added = reading_list_data
                .get("DateAdded")
                .unwrap_or(&default_string)
                .as_str()
                .unwrap_or("")
                .to_string();

            let item = Reading {
                url,
                preview_text,
                date_added,
            };
            res.list.push(item);
        }

        res
    }
}
