use self::nepnep::{
    nepnep_server::{Nepnep, NepnepServer},
    Profiles, Void, ProfileName,
};
use crate::daemon::{interactions::grpc::nepnep::Profile, screenlock::{self, ScreenlockProfile}};
use tonic::{transport::Server, Request, Response, Status};

mod nepnep {
    tonic::include_proto!("neptune");
}

pub async fn grpc_init(bind_addr: String) {
    Server::builder()
        .add_service(NepnepServer::new(Grpc))
        .serve(bind_addr.parse().unwrap())
        .await
        .expect("Fudeu bahia");
}

struct Grpc;    
#[tonic::async_trait]
impl Nepnep for Grpc {
    async fn screen_block_start(&self, _request: Request<Void>) -> Result<Response<Void>, Status> {
        screenlock::block_screen(None).await;
        Ok(Response::new(Void {}))
    }

    async fn get_screen_block_profiles(&self, _request: Request<Void>) -> Result<Response<Profiles>, Status> {
        let profiles = screenlock::PROFILES
            .lock()
            .await
            .iter()
            .map(|x| Profile {
                profile_name: x.profile_name.clone(),
                images: x.images.clone(),
                block_input: x.block_input,
                windowed: x.windowed,
            })
            .collect::<Vec<_>>();

        Ok(Response::new(Profiles { profiles }))
    }

    async fn screen_block_end(&self, _request: Request<Void>) -> Result<Response<Void>, Status> {
        screenlock::kill_screen_block().await;
        Ok(Response::new(Void {}))
    }

    async fn screen_block_start_with_custom_profile(&self, request: Request<Profile>) -> Result<Response<Void>, Status> {
        let profile = request.into_inner();
        screenlock::block_screen(Some(ScreenlockProfile {
            profile_name: profile.profile_name,
            images: profile.images,
            block_input: profile.block_input,
            windowed: profile.windowed,
            keys: Vec::new()
        })).await;
        Ok(Response::new(Void {}))
    }

    async fn screen_block_start_with_profile_name(&self, request: Request<ProfileName>) -> Result<Response<Void>, Status> {
        let name = request.into_inner();
        screenlock::block_screen(screenlock::get_profile_by_name(name.profile_name).await).await;
        Ok(Response::new(Void {}))
    }
}
