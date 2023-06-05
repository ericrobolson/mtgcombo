mod carddb;
mod combo;
mod io;

fn main() {
    io::init_dirs();
    carddb::CardDb::load();

    // Fetch a combo
    let resp = combo::fetch("Phyrexian Altar");
}
