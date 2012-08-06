import module_of_many_things::*;
import dug::too::greedily::and::too::deep::*;

module module_of_many_things {
    export f1;
    export f2;
    export f4;
    fn f1() { debug!{"f1"}; }
    fn f2() { debug!{"f2"}; }
    fn f3() { debug!{"f3"}; }
    fn f4() { debug!{"f4"}; }
}

module dug {
    module too {
        module greedily {
            module and {
                module too {
                    module deep {
                        fn nameless_fear() { debug!{"Boo!"}; }
                        fn also_redstone() { debug!{"Whatever."}; }
                    }
                }
            }
        }
    }
}


fn main() { f1(); f2(); f4(); nameless_fear(); also_redstone(); }
