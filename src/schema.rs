use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Request {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub upvotes: i32,
    pub upvoted: bool,
    pub status: String,
    pub description: String,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Comment {
    pub id: i32,
    pub id_request: i32,
    pub id_parent: Option<i32>,
    pub owner: String,
    pub content: String,
}

// NOTE: I had to manually implement Decode instead of deriving sqlx::Type
// because there's a bug with rustc where it can't properly resolve the lifetype
// of a type when there's also the same type but wrapped in Option<T>
// (i32 and Option<i32> in this case)
// https://github.com/launchbadge/sqlx/issues/1031
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Comment {
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let id_request = decoder.try_decode::<i32>()?;
        let id_parent = decoder.try_decode::<Option<i32>>()?;
        let owner = decoder.try_decode::<String>()?;
        let content = decoder.try_decode::<String>()?;
        ::std::result::Result::Ok(Comment {
            id,
            id_request,
            id_parent,
            owner,
            content,
        })
    }
}

impl ::sqlx::Type<::sqlx::Postgres> for Comment {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Comment")
    }
}
