use diesel::{dsl::{count_star, CountStar}, prelude::*, query_dsl::LoadQuery, query_dsl::{
    methods::{LimitDsl, OffsetDsl},
    select_dsl::SelectDsl,
}, debug_query};
use diesel::backend::Backend;
use diesel::query_builder::QueryFragment;
use log::{debug};

const DEFAULT_PER_PAGE: i64 = 10;
pub const DEFAULT_PAGE: i64 = 1;

pub struct Paginator<U> {
    pub(crate) data: Vec<U>,
    pub(crate) total: i64,
    pub(crate) per_page: i64,
    pub(crate) current_page: i64,
    #[allow(dead_code)]
    pub(crate) last_page: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Paginate<F> {
    query_maker: F,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<F, T> Paginate<F>
    where
        F: Fn() -> T,
        T: LimitDsl + SelectDsl<CountStar>,
        <T as LimitDsl>::Output: OffsetDsl,
        <T as SelectDsl<CountStar>>::Output: LimitDsl,
{
    pub fn new(f: F, page: Option<i64>) -> Paginate<F> {
        let page = match page {
            Some(0) | None => DEFAULT_PAGE,
            Some(page) => page,
        };

        Self {
            query_maker: f,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }

    pub fn per_page(self, per_page: Option<i64>) -> Self {
        let per_page = match per_page {
            Some(0) | None => DEFAULT_PER_PAGE,
            Some(per_page) => per_page,
        };

        Self {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }

    pub fn load_with_total<'a, U, Conn>(self, conn: &mut Conn) -> QueryResult<(Vec<U>, i64)>
        where
            Conn: Connection,
            <Conn as Connection>::Backend: Default,
            <<Conn as Connection>::Backend as Backend>::QueryBuilder: Default,
            <<T as LimitDsl>::Output as OffsetDsl>::Output: LoadQuery<'a, Conn, U> + QueryFragment<Conn::Backend>,
            <T as SelectDsl<CountStar>>::Output: RunQueryDsl<Conn> + QueryFragment<Conn::Backend>,
            <<T as SelectDsl<CountStar>>::Output as LimitDsl>::Output: LoadQuery<'a, Conn, i64>,
    {
        let result_query = (self.query_maker)()
            .limit(self.per_page)
            .offset(self.offset);

        debug!("Executing Query: {}",debug_query(&result_query));

        let results = result_query
            .load(conn)?;

        let tatol_query = (self.query_maker)()
            .select(count_star());

        debug!("Executing Query: {}",debug_query(&tatol_query));

        let total = tatol_query
            .first::<i64>(conn)?;

        Ok((results, total))
    }

    pub fn load_with_paginator<'a, U, Conn>(self, conn: &mut Conn) -> QueryResult<Paginator<U>>
        where
            Conn: Connection,
            <Conn as Connection>::Backend: Default,
            <<Conn as Connection>::Backend as Backend>::QueryBuilder: Default,
            <<T as LimitDsl>::Output as OffsetDsl>::Output: LoadQuery<'a, Conn, U> + QueryFragment<Conn::Backend>,
            <T as SelectDsl<CountStar>>::Output: RunQueryDsl<Conn> + QueryFragment<Conn::Backend>,
            <<T as SelectDsl<CountStar>>::Output as LimitDsl>::Output: LoadQuery<'a, Conn, i64>,
    {
        let current_page = self.page;
        let per_page = self.per_page;

        let (data, total) = self.load_with_total::<U, Conn>(conn)?;

        Ok(Paginator {
            data,
            total,
            per_page,
            current_page,
            last_page: (total as f64 / per_page as f64).ceil() as i64,
        })
    }
}
