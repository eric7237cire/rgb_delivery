
#[macro_export]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[cfg( not(target_arch = "x86_64"))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg( target_arch = "x86_64")]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!( $( $t )* );
    }
}

macro_rules! log_trace {
     ( $( $t:tt )* ) => {
        //println!( $( $t )* );
        //web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

//macro_rules! expr_nop { ($body:expr) => { $body } }

//macro_rules! noop { () => (); }