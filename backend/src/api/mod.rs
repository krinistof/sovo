mod dbops;
mod validators;

use actix_web::{
    get,
    //post,
    web,
    Responder,
    HttpResponse,
};
use bson::{
    doc,
    Array,
    Document,
    oid::ObjectId
};
use mongodb::Client;
use serde::{Serialize, Deserialize};
use dbops::{
    add_propose,
    get_queue,
    process_vote,
    create_session,
    sort_by_rank,
    add_party,
    pop_popular_song,
    toggle_live
};
use futures::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "_id")]
    id: ObjectId,
    artist: String,
    title: String,
    duration: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
    #[serde(rename = "_id")]
    id: String,
    queue: Array,
    #[serde(rename = "isLive")]
    is_live: bool,
    #[serde(rename = "currentSong")]
    current_song: ObjectId,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartyRequest {
    partyid: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteRequest {
    partyid: String,
    songid: String,
    sessionid: String,
    is_like: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposeRequest {
    partyid: String,
    songid: String,
    sessionid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueueRequest {
    partyid: String,
    sessionid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "_id")]
    id: String,
    joined: String,
}

/// Creates session id by adding current time to 'sessions' collection
/// returns the generated id, which the client will send with requests
#[get("/auth/get-session")]
pub async fn get_session(database: web::Data<Client>) -> impl Responder {
    let id = create_session(database)
        .await;

    match id {
        Ok(session) => HttpResponse::Ok().json(
            session.inserted_id.as_object_id().unwrap().to_hex()
        ),
        Err(_) => HttpResponse::InternalServerError().json( doc! {
            "error": "database error"
        })
    }
}

#[get("/queue")]
pub async fn show_queue(database: web::Data<Client>, request : web::Query<QueueRequest>) -> impl Responder {
    /*
    sessionid is needed to determine if the user has liked or disliked
    songs. Without it we wouldn't be able to prevent double voting but those
    tokens are secrets of clients, that's why when requesting the queue
    the tokens are shadowed.

    By the way hacking this is easy because requesting token has zero cost.
    With few lines of python one can generate loads of sessions without any
    limitations
    */
    log::debug!("{:?}", &request);

    let sessionid = &request.sessionid;
    let oid = ObjectId::parse_str(sessionid);

    //wrong input for sessionid
    if let Err(_) = oid {
        return HttpResponse::BadRequest().json(doc! {
            "error": "sessionid isn't MongoDB ObjectId"
        });
    }
    let session = oid.unwrap();

    let contains = database
        .database("sovo")
        .collection::<Session>("sessions")
        .find_one(doc!{"_id": session}, None)
        .await;

    //no such session id
    if let Ok(None) = contains {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such session id"
        });
    }

    //TODO validation

    let partyid = &request.partyid;
    let party = get_queue(database, session, partyid)
        .await;


    if let Err(_) = &party {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such party id"
        });
    }

    let party = party
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();

    let party = party.get(0);
    if let None = party {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such party"
        });
    }

    HttpResponse::Ok().json(party.unwrap())
}

#[get("/vote")]
pub async fn vote(
        database: web::Data<Client>,
    vote_request: web::Query<VoteRequest>)
   -> impl Responder {
    // validate input
    log::debug!("{:?}", &vote_request);

    let oid = ObjectId::parse_str(&vote_request.sessionid);

    // wrong input for sessionid
    if let Err(_) = oid {
        return HttpResponse::BadRequest().json(doc! {
            "error": "sessionid isn't MongoDB ObjectId"
        });
    }
    let session = oid.unwrap();

    let db = database
        .database("sovo");

    // is session active
    let exists = db
        .collection::<Document>("sessions")
        .find_one(doc! {"_id": session}, None)
        .await;

    if let Ok(None) = exists {
        return HttpResponse::BadRequest().json(doc! {
            "error": "No such session active"
        });
    }

    // non-format songid
    let song_oid = ObjectId::parse_str(&vote_request.songid);
    if let Err(_) = song_oid {
        return HttpResponse::BadRequest().json(doc! {
            "error": "songid isn't MongoDB ObjectId"
        });
    }
    let song_oid = song_oid.unwrap();


    /* TODO fix validation
    let parties = db
        .collection::<Party>("parties");

    // is party existing
    let party = parties
        .find_one(doc! {"_id": &vote_request.partyid},None)
        .await;

    if let Err(_) = &party {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such partyid",
        });
    }
    let party = party.unwrap();

    if let None = party {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such partyid"
        });
    }
    let party = party.unwrap();

    // no such song in queue
    if !&party.queue.iter().any(|elem|
        elem.as_document()
            .unwrap()
            .get("songid")
            .unwrap()
            .as_object_id()
            .unwrap()
            .to_hex()
            .eq(&vote_request.songid)
    ) {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such song with given songid in queue"
        });
    }
    */

    let vote_operation =
        process_vote(database,
                     session,
                     &vote_request.partyid,
                     song_oid,
                     vote_request.is_like)
        .await;

    if let Err(_) = vote_operation {
        return HttpResponse::BadRequest().json(doc! {
            "error": "database error while processing vote"
        });
    }

    //TODO: return new queue
    HttpResponse::Ok().json("OK")
}

#[get("/propose")]
pub async fn propose(
    database: web::Data<Client>,
    propose_request: web::Query<ProposeRequest>) -> impl Responder {
    log::debug!("{:?}", &propose_request);
    // TODO validate

    //partyid
    //sessionid
    //TODO songid via server request of meta.json
    //TODO no double promotion


    let session = ObjectId::parse_str(&propose_request.sessionid).unwrap();
    let partyid = &propose_request.partyid;
    let songid = ObjectId::parse_str(&propose_request.songid).unwrap();

    add_propose(database.clone(), session, partyid, songid)
        .await
        .expect("Database error");

    let parties =
    database.database("sovo")
        .collection::<Party>("parties");
    sort_by_rank(&parties, partyid)
        .await
        .expect("Database error");

    HttpResponse::Ok().finish()
}

#[get("/create-party")]
pub async fn create_party(
    database: web::Data<Client>,
    party_request: web::Query<PartyRequest>) -> impl Responder {
    log::debug!("{:?}", &party_request);
    // TODO validate

    let partyid = &party_request.partyid;
    let password = &party_request.password;

    add_party(database, &partyid, &password)
        .await
        .expect("Database error");

    HttpResponse::Ok().finish()
}

#[get("/next")]
pub async fn next(
    database: web::Data<Client>,
    next_request: web::Query<PartyRequest>) -> impl Responder {
    log::debug!("{:?}", &next_request);
    // TODO validate

    let partyid = &next_request.partyid;
    let password = &next_request.password;

    let party = pop_popular_song(database, &partyid, &password)
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap();

    let party = party.get(0);
    if let None = party {
        return HttpResponse::BadRequest().json(doc! {
            "error": "no such party"
        });
    }

    HttpResponse::Ok().json(party.unwrap())
}
#[get("/toggle")]
pub async fn toggle(
    database: web::Data<Client>,
    toggle_request: web::Query<PartyRequest>) -> impl Responder {
    log::debug!("{:?}", &toggle_request);
    // TODO validate

    let partyid = &toggle_request.partyid;
    let password= &toggle_request.password;

    toggle_live(database, partyid, password)
        .await
        .expect("Database error");

    HttpResponse::Ok().finish()
}
