#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    // TODO: 定义一些消息类型，如下面示例中使用的
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
