// 生成一个trait，用于处理用户相关的业务逻辑
pub trait UserService {
    // 用户注册
    fn register(&self, username: &str, password: &str) -> Result<(), String>;

    // 用户登录
    fn login(&self, username: &str, password: &str) -> Result<(), String>;

    // 用户注销
    fn logout(&self) -> Result<(), String>; // 用户修改密码
}

// 实现UserService trait
pub struct UserServiceImpl;

impl UserService for UserServiceImpl {
    fn register(&self, username: &str, password: &str) -> Result<(), String> {
        // TODO: 实现用户注册逻辑
        Ok(())
    }

    fn login(&self, username: &str, password: &str) -> Result<(), String> {
        // TODO: 实现用户登录逻辑
        Ok(())
    }

    fn logout(&self) -> Result<(), String> {
        // TODO: 实现用户注销逻辑
        Ok(())
    }
}

// 测试UserService trait
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let service = UserServiceImpl;
        assert!(service.register("test", "password").is_ok());
    }

    #[test]
    fn test_login() {
        let service = UserServiceImpl;
        assert!(service.login("test", "password").is_ok());
    }
}