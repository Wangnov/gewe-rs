use crate::client::GeweHttpClient;
use gewe_core::{AdminOperateRequest, GeweError};
use tracing::instrument;

impl GeweHttpClient {
    #[instrument(skip(self))]
    pub async fn admin_operate(&self, req: AdminOperateRequest<'_>) -> Result<(), GeweError> {
        let _ = self
            .post_api::<_, ()>("gewe/v2/api/group/adminOperate", &req)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use gewe_core::AdminOperateRequest;

    #[test]
    fn test_admin_operate_request_serialization() {
        let req = AdminOperateRequest {
            app_id: "test_app",
            chatroom_id: "room123@chatroom",
            wxid: "wxid_test123",
            is_admin: true,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("appId"));
        assert!(json.contains("test_app"));
        assert!(json.contains("chatroomId"));
        assert!(json.contains("room123@chatroom"));
        assert!(json.contains("wxid"));
        assert!(json.contains("wxid_test123"));
        assert!(json.contains("isAdmin"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_admin_operate_request_field_names() {
        let req = AdminOperateRequest {
            app_id: "app",
            chatroom_id: "room",
            wxid: "wx123",
            is_admin: false,
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // 验证字段名是 camelCase
        assert!(value.get("appId").is_some());
        assert!(value.get("chatroomId").is_some());
        assert!(value.get("wxid").is_some());
        assert!(value.get("isAdmin").is_some());
        assert_eq!(value.get("isAdmin").unwrap().as_bool().unwrap(), false);
    }

    #[test]
    fn test_admin_operate_request_set_admin_true() {
        let req = AdminOperateRequest {
            app_id: "app_id_123",
            chatroom_id: "34757816141@chatroom",
            wxid: "wxid_0xsqb3o0tsvz22",
            is_admin: true,
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value.get("appId").unwrap().as_str().unwrap(), "app_id_123");
        assert_eq!(
            value.get("chatroomId").unwrap().as_str().unwrap(),
            "34757816141@chatroom"
        );
        assert_eq!(
            value.get("wxid").unwrap().as_str().unwrap(),
            "wxid_0xsqb3o0tsvz22"
        );
        assert_eq!(value.get("isAdmin").unwrap().as_bool().unwrap(), true);
    }

    #[test]
    fn test_admin_operate_request_set_admin_false() {
        let req = AdminOperateRequest {
            app_id: "app_id_123",
            chatroom_id: "34757816141@chatroom",
            wxid: "wxid_0xsqb3o0tsvz22",
            is_admin: false,
        };

        let json = serde_json::to_string(&req).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value.get("isAdmin").unwrap().as_bool().unwrap(), false);
    }

    #[test]
    fn test_admin_operate_request_with_special_chars() {
        let req = AdminOperateRequest {
            app_id: "测试应用",
            chatroom_id: "房间123@chatroom",
            wxid: "微信号",
            is_admin: true,
        };

        let json = serde_json::to_string(&req).unwrap();
        // 验证 Unicode 字符能正确序列化
        assert!(json.contains("测试应用") || json.contains("\\u"));
    }

    #[test]
    fn test_admin_operate_request_clone() {
        let req = AdminOperateRequest {
            app_id: "app",
            chatroom_id: "room",
            wxid: "wx123",
            is_admin: true,
        };

        let cloned = req.clone();
        assert_eq!(req.app_id, cloned.app_id);
        assert_eq!(req.chatroom_id, cloned.chatroom_id);
        assert_eq!(req.wxid, cloned.wxid);
        assert_eq!(req.is_admin, cloned.is_admin);
    }

    #[test]
    fn test_admin_operate_request_debug() {
        let req = AdminOperateRequest {
            app_id: "app",
            chatroom_id: "room",
            wxid: "wx123",
            is_admin: true,
        };

        let debug_str = format!("{:?}", req);
        assert!(debug_str.contains("AdminOperateRequest"));
        assert!(debug_str.contains("app"));
        assert!(debug_str.contains("room"));
        assert!(debug_str.contains("wx123"));
    }

    #[test]
    fn test_admin_operate_request_deserialization() {
        let json = r#"{
            "appId": "test_app",
            "chatroomId": "room@chatroom",
            "wxid": "wxid_test",
            "isAdmin": true
        }"#;

        let req: AdminOperateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.app_id, "test_app");
        assert_eq!(req.chatroom_id, "room@chatroom");
        assert_eq!(req.wxid, "wxid_test");
        assert_eq!(req.is_admin, true);
    }
}
