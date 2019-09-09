fn main() {

    //enumerate
    enum MessageEnum {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl MessageEnum {
        fn call(&self) {
        }
    }

    let m = MessageEnum::Write(String::from("hello"));
    m.call();

    //structure takes more line to construct this data structure
    // we have to specify each data type
    struct MessageStruct {
        Quit: QuitMessage,
        Move: MoveMessage,
        Write: WriteMessage,
        ChangeColor: ChangeColorMessage,
    }
    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);
}
