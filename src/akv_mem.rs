use libactionkv::mem::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
  akv_mem.exe FILE get KEY
  akv_mem.exe FILE delete KEY VALUE
  akv_mem.exe FILE insert KEY VALUE
  akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
  akv_mem FILE get KEY
  akv_mem FILE delete KEY VALUE
  akv_mem FILE insert KEY VALUE
  akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE);
    let key = args.get(3).expect(USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("Could not open file");
    store.load().expect("Could not load file");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },

        "delete" => store.delete(key).upwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }

        _ => eprintln!("{}", &USAGE),
    }
}
