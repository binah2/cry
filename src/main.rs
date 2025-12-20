use std::io;

//비밀번호 기반 키 추출 크레이트
use password_hash::Argon2;

//난수 생성 크레이트
use rand::rngs::StdRng;
use rand::rngs::OsRng;


fn make_key(password: String, m_cost: u32, t_cost: u32, parallel: u32,salt: String) -> String{
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::Version19, m_cost, t_cost, parallel);
    let key: String = argon2.hash_password(password, salt);
    return key;
}

fn make_salt(byte_size: u64) -> [u8; 64]{
    let rng = StdRng::from_os_rng();
    let mut salt:[u8; 64] = [0u8; 64];
    rng.fill_bytes(&mut self, &mut salt);
    return salt
}

//헤더 내 파라미터의 구분자로는 $를 사용하며 헤더 종료는 @으로 선언한다
fn create_header(version: String, algorithm: String, mode: String, iv: String, chunk_size: String) -> String{
    let header: String = version + "$" + algorithm + "$" + mode + "$" + iv + "$" + chunk_size + "@";
    return header;
}


