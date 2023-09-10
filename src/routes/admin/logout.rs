use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use crate::{utils::{see_other, e500}, session_state::TypedSession};

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(see_other("/login"))
    } else {
        session.log_out();
        FlashMessage::info("You have successfully logged out.").send();
        Ok(see_other("/login"))
    }
}