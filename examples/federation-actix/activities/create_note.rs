use crate::{
    instance::DatabaseHandle,
    objects::{note::Note, person::MyUser},
    MyPost,
};
use activitypub_federation::{
    core::object_id::ObjectId,
    deser::helpers::deserialize_one_or_many,
    request_data::RequestData,
    traits::{ActivityHandler, ApubObject},
};
use activitystreams_kinds::activity::CreateType;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateNote {
    pub(crate) actor: ObjectId<MyUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    pub(crate) object: Note,
    #[serde(rename = "type")]
    pub(crate) kind: CreateType,
    pub(crate) id: Url,
}

impl CreateNote {
    pub fn new(note: Note, id: Url) -> CreateNote {
        CreateNote {
            actor: note.attributed_to.clone(),
            to: note.to.clone(),
            object: note,
            kind: CreateType::Create,
            id,
        }
    }
}

#[async_trait::async_trait]
impl ActivityHandler for CreateNote {
    type DataType = DatabaseHandle;
    type Error = crate::error::Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn receive(self, data: &RequestData<Self::DataType>) -> Result<(), Self::Error> {
        MyPost::from_apub(self.object, data).await?;
        Ok(())
    }
}
