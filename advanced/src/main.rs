use advanced;

fn main() {
    let meters = advanced::Meters(2);
    let millis = advanced::Millimeters(250);

    println!("{}", (millis + millis).0); // 500
    println!("{}", (millis + meters).0); // 2250
    println!("{}", (meters + millis).0); // truncates to 2
    println!("{}", (meters + meters).0); // 4
}
