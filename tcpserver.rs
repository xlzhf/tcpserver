use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];                                //声明一个buf数组
    
    for _ in 0..10 {                                     //获取客户端的10次输入
        let bytes_read = stream.read(&mut buf)?;         //将stream读入buf里面
        if bytes_read == 0 {                            //判断数据长度为0后返回Ok()
            return Ok(());
        }
        
        stream.write("server send message :".as_bytes())?;          //stream写入收到信息的标志
        stream.write(&buf[..bytes_read])?;                          //stream写入收到的客户端信息
        println!("receive client message : {}", str::from_utf8(&buf[..bytes_read])
            .expect("Could not write buffer as string"));            //打印客户端发送的消息
        thread::sleep(time::Duration::from_secs(1 as u64));          //让程序暂停1秒
    }

    Ok(())                                              //表示程序正常执行到最后
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;      //创建一个监听127.0.0.1:8080的变量

    for stream in listener.incoming() {                        //遍历连接到127.0.0.1:8080的连接
        match stream {                                         //模式匹配stream是否有异常
            Ok(d) => {                                         //没有异常
                handle_client(d).expect("error");              //调用handle_client()函数
            },
            Err(msg) => {                                       //有异常
                println!("connection error : {}", msg);         //打印错误信息
            }
        }
        
    }
    Ok(())                                                      //表示程序正常结束
}
