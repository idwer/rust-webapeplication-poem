use poem::handler;
use poem::IntoResponse;
use poem::Response;
use poem::http::StatusCode;
use poem::web::Json;
use validator::Validate;

use crate::lib::ape::ApeIndexInput;

#[handler]
pub async fn ape_post(apeIndexInput: Json<ApeIndexInput>) -> Response {
    let validated_input = apeIndexInput.validate();

    match validated_input {
        Ok(_) => {
            let input = ApeIndexInput {
                height: apeIndexInput.height,
                wingspan: apeIndexInput.wingspan,
            };

            let output = input.ape_index_from_json();
            
            Json(output).into_response()
       }

       Err(err) => {
           Response::builder()
               .status(StatusCode::BAD_REQUEST)
               .body(err.to_string())
       }
    }
}
