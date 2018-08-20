// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_API_ACTIVATE: ::grpcio::Method<super::auth::ActivateRequest, super::auth::ActivateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/Activate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DEACTIVATE: ::grpcio::Method<super::auth::DeactivateRequest, super::auth::DeactivateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/Deactivate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_CONFIGURATION: ::grpcio::Method<super::auth::GetConfigurationRequest, super::auth::GetConfigurationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetConfiguration",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_SET_CONFIGURATION: ::grpcio::Method<super::auth::SetConfigurationRequest, super::auth::SetConfigurationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/SetConfiguration",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_ADMINS: ::grpcio::Method<super::auth::GetAdminsRequest, super::auth::GetAdminsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetAdmins",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_MODIFY_ADMINS: ::grpcio::Method<super::auth::ModifyAdminsRequest, super::auth::ModifyAdminsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/ModifyAdmins",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_AUTHENTICATE: ::grpcio::Method<super::auth::AuthenticateRequest, super::auth::AuthenticateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/Authenticate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_AUTHORIZE: ::grpcio::Method<super::auth::AuthorizeRequest, super::auth::AuthorizeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/Authorize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_WHO_AM_I: ::grpcio::Method<super::auth::WhoAmIRequest, super::auth::WhoAmIResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/WhoAmI",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_SCOPE: ::grpcio::Method<super::auth::GetScopeRequest, super::auth::GetScopeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetScope",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_SET_SCOPE: ::grpcio::Method<super::auth::SetScopeRequest, super::auth::SetScopeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/SetScope",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_ACL: ::grpcio::Method<super::auth::GetACLRequest, super::auth::GetACLResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetACL",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_SET_ACL: ::grpcio::Method<super::auth::SetACLRequest, super::auth::SetACLResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/SetACL",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_AUTH_TOKEN: ::grpcio::Method<super::auth::GetAuthTokenRequest, super::auth::GetAuthTokenResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetAuthToken",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_EXTEND_AUTH_TOKEN: ::grpcio::Method<super::auth::ExtendAuthTokenRequest, super::auth::ExtendAuthTokenResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/ExtendAuthToken",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_REVOKE_AUTH_TOKEN: ::grpcio::Method<super::auth::RevokeAuthTokenRequest, super::auth::RevokeAuthTokenResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/RevokeAuthToken",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_SET_GROUPS_FOR_USER: ::grpcio::Method<super::auth::SetGroupsForUserRequest, super::auth::SetGroupsForUserResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/SetGroupsForUser",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_MODIFY_MEMBERS: ::grpcio::Method<super::auth::ModifyMembersRequest, super::auth::ModifyMembersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/ModifyMembers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_GROUPS: ::grpcio::Method<super::auth::GetGroupsRequest, super::auth::GetGroupsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetGroups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_USERS: ::grpcio::Method<super::auth::GetUsersRequest, super::auth::GetUsersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetUsers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_AUTHENTICATION_CODE: ::grpcio::Method<super::auth::GetAuthenticationCodeRequest, super::auth::GetAuthenticationCodeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/auth.API/GetAuthenticationCode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ApiClient {
    client: ::grpcio::Client,
}

