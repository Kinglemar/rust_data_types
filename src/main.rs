fn main() {
    //##Scalar Data types

    //Integers
    let _x: u32 = 200_000; //Decimal
    let _x: i32 = 0xff; // HexDecimal
    let _x: i32 = 0o76; //  Octal
    let _x: i32 = 0b111_1101;
    let _x: u8 = b'A'; // ASCII value of A is 6

    // Float
    let _x: f64 = 3.142;

    //Boolean
    let _x: bool = true; //or false

    // Char
    let _x: char = 'Y'; //Unicode

    //## Compound types

    //Tuple
    let _tup: (bool, u32, &str) = (false, 640, "Technician");

    //Desctructuring a tuple
    let (_contains_auth, _uuid, _last_name) = _tup;
    let _x: u32 = _tup.1;

    //Arrays

    let error_codes: [u16; 5] = [500, 404, 301, 200, 400];
    let _bytes: [u8; 5] = [0; 5];
    println!("{:?}", error_codes);
    println!("{}", error_codes[3]);

    //***Function***
    println!("{}", return_remainder(200_000, 376));

    //***Control flow***
    println!("{}", conditionals());

    //*conditionsals */
    count_down_to_space();

    //Loops//
    iterate();
}

fn return_remainder(dividend: u32, divisor: u32) -> u32 {
    let remainder: u32 = dividend % divisor;
    remainder //Return remainder
}

fn conditionals() -> u16 {
    let mut increment = 0;

    let result = loop {
        // set expression result to result variable
        increment += 1;

        if increment >= 10 {
            break increment * 10; // Returning a value after loop
        }
    };

    result //Return the final value.
}

fn count_down_to_space() {
    let mut _x: u8 = 5;
    while _x != 0 {
        println!("{}", _x);
        _x -= 1;
    }

    println!("LIFTOFF!!");
}

fn iterate() {
    let x: [u32; 5] = [0x8; 5];

    //Loop an array
    for item in x.iter() {
        //Do something

        println!("{}", item);
    }

    /*Loop with range */

    for val in 1..5 {
        println!("{}", val);
    }
}
