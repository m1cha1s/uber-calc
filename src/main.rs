use rui::*;

mod uber_calc;
use uber_calc::*;

fn main() {
    let mut calc = UberCalc::new();

    calc.push_value(3.0).push_value(7.0).add();
    println!("{}", calc.get_value_stack()[0]);

    calc.push_input_buffer("1")
        .push_input_buffer("0")
        .push_input_buffer(".")
        .push_input_buffer("1")
        .flush_input_buffer()
        .add();

    println!("{}", calc.get_value_stack()[0]);

    // rui(vstack((
    //     circle().color(RED_HIGHLIGHT).padding(Auto),
    //     rectangle()
    //         .corner_radius(5.0)
    //         .color(AZURE_HIGHLIGHT)
    //         .padding(Auto),
    // )))
}
