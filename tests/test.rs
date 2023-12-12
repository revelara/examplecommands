use cyberclub_io::CyberMessageIn;
use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(2, String::from("INIT MESSAGE"));

    let _res = program.send(2, CyberMessageIn::AddNewUser(3.into()));
}
