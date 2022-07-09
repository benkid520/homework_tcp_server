use std::io;
use std::io::{Read,Write};
use std::net::{IpAddr,Ipv4Addr,TcpListener,SocketAddr};
use std::str::FromStr;
//获取输入字符串函数
fn get_str()->String{
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Error");
    //截掉换行符
    input.trim().to_string()
}
//传入一个ip字符串 返回一个IpAddr对象
fn get_ip_addr(ip_str:String)->IpAddr{
    let ip = Ipv4Addr::from_str(&ip_str).expect("IP Error");
    IpAddr::V4(ip)
}
//传入一个字符串 把字符串转成符合端口格式的u16数据类型
fn get_port(port_str:String)->u16{
    port_str.parse::<u16>().expect("Port Error")
}
fn main() {
    //声明一些变量
    let mut input:String;
    let ip:IpAddr;
    let port:u16;
    //输入ip地址
    println!("Input IP address:");
    input = get_str();
    ip = get_ip_addr(input);
    //输入端口地址
    println!("Input port number:");
    input = get_str();
    port = get_port(input);
    //利用输入的ip和端口地址创建一个监听对象
    let listener = TcpListener::bind(SocketAddr::new(ip,port)).expect("Binding Failed");
    //输出提示
    println!("Your binding address is <{}> on listening",
        listener.local_addr()
            .expect("Error")
            .to_string()
    );
    //循环监听接入主机
    for stream in listener.incoming() {
        let mut stream = stream.expect("Connection Failed");
        //提示成功
        println!("Connected");
        //声明和定义名为buffer的u8数组
        let mut buffer=[0;1024];
        //利用buffer接受信息
        stream.read(&mut buffer).expect("Error");
        //解码并打印出屏幕
        println!("{}",String::from_utf8_lossy(&buffer));
        //发送信息相应连接的主机
        stream.write("I got it!".as_bytes()).expect("Error");
        println!("Disconnected");
    }
}
