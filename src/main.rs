use clap::{Arg, Command};
use rand::Rng;
use std::io::Write;
use std::net::TcpListener;
use std::thread;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Latency{
    time: Vec<i32>,
}
impl Latency{
    fn new(time: Vec<i32>) -> Self{
        Self{
        time,
        }
    }
    fn tick(&mut self)  {
        self.time.reverse();
        self.time.pop();
        self.time.reverse();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dam {
    water_level: i32,
    turbine_speed: i32,
    power: i32,
    sensor1: i32,
    sensor2: i32,
    sensor3: i32,
}
impl Dam {
    pub fn new(water_level: i32, turbine_speed: i32, power: i32, sensor1: i32, sensor2: i32, sensor3: i32) -> Self {
        Self {
            water_level,
            turbine_speed,
            power,
            sensor1,
            sensor2,
            sensor3,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Fuelsilo {
    pressure: i32,
    fuel_level: i32,
    temp: i32,
    sensor1: i32,
    sensor2: i32,
    sensor3: i32,
}
impl Fuelsilo {
    pub fn new(pressure: i32, fuel_level: i32, temp: i32, sensor1: i32, sensor2: i32, sensor3: i32) -> Self {
        Self {
            pressure,
            fuel_level,
            temp,
            sensor1,
            sensor2,
            sensor3,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new("app")
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .help("Port you want to bind to.")
                .default_value("1337"),
        )
        .get_matches();
    let addr = format!("0.0.0.0:{}", app.get_one::<String>("port").unwrap());
    let server = TcpListener::bind(&addr)?;
    for connection in server.incoming() {
        let _handler = thread::spawn(|| {
        let mut conn = connection.unwrap();
        let vec = Vec::new();
        let mut latency = Latency::new(vec);
        println!("New connection from {:?}", conn.peer_addr().unwrap());
        loop {
                latency.time.push(get_one(1, 10));
                let dam = Dam::new(get_one(10,100),get_one(10,100),get_one(10,100),get_one(10,100),get_one(10,100),get_one(10,100)); 
                let silo = Fuelsilo::new(get_one(20, 40), get_one(50, 100), get_one(100, 200),get_one(20, 40), get_one(50, 100), get_one(1, 50));
                let silo_cereal = serde_json::to_string(&silo).unwrap();
                let dam_cereal = serde_json::to_string(&dam).unwrap();
                let latency_cereal = serde_json::to_string(&latency).unwrap();
                match conn.write(silo_cereal.as_bytes()) {
                    Ok(conn) => conn,
                    Err(_) => break,
                };
                match conn.write(dam_cereal.as_bytes()) {
                    Ok(conn) => conn,
                    Err(_) => break,
                };
                match conn.write(latency_cereal.as_bytes()) {
                    Ok(conn) => conn,
                    Err(_) => break,
                };
                match conn.write("\n".as_bytes()) {
                    Ok(conn) => conn,
                    Err(_) => break,
                };
                conn.flush().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(200));
                if latency.time.len() > 120 {
                   /* latency.time.reverse();
                    latency.time.pop();
                    latency.time.reverse();
                    */
                    latency.tick();
                }
            }
        });
        }
    Ok(())
}

fn get_one(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let var = rng.gen_range(min..max);
    var
}

