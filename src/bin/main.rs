use rust::list::ReadingList;
pub fn main() {
    let list = ReadingList::get_reading_list();
    let printed_values = list.print_values();

    print!("{}", printed_values);
}
