pub mod common;
pub mod contact;
pub mod favorite;
pub mod group;
pub mod login;
pub mod message;
pub mod moments;
pub mod personal;
pub mod tag;
pub mod video_account;

pub use common::*;
#[allow(ambiguous_glob_reexports)]
pub use contact::{info::*, manage::*, wecom::*};
pub use favorite::*;
#[allow(ambiguous_glob_reexports)]
pub use group::{admin::*, manage::*, member::*, settings::*};
pub use login::*;
pub use message::*;
#[allow(ambiguous_glob_reexports)]
pub use moments::{interact::*, manage::*, media::*, publish::*, settings::*, timeline::*};
pub use personal::{profile::*, safety::*, settings::*};
pub use tag::*;
#[allow(ambiguous_glob_reexports)]
pub use video_account::{
    common::*, follow::*, interact::*, message::*, profile::*, publish::*, scan::*, search::*,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // ===== common.rs tests =====
    #[test]
    fn test_app_id_serialize_deserialize() {
        let app_id = AppId("test_app_123".to_string());
        let json = serde_json::to_string(&app_id).unwrap();
        assert_eq!(json, "\"test_app_123\"");

        let deserialized: AppId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, app_id);
    }

    #[test]
    fn test_bot_id_from_string() {
        let bot_id: BotId = "wxid_test123".to_string().into();
        assert_eq!(bot_id.0, "wxid_test123");
    }

    #[test]
    fn test_bot_id_from_str() {
        let bot_id: BotId = "wxid_test456".into();
        assert_eq!(bot_id.0, "wxid_test456");
    }

    #[test]
    fn test_bot_id_serialize_deserialize() {
        let bot_id = BotId("wxid_abc".to_string());
        let json = serde_json::to_string(&bot_id).unwrap();
        assert_eq!(json, "\"wxid_abc\"");

        let deserialized: BotId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, bot_id);
    }

    #[test]
    fn test_bot_context_serialize() {
        let ctx = BotContext {
            app_id: AppId("app123".to_string()),
            token: "token456".to_string(),
            webhook_secret: Some("secret789".to_string()),
            description: Some("Test bot".to_string()),
        };
        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"token\":\"token456\""));
        assert!(json.contains("\"webhookSecret\":\"secret789\""));
        assert!(json.contains("\"description\":\"Test bot\""));
    }

    #[test]
    fn test_bot_context_deserialize() {
        let json = r#"{"appId":"app123","token":"token456","webhookSecret":"secret","description":"desc"}"#;
        let ctx: BotContext = serde_json::from_str(json).unwrap();
        assert_eq!(ctx.app_id.0, "app123");
        assert_eq!(ctx.token, "token456");
        assert_eq!(ctx.webhook_secret, Some("secret".to_string()));
        assert_eq!(ctx.description, Some("desc".to_string()));
    }

    #[test]
    fn test_bot_context_optional_fields() {
        let json = r#"{"appId":"app123","token":"token456"}"#;
        let ctx: BotContext = serde_json::from_str(json).unwrap();
        assert_eq!(ctx.webhook_secret, None);
        assert_eq!(ctx.description, None);
    }

    #[test]
    fn test_callback_envelope() {
        let envelope = CallbackEnvelope {
            appid: "app123".to_string(),
            data: "test_data".to_string(),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("\"appid\":\"app123\""));
        assert!(json.contains("\"data\":\"test_data\""));
    }

    #[test]
    fn test_api_envelope_with_data() {
        let envelope: ApiEnvelope<String> = ApiEnvelope {
            ret: 200,
            msg: "success".to_string(),
            data: Some("result".to_string()),
        };
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("\"ret\":200"));
        assert!(json.contains("\"msg\":\"success\""));
        assert!(json.contains("\"data\":\"result\""));
    }

    #[test]
    fn test_api_envelope_without_data() {
        let envelope: ApiEnvelope<String> = ApiEnvelope {
            ret: 400,
            msg: "error".to_string(),
            data: None,
        };
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("\"ret\":400"));
        assert!(!json.contains("\"data\""));
    }

    #[test]
    fn test_gewe_error_display() {
        let http_err = GeweError::Http("connection failed".to_string());
        assert_eq!(format!("{}", http_err), "http error: connection failed");

        let api_err = GeweError::Api {
            code: 500,
            message: "server error".to_string(),
        };
        assert_eq!(format!("{}", api_err), "api error code=500: server error");

        let decode_err = GeweError::Decode("invalid json".to_string());
        assert_eq!(format!("{}", decode_err), "decode error: invalid json");

        let missing_err = GeweError::MissingData;
        assert_eq!(format!("{}", missing_err), "missing data");
    }

    // ===== login.rs tests =====
    #[test]
    fn test_get_login_qr_code_request_serialize() {
        let req = GetLoginQrCodeRequest {
            app_id: "app123",
            r#type: "ipad",
            region_id: "CN",
            proxy_ip: Some("127.0.0.1:8080"),
            ttuid: None,
            aid: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"type\":\"ipad\""));
        assert!(json.contains("\"regionId\":\"CN\""));
        assert!(json.contains("\"proxyIp\":\"127.0.0.1:8080\""));
    }

    #[test]
    fn test_get_login_qr_code_response_deserialize() {
        let json = r#"{"qrData":"data","qrImgBase64":"base64","uuid":"uuid123","appId":"app123"}"#;
        let resp: GetLoginQrCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.qr_data, "data");
        assert_eq!(resp.qr_img_base64, "base64");
        assert_eq!(resp.uuid, "uuid123");
        assert_eq!(resp.app_id, "app123");
    }

    #[test]
    fn test_check_login_request_serialize() {
        let req = CheckLoginRequest {
            app_id: "app123",
            uuid: "uuid456",
            proxy_ip: None,
            captch_code: Some("1234"),
            auto_sliding: Some(true),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"uuid\":\"uuid456\""));
        assert!(json.contains("\"captchCode\":\"1234\""));
        assert!(json.contains("\"autoSliding\":true"));
    }

    #[test]
    fn test_check_login_response_deserialize() {
        let json =
            r#"{"uuid":"uuid123","status":1,"headImgUrl":"http://img.url","nickName":"test"}"#;
        let resp: CheckLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.uuid, "uuid123");
        assert_eq!(resp.status, 1);
        assert_eq!(resp.head_img_url, Some("http://img.url".to_string()));
        assert_eq!(resp.nick_name, Some("test".to_string()));
    }

    #[test]
    fn test_login_info_deserialize() {
        let json = r#"{"uin":123456,"wxid":"wxid_test","nickName":"Test","mobile":"13800000000","alias":"test_alias"}"#;
        let info: LoginInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.uin, Some(123456));
        assert_eq!(info.wxid, Some("wxid_test".to_string()));
        assert_eq!(info.nick_name, Some("Test".to_string()));
        assert_eq!(info.mobile, Some("13800000000".to_string()));
        assert_eq!(info.alias, Some("test_alias".to_string()));
    }

    #[test]
    fn test_dialog_login_request_serialize() {
        let req = DialogLoginRequest {
            app_id: "app123",
            region_id: "CN",
            proxy_ip: None,
            aid: Some("aid456"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"regionId\":\"CN\""));
        assert!(json.contains("\"aid\":\"aid456\""));
    }

    #[test]
    fn test_dialog_login_response_deserialize() {
        let json = r#"{"appId":"app123","uuid":"uuid456"}"#;
        let resp: DialogLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.app_id, "app123");
        assert_eq!(resp.uuid, "uuid456");
    }

    #[test]
    fn test_login_by_account_request_serialize() {
        let req = LoginByAccountRequest {
            app_id: "app123",
            proxy_ip: "127.0.0.1:8080",
            region_id: "CN",
            account: "user@example.com",
            password: "password123",
            step: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"proxyIp\":\"127.0.0.1:8080\""));
        assert!(json.contains("\"account\":\"user@example.com\""));
        assert!(json.contains("\"step\":1"));
    }

    #[test]
    fn test_set_callback_request_serialize() {
        let req = SetCallbackRequest {
            token: "token123",
            callback_url: "https://example.com/callback",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"token\":\"token123\""));
        assert!(json.contains("\"callbackUrl\":\"https://example.com/callback\""));
    }

    #[test]
    fn test_check_online_request_serialize() {
        let req = CheckOnlineRequest { app_id: "app123" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
    }

    #[test]
    fn test_reconnection_response_deserialize() {
        let json =
            r#"{"uuid":"uuid123","status":1,"headImgUrl":"http://img.url","nickName":"test"}"#;
        let resp: ReconnectionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.uuid, "uuid123");
        assert_eq!(resp.status, 1);
    }

    #[test]
    fn test_logout_request_serialize() {
        let req = LogoutRequest { app_id: "app123" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
    }

    // ===== tag/mod.rs tests =====
    #[test]
    fn test_add_label_request_serialize() {
        let req = AddLabelRequest {
            app_id: "app123",
            label_name: "Friends",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"labelName\":\"Friends\""));
    }

    #[test]
    fn test_label_info_deserialize() {
        let json = r#"{"labelName":"Friends","labelId":123}"#;
        let info: LabelInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.label_name, "Friends");
        assert_eq!(info.label_id, 123);
    }

    #[test]
    fn test_delete_label_request_serialize() {
        let req = DeleteLabelRequest {
            app_id: "app123",
            label_ids: "1,2,3",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"labelIds\":\"1,2,3\""));
    }

    #[test]
    fn test_list_label_response_deserialize() {
        let json = r#"{"labelList":[{"labelName":"Friends","labelId":1},{"labelName":"Family","labelId":2}]}"#;
        let resp: ListLabelResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.label_list.len(), 2);
        assert_eq!(resp.label_list[0].label_name, "Friends");
        assert_eq!(resp.label_list[1].label_id, 2);
    }

    #[test]
    fn test_modify_label_member_request_serialize() {
        let req = ModifyLabelMemberRequest {
            app_id: "app123",
            label_ids: "1,2",
            wx_ids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"labelIds\":\"1,2\""));
        assert!(json.contains("\"wxIds\":[\"wxid1\",\"wxid2\"]"));
    }

    // ===== contact/info.rs tests =====
    #[test]
    fn test_fetch_contacts_list_response_deserialize() {
        let json = r#"{"friends":["wxid1","wxid2"],"chatrooms":["room1"],"ghs":["gh1"]}"#;
        let resp: FetchContactsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.friends, vec!["wxid1", "wxid2"]);
        assert_eq!(resp.chatrooms, vec!["room1"]);
        assert_eq!(resp.ghs, vec!["gh1"]);
    }

    #[test]
    fn test_search_contacts_request_serialize() {
        let req = SearchContactsRequest {
            app_id: "app123",
            contacts_info: "test@example.com",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"contactsInfo\":\"test@example.com\""));
    }

    #[test]
    fn test_search_contacts_response_deserialize() {
        let json = r#"{"v3":"v3data","nickName":"Test","sex":1,"bigHeadImgUrl":"http://big.img","smallHeadImgUrl":"http://small.img"}"#;
        let resp: SearchContactsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.v3, "v3data");
        assert_eq!(resp.nick_name, "Test");
        assert_eq!(resp.sex, 1);
    }

    #[test]
    fn test_get_contact_brief_info_request_serialize() {
        let req = GetContactBriefInfoRequest {
            app_id: "app123",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"wxids\":[\"wxid1\",\"wxid2\"]"));
    }

    #[test]
    fn test_contact_brief_info_deserialize() {
        let json = r#"{"userName":"user1","nickName":"Nick","pyInitial":"N","quanPin":"nick","sex":1,"remark":"","remarkPyInitial":"","remarkQuanPin":"","alias":"","country":"CN","bigHeadImgUrl":"http://big.img","smallHeadImgUrl":"http://small.img","labelList":"1,2","province":"BJ","city":"BJ"}"#;
        let info: ContactBriefInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.user_name, "user1");
        assert_eq!(info.nick_name, "Nick");
        assert_eq!(info.sex, 1);
    }

    #[test]
    fn test_check_relation_request_serialize() {
        let req = CheckRelationRequest {
            app_id: "app123",
            wxids: vec!["wxid1"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
    }

    #[test]
    fn test_contact_relation_status_deserialize() {
        let json = r#"{"wxid":"wxid123","relation":1}"#;
        let status: ContactRelationStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.wxid, "wxid123");
        assert_eq!(status.relation, 1);
    }

    // ===== contact/manage.rs tests =====
    #[test]
    fn test_add_contacts_request_serialize() {
        let req = AddContactsRequest {
            app_id: "app123",
            scene: 1,
            option: 2,
            v3: "v3data",
            v4: "v4data",
            content: "Hello",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"scene\":1"));
        assert!(json.contains("\"v3\":\"v3data\""));
    }

    #[test]
    fn test_set_friend_remark_request_serialize() {
        let req = SetFriendRemarkRequest {
            app_id: "app123",
            wxid: "wxid456",
            remark: "Best Friend",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"wxid\":\"wxid456\""));
        assert!(json.contains("\"remark\":\"Best Friend\""));
    }

    #[test]
    fn test_set_friend_permissions_request_serialize() {
        let req = SetFriendPermissionsRequest {
            app_id: "app123",
            wxid: "wxid456",
            only_chat: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"onlyChat\":true"));
    }

    #[test]
    fn test_upload_phone_address_list_request_serialize() {
        let req = UploadPhoneAddressListRequest {
            app_id: "app123",
            phones: vec!["13800000001", "13800000002"],
            op_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"phones\":[\"13800000001\",\"13800000002\"]"));
        assert!(json.contains("\"opType\":1"));
    }

    #[test]
    fn test_delete_friend_request_serialize() {
        let req = DeleteFriendRequest {
            app_id: "app123",
            wxid: "wxid456",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"wxid\":\"wxid456\""));
    }

    // ===== contact/wecom.rs tests =====
    #[test]
    fn test_search_wecom_request_serialize() {
        let req = SearchWecomRequest {
            app_id: "app123",
            scene: 1,
            content: "test@company.com",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"scene\":1"));
        assert!(json.contains("\"content\":\"test@company.com\""));
    }

    #[test]
    fn test_search_wecom_response_deserialize() {
        let json = r#"{"nickName":"Test","sex":1,"signature":"Hello","bigHeadImgUrl":"http://big.img","smallHeadImgUrl":"http://small.img","v3":"v3","v4":"v4"}"#;
        let resp: SearchWecomResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.nick_name, "Test");
        assert_eq!(resp.v3, "v3");
        assert_eq!(resp.v4, "v4");
    }

    #[test]
    fn test_wecom_contact_deserialize() {
        let json = r#"{"userName":"user1","nickName":"Nick","remark":"","bigHeadImg":"http://big","smallHeadImg":"http://small","appId":"app1","descWordingId":"desc1"}"#;
        let contact: WecomContact = serde_json::from_str(json).unwrap();
        assert_eq!(contact.user_name, "user1");
        assert_eq!(contact.nick_name, "Nick");
    }

    #[test]
    fn test_wecom_contact_detail_deserialize() {
        let json = r#"{"userName":"user1","nickName":"Nick","remark":"","bigHeadImg":"http://big","smallHeadImg":"http://small","appId":"app1","descWordingId":"desc1","wording":"work","wordingPinyin":"gong","wordingQuanpin":"gongzuo"}"#;
        let detail: WecomContactDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.wording, "work");
    }

    // ===== message/send.rs tests =====
    #[test]
    fn test_send_text_request_serialize() {
        let req = SendTextRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            content: "Hello World",
            ats: Some("wxid789"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"toWxid\":\"wxid456\""));
        assert!(json.contains("\"content\":\"Hello World\""));
        assert!(json.contains("\"ats\":\"wxid789\""));
    }

    #[test]
    fn test_send_text_response_deserialize() {
        let json =
            r#"{"toWxid":"wxid456","createTime":1234567890,"msgId":123,"newMsgId":456,"type":1}"#;
        let resp: SendTextResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.to_wxid, "wxid456");
        assert_eq!(resp.create_time, 1234567890);
        assert_eq!(resp.msg_id, 123);
        assert_eq!(resp.new_msg_id, 456);
        assert_eq!(resp.msg_type, 1);
    }

    #[test]
    fn test_post_image_request_serialize() {
        let req = PostImageRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            img_url: "http://example.com/image.jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"imgUrl\":\"http://example.com/image.jpg\""));
    }

    #[test]
    fn test_post_image_response_deserialize() {
        let json = r#"{"toWxid":"wxid456","createTime":1234567890,"msgId":123,"newMsgId":456,"aesKey":"key","fileId":"file1","length":1000,"width":100,"height":200,"md5":"md5hash"}"#;
        let resp: PostImageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.aes_key, "key");
        assert_eq!(resp.width, 100);
        assert_eq!(resp.height, 200);
    }

    #[test]
    fn test_post_voice_request_serialize() {
        let req = PostVoiceRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            voice_url: "http://example.com/voice.mp3",
            voice_duration: 5000,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"voiceUrl\":\"http://example.com/voice.mp3\""));
        assert!(json.contains("\"voiceDuration\":5000"));
    }

    #[test]
    fn test_post_video_request_serialize() {
        let req = PostVideoRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            video_url: "http://example.com/video.mp4",
            thumb_url: "http://example.com/thumb.jpg",
            video_duration: 10000,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"videoUrl\":\"http://example.com/video.mp4\""));
        assert!(json.contains("\"thumbUrl\":\"http://example.com/thumb.jpg\""));
    }

    #[test]
    fn test_post_file_request_serialize() {
        let req = PostFileRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            file_url: "http://example.com/file.pdf",
            file_name: "document.pdf",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"fileUrl\":\"http://example.com/file.pdf\""));
        assert!(json.contains("\"fileName\":\"document.pdf\""));
    }

    #[test]
    fn test_post_link_request_serialize() {
        let req = PostLinkRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            title: "Article Title",
            desc: "Article description",
            link_url: "http://example.com/article",
            thumb_url: "http://example.com/thumb.jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"title\":\"Article Title\""));
        assert!(json.contains("\"linkUrl\":\"http://example.com/article\""));
    }

    #[test]
    fn test_post_emoji_request_serialize() {
        let req = PostEmojiRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            emoji_md5: "md5hash",
            emoji_size: 1024,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"emojiMd5\":\"md5hash\""));
        assert!(json.contains("\"emojiSize\":1024"));
    }

    #[test]
    fn test_post_app_msg_request_serialize() {
        let req = PostAppMsgRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            appmsg: "<xml>app message</xml>",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appmsg\":\"<xml>app message</xml>\""));
    }

    #[test]
    fn test_post_mini_app_request_serialize() {
        let req = PostMiniAppRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            mini_app_id: "miniapp123",
            display_name: "Mini App",
            page_path: "/pages/index",
            cover_img_url: "http://example.com/cover.jpg",
            title: "Mini App Title",
            user_name: "gh_miniapp",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"miniAppId\":\"miniapp123\""));
        assert!(json.contains("\"pagePath\":\"/pages/index\""));
    }

    #[test]
    fn test_post_name_card_request_serialize() {
        let req = PostNameCardRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            nick_name: "Contact Name",
            name_card_wxid: "wxid_card",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"nickName\":\"Contact Name\""));
        assert!(json.contains("\"nameCardWxid\":\"wxid_card\""));
    }

    // ===== message/download.rs tests =====
    #[test]
    fn test_download_image_request_serialize() {
        let req = DownloadImageRequest {
            app_id: "app123",
            xml: "<xml>image data</xml>",
            image_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"xml\":\"<xml>image data</xml>\""));
        assert!(json.contains("\"type\":1"));
    }

    #[test]
    fn test_download_image_response_deserialize() {
        let json = r#"{"fileUrl":"http://example.com/image.jpg"}"#;
        let resp: DownloadImageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.file_url, "http://example.com/image.jpg");
    }

    #[test]
    fn test_download_voice_request_serialize() {
        let req = DownloadVoiceRequest {
            app_id: "app123",
            xml: "<xml>voice data</xml>",
            msg_id: 12345,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"msgId\":12345"));
    }

    #[test]
    fn test_download_emoji_request_serialize() {
        let req = DownloadEmojiRequest {
            app_id: "app123",
            emoji_md5: "md5hash",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"emojiMd5\":\"md5hash\""));
    }

    #[test]
    fn test_download_emoji_response_deserialize() {
        let json = r#"{"url":"http://example.com/emoji.gif"}"#;
        let resp: DownloadEmojiResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.url, "http://example.com/emoji.gif");
    }

    #[test]
    fn test_download_cdn_request_serialize() {
        let req = DownloadCdnRequest {
            app_id: "app123",
            aes_key: "aeskey",
            file_id: "fileid",
            r#type: "image",
            total_size: "1024",
            suffix: "jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"aesKey\":\"aeskey\""));
        assert!(json.contains("\"fileId\":\"fileid\""));
    }

    // ===== message/forward.rs tests =====
    #[test]
    fn test_forward_image_request_serialize() {
        let req = ForwardImageRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            xml: "<xml>forward data</xml>",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"toWxid\":\"wxid456\""));
        assert!(json.contains("\"xml\":\"<xml>forward data</xml>\""));
    }

    #[test]
    fn test_forward_image_response_deserialize() {
        let json =
            r#"{"toWxid":"wxid456","msgId":123,"newMsgId":456,"aesKey":"key","fileId":"file1"}"#;
        let resp: ForwardImageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.aes_key, "key");
        assert_eq!(resp.file_id, "file1");
    }

    #[test]
    fn test_forward_mini_app_request_serialize() {
        let req = ForwardMiniAppRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            xml: "<xml>miniapp</xml>",
            cover_img_url: "http://example.com/cover.jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"coverImgUrl\":\"http://example.com/cover.jpg\""));
    }

    // ===== message/revoke.rs tests =====
    #[test]
    fn test_revoke_message_request_serialize() {
        let req = RevokeMessageRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            msg_id: "123",
            new_msg_id: "456",
            create_time: "1234567890",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"msgId\":\"123\""));
        assert!(json.contains("\"newMsgId\":\"456\""));
        assert!(json.contains("\"createTime\":\"1234567890\""));
    }

    // ===== group/member.rs tests =====
    #[test]
    fn test_invite_member_request_serialize() {
        let req = InviteMemberRequest {
            app_id: "app123",
            chatroom_id: "room456",
            reason: "Welcome",
            wxids: vec!["wxid1", "wxid2"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"chatroomId\":\"room456\""));
        assert!(json.contains("\"reason\":\"Welcome\""));
    }

    #[test]
    fn test_remove_member_request_serialize() {
        let req = RemoveMemberRequest {
            app_id: "app123",
            chatroom_id: "room456",
            wxids: vec!["wxid1"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"chatroomId\":\"room456\""));
    }

    #[test]
    fn test_get_chatroom_member_list_response_deserialize() {
        let json = r#"{"chatRoomOwner":"owner1","chatroomMembers":[{"wxid":"wxid1","inviterUserName":"inviter1","bigHeadImgUrl":"http://big.img","smallHeadImgUrl":"http://small.img","inviteTicket":"","memberFlag":0,"nickName":"Nick","remarkName":"","sex":1,"userName":"user1","displayName":""}]}"#;
        let resp: GetChatroomMemberListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chat_room_owner, "owner1");
        assert_eq!(resp.chatroom_members.len(), 1);
        assert_eq!(resp.chatroom_members[0].wxid, "wxid1");
    }

    #[test]
    fn test_get_chatroom_info_response_deserialize() {
        let json = r#"{"chatroomId":"room123","nickName":"Group Name","pyInitial":"GN","quanPin":"groupname","sex":0,"chatRoomNotify":1,"chatRoomOwner":"owner1","smallHeadImgUrl":"http://small.img","memberList":[]}"#;
        let resp: GetChatroomInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chatroom_id, "room123");
        assert_eq!(resp.nick_name, "Group Name");
    }

    // ===== group/admin.rs tests =====
    #[test]
    fn test_admin_operate_request_serialize() {
        let req = AdminOperateRequest {
            app_id: "app123",
            chatroom_id: "room456",
            wxid: "wxid789",
            is_admin: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"isAdmin\":true"));
    }

    // ===== group/settings.rs tests =====
    #[test]
    fn test_set_chatroom_announcement_request_serialize() {
        let req = SetChatroomAnnouncementRequest {
            app_id: "app123",
            chatroom_id: "room456",
            content: "Important announcement",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"content\":\"Important announcement\""));
    }

    #[test]
    fn test_get_chatroom_announcement_response_deserialize() {
        let json = r#"{"chatRoomAnnouncement":"Hello","sender":"wxid1","createTime":"1234567890","expireTime":"9999999999"}"#;
        let resp: GetChatroomAnnouncementResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chat_room_announcement, "Hello");
        assert_eq!(resp.sender, "wxid1");
    }

    #[test]
    fn test_get_chatroom_qr_code_response_deserialize() {
        let json =
            r#"{"qrImgBase64":"base64data","headImgBase64":"headbase64","qrUrl":"http://qr.url"}"#;
        let resp: GetChatroomQrCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.qr_img_base64, "base64data");
        assert_eq!(resp.qr_url, "http://qr.url");
    }

    #[test]
    fn test_save_contract_list_request_serialize() {
        let req = SaveContractListRequest {
            app_id: "app123",
            chatroom_id: "room456",
            save: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"save\":true"));
    }

    #[test]
    fn test_pin_chat_request_serialize() {
        let req = PinChatRequest {
            app_id: "app123",
            chatroom_id: "room456",
            add: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"add\":true"));
    }

    #[test]
    fn test_set_msg_silence_request_serialize() {
        let req = SetMsgSilenceRequest {
            app_id: "app123",
            chatroom_id: "room456",
            switch_: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"switch\":true"));
    }

    // ===== personal/profile.rs tests =====
    #[test]
    fn test_get_profile_request_serialize() {
        let req = GetProfileRequest { app_id: "app123" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
    }

    #[test]
    fn test_get_profile_response_deserialize() {
        let json = r#"{"alias":"myalias","wxid":"wxid123","nickName":"Nick","mobile":"138","uin":12345,"sex":1,"province":"BJ","city":"BJ","signature":"Hello","country":"CN","bigHeadImgUrl":"http://big.img","smallHeadImgUrl":"http://small.img","regCountry":"CN","snsBgImg":"http://bg.img"}"#;
        let resp: GetProfileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.alias, "myalias");
        assert_eq!(resp.wxid, "wxid123");
        assert_eq!(resp.uin, 12345);
    }

    #[test]
    fn test_update_profile_request_serialize() {
        let req = UpdateProfileRequest {
            app_id: "app123",
            nick_name: Some("NewNick"),
            country: Some("CN"),
            province: Some("BJ"),
            city: Some("BJ"),
            sex: Some(1),
            signature: Some("Hello World"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"nickName\":\"NewNick\""));
        assert!(json.contains("\"signature\":\"Hello World\""));
    }

    #[test]
    fn test_update_head_img_request_serialize() {
        let req = UpdateHeadImgRequest {
            app_id: "app123",
            head_img_url: "http://example.com/avatar.jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"headImgUrl\":\"http://example.com/avatar.jpg\""));
    }

    #[test]
    fn test_get_qr_code_response_deserialize() {
        let json = r#"{"qrCode":"base64qrdata"}"#;
        let resp: GetQrCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.qr_code, "base64qrdata");
    }

    // ===== personal/safety.rs tests =====
    #[test]
    fn test_get_safety_info_request_serialize() {
        let req = GetSafetyInfoRequest { app_id: "app123" };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
    }

    #[test]
    fn test_safety_device_record_deserialize() {
        let json =
            r#"{"uuid":"uuid123","deviceName":"iPhone","deviceType":"iOS","lastTime":1234567890}"#;
        let record: SafetyDeviceRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.uuid, "uuid123");
        assert_eq!(record.device_name, "iPhone");
        assert_eq!(record.last_time, 1234567890);
    }

    #[test]
    fn test_get_safety_info_response_deserialize() {
        let json = r#"{"list":[{"uuid":"uuid1","deviceName":"Device1","deviceType":"iOS","lastTime":123}]}"#;
        let resp: GetSafetyInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.list.len(), 1);
        assert_eq!(resp.list[0].uuid, "uuid1");
    }

    // ===== personal/settings.rs tests =====
    #[test]
    fn test_privacy_settings_request_serialize() {
        let req = PrivacySettingsRequest {
            app_id: "app123",
            option: 1,
            open: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"option\":1"));
        assert!(json.contains("\"open\":true"));
    }

    // ===== favorite/mod.rs tests =====
    #[test]
    fn test_sync_favor_request_serialize() {
        let req = SyncFavorRequest {
            app_id: "app123",
            sync_key: Some("key123"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"syncKey\":\"key123\""));
    }

    #[test]
    fn test_sync_favor_request_without_key() {
        let req = SyncFavorRequest {
            app_id: "app123",
            sync_key: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("syncKey"));
    }

    #[test]
    fn test_favor_summary_deserialize() {
        let json = r#"{"favId":123,"type":1,"flag":0,"updateTime":1234567890}"#;
        let summary: FavorSummary = serde_json::from_str(json).unwrap();
        assert_eq!(summary.fav_id, 123);
        assert_eq!(summary.favor_type, 1);
        assert_eq!(summary.update_time, 1234567890);
    }

    #[test]
    fn test_sync_favor_response_deserialize() {
        let json =
            r#"{"syncKey":"newkey","list":[{"favId":1,"type":1,"flag":0,"updateTime":123}]}"#;
        let resp: SyncFavorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.sync_key, "newkey");
        assert_eq!(resp.list.len(), 1);
    }

    #[test]
    fn test_get_favor_content_request_serialize() {
        let req = GetFavorContentRequest {
            app_id: "app123",
            fav_id: 456,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"favId\":456"));
    }

    #[test]
    fn test_get_favor_content_response_deserialize() {
        let json = r#"{"favId":123,"status":1,"flag":0,"updateTime":1234567890,"content":"favorite content"}"#;
        let resp: GetFavorContentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.fav_id, 123);
        assert_eq!(resp.content, "favorite content");
    }

    #[test]
    fn test_delete_favor_request_serialize() {
        let req = DeleteFavorRequest {
            app_id: "app123",
            fav_id: 456,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"favId\":456"));
    }

    // ===== moments/timeline.rs tests =====
    #[test]
    fn test_get_self_sns_list_request_serialize() {
        let req = GetSelfSnsListRequest {
            app_id: "app123",
            max_id: Some(12345),
            decrypt: Some(true),
            first_page_md5: Some("md5hash"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"maxId\":12345"));
        assert!(json.contains("\"decrypt\":true"));
        assert!(json.contains("\"firstPageMd5\":\"md5hash\""));
    }

    #[test]
    fn test_get_self_sns_list_request_optional_fields() {
        let req = GetSelfSnsListRequest {
            app_id: "app123",
            max_id: None,
            decrypt: None,
            first_page_md5: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("maxId"));
        assert!(!json.contains("decrypt"));
        assert!(!json.contains("firstPageMd5"));
    }

    #[test]
    fn test_get_contacts_sns_list_request_serialize() {
        let req = GetContactsSnsListRequest {
            app_id: "app123",
            wxid: "wxid456",
            max_id: Some(67890),
            decrypt: Some(false),
            first_page_md5: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"wxid\":\"wxid456\""));
        assert!(json.contains("\"maxId\":67890"));
    }

    #[test]
    fn test_get_sns_details_request_serialize() {
        let req = GetSnsDetailsRequest {
            app_id: "app123",
            sns_id: 999,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"snsId\":999"));
    }

    #[test]
    fn test_sns_like_entry_deserialize() {
        let json =
            r#"{"userName":"user1","nickName":"Nick","source":1,"type":2,"createTime":1234567890}"#;
        let entry: SnsLikeEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.user_name, "user1");
        assert_eq!(entry.nick_name, "Nick");
        assert_eq!(entry.source, 1);
        assert_eq!(entry.entry_type, 2);
        assert_eq!(entry.create_time, 1234567890);
    }

    #[test]
    fn test_sns_comment_entry_deserialize() {
        let json = r#"{"userName":"user2","nickName":"Nick2","source":2,"type":3,"content":"test comment","createTime":9876543210,"commentId":111,"replyCommentId":222,"isNotRichText":1}"#;
        let entry: SnsCommentEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.user_name, "user2");
        assert_eq!(entry.content, Some("test comment".to_string()));
        assert_eq!(entry.comment_id, Some(111));
        assert_eq!(entry.reply_comment_id, Some(222));
    }

    #[test]
    fn test_sns_timeline_item_deserialize() {
        let json = r#"{"id":1,"userName":"user1","nickName":"Nick","createTime":123,"snsXml":"<xml>test</xml>","likeCount":10,"commentCount":5,"withUserCount":2}"#;
        let item: SnsTimelineItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, 1);
        assert_eq!(item.user_name, "user1");
        assert_eq!(item.sns_xml, "<xml>test</xml>");
        assert_eq!(item.like_count, 10);
        assert_eq!(item.comment_count, 5);
    }

    #[test]
    fn test_sns_list_response_deserialize() {
        let json = r#"{"firstPageMd5":"md5","maxId":100,"snsCount":5,"requestTime":123456,"snsList":[{"id":1,"userName":"user1","nickName":"Nick","createTime":123,"snsXml":"<xml>test</xml>","likeCount":10,"commentCount":5,"withUserCount":2}]}"#;
        let resp: SnsListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.first_page_md5, "md5");
        assert_eq!(resp.max_id, 100);
        assert_eq!(resp.sns_count, 5);
        assert_eq!(resp.sns_list.len(), 1);
    }

    // ===== moments/media.rs tests =====
    #[test]
    fn test_upload_sns_image_request_serialize() {
        let req = UploadSnsImageRequest {
            app_id: "app123",
            img_urls: vec!["http://img1.jpg", "http://img2.jpg"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"imgUrls\":["));
        assert!(json.contains("\"http://img1.jpg\""));
    }

    #[test]
    fn test_uploaded_sns_image_deserialize() {
        let json = r#"{"fileUrl":"http://file.jpg","thumbUrl":"http://thumb.jpg","fileMd5":"md5hash","length":1024,"width":800,"height":600}"#;
        let img: UploadedSnsImage = serde_json::from_str(json).unwrap();
        assert_eq!(img.file_url, "http://file.jpg");
        assert_eq!(img.thumb_url, "http://thumb.jpg");
        assert_eq!(img.file_md5, "md5hash");
        assert_eq!(img.length, 1024);
        assert_eq!(img.width, 800);
        assert_eq!(img.height, 600);
    }

    #[test]
    fn test_upload_sns_video_request_serialize() {
        let req = UploadSnsVideoRequest {
            app_id: "app123",
            thumb_url: "http://thumb.jpg",
            video_url: "http://video.mp4",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"thumbUrl\":\"http://thumb.jpg\""));
        assert!(json.contains("\"videoUrl\":\"http://video.mp4\""));
    }

    #[test]
    fn test_upload_sns_video_response_deserialize() {
        let json = r#"{"fileUrl":"http://file.mp4","thumbUrl":"http://thumb.jpg","fileMd5":"md5","length":2048}"#;
        let resp: UploadSnsVideoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.file_url, "http://file.mp4");
        assert_eq!(resp.thumb_url, "http://thumb.jpg");
        assert_eq!(resp.file_md5, "md5");
        assert_eq!(resp.length, 2048);
    }

    #[test]
    fn test_download_sns_video_request_serialize() {
        let req = DownloadSnsVideoRequest {
            app_id: "app123",
            sns_xml: "<xml>video</xml>",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"snsXml\":\"<xml>video</xml>\""));
    }

    #[test]
    fn test_download_sns_video_response_deserialize() {
        let json = r#"{"fileUrl":"http://downloaded.mp4"}"#;
        let resp: DownloadSnsVideoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.file_url, "http://downloaded.mp4");
    }

    // ===== moments/interact.rs tests =====
    #[test]
    fn test_like_sns_request_serialize() {
        let req = LikeSnsRequest {
            app_id: "app123",
            sns_id: 456,
            oper_type: 1,
            wxid: "wxid789",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"snsId\":456"));
        assert!(json.contains("\"operType\":1"));
        assert!(json.contains("\"wxid\":\"wxid789\""));
    }

    #[test]
    fn test_comment_sns_request_serialize() {
        let req = CommentSnsRequest {
            app_id: "app123",
            sns_id: 456,
            oper_type: 1,
            wxid: "wxid789",
            comment_id: Some("cmt123"),
            content: Some("Great post!"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"snsId\":456"));
        assert!(json.contains("\"commentId\":\"cmt123\""));
        assert!(json.contains("\"content\":\"Great post!\""));
    }

    #[test]
    fn test_comment_sns_request_without_optional() {
        let req = CommentSnsRequest {
            app_id: "app123",
            sns_id: 456,
            oper_type: 2,
            wxid: "wxid789",
            comment_id: None,
            content: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("commentId"));
        assert!(!json.contains("content"));
    }

    // ===== moments/publish.rs tests =====
    #[test]
    fn test_sns_audience_serialize() {
        let audience = SnsAudience {
            allow_wxids: Some(vec!["wxid1", "wxid2"]),
            at_wxids: Some(vec!["wxid3"]),
            disable_wxids: None,
            allow_tag_ids: Some(vec!["tag1"]),
            disable_tag_ids: None,
            privacy: Some(true),
        };
        let json = serde_json::to_string(&audience).unwrap();
        assert!(json.contains("\"allowWxIds\":[\"wxid1\",\"wxid2\"]"));
        assert!(json.contains("\"atWxIds\":[\"wxid3\"]"));
        assert!(json.contains("\"privacy\":true"));
    }

    #[test]
    fn test_send_text_sns_request_serialize() {
        let req = SendTextSnsRequest {
            app_id: "app123",
            audience: SnsAudience::default(),
            content: "Hello moments!",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"content\":\"Hello moments!\""));
    }

    #[test]
    fn test_sns_image_info_serialize() {
        let info = SnsImageInfo {
            file_url: "http://file.jpg".to_string(),
            thumb_url: "http://thumb.jpg".to_string(),
            file_md5: "md5".to_string(),
            length: Some(1024),
            width: 800,
            height: 600,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"fileUrl\":\"http://file.jpg\""));
        assert!(json.contains("\"width\":800"));
    }

    #[test]
    fn test_send_img_sns_request_serialize() {
        let req = SendImgSnsRequest {
            app_id: "app123",
            audience: SnsAudience::default(),
            img_infos: vec![],
            content: Some("Image post"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"imgInfos\":[]"));
        assert!(json.contains("\"content\":\"Image post\""));
    }

    #[test]
    fn test_send_video_sns_request_serialize() {
        let video_info = SnsVideoInfo {
            file_url: "http://video.mp4".to_string(),
            thumb_url: "http://thumb.jpg".to_string(),
            file_md5: "vmd5".to_string(),
            length: Some(2048),
        };
        let req = SendVideoSnsRequest {
            app_id: "app123",
            audience: SnsAudience::default(),
            content: None,
            video_info,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"videoInfo\":{"));
        assert!(json.contains("\"fileUrl\":\"http://video.mp4\""));
    }

    #[test]
    fn test_send_url_sns_request_serialize() {
        let req = SendUrlSnsRequest {
            app_id: "app123",
            audience: SnsAudience::default(),
            content: Some("Check this out"),
            thumb_url: "http://thumb.jpg",
            link_url: "http://link.com",
            title: "Link Title",
            description: "Link description",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"linkUrl\":\"http://link.com\""));
        assert!(json.contains("\"title\":\"Link Title\""));
    }

    #[test]
    fn test_forward_sns_request_serialize() {
        let req = ForwardSnsRequest {
            app_id: "app123",
            audience: SnsAudience::default(),
            sns_xml: "<xml>forward</xml>",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"snsXml\":\"<xml>forward</xml>\""));
    }

    #[test]
    fn test_send_sns_response_deserialize() {
        let json = r#"{"id":123,"userName":"user1","nickName":"Nick","createTime":1234567890}"#;
        let resp: SendSnsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, 123);
        assert_eq!(resp.user_name, "user1");
        assert_eq!(resp.nick_name, "Nick");
        assert_eq!(resp.create_time, 1234567890);
    }

    // ===== moments/manage.rs tests =====
    #[test]
    fn test_delete_sns_request_serialize() {
        let req = DeleteSnsRequest {
            app_id: "app123",
            sns_id: 456,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"snsId\":456"));
    }

    // ===== moments/settings.rs tests =====
    #[test]
    fn test_stranger_visibility_request_serialize() {
        let req = StrangerVisibilityRequest {
            app_id: "app123",
            enabled: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"enabled\":true"));
    }

    #[test]
    fn test_set_sns_visible_scope_request_serialize() {
        let req = SetSnsVisibleScopeRequest {
            app_id: "app123",
            option: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"option\":1"));
    }

    #[test]
    fn test_set_sns_privacy_request_serialize() {
        let req = SetSnsPrivacyRequest {
            app_id: "app123",
            sns_id: 789,
            open: false,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"snsId\":789"));
        assert!(json.contains("\"open\":false"));
    }

    // ===== video_account/message.rs tests =====
    #[test]
    fn test_send_finder_msg_request_serialize() {
        let req = SendFinderMsgRequest {
            app_id: "app123",
            to_wxid: "wxid456",
            id: 789,
            username: "finder_user",
            nickname: "Finder Nick",
            head_url: "http://head.jpg",
            nonce_id: "nonce123",
            media_type: "video",
            width: "1920",
            height: "1080",
            url: "http://video.mp4",
            thumb_url: "http://thumb.jpg",
            thumb_url_token: "token123",
            description: "Video description",
            video_play_len: "60",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"toWxid\":\"wxid456\""));
        assert!(json.contains("\"mediaType\":\"video\""));
        assert!(json.contains("\"videoPlayLen\":\"60\""));
    }

    #[test]
    fn test_post_private_letter_request_serialize() {
        let req = PostPrivateLetterRequest {
            app_id: "app123",
            content: "Hello!",
            to_user_name: "user1",
            my_user_name: "myuser",
            msg_session_id: "session123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"content\":\"Hello!\""));
        assert!(json.contains("\"toUserName\":\"user1\""));
        assert!(json.contains("\"msgSessionId\":\"session123\""));
    }

    #[test]
    fn test_post_private_letter_img_request_serialize() {
        let req = PostPrivateLetterImgRequest {
            app_id: "app123",
            to_user_name: "user1",
            my_user_name: "myuser",
            msg_session_id: "session123",
            img_url: "http://img.jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"imgUrl\":\"http://img.jpg\""));
    }

    #[test]
    fn test_private_letter_response_deserialize() {
        let json = r#"{"newMsgId":123456}"#;
        let resp: PrivateLetterResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.new_msg_id, 123456);
    }

    #[test]
    fn test_sync_private_letter_msg_request_serialize() {
        let req = SyncPrivateLetterMsgRequest {
            app_id: "app123",
            key_buff: Some("keybuff123"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"keyBuff\":\"keybuff123\""));
    }

    #[test]
    fn test_mention_list_request_serialize() {
        let req = MentionListRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            req_scene: 2,
            last_buff: "buff123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"myUserName\":\"myuser\""));
        assert!(json.contains("\"reqScene\":2"));
    }

    #[test]
    fn test_contact_list_request_serialize() {
        let req = ContactListRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            query_info: "query123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"queryInfo\":\"query123\""));
    }

    #[test]
    fn test_contact_ext_info_deserialize() {
        let json = r#"{"country":"CN","province":"BJ","city":"Beijing","sex":1}"#;
        let info: ContactExtInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.country, "CN");
        assert_eq!(info.province, "BJ");
        assert_eq!(info.sex, 1);
    }

    #[test]
    fn test_contact_list_entry_deserialize() {
        let json = r#"{"username":"user1","nickname":"Nick","headUrl":"http://head.jpg","signature":"Hi"}"#;
        let entry: ContactListEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.username, Some("user1".to_string()));
        assert_eq!(entry.nickname, Some("Nick".to_string()));
    }

    // ===== video_account/profile.rs tests =====
    #[test]
    fn test_create_finder_request_serialize() {
        let req = CreateFinderRequest {
            app_id: "app123",
            nick_name: "MyFinder",
            head_img: "http://head.jpg",
            signature: Some("Hello world"),
            sex: Some(1),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"nickName\":\"MyFinder\""));
        assert!(json.contains("\"signature\":\"Hello world\""));
        assert!(json.contains("\"sex\":1"));
    }

    #[test]
    fn test_create_finder_response_deserialize() {
        let json = r#"{"username":"finder_user","nickName":"Nick","headUrl":"http://head.jpg","signature":"Hi","followFlag":0}"#;
        let resp: CreateFinderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.username, "finder_user");
        assert_eq!(resp.nick_name, "Nick");
        assert_eq!(resp.follow_flag, 0);
    }

    #[test]
    fn test_update_finder_profile_request_serialize() {
        let req = UpdateFinderProfileRequest {
            app_id: "app123",
            nick_name: Some("NewNick"),
            head_img: None,
            signature: Some("New signature"),
            sex: Some(2),
            country: Some("US"),
            province: None,
            city: None,
            my_user_name: "myuser",
            my_role_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"nickName\":\"NewNick\""));
        assert!(json.contains("\"signature\":\"New signature\""));
        assert!(!json.contains("headImg"));
    }

    #[test]
    fn test_get_finder_qr_code_request_serialize() {
        let req = GetFinderQrCodeRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"myUserName\":\"myuser\""));
    }

    #[test]
    fn test_get_finder_qr_code_response_deserialize() {
        let json = r#"{"qrcodeUrl":"http://qrcode.jpg"}"#;
        let resp: GetFinderQrCodeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.qrcode_url, "http://qrcode.jpg");
    }

    #[test]
    fn test_finder_alias_info_deserialize() {
        let json = r#"{"nickname":"AliasNick","headImgUrl":"http://head.jpg","roleType":2}"#;
        let info: FinderAliasInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.nickname, "AliasNick");
        assert_eq!(info.role_type, 2);
    }

    // ===== video_account/follow.rs tests =====
    #[test]
    fn test_follow_finder_request_serialize() {
        let req = FollowFinderRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            to_user_name: "touser",
            op_type: 1,
            search_info: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"myUserName\":\"myuser\""));
        assert!(json.contains("\"toUserName\":\"touser\""));
        assert!(json.contains("\"opType\":1"));
    }

    #[test]
    fn test_follow_finder_response_deserialize() {
        let json = r#"{"username":"user1","nickname":"Nick","headUrl":"http://head.jpg","signature":"Hi","followFlag":1,"auth_info":null,"coverImgUrl":"http://cover.jpg","spamStatus":0,"extFlag":0,"liveStatus":0,"liveCoverImgUrl":"","status":0}"#;
        let resp: FollowFinderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.username, "user1");
        assert_eq!(resp.follow_flag, 1);
    }

    #[test]
    fn test_follow_list_request_serialize() {
        let req = FollowListRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            last_buffer: Some("buffer123"),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"lastBuffer\":\"buffer123\""));
    }

    #[test]
    fn test_follow_list_data_deserialize() {
        let json = r#"{"contactList":[],"lastBuffer":"buff","continueFlag":1,"followCount":10}"#;
        let data: FollowListData = serde_json::from_str(json).unwrap();
        assert_eq!(data.last_buffer, "buff");
        assert_eq!(data.follow_count, 10);
    }

    #[test]
    fn test_search_follow_request_serialize() {
        let req = SearchFollowRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            to_user_name: "touser",
            keyword: "search",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"keyword\":\"search\""));
    }

    // ===== video_account/interact.rs tests =====
    #[test]
    fn test_finder_opt_request_serialize() {
        let req = FinderOptRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            to_user_name: "touser",
            op_type: 1,
            id: "id123",
            remain: 10,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"opType\":1"));
        assert!(json.contains("\"id\":\"id123\""));
        assert!(json.contains("\"remain\":10"));
    }

    #[test]
    fn test_browse_finder_request_serialize() {
        let req = BrowseFinderRequest {
            app_id: "app123",
            object_id: 456,
            session_buffer: Some("buffer"),
            object_nonce_id: "nonce123",
            my_user_name: "myuser",
            my_role_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"objectId\":456"));
        assert!(json.contains("\"objectNonceId\":\"nonce123\""));
    }

    #[test]
    fn test_id_fav_request_serialize() {
        let req = IdFavRequest {
            app_id: "app123",
            my_user_name: "myuser",
            op_type: 1,
            object_nonce_id: "nonce123",
            session_buffer: "buffer",
            object_id: 789,
            to_user_name: "touser",
            my_role_type: 1,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"objectId\":789"));
        assert!(json.contains("\"sessionBuffer\":\"buffer\""));
    }

    #[test]
    fn test_id_like_request_serialize() {
        let req = IdLikeRequest {
            app_id: "app123",
            object_id: 999,
            session_buffer: Some("buffer"),
            object_nonce_id: "nonce",
            op_type: 1,
            my_user_name: "myuser",
            my_role_type: 1,
            to_user_name: "touser",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"objectId\":999"));
        assert!(json.contains("\"opType\":1"));
    }

    #[test]
    fn test_like_fav_list_request_serialize() {
        let req = LikeFavListRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            last_buffer: None,
            flag: 2,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"flag\":2"));
        assert!(!json.contains("last_buffer"));
    }

    #[test]
    fn test_comment_finder_request_serialize() {
        let req = CommentFinderRequest {
            app_id: "app123",
            proxy_ip: "127.0.0.1",
            my_user_name: "myuser",
            op_type: 1,
            object_nonce_id: "nonce",
            session_buffer: "buffer",
            object_id: 123,
            my_role_type: 1,
            content: "Nice video!",
            comment_id: "cmt123",
            reply_user_name: "replyuser",
            ref_comment_id: 456,
            root_comment_id: 789,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"content\":\"Nice video!\""));
        assert!(json.contains("\"commentId\":\"cmt123\""));
    }

    #[test]
    fn test_comment_finder_response_deserialize() {
        let json = r#"{"commentId":"cmt456"}"#;
        let resp: CommentFinderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.comment_id, Some("cmt456".to_string()));
    }

    #[test]
    fn test_comment_list_request_serialize() {
        let req = CommentListRequest {
            app_id: "app123",
            object_id: 111,
            last_buffer: Some("buff"),
            session_buffer: "session",
            object_nonce_id: None,
            ref_comment_id: Some(222),
            root_comment_id: Some(333),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"objectId\":111"));
        assert!(json.contains("\"refCommentId\":222"));
    }

    // ===== video_account/publish.rs tests =====
    #[test]
    fn test_upload_finder_video_request_serialize() {
        let req = UploadFinderVideoRequest {
            app_id: "app123",
            video_url: "http://video.mp4",
            cover_img_url: "http://cover.jpg",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"videoUrl\":\"http://video.mp4\""));
        assert!(json.contains("\"coverImgUrl\":\"http://cover.jpg\""));
    }

    #[test]
    fn test_upload_finder_video_response_deserialize() {
        let json = r#"{"fileUrl":"http://file.mp4","thumbUrl":"http://thumb.jpg","mp4Identify":"mp4id","fileSize":2048,"thumbMD5":"tmd5","fileKey":"fkey"}"#;
        let resp: UploadFinderVideoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.file_url, "http://file.mp4");
        assert_eq!(resp.file_size, 2048);
        assert_eq!(resp.file_key, "fkey");
    }

    #[test]
    fn test_finder_video_cdn_deserialize() {
        let json = r#"{"fileUrl":"http://cdn.mp4","thumbUrl":"http://thumb.jpg","mp4Identify":"mp4","fileSize":1024,"thumbMD5":"md5","fileKey":"key"}"#;
        let cdn: FinderVideoCdn = serde_json::from_str(json).unwrap();
        assert_eq!(cdn.file_url, "http://cdn.mp4");
        assert_eq!(cdn.file_size, 1024);
    }

    #[test]
    fn test_publish_finder_cdn_request_serialize() {
        let video_cdn = FinderVideoCdn {
            file_url: "http://cdn.mp4".to_string(),
            thumb_url: "http://thumb.jpg".to_string(),
            mp4_identify: "mp4".to_string(),
            file_size: 1024,
            thumb_md5: "md5".to_string(),
            file_key: "key".to_string(),
        };
        let req = PublishFinderCdnRequest {
            app_id: "app123",
            topic: vec!["topic1", "topic2"],
            my_user_name: "myuser",
            my_role_type: 1,
            description: "Video desc",
            video_cdn,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"topic\":[\"topic1\",\"topic2\"]"));
        assert!(json.contains("\"description\":\"Video desc\""));
    }

    #[test]
    fn test_publish_finder_cdn_response_deserialize() {
        let json = r#"{"code":"0","msg":"success"}"#;
        let resp: PublishFinderCdnResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, "0");
        assert_eq!(resp.msg, Some("success".to_string()));
    }

    #[test]
    fn test_publish_finder_web_request_serialize() {
        let req = PublishFinderWebRequest {
            app_id: "app123",
            title: "Video Title",
            video_url: "http://video.mp4",
            thumb_url: "http://thumb.jpg",
            description: "Description",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"title\":\"Video Title\""));
        assert!(json.contains("\"description\":\"Description\""));
    }

    #[test]
    fn test_publish_finder_web_response_deserialize() {
        let json = r#"{"id":12345}"#;
        let resp: PublishFinderWebResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, 12345);
    }

    #[test]
    fn test_send_finder_sns_request_serialize() {
        let req = SendFinderSnsRequest {
            app_id: "app123",
            allow_wx_ids: vec!["wxid1"],
            at_wx_ids: vec!["wxid2"],
            disable_wx_ids: vec![],
            id: 789,
            username: "user",
            nickname: "Nick",
            head_url: "http://head.jpg",
            nonce_id: "nonce",
            media_type: "video",
            width: "1920",
            height: "1080",
            url: "http://video.mp4",
            thumb_url: "http://thumb.jpg",
            thumb_url_token: "token",
            description: "desc",
            video_play_len: "60",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"allowWxIds\":[\"wxid1\"]"));
        assert!(json.contains("\"id\":789"));
    }

    // ===== video_account/search.rs tests =====
    #[test]
    fn test_search_finder_request_serialize() {
        let req = SearchFinderRequest {
            app_id: "app123",
            content: "search query",
            category: Some(1),
            filter: Some(2),
            page: Some(1),
            cookie: Some("cookie123"),
            search_id: Some("sid123"),
            offset: Some(0),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"content\":\"search query\""));
        assert!(json.contains("\"category\":1"));
        assert!(json.contains("\"searchId\":\"sid123\""));
    }

    #[test]
    fn test_search_finder_request_minimal() {
        let req = SearchFinderRequest {
            app_id: "app123",
            content: "query",
            category: None,
            filter: None,
            page: None,
            cookie: None,
            search_id: None,
            offset: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(!json.contains("category"));
        assert!(!json.contains("filter"));
    }

    // ===== video_account/scan.rs tests =====
    #[test]
    fn test_scan_follow_request_serialize() {
        let req = ScanFollowRequest {
            app_id: "app123",
            proxy_ip: "127.0.0.1",
            my_user_name: "myuser",
            my_role_type: 1,
            qr_content: "qr123",
            object_id: "oid123",
            object_nonce_id: "nonce123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"qrContent\":\"qr123\""));
        assert!(json.contains("\"objectId\":\"oid123\""));
    }

    #[test]
    fn test_scan_browse_request_serialize() {
        let req = ScanBrowseRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            qr_content: "qr123",
            object_id: 456,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"objectId\":456"));
    }

    #[test]
    fn test_scan_fav_request_serialize() {
        let req = ScanFavRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            qr_content: "qr123",
            object_id: 789,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"objectId\":789"));
    }

    #[test]
    fn test_scan_comment_request_serialize() {
        let req = ScanCommentRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            qr_content: "qr123",
            object_id: 111,
            comment_content: "Nice!",
            reply_username: Some("replyuser"),
            ref_comment_id: Some(222),
            root_comment_id: Some(333),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"commentContent\":\"Nice!\""));
        assert!(json.contains("\"refCommentId\":222"));
    }

    #[test]
    fn test_scan_qr_code_request_serialize() {
        let req = ScanQrCodeRequest {
            app_id: "app123",
            my_user_name: "myuser",
            my_role_type: 1,
            qr_content: "qr123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"qrContent\":\"qr123\""));
    }

    #[test]
    fn test_scan_login_channels_request_serialize() {
        let req = ScanLoginChannelsRequest {
            app_id: "app123",
            qr_content: "qr123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"qrContent\":\"qr123\""));
    }

    #[test]
    fn test_scan_login_finder_info_deserialize() {
        let json = r#"{"finderUsername":"user","nickname":"Nick","headImgUrl":"http://head.jpg","spamFlag":0}"#;
        let info: ScanLoginFinderInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.finder_username, Some("user".to_string()));
        assert_eq!(info.nickname, Some("Nick".to_string()));
    }

    #[test]
    fn test_scan_login_channels_response_deserialize() {
        let json = r#"{"sessionId":"session123","finderList":[],"acctStatus":1}"#;
        let resp: ScanLoginChannelsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.session_id, "session123");
        assert_eq!(resp.acct_status, 1);
    }

    // ===== video_account/common.rs tests =====
    #[test]
    fn test_finder_ext_info_deserialize() {
        let json = r#"{"country":"CN","province":"BJ","city":"Beijing","sex":1}"#;
        let info: FinderExtInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.country, Some("CN".to_string()));
        assert_eq!(info.sex, Some(1));
    }

    #[test]
    fn test_finder_contact_profile_deserialize() {
        let json = r#"{"username":"user1","nickname":"Nick","headUrl":"http://head.jpg","signature":"Hi","auth_info":null}"#;
        let profile: FinderContactProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.username, "user1");
        assert_eq!(profile.nickname, "Nick");
        assert_eq!(profile.head_url, "http://head.jpg");
        assert_eq!(profile.signature, "Hi");
    }

    #[test]
    fn test_finder_search_info_serialize() {
        let info = FinderSearchInfo {
            cookies: Some("cookie123"),
            search_id: Some("sid123"),
            doc_id: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"cookies\":\"cookie123\""));
        assert!(json.contains("\"searchId\":\"sid123\""));
        assert!(!json.contains("docId"));
    }

    // ===== group/manage.rs tests =====
    #[test]
    fn test_create_chatroom_request_serialize() {
        let req = CreateChatroomRequest {
            app_id: "app123",
            wxids: vec!["wxid1", "wxid2", "wxid3"],
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"appId\":\"app123\""));
        assert!(json.contains("\"wxids\":[\"wxid1\",\"wxid2\",\"wxid3\"]"));
    }

    #[test]
    fn test_create_chatroom_response_deserialize() {
        let json = r#"{"headImgBase64":"base64data","chatroomId":"room123"}"#;
        let resp: CreateChatroomResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chatroom_id, "room123");
        assert_eq!(resp.head_img_base64, Some("base64data".to_string()));
    }

    #[test]
    fn test_create_chatroom_response_without_head_img() {
        let json = r#"{"chatroomId":"room456"}"#;
        let resp: CreateChatroomResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.chatroom_id, "room456");
        assert_eq!(resp.head_img_base64, None);
    }

    #[test]
    fn test_disband_chatroom_request_serialize() {
        let req = DisbandChatroomRequest {
            app_id: "app123",
            chatroom_id: "room456",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"chatroomId\":\"room456\""));
    }

    #[test]
    fn test_quit_chatroom_request_serialize() {
        let req = QuitChatroomRequest {
            app_id: "app123",
            chatroom_id: "room789",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"chatroomId\":\"room789\""));
    }

    #[test]
    fn test_modify_chatroom_name_request_serialize() {
        let req = ModifyChatroomNameRequest {
            app_id: "app123",
            chatroom_name: "New Group Name",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"chatroomName\":\"New Group Name\""));
        assert!(json.contains("\"chatroomId\":\"room123\""));
    }

    #[test]
    fn test_modify_chatroom_remark_request_serialize() {
        let req = ModifyChatroomRemarkRequest {
            app_id: "app123",
            chatroom_remark: "Important Group",
            chatroom_id: "room123",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"chatroomRemark\":\"Important Group\""));
    }

    #[test]
    fn test_modify_chatroom_nickname_for_self_request_serialize() {
        let req = ModifyChatroomNickNameForSelfRequest {
            app_id: "app123",
            chatroom_id: "room123",
            nick_name: "MyNickname",
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"nickName\":\"MyNickname\""));
        assert!(json.contains("\"chatroomId\":\"room123\""));
    }

    // ===== Default trait tests =====
    #[test]
    fn test_get_login_qr_code_response_default() {
        let resp = GetLoginQrCodeResponse::default();
        assert_eq!(resp.qr_data, "");
        assert_eq!(resp.uuid, "");
    }

    #[test]
    fn test_check_login_response_default() {
        let resp = CheckLoginResponse::default();
        assert_eq!(resp.status, 0);
        assert!(resp.login_info.is_none());
    }

    #[test]
    fn test_login_info_default() {
        let info = LoginInfo::default();
        assert!(info.uin.is_none());
        assert!(info.wxid.is_none());
    }

    #[test]
    fn test_label_info_default() {
        let info = LabelInfo::default();
        assert_eq!(info.label_name, "");
        assert_eq!(info.label_id, 0);
    }

    #[test]
    fn test_send_text_response_default() {
        let resp = SendTextResponse::default();
        assert_eq!(resp.to_wxid, "");
        assert_eq!(resp.msg_type, 0);
    }

    #[test]
    fn test_chatroom_member_default() {
        let member = ChatroomMember::default();
        assert_eq!(member.wxid, "");
        assert_eq!(member.sex, 0);
    }

    // ===== Clone tests =====
    #[test]
    fn test_app_id_clone() {
        let app_id = AppId("test".to_string());
        let cloned = app_id.clone();
        assert_eq!(app_id, cloned);
    }

    #[test]
    fn test_bot_context_clone() {
        let ctx = BotContext {
            app_id: AppId("app".to_string()),
            token: "token".to_string(),
            webhook_secret: None,
            description: None,
        };
        let cloned = ctx.clone();
        assert_eq!(ctx.app_id, cloned.app_id);
        assert_eq!(ctx.token, cloned.token);
    }

    // ===== Debug tests =====
    #[test]
    fn test_gewe_error_debug() {
        let err = GeweError::MissingData;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("MissingData"));
    }

    #[test]
    fn test_app_id_debug() {
        let app_id = AppId("test123".to_string());
        let debug_str = format!("{:?}", app_id);
        assert!(debug_str.contains("test123"));
    }

    // ===== Hash tests =====
    #[test]
    fn test_app_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AppId("test1".to_string()));
        set.insert(AppId("test2".to_string()));
        set.insert(AppId("test1".to_string())); // duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_bot_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BotId("bot1".to_string()));
        set.insert(BotId("bot2".to_string()));
        assert_eq!(set.len(), 2);
    }
}
