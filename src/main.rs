fn main() {
    println!("Hello, world!");
    //variables
    // let -> immutable (unchanged)
    // let mut -> mutable
    // shadowing -> allows immutable to become mutable

    let name = "Yagmur";
    let surname = "...";

    println!("My name is {} {}!", name, surname);

    // integer -> signed integer i8, i16, i32, i64, i128
    // unsigned integer -> u8, u16, u32, u64, u128

    let number1 = 24;
    let number2: u32 = 16;

    let sum = number1 + number2;

    println!("sum of the two numbers is {} + {} = {}", number1, number2, sum);

    // &str -> string slice
    // String -> string

    // float -> f32, f64
    // bool -> true, false

    // struct -> structure -> class

    // enum -> enumeration -> Option(1, 2, 3, 4)

    struct Student {
        name: String,
        surname: String,
        age: i32
    }

    enum SoftwareEvent {
        NoAttendance,
        NoEvent,
        NoLocation
    }

    let student_info = Student {
        name: "Alfie".to_string(),
        surname: "...".to_string(),
        age: 22
    };

    println!("{} {} {}", student_info.name, student_info.surname, student_info.age);

    // println!("{:?}", student_info.name);

    // functions implementation

    let attendance = SoftwareEvent::NoAttendance;
    let evt = SoftwareEvent::NoEvent;
    let location = SoftwareEvent::NoLocation;

    event(attendance);
    event(evt);
    event(location);

    fn event(event: SoftwareEvent) {
        match event {
            SoftwareEvent::NoAttendance => println!("No attendance!"),
            SoftwareEvent::NoEvent => println!("No event!"),
            SoftwareEvent::NoLocation => println!("No location!"),
        };
    }
    }
