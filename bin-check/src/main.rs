fn main() {
    let (file_type, class, machine) = binrs::check_path("/bin/ls");

    println!("{:?} {:?} {:?}", file_type, class, machine);
}
