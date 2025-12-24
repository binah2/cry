use std::io;
use std::path;

//비밀번호 기반 키 추출 크레이트
use password_hash::Argon2;

//난수 생성 크레이트
use rand::rngs::StdRng;
use rand::rngs::OsRng;

//멀티스레드 라이브러리
use rayon::prelude::*;
use crossbeam_channel::{unbounded,bounded};

//헤더크기 설정 상수
const HEADER_SIZE: usize = 4096; //4kb

struct Ciper {
    file_route: String,
    ciper_route: String,
    password: String,
    algorithm: String,
    mode: String,
    iv: String,
    chunk_size: u64,
    m_cost: u32,
    t_cost: u32,
    parallel: u32
}

struct CiperMessage {
    offset: u64,
    data: Vec<u8>
}


fn make_key(password: String, m_cost: u32, t_cost: u32, parallel: u32,salt: String) -> String{
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::Version19, m_cost, t_cost, parallel);
    let key: String = argon2.hash_password(password, salt);
    return key;
}

fn make_salt(byte_size: u64) -> [u8; 64]{
    let rng = StdRng::from_os_rng();
    let mut salt:[u8; 64] = [0u8; 64];
    rng.try_fill_bytes(&mut self, &mut salt)
        .expect("OS Rng failed.");
    return salt
}

//헤더 내 파라미터의 구분자로는 $를 사용하며 헤더 종료는 @으로 선언한다
//만약 헤더 내용 길이가 HEADER_SIZE 보다 클 경우 헤더가 깨진다
fn create_header(header_size: usize, version: String, algorithm: String, mode: String, iv: String, chunk_size: String) -> String{
    let header: String = version + "$" + algorithm + "$" + mode + "$" + iv + "$" + chunk_size + "@";
    let mut header: Vec<u8> = header.as_bytes().to_vec();
    if header.len() < header_size{
        header.resize(header_size, 0);
    }
    else{
        panic!("Header is too big!");
    }
    return header;
}

fn set_file_route(file_name: &Path, file_route_opt: Option<&Path>) -> PathBuf{
    let mut route: PathBuf = file_name.to_path_buf();
    if let Some(file_route) = file_route_opt{
        route = PathBuf::from(file_route)
            .join(file_name.file_name());                
    }

    return route;
}

fn write_thread(file_route: PathBuf, header: String, ciper_pack: Ciper, rx: Receiver(CiperMessage)) -> Result<>{
    let mut file = File.create(file_route);
    
}

fn encryption_thread(plain_route: PathBuf, ciper_pack: Ciper, tx: Sender(CiperMessage)) -> Result<> {

}