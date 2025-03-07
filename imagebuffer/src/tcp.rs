use std::io::{Read, Write};
use std::net::TcpStream;

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

mod socket;

fn main() {
    // 启动服务器
    std::thread::spawn(|| {
        socket::start_server();
    });

    // 等待服务器启动
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 启动客户端
    socket::start_client();
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

mod socket;

fn main() {
    // 启动服务器
    std::thread::spawn(|| {
        socket::start_server();
    });

    // 等待服务器启动
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 启动客户端
    socket::start_client();
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

mod socket;

fn main() {
    // 启动服务器
    std::thread::spawn(|| {
        socket::start_server();
    });

    // 等待服务器启动
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 启动客户端
    socket::start_client();
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

pub fn start_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let msg = b"Hello, server!";
            stream.write(msg).unwrap();
            println!("Sent message: {:?}", msg);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Received message: {:?}", &buffer[..]);
                }
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
