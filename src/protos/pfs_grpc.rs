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

const METHOD_API_CREATE_REPO: ::grpcio::Method<super::pfs::CreateRepoRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/CreateRepo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_INSPECT_REPO: ::grpcio::Method<super::pfs::InspectRepoRequest, super::pfs::RepoInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/InspectRepo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_LIST_REPO: ::grpcio::Method<super::pfs::ListRepoRequest, super::pfs::ListRepoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/ListRepo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DELETE_REPO: ::grpcio::Method<super::pfs::DeleteRepoRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/DeleteRepo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_START_COMMIT: ::grpcio::Method<super::pfs::StartCommitRequest, super::pfs::Commit> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/StartCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_FINISH_COMMIT: ::grpcio::Method<super::pfs::FinishCommitRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/FinishCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_INSPECT_COMMIT: ::grpcio::Method<super::pfs::InspectCommitRequest, super::pfs::CommitInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/InspectCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_LIST_COMMIT: ::grpcio::Method<super::pfs::ListCommitRequest, super::pfs::CommitInfos> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/ListCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_LIST_COMMIT_STREAM: ::grpcio::Method<super::pfs::ListCommitRequest, super::pfs::CommitInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.API/ListCommitStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DELETE_COMMIT: ::grpcio::Method<super::pfs::DeleteCommitRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/DeleteCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_FLUSH_COMMIT: ::grpcio::Method<super::pfs::FlushCommitRequest, super::pfs::CommitInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.API/FlushCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_SUBSCRIBE_COMMIT: ::grpcio::Method<super::pfs::SubscribeCommitRequest, super::pfs::CommitInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.API/SubscribeCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_BUILD_COMMIT: ::grpcio::Method<super::pfs::BuildCommitRequest, super::pfs::Commit> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/BuildCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_CREATE_BRANCH: ::grpcio::Method<super::pfs::CreateBranchRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/CreateBranch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_INSPECT_BRANCH: ::grpcio::Method<super::pfs::InspectBranchRequest, super::pfs::BranchInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/InspectBranch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_LIST_BRANCH: ::grpcio::Method<super::pfs::ListBranchRequest, super::pfs::BranchInfos> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/ListBranch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DELETE_BRANCH: ::grpcio::Method<super::pfs::DeleteBranchRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/DeleteBranch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_PUT_FILE: ::grpcio::Method<super::pfs::PutFileRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/pfs.API/PutFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_COPY_FILE: ::grpcio::Method<super::pfs::CopyFileRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/CopyFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GET_FILE: ::grpcio::Method<super::pfs::GetFileRequest, super::wrappers::BytesValue> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.API/GetFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_INSPECT_FILE: ::grpcio::Method<super::pfs::InspectFileRequest, super::pfs::FileInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/InspectFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_LIST_FILE: ::grpcio::Method<super::pfs::ListFileRequest, super::pfs::FileInfos> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/ListFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_LIST_FILE_STREAM: ::grpcio::Method<super::pfs::ListFileRequest, super::pfs::FileInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.API/ListFileStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GLOB_FILE: ::grpcio::Method<super::pfs::GlobFileRequest, super::pfs::FileInfos> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/GlobFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_GLOB_FILE_STREAM: ::grpcio::Method<super::pfs::GlobFileRequest, super::pfs::FileInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.API/GlobFileStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DIFF_FILE: ::grpcio::Method<super::pfs::DiffFileRequest, super::pfs::DiffFileResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/DiffFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DELETE_FILE: ::grpcio::Method<super::pfs::DeleteFileRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/DeleteFile",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_API_DELETE_ALL: ::grpcio::Method<super::empty::Empty, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.API/DeleteAll",
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

    pub fn create_repo_opt(&self, req: &super::pfs::CreateRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_CREATE_REPO, req, opt)
    }

    pub fn create_repo(&self, req: &super::pfs::CreateRepoRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.create_repo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_repo_async_opt(&self, req: &super::pfs::CreateRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_CREATE_REPO, req, opt)
    }

    pub fn create_repo_async(&self, req: &super::pfs::CreateRepoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.create_repo_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_repo_opt(&self, req: &super::pfs::InspectRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::RepoInfo> {
        self.client.unary_call(&METHOD_API_INSPECT_REPO, req, opt)
    }

    pub fn inspect_repo(&self, req: &super::pfs::InspectRepoRequest) -> ::grpcio::Result<super::pfs::RepoInfo> {
        self.inspect_repo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_repo_async_opt(&self, req: &super::pfs::InspectRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::RepoInfo>> {
        self.client.unary_call_async(&METHOD_API_INSPECT_REPO, req, opt)
    }

    pub fn inspect_repo_async(&self, req: &super::pfs::InspectRepoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::RepoInfo>> {
        self.inspect_repo_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_repo_opt(&self, req: &super::pfs::ListRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::ListRepoResponse> {
        self.client.unary_call(&METHOD_API_LIST_REPO, req, opt)
    }

    pub fn list_repo(&self, req: &super::pfs::ListRepoRequest) -> ::grpcio::Result<super::pfs::ListRepoResponse> {
        self.list_repo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_repo_async_opt(&self, req: &super::pfs::ListRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::ListRepoResponse>> {
        self.client.unary_call_async(&METHOD_API_LIST_REPO, req, opt)
    }

    pub fn list_repo_async(&self, req: &super::pfs::ListRepoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::ListRepoResponse>> {
        self.list_repo_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_repo_opt(&self, req: &super::pfs::DeleteRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_DELETE_REPO, req, opt)
    }

    pub fn delete_repo(&self, req: &super::pfs::DeleteRepoRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_repo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_repo_async_opt(&self, req: &super::pfs::DeleteRepoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_DELETE_REPO, req, opt)
    }

    pub fn delete_repo_async(&self, req: &super::pfs::DeleteRepoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_repo_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_commit_opt(&self, req: &super::pfs::StartCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::Commit> {
        self.client.unary_call(&METHOD_API_START_COMMIT, req, opt)
    }

    pub fn start_commit(&self, req: &super::pfs::StartCommitRequest) -> ::grpcio::Result<super::pfs::Commit> {
        self.start_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_commit_async_opt(&self, req: &super::pfs::StartCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::Commit>> {
        self.client.unary_call_async(&METHOD_API_START_COMMIT, req, opt)
    }

    pub fn start_commit_async(&self, req: &super::pfs::StartCommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::Commit>> {
        self.start_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn finish_commit_opt(&self, req: &super::pfs::FinishCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_FINISH_COMMIT, req, opt)
    }

    pub fn finish_commit(&self, req: &super::pfs::FinishCommitRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.finish_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn finish_commit_async_opt(&self, req: &super::pfs::FinishCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_FINISH_COMMIT, req, opt)
    }

    pub fn finish_commit_async(&self, req: &super::pfs::FinishCommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.finish_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_commit_opt(&self, req: &super::pfs::InspectCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::CommitInfo> {
        self.client.unary_call(&METHOD_API_INSPECT_COMMIT, req, opt)
    }

    pub fn inspect_commit(&self, req: &super::pfs::InspectCommitRequest) -> ::grpcio::Result<super::pfs::CommitInfo> {
        self.inspect_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_commit_async_opt(&self, req: &super::pfs::InspectCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::CommitInfo>> {
        self.client.unary_call_async(&METHOD_API_INSPECT_COMMIT, req, opt)
    }

    pub fn inspect_commit_async(&self, req: &super::pfs::InspectCommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::CommitInfo>> {
        self.inspect_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_commit_opt(&self, req: &super::pfs::ListCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::CommitInfos> {
        self.client.unary_call(&METHOD_API_LIST_COMMIT, req, opt)
    }

    pub fn list_commit(&self, req: &super::pfs::ListCommitRequest) -> ::grpcio::Result<super::pfs::CommitInfos> {
        self.list_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_commit_async_opt(&self, req: &super::pfs::ListCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::CommitInfos>> {
        self.client.unary_call_async(&METHOD_API_LIST_COMMIT, req, opt)
    }

    pub fn list_commit_async(&self, req: &super::pfs::ListCommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::CommitInfos>> {
        self.list_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_commit_stream_opt(&self, req: &super::pfs::ListCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::CommitInfo>> {
        self.client.server_streaming(&METHOD_API_LIST_COMMIT_STREAM, req, opt)
    }

    pub fn list_commit_stream(&self, req: &super::pfs::ListCommitRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::CommitInfo>> {
        self.list_commit_stream_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_commit_opt(&self, req: &super::pfs::DeleteCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_DELETE_COMMIT, req, opt)
    }

    pub fn delete_commit(&self, req: &super::pfs::DeleteCommitRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_commit_async_opt(&self, req: &super::pfs::DeleteCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_DELETE_COMMIT, req, opt)
    }

    pub fn delete_commit_async(&self, req: &super::pfs::DeleteCommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn flush_commit_opt(&self, req: &super::pfs::FlushCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::CommitInfo>> {
        self.client.server_streaming(&METHOD_API_FLUSH_COMMIT, req, opt)
    }

    pub fn flush_commit(&self, req: &super::pfs::FlushCommitRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::CommitInfo>> {
        self.flush_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_commit_opt(&self, req: &super::pfs::SubscribeCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::CommitInfo>> {
        self.client.server_streaming(&METHOD_API_SUBSCRIBE_COMMIT, req, opt)
    }

    pub fn subscribe_commit(&self, req: &super::pfs::SubscribeCommitRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::CommitInfo>> {
        self.subscribe_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn build_commit_opt(&self, req: &super::pfs::BuildCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::Commit> {
        self.client.unary_call(&METHOD_API_BUILD_COMMIT, req, opt)
    }

    pub fn build_commit(&self, req: &super::pfs::BuildCommitRequest) -> ::grpcio::Result<super::pfs::Commit> {
        self.build_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn build_commit_async_opt(&self, req: &super::pfs::BuildCommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::Commit>> {
        self.client.unary_call_async(&METHOD_API_BUILD_COMMIT, req, opt)
    }

    pub fn build_commit_async(&self, req: &super::pfs::BuildCommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::Commit>> {
        self.build_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_branch_opt(&self, req: &super::pfs::CreateBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_CREATE_BRANCH, req, opt)
    }

    pub fn create_branch(&self, req: &super::pfs::CreateBranchRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.create_branch_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_branch_async_opt(&self, req: &super::pfs::CreateBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_CREATE_BRANCH, req, opt)
    }

    pub fn create_branch_async(&self, req: &super::pfs::CreateBranchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.create_branch_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_branch_opt(&self, req: &super::pfs::InspectBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::BranchInfo> {
        self.client.unary_call(&METHOD_API_INSPECT_BRANCH, req, opt)
    }

    pub fn inspect_branch(&self, req: &super::pfs::InspectBranchRequest) -> ::grpcio::Result<super::pfs::BranchInfo> {
        self.inspect_branch_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_branch_async_opt(&self, req: &super::pfs::InspectBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::BranchInfo>> {
        self.client.unary_call_async(&METHOD_API_INSPECT_BRANCH, req, opt)
    }

    pub fn inspect_branch_async(&self, req: &super::pfs::InspectBranchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::BranchInfo>> {
        self.inspect_branch_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_branch_opt(&self, req: &super::pfs::ListBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::BranchInfos> {
        self.client.unary_call(&METHOD_API_LIST_BRANCH, req, opt)
    }

    pub fn list_branch(&self, req: &super::pfs::ListBranchRequest) -> ::grpcio::Result<super::pfs::BranchInfos> {
        self.list_branch_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_branch_async_opt(&self, req: &super::pfs::ListBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::BranchInfos>> {
        self.client.unary_call_async(&METHOD_API_LIST_BRANCH, req, opt)
    }

    pub fn list_branch_async(&self, req: &super::pfs::ListBranchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::BranchInfos>> {
        self.list_branch_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_branch_opt(&self, req: &super::pfs::DeleteBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_DELETE_BRANCH, req, opt)
    }

    pub fn delete_branch(&self, req: &super::pfs::DeleteBranchRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_branch_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_branch_async_opt(&self, req: &super::pfs::DeleteBranchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_DELETE_BRANCH, req, opt)
    }

    pub fn delete_branch_async(&self, req: &super::pfs::DeleteBranchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_branch_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_file_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::pfs::PutFileRequest>, ::grpcio::ClientCStreamReceiver<super::empty::Empty>)> {
        self.client.client_streaming(&METHOD_API_PUT_FILE, opt)
    }

    pub fn put_file(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::pfs::PutFileRequest>, ::grpcio::ClientCStreamReceiver<super::empty::Empty>)> {
        self.put_file_opt(::grpcio::CallOption::default())
    }

    pub fn copy_file_opt(&self, req: &super::pfs::CopyFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_COPY_FILE, req, opt)
    }

    pub fn copy_file(&self, req: &super::pfs::CopyFileRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.copy_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn copy_file_async_opt(&self, req: &super::pfs::CopyFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_COPY_FILE, req, opt)
    }

    pub fn copy_file_async(&self, req: &super::pfs::CopyFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.copy_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_file_opt(&self, req: &super::pfs::GetFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.client.server_streaming(&METHOD_API_GET_FILE, req, opt)
    }

    pub fn get_file(&self, req: &super::pfs::GetFileRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.get_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_file_opt(&self, req: &super::pfs::InspectFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::FileInfo> {
        self.client.unary_call(&METHOD_API_INSPECT_FILE, req, opt)
    }

    pub fn inspect_file(&self, req: &super::pfs::InspectFileRequest) -> ::grpcio::Result<super::pfs::FileInfo> {
        self.inspect_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_file_async_opt(&self, req: &super::pfs::InspectFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::FileInfo>> {
        self.client.unary_call_async(&METHOD_API_INSPECT_FILE, req, opt)
    }

    pub fn inspect_file_async(&self, req: &super::pfs::InspectFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::FileInfo>> {
        self.inspect_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_file_opt(&self, req: &super::pfs::ListFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::FileInfos> {
        self.client.unary_call(&METHOD_API_LIST_FILE, req, opt)
    }

    pub fn list_file(&self, req: &super::pfs::ListFileRequest) -> ::grpcio::Result<super::pfs::FileInfos> {
        self.list_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_file_async_opt(&self, req: &super::pfs::ListFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::FileInfos>> {
        self.client.unary_call_async(&METHOD_API_LIST_FILE, req, opt)
    }

    pub fn list_file_async(&self, req: &super::pfs::ListFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::FileInfos>> {
        self.list_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_file_stream_opt(&self, req: &super::pfs::ListFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::FileInfo>> {
        self.client.server_streaming(&METHOD_API_LIST_FILE_STREAM, req, opt)
    }

    pub fn list_file_stream(&self, req: &super::pfs::ListFileRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::FileInfo>> {
        self.list_file_stream_opt(req, ::grpcio::CallOption::default())
    }

    pub fn glob_file_opt(&self, req: &super::pfs::GlobFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::FileInfos> {
        self.client.unary_call(&METHOD_API_GLOB_FILE, req, opt)
    }

    pub fn glob_file(&self, req: &super::pfs::GlobFileRequest) -> ::grpcio::Result<super::pfs::FileInfos> {
        self.glob_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn glob_file_async_opt(&self, req: &super::pfs::GlobFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::FileInfos>> {
        self.client.unary_call_async(&METHOD_API_GLOB_FILE, req, opt)
    }

    pub fn glob_file_async(&self, req: &super::pfs::GlobFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::FileInfos>> {
        self.glob_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn glob_file_stream_opt(&self, req: &super::pfs::GlobFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::FileInfo>> {
        self.client.server_streaming(&METHOD_API_GLOB_FILE_STREAM, req, opt)
    }

    pub fn glob_file_stream(&self, req: &super::pfs::GlobFileRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::FileInfo>> {
        self.glob_file_stream_opt(req, ::grpcio::CallOption::default())
    }

    pub fn diff_file_opt(&self, req: &super::pfs::DiffFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::DiffFileResponse> {
        self.client.unary_call(&METHOD_API_DIFF_FILE, req, opt)
    }

    pub fn diff_file(&self, req: &super::pfs::DiffFileRequest) -> ::grpcio::Result<super::pfs::DiffFileResponse> {
        self.diff_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn diff_file_async_opt(&self, req: &super::pfs::DiffFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::DiffFileResponse>> {
        self.client.unary_call_async(&METHOD_API_DIFF_FILE, req, opt)
    }

    pub fn diff_file_async(&self, req: &super::pfs::DiffFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::DiffFileResponse>> {
        self.diff_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_file_opt(&self, req: &super::pfs::DeleteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_DELETE_FILE, req, opt)
    }

    pub fn delete_file(&self, req: &super::pfs::DeleteFileRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_file_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_file_async_opt(&self, req: &super::pfs::DeleteFileRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_DELETE_FILE, req, opt)
    }

    pub fn delete_file_async(&self, req: &super::pfs::DeleteFileRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_file_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_all_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_API_DELETE_ALL, req, opt)
    }

    pub fn delete_all(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_all_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_all_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_API_DELETE_ALL, req, opt)
    }

    pub fn delete_all_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_all_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Api {
    fn create_repo(&self, ctx: ::grpcio::RpcContext, req: super::pfs::CreateRepoRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn inspect_repo(&self, ctx: ::grpcio::RpcContext, req: super::pfs::InspectRepoRequest, sink: ::grpcio::UnarySink<super::pfs::RepoInfo>);
    fn list_repo(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListRepoRequest, sink: ::grpcio::UnarySink<super::pfs::ListRepoResponse>);
    fn delete_repo(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DeleteRepoRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn start_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::StartCommitRequest, sink: ::grpcio::UnarySink<super::pfs::Commit>);
    fn finish_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::FinishCommitRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn inspect_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::InspectCommitRequest, sink: ::grpcio::UnarySink<super::pfs::CommitInfo>);
    fn list_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListCommitRequest, sink: ::grpcio::UnarySink<super::pfs::CommitInfos>);
    fn list_commit_stream(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListCommitRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::CommitInfo>);
    fn delete_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DeleteCommitRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn flush_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::FlushCommitRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::CommitInfo>);
    fn subscribe_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::SubscribeCommitRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::CommitInfo>);
    fn build_commit(&self, ctx: ::grpcio::RpcContext, req: super::pfs::BuildCommitRequest, sink: ::grpcio::UnarySink<super::pfs::Commit>);
    fn create_branch(&self, ctx: ::grpcio::RpcContext, req: super::pfs::CreateBranchRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn inspect_branch(&self, ctx: ::grpcio::RpcContext, req: super::pfs::InspectBranchRequest, sink: ::grpcio::UnarySink<super::pfs::BranchInfo>);
    fn list_branch(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListBranchRequest, sink: ::grpcio::UnarySink<super::pfs::BranchInfos>);
    fn delete_branch(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DeleteBranchRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn put_file(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::pfs::PutFileRequest>, sink: ::grpcio::ClientStreamingSink<super::empty::Empty>);
    fn copy_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::CopyFileRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn get_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::GetFileRequest, sink: ::grpcio::ServerStreamingSink<super::wrappers::BytesValue>);
    fn inspect_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::InspectFileRequest, sink: ::grpcio::UnarySink<super::pfs::FileInfo>);
    fn list_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListFileRequest, sink: ::grpcio::UnarySink<super::pfs::FileInfos>);
    fn list_file_stream(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListFileRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::FileInfo>);
    fn glob_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::GlobFileRequest, sink: ::grpcio::UnarySink<super::pfs::FileInfos>);
    fn glob_file_stream(&self, ctx: ::grpcio::RpcContext, req: super::pfs::GlobFileRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::FileInfo>);
    fn diff_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DiffFileRequest, sink: ::grpcio::UnarySink<super::pfs::DiffFileResponse>);
    fn delete_file(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DeleteFileRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn delete_all(&self, ctx: ::grpcio::RpcContext, req: super::empty::Empty, sink: ::grpcio::UnarySink<super::empty::Empty>);
}

pub fn create_api<S: Api + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_CREATE_REPO, move |ctx, req, resp| {
        instance.create_repo(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_INSPECT_REPO, move |ctx, req, resp| {
        instance.inspect_repo(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_LIST_REPO, move |ctx, req, resp| {
        instance.list_repo(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DELETE_REPO, move |ctx, req, resp| {
        instance.delete_repo(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_START_COMMIT, move |ctx, req, resp| {
        instance.start_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_FINISH_COMMIT, move |ctx, req, resp| {
        instance.finish_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_INSPECT_COMMIT, move |ctx, req, resp| {
        instance.inspect_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_LIST_COMMIT, move |ctx, req, resp| {
        instance.list_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_API_LIST_COMMIT_STREAM, move |ctx, req, resp| {
        instance.list_commit_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DELETE_COMMIT, move |ctx, req, resp| {
        instance.delete_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_API_FLUSH_COMMIT, move |ctx, req, resp| {
        instance.flush_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_API_SUBSCRIBE_COMMIT, move |ctx, req, resp| {
        instance.subscribe_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_BUILD_COMMIT, move |ctx, req, resp| {
        instance.build_commit(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_CREATE_BRANCH, move |ctx, req, resp| {
        instance.create_branch(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_INSPECT_BRANCH, move |ctx, req, resp| {
        instance.inspect_branch(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_LIST_BRANCH, move |ctx, req, resp| {
        instance.list_branch(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DELETE_BRANCH, move |ctx, req, resp| {
        instance.delete_branch(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_API_PUT_FILE, move |ctx, req, resp| {
        instance.put_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_COPY_FILE, move |ctx, req, resp| {
        instance.copy_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_API_GET_FILE, move |ctx, req, resp| {
        instance.get_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_INSPECT_FILE, move |ctx, req, resp| {
        instance.inspect_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_LIST_FILE, move |ctx, req, resp| {
        instance.list_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_API_LIST_FILE_STREAM, move |ctx, req, resp| {
        instance.list_file_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_GLOB_FILE, move |ctx, req, resp| {
        instance.glob_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_API_GLOB_FILE_STREAM, move |ctx, req, resp| {
        instance.glob_file_stream(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DIFF_FILE, move |ctx, req, resp| {
        instance.diff_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DELETE_FILE, move |ctx, req, resp| {
        instance.delete_file(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_API_DELETE_ALL, move |ctx, req, resp| {
        instance.delete_all(ctx, req, resp)
    });
    builder.build()
}

const METHOD_OBJECT_API_PUT_OBJECT: ::grpcio::Method<super::pfs::PutObjectRequest, super::pfs::Object> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/pfs.ObjectAPI/PutObject",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_PUT_OBJECT_SPLIT: ::grpcio::Method<super::pfs::PutObjectRequest, super::pfs::Objects> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/pfs.ObjectAPI/PutObjectSplit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_GET_OBJECT: ::grpcio::Method<super::pfs::Object, super::wrappers::BytesValue> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.ObjectAPI/GetObject",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_GET_OBJECTS: ::grpcio::Method<super::pfs::GetObjectsRequest, super::wrappers::BytesValue> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.ObjectAPI/GetObjects",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_TAG_OBJECT: ::grpcio::Method<super::pfs::TagObjectRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/TagObject",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_INSPECT_OBJECT: ::grpcio::Method<super::pfs::Object, super::pfs::ObjectInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/InspectObject",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_CHECK_OBJECT: ::grpcio::Method<super::pfs::CheckObjectRequest, super::pfs::CheckObjectResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/CheckObject",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_LIST_OBJECTS: ::grpcio::Method<super::pfs::ListObjectsRequest, super::pfs::Object> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.ObjectAPI/ListObjects",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_DELETE_OBJECTS: ::grpcio::Method<super::pfs::DeleteObjectsRequest, super::pfs::DeleteObjectsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/DeleteObjects",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_GET_TAG: ::grpcio::Method<super::pfs::Tag, super::wrappers::BytesValue> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.ObjectAPI/GetTag",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_INSPECT_TAG: ::grpcio::Method<super::pfs::Tag, super::pfs::ObjectInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/InspectTag",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_LIST_TAGS: ::grpcio::Method<super::pfs::ListTagsRequest, super::pfs::ListTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/pfs.ObjectAPI/ListTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_DELETE_TAGS: ::grpcio::Method<super::pfs::DeleteTagsRequest, super::pfs::DeleteTagsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/DeleteTags",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_OBJECT_API_COMPACT: ::grpcio::Method<super::empty::Empty, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/pfs.ObjectAPI/Compact",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ObjectApiClient {
    client: ::grpcio::Client,
}

impl ObjectApiClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ObjectApiClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn put_object_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::pfs::PutObjectRequest>, ::grpcio::ClientCStreamReceiver<super::pfs::Object>)> {
        self.client.client_streaming(&METHOD_OBJECT_API_PUT_OBJECT, opt)
    }

    pub fn put_object(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::pfs::PutObjectRequest>, ::grpcio::ClientCStreamReceiver<super::pfs::Object>)> {
        self.put_object_opt(::grpcio::CallOption::default())
    }

    pub fn put_object_split_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::pfs::PutObjectRequest>, ::grpcio::ClientCStreamReceiver<super::pfs::Objects>)> {
        self.client.client_streaming(&METHOD_OBJECT_API_PUT_OBJECT_SPLIT, opt)
    }

    pub fn put_object_split(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::pfs::PutObjectRequest>, ::grpcio::ClientCStreamReceiver<super::pfs::Objects>)> {
        self.put_object_split_opt(::grpcio::CallOption::default())
    }

    pub fn get_object_opt(&self, req: &super::pfs::Object, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.client.server_streaming(&METHOD_OBJECT_API_GET_OBJECT, req, opt)
    }

    pub fn get_object(&self, req: &super::pfs::Object) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.get_object_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_objects_opt(&self, req: &super::pfs::GetObjectsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.client.server_streaming(&METHOD_OBJECT_API_GET_OBJECTS, req, opt)
    }

    pub fn get_objects(&self, req: &super::pfs::GetObjectsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.get_objects_opt(req, ::grpcio::CallOption::default())
    }

    pub fn tag_object_opt(&self, req: &super::pfs::TagObjectRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_OBJECT_API_TAG_OBJECT, req, opt)
    }

    pub fn tag_object(&self, req: &super::pfs::TagObjectRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.tag_object_opt(req, ::grpcio::CallOption::default())
    }

    pub fn tag_object_async_opt(&self, req: &super::pfs::TagObjectRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_TAG_OBJECT, req, opt)
    }

    pub fn tag_object_async(&self, req: &super::pfs::TagObjectRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.tag_object_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_object_opt(&self, req: &super::pfs::Object, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::ObjectInfo> {
        self.client.unary_call(&METHOD_OBJECT_API_INSPECT_OBJECT, req, opt)
    }

    pub fn inspect_object(&self, req: &super::pfs::Object) -> ::grpcio::Result<super::pfs::ObjectInfo> {
        self.inspect_object_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_object_async_opt(&self, req: &super::pfs::Object, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::ObjectInfo>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_INSPECT_OBJECT, req, opt)
    }

    pub fn inspect_object_async(&self, req: &super::pfs::Object) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::ObjectInfo>> {
        self.inspect_object_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_object_opt(&self, req: &super::pfs::CheckObjectRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::CheckObjectResponse> {
        self.client.unary_call(&METHOD_OBJECT_API_CHECK_OBJECT, req, opt)
    }

    pub fn check_object(&self, req: &super::pfs::CheckObjectRequest) -> ::grpcio::Result<super::pfs::CheckObjectResponse> {
        self.check_object_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_object_async_opt(&self, req: &super::pfs::CheckObjectRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::CheckObjectResponse>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_CHECK_OBJECT, req, opt)
    }

    pub fn check_object_async(&self, req: &super::pfs::CheckObjectRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::CheckObjectResponse>> {
        self.check_object_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_objects_opt(&self, req: &super::pfs::ListObjectsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::Object>> {
        self.client.server_streaming(&METHOD_OBJECT_API_LIST_OBJECTS, req, opt)
    }

    pub fn list_objects(&self, req: &super::pfs::ListObjectsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::Object>> {
        self.list_objects_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_objects_opt(&self, req: &super::pfs::DeleteObjectsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::DeleteObjectsResponse> {
        self.client.unary_call(&METHOD_OBJECT_API_DELETE_OBJECTS, req, opt)
    }

    pub fn delete_objects(&self, req: &super::pfs::DeleteObjectsRequest) -> ::grpcio::Result<super::pfs::DeleteObjectsResponse> {
        self.delete_objects_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_objects_async_opt(&self, req: &super::pfs::DeleteObjectsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::DeleteObjectsResponse>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_DELETE_OBJECTS, req, opt)
    }

    pub fn delete_objects_async(&self, req: &super::pfs::DeleteObjectsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::DeleteObjectsResponse>> {
        self.delete_objects_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_tag_opt(&self, req: &super::pfs::Tag, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.client.server_streaming(&METHOD_OBJECT_API_GET_TAG, req, opt)
    }

    pub fn get_tag(&self, req: &super::pfs::Tag) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::wrappers::BytesValue>> {
        self.get_tag_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_tag_opt(&self, req: &super::pfs::Tag, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::ObjectInfo> {
        self.client.unary_call(&METHOD_OBJECT_API_INSPECT_TAG, req, opt)
    }

    pub fn inspect_tag(&self, req: &super::pfs::Tag) -> ::grpcio::Result<super::pfs::ObjectInfo> {
        self.inspect_tag_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inspect_tag_async_opt(&self, req: &super::pfs::Tag, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::ObjectInfo>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_INSPECT_TAG, req, opt)
    }

    pub fn inspect_tag_async(&self, req: &super::pfs::Tag) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::ObjectInfo>> {
        self.inspect_tag_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_tags_opt(&self, req: &super::pfs::ListTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::ListTagsResponse>> {
        self.client.server_streaming(&METHOD_OBJECT_API_LIST_TAGS, req, opt)
    }

    pub fn list_tags(&self, req: &super::pfs::ListTagsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::pfs::ListTagsResponse>> {
        self.list_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_tags_opt(&self, req: &super::pfs::DeleteTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::pfs::DeleteTagsResponse> {
        self.client.unary_call(&METHOD_OBJECT_API_DELETE_TAGS, req, opt)
    }

    pub fn delete_tags(&self, req: &super::pfs::DeleteTagsRequest) -> ::grpcio::Result<super::pfs::DeleteTagsResponse> {
        self.delete_tags_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_tags_async_opt(&self, req: &super::pfs::DeleteTagsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::DeleteTagsResponse>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_DELETE_TAGS, req, opt)
    }

    pub fn delete_tags_async(&self, req: &super::pfs::DeleteTagsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::pfs::DeleteTagsResponse>> {
        self.delete_tags_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_OBJECT_API_COMPACT, req, opt)
    }

    pub fn compact(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_OBJECT_API_COMPACT, req, opt)
    }

    pub fn compact_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ObjectApi {
    fn put_object(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::pfs::PutObjectRequest>, sink: ::grpcio::ClientStreamingSink<super::pfs::Object>);
    fn put_object_split(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::pfs::PutObjectRequest>, sink: ::grpcio::ClientStreamingSink<super::pfs::Objects>);
    fn get_object(&self, ctx: ::grpcio::RpcContext, req: super::pfs::Object, sink: ::grpcio::ServerStreamingSink<super::wrappers::BytesValue>);
    fn get_objects(&self, ctx: ::grpcio::RpcContext, req: super::pfs::GetObjectsRequest, sink: ::grpcio::ServerStreamingSink<super::wrappers::BytesValue>);
    fn tag_object(&self, ctx: ::grpcio::RpcContext, req: super::pfs::TagObjectRequest, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn inspect_object(&self, ctx: ::grpcio::RpcContext, req: super::pfs::Object, sink: ::grpcio::UnarySink<super::pfs::ObjectInfo>);
    fn check_object(&self, ctx: ::grpcio::RpcContext, req: super::pfs::CheckObjectRequest, sink: ::grpcio::UnarySink<super::pfs::CheckObjectResponse>);
    fn list_objects(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListObjectsRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::Object>);
    fn delete_objects(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DeleteObjectsRequest, sink: ::grpcio::UnarySink<super::pfs::DeleteObjectsResponse>);
    fn get_tag(&self, ctx: ::grpcio::RpcContext, req: super::pfs::Tag, sink: ::grpcio::ServerStreamingSink<super::wrappers::BytesValue>);
    fn inspect_tag(&self, ctx: ::grpcio::RpcContext, req: super::pfs::Tag, sink: ::grpcio::UnarySink<super::pfs::ObjectInfo>);
    fn list_tags(&self, ctx: ::grpcio::RpcContext, req: super::pfs::ListTagsRequest, sink: ::grpcio::ServerStreamingSink<super::pfs::ListTagsResponse>);
    fn delete_tags(&self, ctx: ::grpcio::RpcContext, req: super::pfs::DeleteTagsRequest, sink: ::grpcio::UnarySink<super::pfs::DeleteTagsResponse>);
    fn compact(&self, ctx: ::grpcio::RpcContext, req: super::empty::Empty, sink: ::grpcio::UnarySink<super::empty::Empty>);
}

pub fn create_object_api<S: ObjectApi + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_OBJECT_API_PUT_OBJECT, move |ctx, req, resp| {
        instance.put_object(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_OBJECT_API_PUT_OBJECT_SPLIT, move |ctx, req, resp| {
        instance.put_object_split(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_OBJECT_API_GET_OBJECT, move |ctx, req, resp| {
        instance.get_object(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_OBJECT_API_GET_OBJECTS, move |ctx, req, resp| {
        instance.get_objects(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_TAG_OBJECT, move |ctx, req, resp| {
        instance.tag_object(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_INSPECT_OBJECT, move |ctx, req, resp| {
        instance.inspect_object(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_CHECK_OBJECT, move |ctx, req, resp| {
        instance.check_object(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_OBJECT_API_LIST_OBJECTS, move |ctx, req, resp| {
        instance.list_objects(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_DELETE_OBJECTS, move |ctx, req, resp| {
        instance.delete_objects(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_OBJECT_API_GET_TAG, move |ctx, req, resp| {
        instance.get_tag(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_INSPECT_TAG, move |ctx, req, resp| {
        instance.inspect_tag(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_OBJECT_API_LIST_TAGS, move |ctx, req, resp| {
        instance.list_tags(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_DELETE_TAGS, move |ctx, req, resp| {
        instance.delete_tags(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_OBJECT_API_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    builder.build()
}