impl ApiClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ApiClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn activate_opt(&self, req: &super::auth::ActivateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::ActivateResponse> {
        self.client.unary_call(&METHOD_API_ACTIVATE, req, opt)
    }

    pub fn activate(&self, req: &super::auth::ActivateRequest) -> ::grpcio::Result<super::auth::ActivateResponse> {
        self.activate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn activate_async_opt(&self, req: &super::auth::ActivateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ActivateResponse>> {
        self.client.unary_call_async(&METHOD_API_ACTIVATE, req, opt)
    }

    pub fn activate_async(&self, req: &super::auth::ActivateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ActivateResponse>> {
        self.activate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deactivate_opt(&self, req: &super::auth::DeactivateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::DeactivateResponse> {
        self.client.unary_call(&METHOD_API_DEACTIVATE, req, opt)
    }

    pub fn deactivate(&self, req: &super::auth::DeactivateRequest) -> ::grpcio::Result<super::auth::DeactivateResponse> {
        self.deactivate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deactivate_async_opt(&self, req: &super::auth::DeactivateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::DeactivateResponse>> {
        self.client.unary_call_async(&METHOD_API_DEACTIVATE, req, opt)
    }

    pub fn deactivate_async(&self, req: &super::auth::DeactivateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::DeactivateResponse>> {
        self.deactivate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_configuration_opt(&self, req: &super::auth::GetConfigurationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetConfigurationResponse> {
        self.client.unary_call(&METHOD_API_GET_CONFIGURATION, req, opt)
    }

    pub fn get_configuration(&self, req: &super::auth::GetConfigurationRequest) -> ::grpcio::Result<super::auth::GetConfigurationResponse> {
        self.get_configuration_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_configuration_async_opt(&self, req: &super::auth::GetConfigurationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetConfigurationResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_CONFIGURATION, req, opt)
    }

    pub fn get_configuration_async(&self, req: &super::auth::GetConfigurationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetConfigurationResponse>> {
        self.get_configuration_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_configuration_opt(&self, req: &super::auth::SetConfigurationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::SetConfigurationResponse> {
        self.client.unary_call(&METHOD_API_SET_CONFIGURATION, req, opt)
    }

    pub fn set_configuration(&self, req: &super::auth::SetConfigurationRequest) -> ::grpcio::Result<super::auth::SetConfigurationResponse> {
        self.set_configuration_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_configuration_async_opt(&self, req: &super::auth::SetConfigurationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetConfigurationResponse>> {
        self.client.unary_call_async(&METHOD_API_SET_CONFIGURATION, req, opt)
    }

    pub fn set_configuration_async(&self, req: &super::auth::SetConfigurationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetConfigurationResponse>> {
        self.set_configuration_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_admins_opt(&self, req: &super::auth::GetAdminsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetAdminsResponse> {
        self.client.unary_call(&METHOD_API_GET_ADMINS, req, opt)
    }

    pub fn get_admins(&self, req: &super::auth::GetAdminsRequest) -> ::grpcio::Result<super::auth::GetAdminsResponse> {
        self.get_admins_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_admins_async_opt(&self, req: &super::auth::GetAdminsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetAdminsResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_ADMINS, req, opt)
    }

    pub fn get_admins_async(&self, req: &super::auth::GetAdminsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetAdminsResponse>> {
        self.get_admins_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_admins_opt(&self, req: &super::auth::ModifyAdminsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::ModifyAdminsResponse> {
        self.client.unary_call(&METHOD_API_MODIFY_ADMINS, req, opt)
    }

    pub fn modify_admins(&self, req: &super::auth::ModifyAdminsRequest) -> ::grpcio::Result<super::auth::ModifyAdminsResponse> {
        self.modify_admins_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_admins_async_opt(&self, req: &super::auth::ModifyAdminsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ModifyAdminsResponse>> {
        self.client.unary_call_async(&METHOD_API_MODIFY_ADMINS, req, opt)
    }

    pub fn modify_admins_async(&self, req: &super::auth::ModifyAdminsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ModifyAdminsResponse>> {
        self.modify_admins_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authenticate_opt(&self, req: &super::auth::AuthenticateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::AuthenticateResponse> {
        self.client.unary_call(&METHOD_API_AUTHENTICATE, req, opt)
    }

    pub fn authenticate(&self, req: &super::auth::AuthenticateRequest) -> ::grpcio::Result<super::auth::AuthenticateResponse> {
        self.authenticate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authenticate_async_opt(&self, req: &super::auth::AuthenticateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::AuthenticateResponse>> {
        self.client.unary_call_async(&METHOD_API_AUTHENTICATE, req, opt)
    }

    pub fn authenticate_async(&self, req: &super::auth::AuthenticateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::AuthenticateResponse>> {
        self.authenticate_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authorize_opt(&self, req: &super::auth::AuthorizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::AuthorizeResponse> {
        self.client.unary_call(&METHOD_API_AUTHORIZE, req, opt)
    }

    pub fn authorize(&self, req: &super::auth::AuthorizeRequest) -> ::grpcio::Result<super::auth::AuthorizeResponse> {
        self.authorize_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authorize_async_opt(&self, req: &super::auth::AuthorizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::AuthorizeResponse>> {
        self.client.unary_call_async(&METHOD_API_AUTHORIZE, req, opt)
    }

    pub fn authorize_async(&self, req: &super::auth::AuthorizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::AuthorizeResponse>> {
        self.authorize_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn who_am_i_opt(&self, req: &super::auth::WhoAmIRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::WhoAmIResponse> {
        self.client.unary_call(&METHOD_API_WHO_AM_I, req, opt)
    }

    pub fn who_am_i(&self, req: &super::auth::WhoAmIRequest) -> ::grpcio::Result<super::auth::WhoAmIResponse> {
        self.who_am_i_opt(req, ::grpcio::CallOption::default())
    }

    pub fn who_am_i_async_opt(&self, req: &super::auth::WhoAmIRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::WhoAmIResponse>> {
        self.client.unary_call_async(&METHOD_API_WHO_AM_I, req, opt)
    }

    pub fn who_am_i_async(&self, req: &super::auth::WhoAmIRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::WhoAmIResponse>> {
        self.who_am_i_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_scope_opt(&self, req: &super::auth::GetScopeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetScopeResponse> {
        self.client.unary_call(&METHOD_API_GET_SCOPE, req, opt)
    }

    pub fn get_scope(&self, req: &super::auth::GetScopeRequest) -> ::grpcio::Result<super::auth::GetScopeResponse> {
        self.get_scope_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_scope_async_opt(&self, req: &super::auth::GetScopeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetScopeResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_SCOPE, req, opt)
    }

    pub fn get_scope_async(&self, req: &super::auth::GetScopeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetScopeResponse>> {
        self.get_scope_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_scope_opt(&self, req: &super::auth::SetScopeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::SetScopeResponse> {
        self.client.unary_call(&METHOD_API_SET_SCOPE, req, opt)
    }

    pub fn set_scope(&self, req: &super::auth::SetScopeRequest) -> ::grpcio::Result<super::auth::SetScopeResponse> {
        self.set_scope_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_scope_async_opt(&self, req: &super::auth::SetScopeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetScopeResponse>> {
        self.client.unary_call_async(&METHOD_API_SET_SCOPE, req, opt)
    }

    pub fn set_scope_async(&self, req: &super::auth::SetScopeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetScopeResponse>> {
        self.set_scope_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_acl_opt(&self, req: &super::auth::GetACLRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetACLResponse> {
        self.client.unary_call(&METHOD_API_GET_ACL, req, opt)
    }

    pub fn get_acl(&self, req: &super::auth::GetACLRequest) -> ::grpcio::Result<super::auth::GetACLResponse> {
        self.get_acl_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_acl_async_opt(&self, req: &super::auth::GetACLRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetACLResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_ACL, req, opt)
    }

    pub fn get_acl_async(&self, req: &super::auth::GetACLRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetACLResponse>> {
        self.get_acl_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_acl_opt(&self, req: &super::auth::SetACLRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::SetACLResponse> {
        self.client.unary_call(&METHOD_API_SET_ACL, req, opt)
    }

    pub fn set_acl(&self, req: &super::auth::SetACLRequest) -> ::grpcio::Result<super::auth::SetACLResponse> {
        self.set_acl_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_acl_async_opt(&self, req: &super::auth::SetACLRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetACLResponse>> {
        self.client.unary_call_async(&METHOD_API_SET_ACL, req, opt)
    }

    pub fn set_acl_async(&self, req: &super::auth::SetACLRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetACLResponse>> {
        self.set_acl_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_auth_token_opt(&self, req: &super::auth::GetAuthTokenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetAuthTokenResponse> {
        self.client.unary_call(&METHOD_API_GET_AUTH_TOKEN, req, opt)
    }

    pub fn get_auth_token(&self, req: &super::auth::GetAuthTokenRequest) -> ::grpcio::Result<super::auth::GetAuthTokenResponse> {
        self.get_auth_token_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_auth_token_async_opt(&self, req: &super::auth::GetAuthTokenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetAuthTokenResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_AUTH_TOKEN, req, opt)
    }

    pub fn get_auth_token_async(&self, req: &super::auth::GetAuthTokenRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetAuthTokenResponse>> {
        self.get_auth_token_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn extend_auth_token_opt(&self, req: &super::auth::ExtendAuthTokenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::ExtendAuthTokenResponse> {
        self.client.unary_call(&METHOD_API_EXTEND_AUTH_TOKEN, req, opt)
    }

    pub fn extend_auth_token(&self, req: &super::auth::ExtendAuthTokenRequest) -> ::grpcio::Result<super::auth::ExtendAuthTokenResponse> {
        self.extend_auth_token_opt(req, ::grpcio::CallOption::default())
    }

    pub fn extend_auth_token_async_opt(&self, req: &super::auth::ExtendAuthTokenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ExtendAuthTokenResponse>> {
        self.client.unary_call_async(&METHOD_API_EXTEND_AUTH_TOKEN, req, opt)
    }

    pub fn extend_auth_token_async(&self, req: &super::auth::ExtendAuthTokenRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ExtendAuthTokenResponse>> {
        self.extend_auth_token_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn revoke_auth_token_opt(&self, req: &super::auth::RevokeAuthTokenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::RevokeAuthTokenResponse> {
        self.client.unary_call(&METHOD_API_REVOKE_AUTH_TOKEN, req, opt)
    }

    pub fn revoke_auth_token(&self, req: &super::auth::RevokeAuthTokenRequest) -> ::grpcio::Result<super::auth::RevokeAuthTokenResponse> {
        self.revoke_auth_token_opt(req, ::grpcio::CallOption::default())
    }

    pub fn revoke_auth_token_async_opt(&self, req: &super::auth::RevokeAuthTokenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::RevokeAuthTokenResponse>> {
        self.client.unary_call_async(&METHOD_API_REVOKE_AUTH_TOKEN, req, opt)
    }

    pub fn revoke_auth_token_async(&self, req: &super::auth::RevokeAuthTokenRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::RevokeAuthTokenResponse>> {
        self.revoke_auth_token_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_groups_for_user_opt(&self, req: &super::auth::SetGroupsForUserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::SetGroupsForUserResponse> {
        self.client.unary_call(&METHOD_API_SET_GROUPS_FOR_USER, req, opt)
    }

    pub fn set_groups_for_user(&self, req: &super::auth::SetGroupsForUserRequest) -> ::grpcio::Result<super::auth::SetGroupsForUserResponse> {
        self.set_groups_for_user_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_groups_for_user_async_opt(&self, req: &super::auth::SetGroupsForUserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetGroupsForUserResponse>> {
        self.client.unary_call_async(&METHOD_API_SET_GROUPS_FOR_USER, req, opt)
    }

    pub fn set_groups_for_user_async(&self, req: &super::auth::SetGroupsForUserRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::SetGroupsForUserResponse>> {
        self.set_groups_for_user_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_members_opt(&self, req: &super::auth::ModifyMembersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::ModifyMembersResponse> {
        self.client.unary_call(&METHOD_API_MODIFY_MEMBERS, req, opt)
    }

    pub fn modify_members(&self, req: &super::auth::ModifyMembersRequest) -> ::grpcio::Result<super::auth::ModifyMembersResponse> {
        self.modify_members_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_members_async_opt(&self, req: &super::auth::ModifyMembersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ModifyMembersResponse>> {
        self.client.unary_call_async(&METHOD_API_MODIFY_MEMBERS, req, opt)
    }

    pub fn modify_members_async(&self, req: &super::auth::ModifyMembersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::ModifyMembersResponse>> {
        self.modify_members_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_groups_opt(&self, req: &super::auth::GetGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetGroupsResponse> {
        self.client.unary_call(&METHOD_API_GET_GROUPS, req, opt)
    }

    pub fn get_groups(&self, req: &super::auth::GetGroupsRequest) -> ::grpcio::Result<super::auth::GetGroupsResponse> {
        self.get_groups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_groups_async_opt(&self, req: &super::auth::GetGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetGroupsResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_GROUPS, req, opt)
    }

    pub fn get_groups_async(&self, req: &super::auth::GetGroupsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetGroupsResponse>> {
        self.get_groups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_users_opt(&self, req: &super::auth::GetUsersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetUsersResponse> {
        self.client.unary_call(&METHOD_API_GET_USERS, req, opt)
    }

    pub fn get_users(&self, req: &super::auth::GetUsersRequest) -> ::grpcio::Result<super::auth::GetUsersResponse> {
        self.get_users_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_users_async_opt(&self, req: &super::auth::GetUsersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetUsersResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_USERS, req, opt)
    }

    pub fn get_users_async(&self, req: &super::auth::GetUsersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetUsersResponse>> {
        self.get_users_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_authentication_code_opt(&self, req: &super::auth::GetAuthenticationCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::auth::GetAuthenticationCodeResponse> {
        self.client.unary_call(&METHOD_API_GET_AUTHENTICATION_CODE, req, opt)
    }

    pub fn get_authentication_code(&self, req: &super::auth::GetAuthenticationCodeRequest) -> ::grpcio::Result<super::auth::GetAuthenticationCodeResponse> {
        self.get_authentication_code_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_authentication_code_async_opt(&self, req: &super::auth::GetAuthenticationCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetAuthenticationCodeResponse>> {
        self.client.unary_call_async(&METHOD_API_GET_AUTHENTICATION_CODE, req, opt)
    }

    pub fn get_authentication_code_async(&self, req: &super::auth::GetAuthenticationCodeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::auth::GetAuthenticationCodeResponse>> {
        self.get_authentication_code_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Api {
    fn activate(&self, ctx: ::grpcio::RpcContext, req: super::auth::ActivateRequest, sink: ::grpcio::UnarySink<super::auth::ActivateResponse>);
    fn deactivate(&self, ctx: ::grpcio::RpcContext, req: super::auth::DeactivateRequest, sink: ::grpcio::UnarySink<super::auth::DeactivateResponse>);
    fn get_configuration(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetConfigurationRequest, sink: ::grpcio::UnarySink<super::auth::GetConfigurationResponse>);
    fn set_configuration(&self, ctx: ::grpcio::RpcContext, req: super::auth::SetConfigurationRequest, sink: ::grpcio::UnarySink<super::auth::SetConfigurationResponse>);
    fn get_admins(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetAdminsRequest, sink: ::grpcio::UnarySink<super::auth::GetAdminsResponse>);
    fn modify_admins(&self, ctx: ::grpcio::RpcContext, req: super::auth::ModifyAdminsRequest, sink: ::grpcio::UnarySink<super::auth::ModifyAdminsResponse>);
    fn authenticate(&self, ctx: ::grpcio::RpcContext, req: super::auth::AuthenticateRequest, sink: ::grpcio::UnarySink<super::auth::AuthenticateResponse>);
    fn authorize(&self, ctx: ::grpcio::RpcContext, req: super::auth::AuthorizeRequest, sink: ::grpcio::UnarySink<super::auth::AuthorizeResponse>);
    fn who_am_i(&self, ctx: ::grpcio::RpcContext, req: super::auth::WhoAmIRequest, sink: ::grpcio::UnarySink<super::auth::WhoAmIResponse>);
    fn get_scope(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetScopeRequest, sink: ::grpcio::UnarySink<super::auth::GetScopeResponse>);
    fn set_scope(&self, ctx: ::grpcio::RpcContext, req: super::auth::SetScopeRequest, sink: ::grpcio::UnarySink<super::auth::SetScopeResponse>);
    fn get_acl(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetACLRequest, sink: ::grpcio::UnarySink<super::auth::GetACLResponse>);
    fn set_acl(&self, ctx: ::grpcio::RpcContext, req: super::auth::SetACLRequest, sink: ::grpcio::UnarySink<super::auth::SetACLResponse>);
    fn get_auth_token(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetAuthTokenRequest, sink: ::grpcio::UnarySink<super::auth::GetAuthTokenResponse>);
    fn extend_auth_token(&self, ctx: ::grpcio::RpcContext, req: super::auth::ExtendAuthTokenRequest, sink: ::grpcio::UnarySink<super::auth::ExtendAuthTokenResponse>);
    fn revoke_auth_token(&self, ctx: ::grpcio::RpcContext, req: super::auth::RevokeAuthTokenRequest, sink: ::grpcio::UnarySink<super::auth::RevokeAuthTokenResponse>);
    fn set_groups_for_user(&self, ctx: ::grpcio::RpcContext, req: super::auth::SetGroupsForUserRequest, sink: ::grpcio::UnarySink<super::auth::SetGroupsForUserResponse>);
    fn modify_members(&self, ctx: ::grpcio::RpcContext, req: super::auth::ModifyMembersRequest, sink: ::grpcio::UnarySink<super::auth::ModifyMembersResponse>);
    fn get_groups(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetGroupsRequest, sink: ::grpcio::UnarySink<super::auth::GetGroupsResponse>);
    fn get_users(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetUsersRequest, sink: ::grpcio::UnarySink<super::auth::GetUsersResponse>);
    fn get_authentication_code(&self, ctx: ::grpcio::RpcContext, req: super::auth::GetAuthenticationCodeRequest, sink: ::grpcio::UnarySink<super::auth::GetAuthenticationCodeResponse>);
}

pub fn create_api<S: Api + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_ACTIVATE, move |ctx, req, resp| {
        instance.activate(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DEACTIVATE, move |ctx, req, resp| {
        instance.deactivate(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_CONFIGURATION, move |ctx, req, resp| {
        instance.get_configuration(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_SET_CONFIGURATION, move |ctx, req, resp| {
        instance.set_configuration(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_ADMINS, move |ctx, req, resp| {
        instance.get_admins(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_MODIFY_ADMINS, move |ctx, req, resp| {
        instance.modify_admins(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_AUTHENTICATE, move |ctx, req, resp| {
        instance.authenticate(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_AUTHORIZE, move |ctx, req, resp| {
        instance.authorize(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_WHO_AM_I, move |ctx, req, resp| {
        instance.who_am_i(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_SCOPE, move |ctx, req, resp| {
        instance.get_scope(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_SET_SCOPE, move |ctx, req, resp| {
        instance.set_scope(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_ACL, move |ctx, req, resp| {
        instance.get_acl(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_SET_ACL, move |ctx, req, resp| {
        instance.set_acl(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_AUTH_TOKEN, move |ctx, req, resp| {
        instance.get_auth_token(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_EXTEND_AUTH_TOKEN, move |ctx, req, resp| {
        instance.extend_auth_token(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_REVOKE_AUTH_TOKEN, move |ctx, req, resp| {
        instance.revoke_auth_token(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_SET_GROUPS_FOR_USER, move |ctx, req, resp| {
        instance.set_groups_for_user(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_MODIFY_MEMBERS, move |ctx, req, resp| {
        instance.modify_members(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_GROUPS, move |ctx, req, resp| {
        instance.get_groups(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_USERS, move |ctx, req, resp| {
        instance.get_users(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GET_AUTHENTICATION_CODE, move |ctx, req, resp| {
        instance.get_authentication_code(ctx, req, resp)
    });
    builder.build()
}
